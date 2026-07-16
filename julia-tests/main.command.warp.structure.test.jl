"""Read-only command-structure contract for dot-separated Warp eventness slices."""

include("runtests.setup.jl")

const WARP_COMMAND_FIXTURE_ROOT = joinpath(@__DIR__, "fixtures", "warp-command-v1")
const WARP_ALPHA_LANE = "demo.warp.slice.alpha"

paths(items) = sort(String[item["path"] for item in items])

@testset "recur warp command structure" begin
    @testset "status keeps a dot-separated lane boundary" begin
        success, output, error_output = run_recur([
            "warp", "status", WARP_ALPHA_LANE, "-d", WARP_COMMAND_FIXTURE_ROOT, "--json",
        ])

        @test success
        @test error_output == ""
        status = JSON3.read(output)
        @test String(status["schema"]) == "warp-status-v1"
        @test String(status["scope"]) == "demo.warp.slice.alpha.**"
        @test String(status["verdict"]) == "blocked"
        @test Float64(status["objective"]) == 5.0
        @test paths(status["files"]) == [
            "demo.warp.slice.alpha.approval.awaiting.md",
            "demo.warp.slice.alpha.evidence.verified.md",
            "demo.warp.slice.alpha.interface.needs-review.md",
        ]
        @test all(!contains(path, "alphabet") for path in paths(status["files"]))
        @test Int(status["state_groups"]["complete"]) == 1
        @test Int(status["state_groups"]["interesting"]) == 1
        @test Int(status["state_groups"]["blocked"]) == 1
    end

    @testset "collapse-plan preserves each eventness slice" begin
        success, output, error_output = run_recur([
            "warp", "collapse-plan", WARP_ALPHA_LANE, "-d", WARP_COMMAND_FIXTURE_ROOT, "--json",
        ])

        @test success
        @test error_output == ""
        plan = JSON3.read(output)
        @test String(plan["schema"]) == "warp-collapse-plan-v1"
        @test String(plan["lane"]) == WARP_ALPHA_LANE
        @test String(plan["scope"]) == "demo.warp.slice.alpha.**"
        @test String(plan["verdict"]) == "blocked"
        @test paths(plan["collapse_known"]) == ["demo.warp.slice.alpha.evidence.verified.md"]
        @test paths(plan["preserve_interesting"]) == ["demo.warp.slice.alpha.interface.needs-review.md"]
        @test paths(plan["blockers"]) == ["demo.warp.slice.alpha.approval.awaiting.md"]
        @test isempty(plan["ambiguous"])
    end

    @testset "config exposes the policy used by all read-only projections" begin
        success, output, error_output = run_recur([
            "warp", "config", "-d", WARP_COMMAND_FIXTURE_ROOT, "--json",
        ])

        @test success
        @test error_output == ""
        config = JSON3.read(output)
        @test String(config["schema"]) == "warp-config-v1"
        @test String.(config["active_suffixes"]) == ["current"]
        @test String.(config["complete_suffixes"]) == ["verified"]
        @test String.(config["interesting_suffixes"]) == ["needs-review"]
        @test String.(config["blocked_suffixes"]) == ["awaiting"]
    end
end
