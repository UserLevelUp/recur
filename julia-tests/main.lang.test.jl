#!/usr/bin/env julia

using Test
using Random

include(joinpath(@__DIR__, "..", "demos", "main.lang", "main.lang.runtime.jl"))
using .MainLang

const PROGRAM = load_program()

@testset "main.lang parser and compact views" begin
    @test PROGRAM.class_name == "AlgorithmLab"
    @test PROGRAM.version == "0.1"
    @test PROGRAM.scopes["gcd"].flow == "i(a) -> f(a) -> o(b)"
    @test PROGRAM.flows["all"].mode == "async"
    @test occursin("await", PROGRAM.flows["all"].expression)
    @test MainLang.source_text(PROGRAM.scopes["gcd"].warp) ==
          "E0(demo.algorithm.gcd.todo.current) -> " *
          "dE(gcd.f) -> Ef(demo.algorithm.gcd.complete)"

    collapsed = describe(PROGRAM, "gcd.f")
    expanded = describe(PROGRAM, "gcd.f"; expand = true)
    scope_view = describe(PROGRAM, "gcd")
    @test scope_view["input"] == "i(a) := (left: Int, right: Int)"
    @test scope_view["output"] == "o(b) := (value: Int)"
    @test !haskey(collapsed, "expansion")
    @test occursin("while a.right != 0", expanded["expansion"])
    @test expanded["collapsed"] == "i(a) -> f(a) -> o(b)"

    gcd_input = describe(PROGRAM, "gcd.i(a)")
    pyramid_input = describe(PROGRAM, "AlgorithmLab.pyramid.i(a)")
    @test gcd_input["symbol"] == "gcd.i(a)"
    @test gcd_input["role"] == "input"
    @test describe(PROGRAM, "gcd.o(b)")["role"] == "output"
    @test [field["name"] for field in gcd_input["fields"]] == ["left", "right"]
    @test [field["name"] for field in pyramid_input["fields"]] == ["rows", "glyph"]
end

@testset "main.lang exact shared boundaries" begin
    produced = PROGRAM.scopes["bubble"].shapes["b"]
    consumed = PROGRAM.scopes["merge"].shapes["b"]
    @test produced.canonical_name == "bubble.o(b)"
    @test consumed.canonical_name == "bubble.o(b)"
    @test produced.fields == consumed.fields
    @test MainLang.source_text(only(PROGRAM.boundaries)) ==
          "bubble.o(b) -> merge.i(b)"
    @test describe(PROGRAM, "merge.i(b)")["definition"] ==
          "i(b) := bubble.o(b)"

    source = read(MainLang.SOURCE_PATH, String)
    disconnected = replace(
        source,
        "i(b) := bubble.o(b)" => "i(b) := (values: List<Int>)",
    )
    error = try
        parse_program(disconnected)
        nothing
    catch caught
        caught
    end
    @test error isa LanguageError
    @test occursin("does not resolve to one contract", sprint(showerror, error))
end

@testset "main.lang algorithms" begin
    gcd_result = execute_scope(
        PROGRAM,
        "gcd",
        Dict{String,Any}("left" => 1071, "right" => 462),
    )
    @test gcd_result["output"]["value"] == Dict{String,Any}("value" => 21)

    bubble_result = execute_scope(
        PROGRAM,
        "bubble",
        Dict{String,Any}("values" => [5, 1, 4, 2, 8]),
    )
    @test bubble_result["output"]["value"]["values"] == [1, 2, 4, 5, 8]

    merge_result = execute_scope(
        PROGRAM,
        "merge",
        Dict{String,Any}("values" => [9, -1, 3, 3, 7, 1, 4]),
    )
    @test merge_result["input"]["symbol"] == "b"
    @test merge_result["output"]["symbol"] == "c"
    @test merge_result["output"]["value"]["values"] == [-1, 1, 3, 3, 4, 7, 9]

    primes_result = execute_scope(
        PROGRAM,
        "primes",
        Dict{String,Any}("limit" => 30),
    )
    @test primes_result["output"]["value"]["values"] ==
          [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]

    pyramid_result = execute_scope(
        PROGRAM,
        "pyramid",
        Dict{String,Any}("rows" => 3, "glyph" => "*"),
    )
    @test pyramid_result["output"]["value"]["lines"] == ["  *", " ***", "*****"]
end

@testset "main.lang contracts and lanes" begin
    @test_throws LanguageError execute_scope(
        PROGRAM,
        "gcd",
        Dict{String,Any}("left" => "1071", "right" => 462),
    )

    result = execute_all(PROGRAM)
    lanes = result["output"]["value"]
    @test result["mode"] == "async"
    @test result["lane_order"] == ["gcd", "bubble", "merge", "primes", "pyramid"]
    @test lanes["gcd"]["output"]["value"]["value"] == 21
    @test last(lanes["pyramid"]["output"]["value"]["lines"]) == "*********"
end

@testset "main.lang randomized algorithm evidence" begin
    random = MersenneTwister(1729)
    for _ in 1:250
        left = rand(random, -100_000:100_000)
        right = rand(random, -100_000:100_000)
        gcd_result = execute_scope(
            PROGRAM,
            "gcd",
            Dict{String,Any}("left" => left, "right" => right),
        )
        @test gcd_result["output"]["value"]["value"] == gcd(left, right)

        values = rand(random, -1_000:1_000, rand(random, 0:50))
        expected = sort(values)
        for name in ("bubble", "merge")
            result = execute_scope(
                PROGRAM,
                name,
                Dict{String,Any}("values" => values),
            )
            @test result["output"]["value"]["values"] == expected
        end
    end
end

@testset "main.lang JSON surface" begin
    rendered = json_string(describe(PROGRAM, "merge.i(b)"))
    @test occursin("\"canonical\": \"bubble.o(b)\"", rendered)
    @test occursin("\"definition\": \"i(b) := bubble.o(b)\"", rendered)
end

@testset "main.lang CLI bindings" begin
    bindings = parse_bindings(PROGRAM, "gcd", ["left=1071", "right=462"])
    @test bindings == Dict{String,Any}("left" => 1071, "right" => 462)

    merge_bindings = parse_bindings(PROGRAM, "merge", ["values=9,3,7,1,4"])
    rendered = json_string(execute_scope(PROGRAM, "merge", merge_bindings))
    @test occursin("\"symbol\": \"b\"", rendered)
    @test occursin("\"values\"", rendered)
end

@testset "main.lang query and companion contract" begin
    command_doc = read(
        joinpath(@__DIR__, "..", "docs", "main.command.lang.readme.md"),
        String,
    )
    @test occursin(
        "recur lang   = query, validate, expand, contract, trace, explain, exit",
        command_doc,
    )
    @test occursin(
        "recur-lang   = execute confirmed declared action, write state and ACK/NAK",
        command_doc,
    )
    @test occursin("Expansion and contraction", command_doc)
    @test occursin(".recur/lang/recur-lang.<id>.status.current.md", command_doc)
    @test occursin("Open-world repository scan", command_doc)
    @test occursin("Closed-world formal model", command_doc)

    trace_doc = read(
        joinpath(@__DIR__, "..", "docs", "main.command.trace-id.readme.md"),
        String,
    )
    @test occursin("## Boundary With Recur Lang", trace_doc)
    @test occursin("Recur Lang is a closed-world coordination model", trace_doc)

    eventness_doc = read(
        joinpath(@__DIR__, "..", "README.CORE.EVENTNESS.md"),
        String,
    )
    @test occursin("## Scoped Completion and Subsystem Integration", eventness_doc)
    @test occursin("Child completion never implies parent completion", eventness_doc)

    improvement_doc = read(
        joinpath(@__DIR__, "..", "README.CORE.IMPROVEMENT30.md"),
        String,
    )
    @test occursin(
        "### Subsystem contraction and parent integration",
        improvement_doc,
    )
    @test occursin(r"contract version or\s+content hash", improvement_doc)
end

@testset "main.lang watch coordination design contract" begin
    source = read(
        joinpath(
            @__DIR__,
            "..",
            "demos",
            "main.lang",
            "main.lang.skippy-watch-coordination.recur",
        ),
        String,
    )
    specification = read(
        joinpath(
            @__DIR__,
            "..",
            "docs",
            "main.improvement.30.contract.watch-coordination-v0.todo.future-plan.md",
        ),
        String,
    )
    live_grid_cursor = read(
        joinpath(
            @__DIR__,
            "..",
            "docs",
            "main.improvement.30.live-grid.todo.tracking.md",
        ),
        String,
    )

    @test occursin("recur 0.2 coordination SkippyWorkshop", source)
    @test occursin("grid solution by phase", source)
    @test occursin("state watching initial", source)
    @test occursin("feedback repair", source)
    @test occursin(
        "collapse solution.coordination.current -> solution.coordination.complete",
        source,
    )
    @test occursin("--filter \"worker.**.current.md\"", source)
    @test !occursin("--filter \"worker.**.current\"", source)
    @test occursin("check self_subscription", source)
    @test occursin(
        "watcher process   = recur-watch remains subscribed and emits file events",
        specification,
    )
    @test occursin(
        "The current Julia `main.lang` parser does not yet accept this syntax.",
        specification,
    )
    @test occursin(
        "Neither `recur` nor `recur-lang` needs to contain `dotnet`, Node, Cargo, Git,",
        specification,
    )
    @test occursin("Status: `todo.tracking`", live_grid_cursor)
    @test occursin("Priority: `important / tracked`", live_grid_cursor)
    @test occursin(
        "live grid == snapshot grid == completed report",
        live_grid_cursor,
    )
end
