# CLI contract for Improvement 34; also included in the core runner.
isdefined(@__MODULE__, :run_recur) || include("runtests.setup.jl")
using SHA

function trace34(root, arguments...; expect_success=true)
    ok, output, errors = run_recur(["trace-id", "capability.baseline", "-d", root, "--json", arguments...])
    @test ok == expect_success
    if isempty(output)
        @test !expect_success
        return nothing
    end
    JSON3.read(output)
end

function fixture34(root, path, text)
    file = joinpath(root, path)
    mkpath(dirname(file))
    write(file, text)
    file
end

metadata34(records...) = "<!-- recur-trace-v1\n" * join(records, '\n') * "\n-->\n"

@testset "Improvement 34 trace discovery and revision pins" begin
    @testset "Paths first, scoped legacy details unchanged" begin
        mktempdir() do root
            fixture34(root, "one/lab.first.md", "publish: capability.baseline\ncapability.baseline\n")
            fixture34(root, "two/alt.second.md", "consumer: capability.baseline\n")
            fixture34(root, "two/alt.third.md", "capability.baseline\n")
            fixture34(root, "other/none.md", "capability.baseline_generator\n")
            fixture34(root, "other/alt.txt", "capability.baseline\n")
            fixture34(root, "target/build.md", "capability.baseline\n")
            fixture34(root, ".hidden/private.md", "capability.baseline\n")
            paths = trace34(root, "--ext", ".md")
            if paths !== nothing
                @test paths.schema == "recur-trace-id-paths-v1"
                @test paths.folders == ["one", "two"]
                @test paths.files == ["one/lab.first.md", "two/alt.second.md", "two/alt.third.md"]
                @test paths.coverage.complete
                @test all(key -> !haskey(paths, key), [:define, :produce, :consume, :trigger])
            end
            bounded = trace34(root, "--max-entries", "1")
            bounded === nothing || @test !bounded.coverage.complete
            byte_limited = trace34(root, "--max-bytes", "1")
            byte_limited === nothing || @test !byte_limited.coverage.complete && isempty(byte_limited.files)
            empty_root = mkpath(joinpath(root, "empty"))
            empty = trace34(empty_root; expect_success=false)
            empty === nothing || @test empty.coverage.complete && isempty(empty.files)
            _, output, _ = run_recur(["trace-id", "capability.baseline", "-d", root, "--ext", ".md"])
            @test occursin("one/lab.first.md", output) && !occursin("publish:", output)
            ok, _, errors = run_recur(["trace-id", "capability.baseline", "-d", root, "--depth", "0"])
            @test !ok && occursin("--scope", errors)
            scoped = trace34(root, "--scope", "lab.**", "--ext", ".md")
            if scoped !== nothing
                @test scoped.identifier == "capability.baseline"
                @test all(x -> basename(x.path) == "lab.first.md", scoped.produce)
                @test !haskey(scoped, :schema)
            end
            broad = trace34(root, "--scope", "**", "--ext", ".md")
            broad === nothing || @test any(x -> basename(x.path) == "alt.second.md", broad.consume)
        end
    end

    @testset "Exact declarations, distinct pins and unchanged inputs" begin
        mktempdir() do parent
            root = mkpath(joinpath(parent, "root"))
            four, six = "revision four\n", "revision six\n"
            r04 = fixture34(root, "r04.txt", four)
            fixture34(root, "r06.txt", six)
            artifact(revision, path, bytes) = "artifact: " * JSON3.write((
                id="capability.baseline", revision, path, sha256=bytes2hex(sha256(bytes)), context="dock"))
            pin(expected, actual) = "requires: " * JSON3.write((id="capability.baseline", expected, actual, context="dock"))
            definitions = fixture34(root, "lab.circuit.md", metadata34(
                "defines: capability.baseline", "publish: capability.baseline",
                "publish: capability.baseline_generator", "consumer: capability.baseline.extra",
                artifact("R04", "r04.txt", four), artifact("R06", "r06.txt", six)) *
                "Notes discuss how to publish capability.baseline later.\n")
            fixture34(root, "lab.delivery.md", metadata34(pin("R06", "R04")))
            fixture34(root, "lab.history.md", metadata34(pin("R04", "R04")))
            fixture34(root, "lab.missing.md", metadata34(pin("R08", "R08")))
            fixture34(root, "lab.malformed.md", metadata34("defines: capability.baseline-bad"))
            fixture34(root, "lab.example.md", "```text\n" * metadata34("publish: capability.baseline") * "```\n")
            fixture34(root, "lab.generator.jl", "write(stdout, \"publish: capability.baseline\")\n")
            before = Dict(name => read(joinpath(root, name)) for name in readdir(root))

            paths = trace34(root, "--ext", ".md")
            paths === nothing || @test "lab.circuit.md" in paths.files
            details = trace34(root, "--scope", "lab.**", "--declared")
            if details !== nothing
                @test details.schema == "recur-trace-id-declarations-v1"
                @test [(r.role, r.id, r.span.line) for r in details.declarations] ==
                    [("defines", "capability.baseline", 2), ("publish", "capability.baseline", 3)]
                @test any(d -> d.code == "invalid_identifier" && d.token == "capability.baseline-bad", details.diagnostics)
                @test all(a -> a.integrity == "matched", details.artifacts)
                pins = Dict(p.source.path => p for p in details.pins)
                @test pins["lab.delivery.md"].revision == "mismatch"
                @test pins["lab.delivery.md"].integrity == "matched"
                @test pins["lab.history.md"].revision == "matched"
                @test pins["lab.missing.md"].revision == "unknown"
                @test pins["lab.missing.md"].integrity == "unknown"
                @test Dict(name => read(joinpath(root, name)) for name in readdir(root)) == before
                outside = trace34(root, "--scope", "lab.delivery", "--declared")
                @test only(outside.pins).revision == "unknown"
                write(r04, "changed bytes")
                drift = trace34(root, "--scope", "lab.**", "--declared")
                @test only(filter(p -> p.source.path == "lab.history.md", drift.pins)).integrity == "mismatch"
                @test read(definitions, String) == String(before["lab.circuit.md"])
                fixture34(root, "lab.duplicate.md", metadata34("defines: capability.baseline"))
                duplicate = trace34(root, "--scope", "lab.**", "--declared", "--unique-defines")
                diagnostic = only(filter(d -> d.code == "duplicate_authority", duplicate.diagnostics))
                @test length(diagnostic.sites) == 2
                fixture34(parent, "r09.txt", "outside")
                fixture34(root, "lab.escape.md", metadata34(artifact("R09", "../r09.txt", "outside"), pin("R09", "R09")))
                escaped = trace34(root, "--scope", "lab.escape", "--declared")
                @test only(escaped.pins).reason == "unavailable_artifact"
                @test only(escaped.pins).integrity == "unknown"
            end
        end
    end
end
