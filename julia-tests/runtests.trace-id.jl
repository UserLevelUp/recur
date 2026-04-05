"""
Tests for future 'recur trace-id' command (IMPROVEMENT8)
=======================================================

This suite captures the intended MVP contract before implementation.
Contracts are currently marked with @test_broken until command support lands.
"""

include("runtests.setup.jl")

function seed_trace_id_fixture()
    create_test_file("DotControlEvents.cs", """
    public static class DotControlTopics {
        public const string OwnershipCreate = "ulu.topic.dot.ownership.create";
        public const string RewardProcess = "ulu.topic.dot.reward.process";
    }
    """)

    create_test_file("DotWatcherHostedService.cs", """
    public class DotWatcherHostedService {
        public void RegisterRules() {
            registry.Register("*.level.create", async dot => await _bus.PublishAsync(DotControlTopics.OwnershipCreate));
            registry.Register("*.answer.correct", async dot => await _bus.PublishAsync(DotControlTopics.RewardProcess));
        }
    }
    """)

    create_test_file("OwnershipCreateSubscriber.cs", """
    public class OwnershipCreateSubscriber {
        public void Configure() {
            channel.QueueBind("q.ownership", "x.dot", routingKey: DotControlTopics.OwnershipCreate);
        }
    }
    """)

    create_test_file("OwnershipCreateConsumer.cs", """
    public class OwnershipCreateConsumer {
        private readonly ILogger<OwnershipCreateConsumer> _logger;
        public OwnershipCreateConsumer(ILogger<OwnershipCreateConsumer> logger) {
            _logger = logger;
        }
    }
    """)
end

function recur_cmd(args::Vector{String})
    Cmd(vcat([RECUR_BIN], args))
end

@testset "recur trace-id command (IMPROVEMENT8)" begin
    log_section("Testing: recur trace-id")

    created_here = false
    if !isdir(TEST_DIR)
        setup_test_environment()
        created_here = true
    end

    try
        seed_trace_id_fixture()

        @testset "Phase 1: CLI contract" begin
            success, output, _ = run_recur(["trace-id", "-d", TEST_DIR, "--help"])

            @test success
            @test contains(lowercase(output), "trace-id")
            @test contains(output, "--scope")
            @test contains(output, "--format")
            @test contains(output, "--depth")
            @test contains(output, "--depth-guard")
            @test contains(output, "--force")
            @test contains(output, "--save-run")
            @test contains(output, "--reuse-if-fresh")
            @test contains(output, "--check-run")
            @test contains(output, "--run-name")
        end

        @testset "Phase 2: Role detection contracts" begin
            success, output, _ = run_recur([
                "trace-id",
                "ulu.topic.dot.ownership.create",
                "--scope",
                "**",
                "--ext",
                ".cs",
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

            @test success
            @test parse_ok
            if parse_ok
                @test haskey(parsed, :identifier) || haskey(parsed, "identifier")
                @test haskey(parsed, :define) || haskey(parsed, "define")
                @test haskey(parsed, :produce) || haskey(parsed, "produce")
                @test haskey(parsed, :consume) || haskey(parsed, "consume")
                @test haskey(parsed, :trigger) || haskey(parsed, "trigger")

                define = haskey(parsed, :define) ? parsed[:define] : parsed["define"]
                produce = haskey(parsed, :produce) ? parsed[:produce] : parsed["produce"]
                consume = haskey(parsed, :consume) ? parsed[:consume] : parsed["consume"]
                trigger = haskey(parsed, :trigger) ? parsed[:trigger] : parsed["trigger"]

                @test length(define) >= 1
                @test length(produce) >= 1
                @test length(consume) >= 1
                @test length(trigger) >= 1

                produce_lines = [String(item["line"]) for item in produce]
                consume_lines = [String(item["line"]) for item in consume]

                @test any(line -> contains(lowercase(line), "publishasync"), produce_lines)
                @test any(line -> contains(lowercase(line), "queuebind"), consume_lines)
                @test !any(line -> contains(line, "OwnershipCreateConsumer"), consume_lines)
                @test !any(line -> contains(line, "ILogger<OwnershipCreateConsumer>"), consume_lines)
            end
        end

        @testset "Phase 3: Glob and multi-id contracts" begin
            success, output, _ = run_recur([
                "trace-id",
                "ulu.topic.dot.**",
                "--scope",
                "**",
                "--ext",
                ".cs",
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

            @test success
            @test parse_ok
            if parse_ok
                @test parsed isa AbstractVector
                @test length(parsed) >= 2
            end
        end

        @testset "Phase 3b: JSON site schema - edge_type field" begin
            # Each site in define/produce/consume/trigger arrays includes
            # an edge_type field matching the role name.
            success, output, _ = run_recur([
                "trace-id",
                "ulu.topic.dot.ownership.create",
                "--scope",
                "**",
                "--ext",
                ".cs",
                "--json",
                "-d",
                TEST_DIR,
            ])

            parsed = nothing
            parse_ok = false
            try
                parsed = JSON3.read(output)
                parse_ok = true
            catch
                parse_ok = false
            end

            @test success
            @test parse_ok

            if parse_ok
                for (role_sym, role_str) in [(:define, "define"), (:produce, "produce"), (:consume, "consume"), (:trigger, "trigger")]
                    sites = haskey(parsed, role_sym) ? parsed[role_sym] : get(parsed, role_str, [])
                    if length(sites) > 0
                        first_site = sites[1]
                        has_edge_type = haskey(first_site, :edge_type) || haskey(first_site, "edge_type")
                        @test has_edge_type
                        edge_val = has_edge_type ? String(get(first_site, :edge_type, get(first_site, "edge_type", ""))) : ""
                        @test edge_val == role_str
                    end
                end
            end
        end

        @testset "Phase 4: stdin + guardrail contracts" begin
            stdin_files = join([
                "DotControlEvents.cs",
                "DotWatcherHostedService.cs",
                "OwnershipCreateSubscriber.cs",
            ], "\n")

            success, output, _ = run_recur_stdin(
                stdin_files,
                [
                    "trace-id",
                    "ulu.topic.dot.ownership.create",
                    "--scope",
                    "**",
                    "--stdin",
                    "--ext",
                    ".cs",
                    "--depth",
                    "6",
                    "--depth-guard",
                    "clamp",
                    "--json",
                    "-d",
                    TEST_DIR,
                ],
            )

            parsed = nothing
            parse_ok = false
            try
                parsed = JSON3.read(output)
                parse_ok = true
            catch
                parse_ok = false
            end

            @test success
            @test parse_ok
            has_request = parse_ok && (haskey(parsed, :request) || haskey(parsed, "request"))
            @test has_request
            if has_request
                request = haskey(parsed, :request) ? parsed[:request] : parsed["request"]
                @test Int(request["depth_requested"]) == 6
                @test Int(request["depth_effective"]) == 5
                @test String(request["depth_guard"]) == "clamp"
            end
        end

        @testset "Phase 4b: saved runs persist and refresh" begin
            mkpath(joinpath(TEST_DIR, ".recur"))
            write(
                joinpath(TEST_DIR, ".recur", "config.toml"),
                """
                [traits.trace_id]
                enabled = true
                """,
            )

            run_name = "ownership-create-primary"
            run_dir = joinpath(TEST_DIR, ".recur", "trace-id", "runs", run_name)
            manifest_path = joinpath(run_dir, "manifest.toml")
            latest_json_path = joinpath(run_dir, "latest.json")

            success, output, _ = run_recur([
                "trace-id",
                "ulu.topic.dot.ownership.create",
                "--scope",
                "**",
                "--ext",
                ".cs",
                "--json",
                "--save-run",
                "--run-name",
                run_name,
                "-d",
                TEST_DIR,
            ])

            @test success
            @test isfile(manifest_path)
            @test isfile(latest_json_path)
            @test contains(read(manifest_path, String), "name = \"ownership-create-primary\"")

            saved = JSON3.read(read(latest_json_path, String))
            @test haskey(saved, :request) || haskey(saved, "request")
            @test haskey(saved, :define) || haskey(saved, "define")
            @test contains(output, "\"edge_type\"")

            success, output, _ = run_recur([
                "trace-id",
                "ulu.topic.dot.ownership.create",
                "--scope",
                "**",
                "--ext",
                ".cs",
                "--json",
                "--check-run",
                "--run-name",
                run_name,
                "-d",
                TEST_DIR,
            ])

            @test success
            @test contains(output, "\"status\": \"fresh\"")

            cached_output = replace(
                read(latest_json_path, String),
                "\"pattern\": \"ulu.topic.dot.ownership.create\"" => "\"pattern\": \"cached.reuse.marker\"",
            )
            write(latest_json_path, cached_output)

            success, output, _ = run_recur([
                "trace-id",
                "ulu.topic.dot.ownership.create",
                "--scope",
                "**",
                "--ext",
                ".cs",
                "--json",
                "--reuse-if-fresh",
                "--run-name",
                run_name,
                "-d",
                TEST_DIR,
            ])

            @test success
            @test contains(output, "\"pattern\": \"cached.reuse.marker\"")

            create_test_file(
                "OwnershipCreatePublisher.cs",
                """
                public class OwnershipCreatePublisher {
                    public async Task PublishAgain() {
                        await _bus.PublishAsync(DotControlTopics.OwnershipCreate);
                    }
                }
                """,
            )

            success, output, _ = run_recur([
                "trace-id",
                "ulu.topic.dot.ownership.create",
                "--scope",
                "**",
                "--ext",
                ".cs",
                "--json",
                "--check-run",
                "--run-name",
                run_name,
                "-d",
                TEST_DIR,
            ])

            @test success
            @test contains(output, "\"status\": \"stale\"")
            @test contains(output, "input files changed")
        end

        @testset "Phase 5: cross-command JSON pipeline contracts" begin
            @testset "trace -> merge (edge metadata placeholder)" begin
                # trace/callers/callees JSON has no edge_type concept — descoped.
                # edge_type is a trace-id specific semantic. These commands produce
                # file-path trees, not role-classified sites.
                @test_skip true
            end

            @testset "callers -> merge (edge metadata placeholder)" begin
                # Descoped: callers JSON has no edge_type. See trace-id -> merge below.
                @test_skip true
            end

            @testset "callees -> merge (edge metadata placeholder)" begin
                # Descoped: callees JSON has no edge_type. See trace-id -> merge below.
                @test_skip true
            end

            @testset "trace-id -> merge (full composition placeholder)" begin
                input_cmd = recur_cmd([
                    "trace-id",
                    "ulu.topic.dot.ownership.create",
                    "--scope",
                    "**",
                    "--ext",
                    ".cs",
                    "--json",
                    "-d",
                    TEST_DIR,
                ])

                success, output, _ = run_recur_piped(
                    input_cmd,
                    ["merge", "--stdin", "--base", "pipeline.trace-id", "--sep", ".", "--json"],
                )

                parsed = nothing
                parse_ok = false
                try
                    parsed = JSON3.read(output)
                    parse_ok = true
                catch
                    parse_ok = false
                end

                @test success
                @test parse_ok
                @test contains(output, "\"edge_type\"")
            end
        end
    finally
        if created_here
            teardown_test_environment()
        end
    end
end
