"""
Generator.jl — Sudoku puzzle package generator.

This module knows Sudoku rules. recur knows nothing about Sudoku.
Generator writes flow event files in the eventness format.
recur classifies what it finds in those files.

Separation contract:
  Generator: peer groups, constraint names, flow file authoring
  recur:     classification of what Generator wrote
"""

module Generator

using JSON3
using Random: shuffle!

# ---------------------------------------------------------------------------
# Sudoku geometry (pure Sudoku rules — no recur knowledge)
# ---------------------------------------------------------------------------

"""Box number (1-indexed, left-to-right, top-to-bottom) for a cell."""
function box_of(row::Int, col::Int)::Int
    return ((row - 1) ÷ 3) * 3 + ((col - 1) ÷ 3) + 1
end

"""All cells in the same row as (row, col), excluding self."""
function row_peers(row::Int, col::Int)
    return [(row, c) for c in 1:9 if c != col]
end

"""All cells in the same column as (row, col), excluding self."""
function col_peers(row::Int, col::Int)
    return [(r, col) for r in 1:9 if r != row]
end

"""All cells in the same 3x3 box as (row, col), excluding self."""
function box_peers(row::Int, col::Int)
    box_row_start = ((row - 1) ÷ 3) * 3 + 1
    box_col_start = ((col - 1) ÷ 3) * 3 + 1
    peers = Tuple{Int,Int}[]
    for r in box_row_start:(box_row_start + 2)
        for c in box_col_start:(box_col_start + 2)
            (r == row && c == col) || push!(peers, (r, c))
        end
    end
    return peers
end

"""All unique peers of (row, col) across row, column, and box."""
function all_peers(row::Int, col::Int)
    return collect(Set(vcat(row_peers(row, col), col_peers(row, col), box_peers(row, col))))
end

"""Canonical cell identifier string."""
cell_id(row::Int, col::Int) = "sudoku.r$(row).c$(col)"

# ---------------------------------------------------------------------------
# Solution generation (random valid Sudoku grid)
# ---------------------------------------------------------------------------

"""
    candidates(grid, row, col) -> Vector{Int}

Return which values (1-9) can legally go in grid[row, col].
"""
function candidates(grid::Matrix{Int}, row::Int, col::Int)::Vector{Int}
    grid[row, col] != 0 && return Int[]
    taken = Set{Int}()
    for c in 1:9; v = grid[row, c]; v != 0 && push!(taken, v); end
    for r in 1:9; v = grid[r, col]; v != 0 && push!(taken, v); end
    br = ((row - 1) ÷ 3) * 3 + 1
    bc = ((col - 1) ÷ 3) * 3 + 1
    for r in br:(br+2), c in bc:(bc+2)
        v = grid[r, c]; v != 0 && push!(taken, v)
    end
    return [v for v in 1:9 if v ∉ taken]
end

"""
    generate_solution() -> Matrix{Int}

Generate a random valid 9x9 Sudoku solution using backtracking
with randomized candidate ordering. Returns a 9x9 matrix.
"""
function generate_solution()::Matrix{Int}
    grid = zeros(Int, 9, 9)
    _fill!(grid, 1)
    return grid
end

function _fill!(grid::Matrix{Int}, pos::Int)::Bool
    pos > 81 && return true
    row = (pos - 1) ÷ 9 + 1
    col = (pos - 1) % 9 + 1
    cands = candidates(grid, row, col)
    shuffle!(cands)
    for v in cands
        grid[row, col] = v
        _fill!(grid, pos + 1) && return true
    end
    grid[row, col] = 0
    return false
end

"""
    solution_to_pairs(grid::Matrix{Int}) -> Vector{Tuple{String, Int}}

Convert a 9x9 solution matrix to the (cell_id, value) pairs format
used by generate_cascades.
"""
function solution_to_pairs(grid::Matrix{Int})
    pairs = Tuple{String, Int}[]
    for r in 1:9, c in 1:9
        push!(pairs, (cell_id(r, c), grid[r, c]))
    end
    return pairs
end

"""
    write_solution_file(grid::Matrix{Int}, dir::String) -> String

Write a solution file in the standard format. Returns the file path.
"""
function write_solution_file(grid::Matrix{Int}, dir::String)::String
    mkpath(dir)
    lines = String["# Generated Sudoku solution", "# Format: identifier = value", ""]
    for r in 1:9, c in 1:9
        push!(lines, "$(cell_id(r, c)) = $(grid[r, c])")
    end
    path = joinpath(dir, "sudoku.solution.txt")
    write(path, join(lines, "\n") * "\n")
    return path
end

"""
    generate_full_puzzle(output_dir, recur_module) -> Dict

Full pipeline: generate random solution → write flow events → run recur → write JSON.
Returns Dict with "solution" (81 pairs) and "cascades_path".
"""
function generate_full_puzzle(output_dir::String, recur_module)
    grid = generate_solution()
    pairs = solution_to_pairs(grid)

    # Write solution file
    mkpath(output_dir)
    write_solution_file(grid, output_dir)

    # Generate all cascade data (calls recur 81 times)
    cascades_path = generate_cascades(pairs, output_dir, recur_module)

    return Dict(
        "solution" => pairs,
        "cascades_path" => cascades_path,
    )
end

# ---------------------------------------------------------------------------
# Flow event file authoring
# ---------------------------------------------------------------------------

"""
    write_flow_event(row, col, value, dir) -> String

Write a flow event file for placing `value` at `(row, col)`.
Returns the path of the written file.

File format (every line references the cell identifier):
  <id> = <value>                       → define site
  <id> publish row.<r>.constraint      → produce: row constraint published
  <id> publish col.<c>.constraint      → produce: col constraint published
  <id> publish box.<b>.constraint      → produce: box constraint published
  <peer_id> subscribe <id>             → consume: each peer subscribes
  <id> trigger solve                   → trigger: placement triggers solve
"""
function write_flow_event(row::Int, col::Int, value::Int, dir::String)::String
    id  = cell_id(row, col)
    box = box_of(row, col)

    lines = String[
        "$(id) = $(value)",
        "$(id) publish row.$(row).constraint",
        "$(id) publish col.$(col).constraint",
        "$(id) publish box.$(box).constraint",
    ]

    # Every peer subscribes — sorted for deterministic output
    peers = sort(all_peers(row, col))
    for (pr, pc) in peers
        push!(lines, "$(cell_id(pr, pc)) subscribe $(id)")
    end

    push!(lines, "$(id) trigger solve")

    filename = "sudoku.flow.r$(row)c$(col).txt"
    filepath = joinpath(dir, filename)
    write(filepath, join(lines, "\n") * "\n")
    return filepath
end

# ---------------------------------------------------------------------------
# Cascade generation
# ---------------------------------------------------------------------------

"""
    generate_cascade(row, col, value, dir, recur_module) -> Dict or nothing

Write flow event for (row, col, value), call recur trace-id, return
a Dict with keys: cell, value, cascade (the trace-id JSON result).
"""
function generate_cascade(row::Int, col::Int, value::Int, dir::String, recur_module)
    write_flow_event(row, col, value, dir)
    id = cell_id(row, col)

    cascade = recur_module.trace_id(
        id;
        dir   = dir,
        scope = "sudoku.**",
        ext   = ".txt",
    )

    cascade === nothing && return nothing

    return Dict(
        "cell"    => id,
        "value"   => value,
        "cascade" => cascade,
    )
end

"""
    generate_cascades(solution, puzzle_dir, recur_module) -> String

Generate cascade JSON for each cell in `solution` and write
`sudoku.cascades.json` to `puzzle_dir`. Returns the output path.

`solution` is a vector of `(cell_id_string, value)` pairs,
e.g. `[("sudoku.r1.c1", 5), ("sudoku.r3.c5", 7)]`.
"""
function generate_cascades(solution, puzzle_dir::String, recur_module)::String
    cascades = []

    for (id_str, value) in solution
        m = match(r"sudoku\.r(\d+)\.c(\d+)", id_str)
        m === nothing && continue
        row = parse(Int, m[1])
        col = parse(Int, m[2])

        entry = generate_cascade(row, col, value, puzzle_dir, recur_module)
        entry !== nothing && push!(cascades, entry)
    end

    out_path = joinpath(puzzle_dir, "sudoku.cascades.json")
    open(out_path, "w") do f
        JSON3.write(f, cascades)
    end
    return out_path
end

end # module
