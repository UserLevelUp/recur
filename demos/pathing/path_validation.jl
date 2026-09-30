module PathValidation
export every_path_is_contiguous

value_at(value, key) = value isa AbstractDict ? get(value, key, get(value, Symbol(key), nothing)) : nothing
entries_at(value, key) = (v = value_at(value, key); v isa AbstractVector ? v : Any[])
function tile_key(tile)
    x, y = value_at(tile, "x"), value_at(tile, "y")
    (x isa Integer && y isa Integer) || return nothing
    (Int(x), Int(y))
end

"""Validate nontrivial routes against the declared undirected corridor graph."""
function every_path_is_contiguous(result)
    map = value_at(result, "map")
    graph = value_at(map, "graph")
    paths = entries_at(map, "paths")
    isempty(paths) && return false
    edges = Set{Tuple{Tuple{Int,Int},Tuple{Int,Int}}}()
    for edge in entries_at(graph, "corridors")
        left, right = tile_key(value_at(edge, "from")), tile_key(value_at(edge, "to"))
        (isnothing(left) || isnothing(right)) && return false
        push!(edges, (left, right), (right, left))
    end
    for path in paths
        tiles = [tile_key(tile) for tile in entries_at(path, "tiles")]
        (length(tiles) < 2 || any(isnothing, tiles)) && return false
        all((tiles[i], tiles[i+1]) in edges for i in 1:length(tiles)-1) || return false
    end
    true
end
end
