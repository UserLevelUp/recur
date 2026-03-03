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
end

@testset "recur trace-id command (IMPROVEMENT8, expected broken)" begin
    log_section("Testing: recur trace-id (expected broken)")

    created_here = false
    if !isdir(TEST_DIR)
        setup_test_environment()
        created_here = true
    end

    try
        seed_trace_id_fixture()

        @testset "Phase 1: CLI contract (broken until command exists)" begin
            success, output, _ = run_recur(["trace-id", "--help"])

            @test_broken success
            @test_broken contains(lowercase(output), "trace-id")
            @test_broken contains(output, "--scope")
            @test_broken contains(output, "--format")
            @test_broken contains(output, "--depth")
            @test_broken contains(output, "--depth-guard")
            @test_broken contains(output, "--force")
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

            @test_broken success
            @test_broken parse_ok
            @test_broken parse_ok && haskey(parsed, :identifier)
            @test_broken parse_ok && haskey(parsed, :define)
            @test_broken parse_ok && haskey(parsed, :produce)
            @test_broken parse_ok && haskey(parsed, :consume)
            @test_broken parse_ok && haskey(parsed, :trigger)
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

            @test_broken success
            @test_broken parse_ok
            @test_broken parse_ok && length(parsed) >= 2
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

            @test_broken success
            @test_broken parse_ok
            @test_broken parse_ok && haskey(parsed, :request)
        end
    finally
        if created_here
            teardown_test_environment()
        end
    end
end
