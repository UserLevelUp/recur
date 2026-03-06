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

        @testset "Phase 5: cross-command JSON pipeline contracts" begin
            @testset "trace -> merge (edge metadata placeholder)" begin
                input_cmd = recur_cmd([
                    "trace",
                    "CreateWizard3",
                    "--scope",
                    "**",
                    "--ext",
                    ".cs",
                    "--depth",
                    "1",
                    "-d",
                    TEST_DIR,
                ])

                success, output, _ = run_recur_piped(
                    input_cmd,
                    ["merge", "--stdin", "--base", "pipeline.trace", "--sep", ".", "--json"],
                )

                # Contract target: merged output should eventually retain semantic edge metadata.
                @test_broken success && contains(output, "\"edge_type\"")
            end

            @testset "callers -> merge (edge metadata placeholder)" begin
                input_cmd = recur_cmd([
                    "callers",
                    "ValidateEmail",
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
                    ["merge", "--stdin", "--base", "pipeline.callers", "--sep", ".", "--json"],
                )

                @test_broken success && contains(output, "\"edge_type\"")
            end

            @testset "callees -> merge (edge metadata placeholder)" begin
                input_cmd = recur_cmd([
                    "callees",
                    "CreateUser",
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
                    ["merge", "--stdin", "--base", "pipeline.callees", "--sep", ".", "--json"],
                )

                @test_broken success && contains(output, "\"edge_type\"")
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

                @test_broken success && contains(output, "\"edge_type\"")
            end
        end
    finally
        if created_here
            teardown_test_environment()
        end
    end
end
