"""
Tests for future 'recur unflatten' command (IMPROVEMENT15)
===========================================================

These tests intentionally encode the frozen v1 contract before implementation.
They are marked with @test_broken so they remain visible roadmap debt until
`unflatten` and `merge --format flat` are implemented.
"""

include("runtests.setup.jl")

function run_recur_raw(args::Vector{String}; input::Union{String,Nothing}=nothing)
    display_cmd = join(map(arg -> contains(arg, ' ') ? "\"$arg\"" : arg, args), " ")
    println("  -> recur $display_cmd")

    cmd = `$RECUR_BIN $args`
    out = IOBuffer()
    err = IOBuffer()
    success = true

    try
        if input === nothing
            run(pipeline(cmd, stdout=out, stderr=err))
        else
            run(pipeline(cmd, stdin=IOBuffer(input), stdout=out, stderr=err))
        end
    catch e
        if isa(e, ProcessFailedException)
            success = false
        else
            return (false, "", "Error running command: $e")
        end
    end

    return (success, String(take!(out)), String(take!(err)))
end

@testset "recur unflatten command (IMPROVEMENT15, expected broken)" begin
    log_section("Testing: recur unflatten (expected broken)")

    created_here = false
    if !isdir(TEST_DIR)
        setup_test_environment()
        created_here = true
    end

    try

    @testset "CLI contract surface (broken until command exists)" begin
        success, output, _ = run_recur_raw(["unflatten", "--help"])

        @test_broken success
        @test_broken contains(output, "unflatten")
        @test_broken contains(output, "--format")
        @test_broken contains(output, "--profile")
        @test_broken contains(output, "--on-conflict")
        @test_broken contains(output, "--sort")

        log_test("unflatten CLI surface contract captured (broken)")
    end

    @testset "stdin flat->json reconstruction contract (broken)" begin
        flat_json = """
        [
          { "path": "config.db.host", "value": "localhost", "kind": "text" },
          { "path": "config.db.port", "value": "5432", "kind": "text" },
          { "path": "config.env", "value": "prod", "kind": "text" }
        ]
        """

        success, output, _ = run_recur_raw(
            ["unflatten", "--stdin", "--format", "json"],
            input=flat_json,
        )

        parsed = nothing
        parse_ok = false
        try
            parsed = JSON3.read(output)
            parse_ok = true
        catch
            parse_ok = false
        end

        @test_broken success
        @test_broken parse_ok
        @test_broken parse_ok && String(parsed["config"]["db"]["host"]) == "localhost"
        @test_broken parse_ok && String(parsed["config"]["db"]["port"]) == "5432"
        @test_broken parse_ok && String(parsed["config"]["env"]) == "prod"

        log_test("unflatten stdin->json contract captured (broken)")
    end

    @testset "merge flat output contract (broken until --format flat exists)" begin
        create_test_file("improvement.15.contract.readme.md", "# improvement 15")
        create_test_file("improvement_15_contract_impl.rs", "fn contract_impl() {}")

        success, output, _ = run_recur([
            "merge",
            "--pattern",
            "improvement.15.contract",
            "--sep",
            ".",
            "--pattern",
            "improvement_15_contract",
            "--sep",
            "_",
            "--base",
            "improvement.15.contract",
            "--format",
            "flat",
            "--json",
        ])

        parsed = nothing
        parse_ok = false
        try
            parsed = JSON3.read(output)
            parse_ok = true
        catch
            parse_ok = false
        end

        @test_broken success
        @test_broken parse_ok
        @test_broken parse_ok && length(parsed) >= 2
        @test_broken parse_ok && haskey(parsed[1], :path)
        @test_broken parse_ok && haskey(parsed[1], :kind)

        log_test("merge --format flat contract captured (broken)")
    end

    finally
        if created_here
            teardown_test_environment()
        end
    end
end
