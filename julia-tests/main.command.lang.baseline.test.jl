using Test, JSON3

@testset "Recur Lang baseline pure CLI" begin
    binary = get(ENV, "RECUR_BIN", joinpath(@__DIR__, "..", "target", "release-safe", "recur" * (Sys.iswindows() ? ".exe" : "")))
    repo = abspath(joinpath(@__DIR__, ".."))
    algorithm = joinpath(repo, "demos", "main.lang", "main.lang.algorithm-lab.recur")
    coordination = joinpath(repo, "demos", "main.lang", "main.lang.skippy-watch-coordination.recur")
    function query(args...)
        out, err = IOBuffer(), IOBuffer()
        process = run(pipeline(ignorestatus(Cmd([binary, "lang", args..., "--json"])), stdout=out, stderr=err))
        (process.exitcode, String(take!(out)), String(take!(err)))
    end
    code, output, _ = query("show", algorithm, "--scope", "gcd.f", "-d", repo)
    @test code == 0
    if code == 0
        data = JSON3.read(output)
        @test data.schema == "recur-lang-query-v1"
        @test data.header[1].identity == "gcd.f"
        @test data.header[1].input.canonical_identity == "gcd.i(a)"
        @test data.footer.execution == "not-run"
        @test data.coverage.whole_source_validated == false
    end
    code, output, _ = query("report", coordination, "--scope", "review_bird", "--expand", "-d", repo)
    @test code == 0
    if code == 0
        data = JSON3.read(output)
        @test length(data.header) == 1
        @test length(data.header[1].input.messages) == 4
        @test data.footer.graph.orchestration_sound
        @test length(data.body.boundary_edges) > 0
    end
    code, output, _ = query("show", algorithm, "--scope", "does_not_exist", "-d", repo)
    @test code == 2
    @test occursin("LANG003", output)
    mktempdir() do root
        code, output, _ = query("list", "-d", root)
        @test code == 0
        if code == 0
            @test isempty(JSON3.read(output).sources)
        end
    end
end
