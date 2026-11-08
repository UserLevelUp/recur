# Standalone characterization for the Improvement 34 proposal.
# Run with the existing Julia test environment; no demos or private inputs.
include("runtests.setup.jl")

@testset "Improvement 34 current behavior" begin
    @testset "Legacy trace discovery and scope" begin
        mktempdir() do root
            fixture = joinpath(root, "lab.trace.md")
            source = join([
                "defines: capability.baseline",
                "Notes discuss how to publish capability.baseline later.",
                "publish: capability.baseline_generator",
                "consumer: capability.baseline.extra",
                "publish: capability.baseline",
                "consumer: capability.baseline",
            ], "\n") * "\n"
            write(fixture, source)
            write(joinpath(root, "outside.trace.md"), "publish: capability.baseline\n")
            before = Dict(name => read(joinpath(root, name)) for name in readdir(root))

            ok, output, errors = run_recur([
                "trace-id", "capability.baseline", "--scope", "lab.**",
                "-d", root, "--json",
            ])
            @test ok
            @test isempty(errors)
            record = JSON3.read(output)
            @test record.identifier == "capability.baseline"
            @test any(hit -> hit.line_number == 1, record.define)
            @test any(hit -> hit.line_number == 5, record.produce)
            @test any(hit -> hit.line_number == 6, record.consume)
            @test all(hit -> basename(hit.path) == "lab.trace.md",
                      Iterators.flatten((record.define, record.produce, record.consume, record.trigger)))
            @test Dict(name => read(joinpath(root, name)) for name in readdir(root)) == before

            # Observe the false classifications without making the bug a pass criterion.
            @info("Legacy trace classification (diagnostic only)",
                  incidental_producers=[hit.line_number for hit in record.produce if hit.line_number in (2, 3)],
                  descendant_consumers=[hit.line_number for hit in record.consume if hit.line_number == 4])
        end
    end

    @testset "One invocation displays and appends a checkpoint" begin
        mktempdir() do root
            vault = joinpath(root, ".recur", "review")
            mkpath(vault)
            write(joinpath(root, ".recur", "config.toml"), "[review]\ndir = '.recur/review'\nsep = '.'\n")
            work = "review.work.current.md"
            write(joinpath(vault, work), "Improvement 34 fixture\n")
            log = joinpath(root, ".recur", "checkpoints.md")
            prefix = "### older-checkpoint\nExisting record.\n"
            write(log, prefix)
            git_bin = get(ENV, "RECUR_GIT_BIN", joinpath(dirname(abspath(RECUR_BIN)),
                          "recur-git" * (Sys.iswindows() ? ".exe" : "")))
            args = ["checkpoint", "--snapshot", "--append-parallel", "--checkpoint-id", "ck-34", "-f", log]
            output = read(Cmd(Cmd(vcat([git_bin], args)); dir=root), String)
            history = read(log, String)
            @test startswith(history, prefix)
            @test length(findall("### ck-34", history)) == 1
            snapshot_lane = only(filter(line -> startswith(line, "lane.state.review.current:"), split(output, '\n')))
            @test occursin(work, snapshot_lane)
            @test any(line -> strip(line) == "- " * strip(snapshot_lane), split(history, '\n'))
            @test read(joinpath(vault, work), String) == "Improvement 34 fixture\n"
        end
    end
end
