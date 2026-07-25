#!/usr/bin/env julia

"""
Red-first executable specification for the Recur Lang Pathing demo.

Source-shape assertions are active today. Parser, graph, local-view, and
execution assertions use @test_broken until their corresponding capability
lands. An unexpected pass intentionally asks us to promote that assertion to
@test.
"""

using Test

if !isdefined(Main, :MainLang)
    include(joinpath(@__DIR__, "..", "demos", "main.lang", "main.lang.runtime.jl"))
end
using .MainLang

const PATHING_SOURCE_PATH =
    joinpath(@__DIR__, "..", "demos", "pathing", "main.lang.pathing.recur")
const PATHING_SOURCE = read(PATHING_SOURCE_PATH, String)
const PATHING_REQUEST = Dict{String,Any}(
    "width" => 15,
    "height" => 11,
    "seed" => 30,
    "power_count" => 4,
    "spawn" => Dict{String,Any}("x" => 7, "y" => 5),
)

function occurrence_count(pattern::Regex, source::String)
    return length(collect(eachmatch(pattern, source)))
end

function maybe_call(name::Symbol, arguments...; keywords...)
    isdefined(MainLang, name) || return nothing
    callable = getfield(MainLang, name)
    try
        return Base.invokelatest(callable, arguments...; keywords...)
    catch
        return nothing
    end
end

function value_at(value, key::String, fallback = nothing)
    value isa AbstractDict || return fallback
    haskey(value, key) && return value[key]
    symbol = Symbol(key)
    haskey(value, symbol) && return value[symbol]
    return fallback
end

function entries_at(value, key::String)
    entries = value_at(value, key, Any[])
    return entries isa AbstractVector ? entries : Any[]
end

function string_set(values)
    values isa AbstractVector || return Set{String}()
    return Set(string(value) for value in values)
end

function entry_named(entries, key::String, expected)
    for entry in entries
        value_at(entry, key) == expected && return entry
    end
    return nothing
end

function tile_key(tile)
    x = value_at(tile, "x")
    y = value_at(tile, "y")
    (x isa Integer && y isa Integer) || return nothing
    return (Int(x), Int(y))
end

function map_from_result(result)
    result isa AbstractDict || return nothing
    return value_at(result, "map")
end

function all_power_nodes_reachable(result)
    map = map_from_result(result)
    graph = value_at(map, "graph")
    spawn = tile_key(value_at(graph, "spawn"))
    powers = entries_at(map, "powers")
    corridors = entries_at(graph, "corridors")
    isnothing(spawn) && return false
    isempty(powers) && return false

    neighbors = Dict{Tuple{Int,Int},Set{Tuple{Int,Int}}}()
    for edge in corridors
        left = tile_key(value_at(edge, "from"))
        right = tile_key(value_at(edge, "to"))
        (isnothing(left) || isnothing(right)) && return false
        push!(get!(neighbors, left, Set{Tuple{Int,Int}}()), right)
        push!(get!(neighbors, right, Set{Tuple{Int,Int}}()), left)
    end

    visited = Set([spawn])
    frontier = [spawn]
    while !isempty(frontier)
        current = popfirst!(frontier)
        for neighbor in get(neighbors, current, Set{Tuple{Int,Int}}())
            neighbor in visited && continue
            push!(visited, neighbor)
            push!(frontier, neighbor)
        end
    end

    return all(
        begin
            tile = tile_key(value_at(power, "tile"))
            !isnothing(tile) && tile in visited
        end for power in powers
    )
end

function every_path_is_contiguous(result)
    map = map_from_result(result)
    graph = value_at(map, "graph")
    paths = entries_at(map, "paths")
    corridors = entries_at(graph, "corridors")
    isempty(paths) && return false

    edges = Set{Tuple{Tuple{Int,Int},Tuple{Int,Int}}}()
    for edge in corridors
        left = tile_key(value_at(edge, "from"))
        right = tile_key(value_at(edge, "to"))
        (isnothing(left) || isnothing(right)) && return false
        push!(edges, (left, right))
        push!(edges, (right, left))
    end

    for path in paths
        tiles = [tile_key(tile) for tile in entries_at(path, "tiles")]
        (length(tiles) >= 2 || any(isnothing, tiles)) && return false
        all((tiles[index], tiles[index + 1]) in edges for index in 1:(length(tiles) - 1)) ||
            return false
    end
    return true
end

function breadcrumbs_equal_unique_path_interiors(result)
    map = map_from_result(result)
    graph = value_at(map, "graph")
    spawn = tile_key(value_at(graph, "spawn"))
    powers = Set(
        tile_key(value_at(power, "tile"))
        for power in entries_at(map, "powers")
    )
    pellets = [
        tile_key(value_at(pellet, "tile"))
        for pellet in entries_at(map, "pellets")
        if value_at(pellet, "kind") == "normal"
    ]
    paths = entries_at(map, "paths")

    (isnothing(spawn) || any(isnothing, powers) || any(isnothing, pellets)) &&
        return false

    expected = Set{Tuple{Int,Int}}()
    for path in paths
        for tile in entries_at(path, "tiles")
            key = tile_key(tile)
            isnothing(key) && return false
            key == spawn && continue
            key in powers && continue
            push!(expected, key)
        end
    end

    return length(pellets) == length(Set(pellets)) && Set(pellets) == expected
end

const PATHING_PARSE_RESULT = try
    MainLang.parse_program(PATHING_SOURCE)
catch error
    error
end

const PATHING_GRAPH =
    PATHING_PARSE_RESULT isa MainLang.Program ?
    maybe_call(:static_graph, PATHING_PARSE_RESULT, "solution") : nothing

const PATHING_LOCAL_ROUTE_GRAPH =
    PATHING_GRAPH isa AbstractDict ?
    maybe_call(:local_graph, PATHING_GRAPH, "route[*]"; depth = 1) : nothing

const PATHING_RESULT =
    PATHING_PARSE_RESULT isa MainLang.Program ?
    maybe_call(
        :execute_coordination,
        PATHING_PARSE_RESULT,
        "solution",
        PATHING_REQUEST;
        scheduler = :deterministic,
    ) : nothing

const PATHING_RESULT_REPEAT =
    PATHING_PARSE_RESULT isa MainLang.Program ?
    maybe_call(
        :execute_coordination,
        PATHING_PARSE_RESULT,
        "solution",
        PATHING_REQUEST;
        scheduler = :deterministic,
    ) : nothing

@testset "pathing source freezes the formal parallel graph" begin
    @test occursin("recur 0.3 coordination PacPathGenerator", PATHING_SOURCE)
    @test all(
        occursin("contract $name", PATHING_SOURCE) for name in (
            "MapRequest",
            "MazeGraph",
            "PowerPlan",
            "RouteOrder",
            "PowerPath",
            "PelletPath",
            "PacMap",
            "VerifiedPacMap",
        )
    )
    @test all(
        occursin("scope $name", PATHING_SOURCE) for name in (
            "topology",
            "distance",
            "dead_ends",
            "symmetry",
            "coverage",
            "powers",
            "route",
            "crumbs",
            "assemble",
            "verify",
        )
    )

    @test occurrence_count(
        r"i\(b\)\s*:=\s*topology\.o\(b\)",
        PATHING_SOURCE,
    ) == 4
    @test occursin("o(c) := DistanceScores", PATHING_SOURCE)
    @test occursin("o(d) := DeadEndScores", PATHING_SOURCE)
    @test occursin("o(e) := SymmetryScores", PATHING_SOURCE)
    @test occursin("o(f) := CoverageScores", PATHING_SOURCE)

    @test occursin("graph solution async", PATHING_SOURCE)
    @test occursin(
        "solution.i(a) -> topology.i(a) delivery sync",
        PATHING_SOURCE,
    )
    @test occursin("topology.o(b) -> fork [", PATHING_SOURCE)
    @test occursin("] dispatch parallel delivery async", PATHING_SOURCE)
    @test occursin("await all [", PATHING_SOURCE)
    @test occursin(
        "powers.o(h).route_orders -> parallel each order",
        PATHING_SOURCE,
    )
    @test occursin("route[order.target.id].o(j)", PATHING_SOURCE)
    @test occursin("await all crumbs[*].o(k) -> assemble.i(l)", PATHING_SOURCE)
    @test occursin(
        "assemble.o(m) -> verify.i(m) delivery sync",
        PATHING_SOURCE,
    )

    @test occursin("memo solution", PATHING_SOURCE)
    @test occursin("check every_power_reachable", PATHING_SOURCE)
    @test occursin("check every_path_is_contiguous", PATHING_SOURCE)
    @test occursin("check no_duplicate_pellets", PATHING_SOURCE)
    @test occursin("report local_graph for route", PATHING_SOURCE)
end

@testset "pathing parser and static graph contracts (expected broken)" begin
    @test_broken PATHING_PARSE_RESULT isa MainLang.Program
    @test_broken PATHING_GRAPH isa AbstractDict
    @test_broken value_at(PATHING_GRAPH, "schema") ==
                 "recur-lang-static-graph-report-v1"
    @test_broken value_at(PATHING_GRAPH, "mode") == "async"

    fanout = entry_named(
        entries_at(PATHING_GRAPH, "fanouts"),
        "source",
        "topology.o(b)",
    )
    @test_broken string_set(value_at(fanout, "consumers", Any[])) == Set([
        "distance.i(b)",
        "dead_ends.i(b)",
        "symmetry.i(b)",
        "coverage.i(b)",
    ])
    @test_broken value_at(fanout, "dispatch") == "parallel"
    @test_broken value_at(fanout, "delivery") == "async"

    @test_broken PATHING_LOCAL_ROUTE_GRAPH isa AbstractDict
    @test_broken string_set(
        value_at(PATHING_LOCAL_ROUTE_GRAPH, "upstream", Any[]),
    ) == Set(["powers.o(h).route_orders"])
    @test_broken string_set(
        value_at(PATHING_LOCAL_ROUTE_GRAPH, "downstream", Any[]),
    ) == Set(["crumbs[*].i(j)"])
    @test_broken value_at(PATHING_LOCAL_ROUTE_GRAPH, "dispatch") == "parallel"
end

@testset "pathing deterministic execution contracts (expected broken)" begin
    @test_broken PATHING_RESULT isa AbstractDict
    @test_broken value_at(PATHING_RESULT, "schema") ==
                 "recur-lang-pathing-result-v1"
    @test_broken PATHING_RESULT isa AbstractDict &&
                 PATHING_RESULT_REPEAT isa AbstractDict &&
                 PATHING_RESULT == PATHING_RESULT_REPEAT

    generated_map = map_from_result(PATHING_RESULT)
    @test_broken length(entries_at(generated_map, "powers")) ==
                 PATHING_REQUEST["power_count"]
    @test_broken all_power_nodes_reachable(PATHING_RESULT)
    @test_broken every_path_is_contiguous(PATHING_RESULT)
    @test_broken breadcrumbs_equal_unique_path_interiors(PATHING_RESULT)

    execution = value_at(PATHING_RESULT, "execution")
    @test_broken value_at(execution, "route_branches") ==
                 PATHING_REQUEST["power_count"]
    @test_broken value_at(execution, "crumb_branches") ==
                 PATHING_REQUEST["power_count"]
end
