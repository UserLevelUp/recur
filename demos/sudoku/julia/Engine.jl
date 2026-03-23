"""
Engine.jl — Sudoku game engine.

Knows Sudoku rules and the solution. Validates placements against
the known solution (this is a guided puzzle, not a blind solver).

Separation contract:
  Engine: grid state, placement validation, candidate lookup, solved check
  Generator: flow event file authoring
  Recur: cascade classification
  Display: terminal rendering
"""

module Engine

# ---------------------------------------------------------------------------
# Solution loading
# ---------------------------------------------------------------------------

"""
    load_solution(path::String) -> Dict{String,Int}

Parse a sudoku.solution.txt file. Returns a map of cell identifier
(e.g. "sudoku.r3.c5") to integer value (1-9).

Only lines matching `<id> = <digit>` are parsed; comment lines are skipped.
"""
function load_solution(path::String)::Dict{String,Int}
    solution = Dict{String,Int}()
    for line in eachline(path)
        line = strip(line)
        isempty(line) && continue
        startswith(line, "#") && continue
        m = match(r"^(sudoku\.r\d+\.c\d+)\s*=\s*(\d+)$", line)
        m === nothing && continue
        solution[String(m[1])] = parse(Int, m[2])
    end
    return solution
end

# ---------------------------------------------------------------------------
# Grid construction
# ---------------------------------------------------------------------------

"""
    make_grid(solution, mask) -> Matrix{Union{Int,Nothing}}

Build a 9x9 grid from the solution. Cells in `mask` (a Set of cell
identifier strings) are set to `nothing` — the player must fill them.
"""
function make_grid(solution::Dict{String,Int}, mask)::Matrix{Union{Int,Nothing}}
    grid = Matrix{Union{Int,Nothing}}(undef, 9, 9)
    for row in 1:9
        for col in 1:9
            id = "sudoku.r$(row).c$(col)"
            if id in mask
                grid[row, col] = nothing
            else
                grid[row, col] = get(solution, id, nothing)
            end
        end
    end
    return grid
end

# ---------------------------------------------------------------------------
# Placement
# ---------------------------------------------------------------------------

"""
    is_valid_placement(solution, row, col, value) -> Bool

Return true if `value` matches the solution for `(row, col)`.
This is a guided puzzle — there is exactly one correct answer per cell.
"""
function is_valid_placement(solution::Dict{String,Int}, row::Int, col::Int, value::Int)::Bool
    id = "sudoku.r$(row).c$(col)"
    return get(solution, id, -1) == value
end

"""
    apply_placement!(grid, row, col, value)

Place `value` at `(row, col)` in the grid. Mutates in-place.
"""
function apply_placement!(grid::Matrix{Union{Int,Nothing}}, row::Int, col::Int, value::Int)
    grid[row, col] = value
end

# ---------------------------------------------------------------------------
# Candidates
# ---------------------------------------------------------------------------

"""
    get_candidates(solution, grid, row, col) -> Vector{Int}

Return valid candidates for the empty cell at `(row, col)`.
Since we validate against the known solution, the only candidate
is the solution value itself. Returns empty vector for filled cells.
"""
function get_candidates(solution::Dict{String,Int}, grid::Matrix{Union{Int,Nothing}},
                        row::Int, col::Int)::Vector{Int}
    grid[row, col] !== nothing && return Int[]
    id = "sudoku.r$(row).c$(col)"
    v = get(solution, id, nothing)
    v === nothing && return Int[]
    return [v]
end

# ---------------------------------------------------------------------------
# Solved check
# ---------------------------------------------------------------------------

"""
    is_solved(grid, solution) -> Bool

Return true if every cell in the grid matches the solution.
"""
function is_solved(grid::Matrix{Union{Int,Nothing}}, solution::Dict{String,Int})::Bool
    for row in 1:9
        for col in 1:9
            id = "sudoku.r$(row).c$(col)"
            grid[row, col] != get(solution, id, nothing) && return false
        end
    end
    return true
end

end # module
