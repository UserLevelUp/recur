"""
generate.jl — Regenerate cascade JSON for the HTML5 Sudoku demo.

Usage:
  julia demos/sudoku/html5/generate.jl

This script:
  1. Reads the solution from puzzles/easy-001/
  2. Calls recur trace-id for each cell (81 calls)
  3. Writes sudoku.cascades.json into html5/data/easy-001/

Prerequisites:
  - recur must be installed and on PATH
  - Run from the project root (c:\\src\\recur)

This is the exact pipeline that produced the data this game reads.
Once you understand it, you can build your own cascade for any domain.
"""

# Resolve paths relative to this script
const SCRIPT_DIR = @__DIR__
const PROJECT_ROOT = normpath(joinpath(SCRIPT_DIR, "..", "..", ".."))
const PUZZLE_DIR = normpath(joinpath(SCRIPT_DIR, "..", "puzzles", "easy-001"))
const OUTPUT_DIR = normpath(joinpath(SCRIPT_DIR, "data", "easy-001"))

# Load the demo modules
push!(LOAD_PATH, normpath(joinpath(SCRIPT_DIR, "..", "julia")))
include(joinpath(SCRIPT_DIR, "..", "julia", "Recur.jl"))
include(joinpath(SCRIPT_DIR, "..", "julia", "Generator.jl"))

using .Recur
using .Generator

println("=== Sudoku Cascade Generator ===")
println()
println("Step 1: Reading solution from $(PUZZLE_DIR)")

# Parse solution
solution_path = joinpath(PUZZLE_DIR, "sudoku.solution.txt")
solution = []
for line in readlines(solution_path)
    line = strip(line)
    isempty(line) && continue
    startswith(line, "#") && continue
    m = match(r"^(sudoku\.r\d+\.c\d+)\s*=\s*(\d+)$", line)
    m === nothing && continue
    push!(solution, (m[1], parse(Int, m[2])))
end
println("  Found $(length(solution)) cells")

println()
println("Step 2: Generating flow events + running recur trace-id for each cell")
println("  This calls: recur trace-id \"<cell>\" --scope \"sudoku.**\" --json")
println()

# Generate cascades (this calls recur for each cell)
out_path = Generator.generate_cascades(solution, PUZZLE_DIR, Recur)

# Copy to html5/data
mkpath(OUTPUT_DIR)
cp(out_path, joinpath(OUTPUT_DIR, "sudoku.cascades.json"); force=true)

# Also copy the solution for the game
cp(solution_path, joinpath(OUTPUT_DIR, "sudoku.solution.txt"); force=true)

println()
println("Step 3: Output written to $(OUTPUT_DIR)")
println("  - sudoku.cascades.json ($(filesize(joinpath(OUTPUT_DIR, "sudoku.cascades.json"))) bytes)")
println("  - sudoku.solution.txt")
println()
println("Done! Start the game: julia demos/sudoku/html5/serve.jl")
println("Open: http://localhost:8787")
