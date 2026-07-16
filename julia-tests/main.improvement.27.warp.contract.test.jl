"""Fixture integrity contract for the future `warp-status-v1` scorer."""

using Test
using JSON3

const WARP_FIXTURE_ROOT = joinpath(@__DIR__, "fixtures", "warp-status-v1")
const WARP_FIXTURES = ["optimum", "sub-optimum", "blocked", "config-override"]
const REQUIRED_WARP_FIELDS = [
    "schema", "fixture", "lane", "verdict", "objective", "state_groups",
    "trace_id_roles", "residuals", "next_actions",
]

@testset "warp-status-v1 fixture contract" begin
    for fixture in WARP_FIXTURES
        @testset "$fixture" begin
            path = joinpath(WARP_FIXTURE_ROOT, fixture, "expected.json")
            @test isfile(path)
            expected = JSON3.read(read(path, String))
            @test all(field -> haskey(expected, field), REQUIRED_WARP_FIELDS)
            @test String(expected["schema"]) == "warp-status-v1"
            @test String(expected["fixture"]) == fixture
            @test String(expected["verdict"]) in ["optimum", "sub_optimum", "blocked"]
            @test Float64(expected["objective"]) >= 0
            @test all(name -> haskey(expected["state_groups"], name), ["active", "complete", "interesting", "blocked", "other"])
            @test all(name -> haskey(expected["trace_id_roles"], name), ["define", "consume", "produce", "trigger"])
        end
    end
end
