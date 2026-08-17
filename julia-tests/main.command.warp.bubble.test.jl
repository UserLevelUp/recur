"""Executable contract for compositional Warp bubble maps and Slice layers."""

include("runtests.setup.jl")

const WARP_BUBBLE_FIXTURE_ROOT = joinpath(@__DIR__, "fixtures", "warp-bubble-v1")

@testset "recur warp bubble composition" begin
    @testset "map exposes the declared final bubble" begin
        root = joinpath(WARP_BUBBLE_FIXTURE_ROOT, "complete")
        success, output, error_output = run_recur([
            "warp", "map", "demo.release", "-d", root, "--json",
        ])

        @test success
        @test error_output == ""
        map = JSON3.read(output)
        @test String(map["schema"]) == "warp-bubble-map-view-v1"
        @test String(map["manifest_schema"]) == "warp-bubble-map-v1"
        @test String[entry["slice_id"] for entry in map["required_slices"]] == [
            "alpha", "beta", "gamma",
        ]
    end

    @testset "accepted layers self-report complete coverage" begin
        root = joinpath(WARP_BUBBLE_FIXTURE_ROOT, "complete")
        success, output, error_output = run_recur([
            "warp", "merge", "demo.release", "-d", root, "--json",
        ])

        @test success
        @test error_output == ""
        merged = JSON3.read(output)
        @test String(merged["schema"]) == "warp-bubble-projection-v1"
        @test String(merged["state"]) == "complete"
        @test Int(merged["counts"]["required"]) == 3
        @test Int(merged["counts"]["covered"]) == 3
        @test String.(merged["covered"]) == ["alpha", "beta", "gamma"]
        @test isempty(merged["pending"])

        success, status_output, _ = run_recur([
            "warp", "status", "demo.release", "-d", root, "--json",
        ])
        @test success
        status = JSON3.read(status_output)
        @test String(status["verdict"]) == "optimum"
        @test String(status["bubble"]["state"]) == "complete"
    end

    @testset "missing layers remain visible without imposing order" begin
        root = joinpath(WARP_BUBBLE_FIXTURE_ROOT, "partial")
        success, output, error_output = run_recur([
            "warp", "merge", "demo.partial", "-d", root, "--json",
        ])

        @test success
        @test error_output == ""
        merged = JSON3.read(output)
        @test String(merged["state"]) == "incomplete"
        @test String.(merged["covered"]) == ["alpha"]
        @test String.(merged["pending"]) == ["beta"]

        success, status_output, _ = run_recur([
            "warp", "status", "demo.partial", "-d", root, "--json",
        ])
        @test success
        status = JSON3.read(status_output)
        @test String(status["verdict"]) == "sub_optimum"
        @test "incomplete-warp-coverage" in String[
            residual["name"] for residual in status["residuals"]
        ]
    end

    @testset "incompatible accepted results visibly explode" begin
        root = joinpath(WARP_BUBBLE_FIXTURE_ROOT, "exploded")
        success, output, error_output = run_recur([
            "warp", "merge", "demo.explosion", "-d", root, "--json",
        ])

        @test success
        @test error_output == ""
        merged = JSON3.read(output)
        @test String(merged["state"]) == "exploded"
        @test Int(merged["counts"]["conflicting"]) == 1
        @test String(merged["conflicts"][1]["slice_id"]) == "alpha"

        success, status_output, _ = run_recur([
            "warp", "status", "demo.explosion", "-d", root, "--json",
        ])
        @test success
        status = JSON3.read(status_output)
        @test String(status["verdict"]) == "blocked"
        @test String(status["bubble"]["state"]) == "exploded"
    end
end
