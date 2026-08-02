#!/usr/bin/env julia

using Test
using TOML

include(joinpath(@__DIR__, "..", "demos", "pathing", "proto_map.jl"))
using .PathingProtoMap

@testset "pathing proto map generates deterministic layers" begin
    generated = generate_map()
    repeated = generate_map()

    @test generated.width == 35
    @test generated.height == 15
    @test all(length(row) == generated.width for row in generated.topology)
    @test length(generated.topology) == generated.height
    @test generated.topology == repeated.topology
    @test generated.terrain == repeated.terrain
    @test generated.current_route == repeated.current_route
    @test generated.optimum_route == repeated.optimum_route

    topology = join(generated.topology, "\n")
    @test all(occursin(glyph, topology) for glyph in ("O", ".", "+", "=", "E"))
    @test occursin("~", join(generated.terrain, "\n"))
    @test occursin("^", join(generated.terrain, "\n"))
    @test occursin("@", join(generated.current_route, "\n"))
    @test occursin("*", join(generated.optimum_route, "\n"))

    @test generated.current_cost == 38
    @test generated.optimum_travel_cost == 22
    @test generated.research_cost == 6
    @test generated.optimum_travel_cost + generated.research_cost < generated.current_cost

    @test !isempty(generated.current_decisions)
    @test all(
        decision.terrain == :steps &&
        decision.capability == :climb &&
        decision.outcome == :accepted &&
        decision.cost == 2
        for decision in generated.current_decisions
    )
    @test !isempty(generated.optimum_decisions)
    @test all(
        decision.terrain == :ravine &&
        decision.capability == :glide &&
        decision.outcome == :accepted &&
        decision.cost == 1
        for decision in generated.optimum_decisions
    )
    @test occursin("TERRAIN NEGOTIATION", render(generated))
    @test occursin("ravine + glide -> accepted", render(generated))

    labels = Dict(label.notation => label.description for label in generated.labels)
    @test Set(keys(labels)) == Set(["i(a)", "i(b)", "f(a)", "f(b)", "o(c)"])
    @test occursin("player", labels["i(a)"])
    @test occursin("terrain", labels["f(a)"])
    @test occursin("route", labels["o(c)"])
end

@testset "pathing proto map accepts bounded generation parameters" begin
    generated = generate_map(; width = 41, height = 17, research_cost = 9)

    @test generated.width == 41
    @test generated.height == 17
    @test generated.research_cost == 9
    @test length(generated.topology) == 17
    @test all(length(row) == 41 for row in generated.topology)
    @test render(generated) == render(generate_map(; width = 41, height = 17, research_cost = 9))

    @test_throws ArgumentError generate_map(; width = 24)
    @test_throws ArgumentError generate_map(; height = 10)
    @test_throws ArgumentError generate_map(; width = 34)
    @test_throws ArgumentError generate_map(; height = 14)
    @test_throws ArgumentError generate_map(; research_cost = -1)
end

@testset "pathing proto map writes image-aligned ASCII fixtures" begin
    directory = tempname()
    mkpath(directory)
    try
        generated = generate_map()
        fixture = write_fixture(directory; generated, pixels_per_cell = 4)

        @test fixture == validate_fixture(directory)
        @test fixture["schema"] == "pathing-ascii-map-v1"
        @test fixture["width"] == 35
        @test fixture["height"] == 15
        @test fixture["image_width"] == 140
        @test fixture["image_height"] == 60

        manifest = TOML.parsefile(joinpath(directory, "map.manifest.toml"))
        @test manifest["image"] == "city-pass.ppm"
        @test manifest["topology"]["O"] == "city or powerup"
        @test manifest["terrain"]["~"] == "glide-gated ravine"
        @test manifest["route"]["@"] == "player"
        @test readlines(joinpath(directory, "topology.txt")) == generated.topology
        @test readlines(joinpath(directory, "terrain.txt")) == generated.terrain
        @test isfile(joinpath(directory, "city-pass.ppm"))
    finally
        rm(directory; recursive = true, force = true)
    end
end

@testset "checked-in city-pass fixture remains image aligned" begin
    directory = joinpath(@__DIR__, "..", "demos", "pathing", "maps", "city-pass")
    fixture = validate_fixture(directory)

    @test fixture["schema"] == "pathing-ascii-map-v1"
    @test fixture["width"] == 35
    @test fixture["height"] == 15
    @test fixture["image_width"] == 140
    @test fixture["image_height"] == 60
    @test isfile(joinpath(directory, "city-pass.ppm"))
end