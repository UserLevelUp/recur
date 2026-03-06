"""
Tests for 'recur trait' Command
===============================

Trait command validates centralized .recur config management and
traversal budget configuration flow.

Min-depth traversal budget behavior is captured as @test_broken placeholder
until implementation lands.
"""

include("runtests.setup.jl")

function seed_trait_traversal_fixture()
    create_test_file("DotControlEvents.cs", """
    public static class DotControlTopics {
        public const string OwnershipCreate = "ulu.topic.dot.ownership.create";
    }
    """)

    create_test_file("DotWatcherHostedService.cs", """
    public class DotWatcherHostedService {
        public void RegisterRules() {
            registry.Register("*.level.create", async dot => await _bus.PublishAsync(DotControlTopics.OwnershipCreate));
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

@testset "recur trait command" begin
    log_section("Testing: recur trait")

    created_here = false
    if !isdir(TEST_DIR)
        setup_test_environment()
        created_here = true
    end

    try
        seed_trait_traversal_fixture()

        @testset "trait CLI contract" begin
            success, output, _ = run_recur(["trait", "-d", TEST_DIR, "--help"])

            @test success
            @test contains(lowercase(output), "trait")
            @test contains(lowercase(output), "list")
            @test contains(lowercase(output), "get")
            @test contains(lowercase(output), "set")
        end

        @testset "traversal budget max_depth via trait config" begin
            success, _, _ = run_recur(["init", "-d", TEST_DIR, "--force"])
            @test success

            success, _, _ = run_recur([
                "trait",
                "-d",
                TEST_DIR,
                "set",
                "traversal_budget.max_depth",
                "2",
            ])
            @test success

            success, _, _ = run_recur([
                "trait",
                "-d",
                TEST_DIR,
                "set",
                "traversal_budget.depth_guard",
                "clamp",
            ])
            @test success

            success, output, _ = run_recur([
                "trait",
                "-d",
                TEST_DIR,
                "get",
                "traversal_budget.max_depth",
            ])
            @test success
            @test contains(output, "traits.traversal_budget.max_depth")
            @test contains(output, "2")

            success, output, _ = run_recur([
                "trace-id",
                "ulu.topic.dot.ownership.create",
                "--scope",
                "**",
                "--ext",
                ".cs",
                "--depth",
                "6",
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
                request = haskey(parsed, :request) ? parsed[:request] : parsed["request"]
                @test Int(request["depth_requested"]) == 6
                @test Int(request["depth_effective"]) == 2
                @test Int(request["depth_cap"]) == 2
                @test String(request["depth_guard"]) == "clamp"
            end
        end

        @testset "traversal budget min_depth placeholder" begin
            success, _, _ = run_recur([
                "trait",
                "-d",
                TEST_DIR,
                "set",
                "traversal_budget.min_depth",
                "2",
            ])
            @test success

            success, output, _ = run_recur([
                "trace-id",
                "ulu.topic.dot.ownership.create",
                "--scope",
                "**",
                "--ext",
                ".cs",
                "--depth",
                "1",
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
                request = haskey(parsed, :request) ? parsed[:request] : parsed["request"]
                @test Int(request["depth_requested"]) == 1
                # Future behavior: enforce configured minimum traversal depth.
                @test_broken Int(request["depth_effective"]) == 2
            end
        end
    finally
        if created_here
            teardown_test_environment()
        end
    end
end

