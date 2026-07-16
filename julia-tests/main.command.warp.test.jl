"""Executable contract for the read-only `recur warp status` slice."""

include("runtests.setup.jl")

const WARP_STATUS_FIXTURE_ROOT = joinpath(@__DIR__, "fixtures", "warp-status-v1")
const WARP_STATUS_FIXTURES = ["optimum", "sub-optimum", "blocked", "config-override"]

function fixture_values(parsed, key)
    return parsed[key]
end

@testset "recur warp status" begin
    for fixture in WARP_STATUS_FIXTURES
        @testset "$fixture" begin
            root = joinpath(WARP_STATUS_FIXTURE_ROOT, fixture)
            expected = JSON3.read(read(joinpath(root, "expected.json"), String))
            lane = String(expected["lane"])
            success, output, error_output = run_recur([
                "warp", "status", lane, "-d", root, "--json",
            ])

            @test success
            @test error_output == ""
            actual = JSON3.read(output)
            @test String(actual["schema"]) == String(expected["schema"])
            @test String(actual["lane"]) == lane
            @test String(actual["verdict"]) == String(expected["verdict"])
            @test Float64(actual["objective"]) == Float64(expected["objective"])

            for group in ["active", "complete", "interesting", "blocked", "other"]
                @test Int(actual["state_groups"][group]) == Int(expected["state_groups"][group])
            end
            for role in ["define", "consume", "produce", "trigger"]
                @test Int(actual["trace_id_roles"][role]) == Int(expected["trace_id_roles"][role])
            end

            residual_names = String[item["name"] for item in actual["residuals"]]
            action_kinds = String[item["kind"] for item in actual["next_actions"]]
            @test residual_names == String[item for item in expected["residuals"]]
            @test action_kinds == String[item for item in expected["next_actions"]]

            success, explain_output, _ = run_recur([
                "warp", "explain", lane, "-d", root, "--json",
            ])
            @test success
            explained = JSON3.read(explain_output)
            @test String(explained["schema"]) == "warp-status-v1"
            @test String(explained["verdict"]) == String(expected["verdict"])

            success, next_output, _ = run_recur([
                "warp", "next", lane, "-d", root, "--json",
            ])
            @test success
            next = JSON3.read(next_output)
            @test String(next["schema"]) == "warp-next-v1"
            @test String[next["kind"] for next in next["next_actions"]] == action_kinds
        end
    end
end
