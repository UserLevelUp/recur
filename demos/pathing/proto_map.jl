module PathingProtoMap

using TOML

export GeneratedMap, LabelAnchor, Point, TerrainDecision, generate_map, main, render,
       write_fixture, validate_fixture

struct Point
    x::Int
    y::Int
end

struct LabelAnchor
    notation::String
    point::Point
    description::String
end

struct TerrainDecision
    point::Point
    terrain::Symbol
    capability::Symbol
    outcome::Symbol
    cost::Int
end

struct GeneratedMap
    width::Int
    height::Int
    topology::Vector{String}
    terrain::Vector{String}
    current_route::Vector{String}
    optimum_route::Vector{String}
    labels::Vector{LabelAnchor}
    current_decisions::Vector{TerrainDecision}
    optimum_decisions::Vector{TerrainDecision}
    current_cost::Int
    optimum_travel_cost::Int
    research_cost::Int
end

const DIRECTIONS = (Point(1, 0), Point(-1, 0), Point(0, 1), Point(0, -1))

Base.:(==)(left::Point, right::Point) = left.x == right.x && left.y == right.y
Base.hash(point::Point, seed::UInt) = hash(point.y, hash(point.x, seed))
Base.:+(left::Point, right::Point) = Point(left.x + right.x, left.y + right.y)

function horizontal(y::Int, first_x::Int, last_x::Int)
    step = first_x <= last_x ? 1 : -1
    return [Point(x, y) for x in first_x:step:last_x]
end

function vertical(x::Int, first_y::Int, last_y::Int)
    step = first_y <= last_y ? 1 : -1
    return [Point(x, y) for y in first_y:step:last_y]
end

function joined(parts::Vector{Point}...)
    result = Point[]
    for part in parts
        for point in part
            isempty(result) || result[end] != point || continue
            push!(result, point)
        end
    end
    return result
end

function blank_canvas(width::Int, height::Int)
    return [fill(' ', width) for _ in 1:height]
end

function rendered(canvas::Vector{Vector{Char}})
    return [String(row) for row in canvas]
end

function classify_topology(
    point::Point,
    memberships::Dict{Point,Set{String}},
    cities::Set{Point},
)
    point in cities && return 'O'
    lanes = get(memberships, point, Set{String}())
    lane_count = length(lanes)
    lane_count >= 3 && return 'E'
    lane_count == 1 && return '.'
    lane_count == 0 && return ' '

    shares_neighbor = any(
        get(memberships, point + direction, Set{String}()) == lanes
        for direction in DIRECTIONS
    )
    return shares_neighbor ? '=' : '+'
end

function shortest_route(
    topology::Set{Point},
    start::Point,
    goal::Point,
    terrain::Dict{Point,Symbol},
    capabilities::Set{Symbol},
)
    distance = Dict(start => 0)
    previous = Dict{Point,Point}()
    remaining = Set(topology)

    while !isempty(remaining)
        candidates = [point for point in remaining if haskey(distance, point)]
        isempty(candidates) && break
        sort!(candidates; by = point -> (distance[point], point.y, point.x))
        current = first(candidates)
        delete!(remaining, current)
        current == goal && break

        for direction in DIRECTIONS
            neighbor = current + direction
            neighbor in remaining || continue
            terrain_kind = get(terrain, neighbor, :plain)
            terrain_kind == :ravine && :glide ∉ capabilities && continue
            step_cost = terrain_kind == :steps ? 2 : 1
            candidate = distance[current] + step_cost
            if candidate < get(distance, neighbor, typemax(Int))
                distance[neighbor] = candidate
                previous[neighbor] = current
            end
        end
    end

    haskey(distance, goal) || error("no route from city to city for active capabilities")
    route = Point[goal]
    while route[end] != start
        push!(route, previous[route[end]])
    end
    reverse!(route)
    return route, distance[goal]
end

function terrain_decisions(
    route::Vector{Point},
    terrain::Dict{Point,Symbol},
    capabilities::Set{Symbol},
)
    decisions = TerrainDecision[]
    for point in route
        terrain_kind = get(terrain, point, :plain)
        terrain_kind == :plain && continue
        capability = terrain_kind == :ravine ? :glide : :climb
        active = capability in capabilities
        push!(
            decisions,
            TerrainDecision(
                point,
                terrain_kind,
                capability,
                active ? :accepted : :blocked,
                active ? (terrain_kind == :steps ? 2 : 1) : typemax(Int),
            ),
        )
    end
    return decisions
end

function route_layer(
    width::Int,
    height::Int,
    route::Vector{Point},
    cities::Set{Point},
    player::Point,
)
    canvas = blank_canvas(width, height)
    for point in route
        canvas[point.y][point.x] = point in cities ? 'O' : '*'
    end
    canvas[player.y][player.x] = '@'
    return rendered(canvas)
end

function generate_map(; width::Int = 35, height::Int = 15, research_cost::Int = 6)
    width >= 25 || throw(ArgumentError("width must be at least 25"))
    height >= 11 || throw(ArgumentError("height must be at least 11"))
    isodd(width) || throw(ArgumentError("width must be odd"))
    isodd(height) || throw(ArgumentError("height must be odd"))
    research_cost >= 0 || throw(ArgumentError("research_cost cannot be negative"))

    middle_x = (width + 1) ÷ 2
    middle_y = (height + 1) ÷ 2
    crossing_x = middle_x + 5
    fork_x = middle_x - 5
    direct_y = middle_y + 3
    bottom_y = height - 1

    west = Point(2, middle_y)
    north = Point(crossing_x, 2)
    east = Point(width - 1, middle_y)
    south = Point(middle_x, bottom_y)
    crossing = Point(crossing_x, middle_y)
    fork = Point(fork_x, middle_y)

    direct = joined(
        horizontal(middle_y, west.x, fork_x),
        vertical(fork_x, middle_y, direct_y),
        horizontal(direct_y, fork_x, middle_x),
        vertical(middle_x, direct_y, bottom_y),
    )
    lanes = Dict(
        "ridge" => horizontal(middle_y, west.x, east.x),
        "steps" => joined(
            vertical(crossing_x, north.y, bottom_y),
            horizontal(bottom_y, crossing_x, south.x),
        ),
        "glide-a" => direct,
        "glide-b" => copy(direct),
    )

    memberships = Dict{Point,Set{String}}()
    for (lane_id, points) in lanes
        for point in points
            push!(get!(memberships, point, Set{String}()), lane_id)
        end
    end
    topology_points = Set(keys(memberships))
    cities = Set([west, north, east, south])

    terrain = Dict{Point,Symbol}()
    for point in horizontal(direct_y, fork_x + 1, middle_x)
        terrain[point] = :ravine
    end
    for point in vertical(crossing_x, middle_y + 1, bottom_y)
        terrain[point] = :steps
    end

    topology_canvas = blank_canvas(width, height)
    for point in topology_points
        topology_canvas[point.y][point.x] = classify_topology(point, memberships, cities)
    end

    terrain_canvas = blank_canvas(width, height)
    for (point, terrain_kind) in terrain
        terrain_canvas[point.y][point.x] = terrain_kind == :ravine ? '~' : '^'
    end
    for city in cities
        terrain_canvas[city.y][city.x] = 'O'
    end

    current, current_cost = shortest_route(
        topology_points,
        west,
        south,
        terrain,
        Set([:walk, :climb]),
    )
    optimum, optimum_travel_cost = shortest_route(
        topology_points,
        west,
        south,
        terrain,
        Set([:walk, :climb, :glide]),
    )

    labels = [
        LabelAnchor("i(a)", west, "player and active runner capabilities"),
        LabelAnchor("i(b)", north, "map, terrain, and technology parameters"),
        LabelAnchor("f(a)", fork, "evaluate terrain against active capability"),
        LabelAnchor("f(b)", crossing, "compare current and researched routes"),
        LabelAnchor("o(c)", south, "verified city-to-city route and score input"),
    ]

    return GeneratedMap(
        width,
        height,
        rendered(topology_canvas),
        rendered(terrain_canvas),
        route_layer(width, height, current, cities, west),
        route_layer(width, height, optimum, cities, west),
        labels,
        terrain_decisions(current, terrain, Set([:walk, :climb])),
        terrain_decisions(optimum, terrain, Set([:walk, :climb, :glide])),
        current_cost,
        optimum_travel_cost,
        research_cost,
    )
end

function render(generated::GeneratedMap)
    sections = String[]
    for (name, rows) in (
        ("TOPOLOGY", generated.topology),
        ("TERRAIN", generated.terrain),
        ("CURRENT ROUTE", generated.current_route),
        ("OPTIMUM ROUTE AFTER GLIDE SHOES", generated.optimum_route),
    )
        push!(sections, name)
        append!(sections, rows)
        push!(sections, "")
    end

    push!(sections, "LABELS")
    for label in generated.labels
        push!(
            sections,
            "$(label.notation) @ ($(label.point.x),$(label.point.y)): $(label.description)",
        )
    end
    push!(sections, "")
    push!(sections, "TERRAIN NEGOTIATION")
    for (route_name, decisions) in (
        ("current", generated.current_decisions),
        ("optimum", generated.optimum_decisions),
    )
        for decision in decisions
            push!(
                sections,
                "$route_name @ ($(decision.point.x),$(decision.point.y)): " *
                "$(decision.terrain) + $(decision.capability) -> " *
                "$(decision.outcome), cost=$(decision.cost)",
            )
        end
    end
    push!(sections, "")
    push!(sections, "COSTS")
    push!(sections, "current=$(generated.current_cost)")
    push!(sections, "glide-travel=$(generated.optimum_travel_cost)")
    push!(sections, "glide-research=$(generated.research_cost)")
    push!(
        sections,
        "glide-total=$(generated.optimum_travel_cost + generated.research_cost)",
    )
    push!(sections, "")
    push!(sections, "LEGEND")
    push!(sections, "O city or powerup; . one lane; + crossing; = two shared lanes; E three shared lanes")
    push!(sections, "~ glide-gated ravine; ^ stepped terrain; @ player; * selected route")
    return join(sections, '\n')
end

function write_layer(path::String, rows::Vector{String})
    open(path, "w") do stream
        for row in rows
            println(stream, row)
        end
    end
end

function pixel_color(topology::Char, terrain::Char)
    topology == 'O' && return (226, 167, 59)
    terrain == '~' && return (45, 151, 191)
    terrain == '^' && return (118, 100, 69)
    topology == 'E' && return (80, 171, 119)
    topology == '=' && return (99, 142, 189)
    topology == '+' && return (220, 220, 220)
    topology == '.' && return (166, 166, 166)
    return (35, 43, 52)
end

function write_ppm(path::String, generated::GeneratedMap, pixels_per_cell::Int)
    image_width = generated.width * pixels_per_cell
    image_height = generated.height * pixels_per_cell
    open(path, "w") do stream
        println(stream, "P3")
        println(stream, "# generated by demos/pathing/proto_map.jl")
        println(stream, "$image_width $image_height")
        println(stream, "255")
        for row_index in 1:generated.height
            for _ in 1:pixels_per_cell
                for column_index in 1:generated.width
                    color = pixel_color(
                        generated.topology[row_index][column_index],
                        generated.terrain[row_index][column_index],
                    )
                    for _ in 1:pixels_per_cell
                        print(stream, "$(color[1]) $(color[2]) $(color[3]) ")
                    end
                end
                println(stream)
            end
        end
    end
end

function fixture_manifest(generated::GeneratedMap, pixels_per_cell::Int)
    return """
schema = "pathing-ascii-map-v1"
map_id = "city-pass"
width = $(generated.width)
height = $(generated.height)
image = "city-pass.ppm"
image_width = $(generated.width * pixels_per_cell)
image_height = $(generated.height * pixels_per_cell)

[scale]
pixels_per_cell_x = $pixels_per_cell
pixels_per_cell_y = $pixels_per_cell

[topology]
"O" = "city or powerup"
"." = "one lane"
"+" = "crossing lanes"
"=" = "two shared lanes"
"E" = "three shared lanes"

[terrain]
"O" = "city or powerup location"
"~" = "glide-gated ravine"
"^" = "stepped terrain"

[route]
"O" = "city or powerup endpoint"
"@" = "player"
"*" = "selected route"
"""
end

function write_fixture(directory::AbstractString; generated::GeneratedMap = generate_map(), pixels_per_cell::Int = 4)
    pixels_per_cell > 0 || throw(ArgumentError("pixels_per_cell must be positive"))
    mkpath(directory)
    write_layer(joinpath(directory, "topology.txt"), generated.topology)
    write_layer(joinpath(directory, "terrain.txt"), generated.terrain)
    write_layer(joinpath(directory, "current-route.txt"), generated.current_route)
    write_layer(joinpath(directory, "optimum-route.txt"), generated.optimum_route)
    write_ppm(joinpath(directory, "city-pass.ppm"), generated, pixels_per_cell)
    write(joinpath(directory, "map.manifest.toml"), fixture_manifest(generated, pixels_per_cell))
    return validate_fixture(directory)
end

function ppm_dimensions(path::String)
    tokens = String[]
    open(path, "r") do stream
        for line in eachline(stream)
            startswith(line, "#") && continue
            append!(tokens, split(line))
            length(tokens) >= 4 && break
        end
    end
    length(tokens) >= 4 && tokens[1] == "P3" || error("expected P3 PPM image at '$path'")
    return parse(Int, tokens[2]), parse(Int, tokens[3]), parse(Int, tokens[4])
end

function validate_fixture(directory::AbstractString)
    manifest = TOML.parsefile(joinpath(directory, "map.manifest.toml"))
    manifest["schema"] == "pathing-ascii-map-v1" || error("unexpected map schema")
    width = manifest["width"]
    height = manifest["height"]
    layer_rules = Dict(
        "topology.txt" => Set(keys(manifest["topology"])),
        "terrain.txt" => Set(keys(manifest["terrain"])),
        "current-route.txt" => Set(keys(manifest["route"])),
        "optimum-route.txt" => Set(keys(manifest["route"])),
    )
    for (name, glyphs) in layer_rules
        rows = readlines(joinpath(directory, name))
        length(rows) == height || error("$name height does not match manifest")
        all(length(row) == width for row in rows) || error("$name width does not match manifest")
        all(character == ' ' || string(character) in glyphs for row in rows for character in row) ||
            error("$name contains a glyph not declared in the manifest")
    end
    image_width, image_height, max_value = ppm_dimensions(joinpath(directory, manifest["image"]))
    scale = manifest["scale"]
    image_width == manifest["image_width"] == width * scale["pixels_per_cell_x"] ||
        error("image width does not align with map cells")
    image_height == manifest["image_height"] == height * scale["pixels_per_cell_y"] ||
        error("image height does not align with map cells")
    max_value == 255 || error("PPM max value must be 255")
    return Dict(
        "schema" => manifest["schema"],
        "width" => width,
        "height" => height,
        "image_width" => image_width,
        "image_height" => image_height,
    )
end

function main(arguments = ARGS)
    width = 35
    height = 15
    research_cost = 6
    fixture_directory = nothing
    for argument in arguments
        if startswith(argument, "--width=")
            width = parse(Int, split(argument, "="; limit = 2)[2])
        elseif startswith(argument, "--height=")
            height = parse(Int, split(argument, "="; limit = 2)[2])
        elseif startswith(argument, "--research-cost=")
            research_cost = parse(Int, split(argument, "="; limit = 2)[2])
        elseif startswith(argument, "--write-fixture=")
            fixture_directory = split(argument, "="; limit = 2)[2]
        else
            throw(ArgumentError("unknown argument '$argument'"))
        end
    end
    generated = generate_map(; width, height, research_cost)
    if isnothing(fixture_directory)
        println(render(generated))
    else
        result = write_fixture(fixture_directory; generated)
        println("wrote ", result["schema"], " fixture to ", fixture_directory)
    end
end

end

if abspath(PROGRAM_FILE) == @__FILE__
    using .PathingProtoMap
    PathingProtoMap.main()
end