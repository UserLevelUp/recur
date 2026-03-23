"""
Game.jl — Sudoku game loop.

Orchestrates: Engine validates → Generator writes flow → Recur classifies
→ Display renders. Human input drives the loop.

recur stays pure: it classifies file content, knows nothing about Sudoku.
"""

module Game

include("Engine.jl")
include("Generator.jl")
include("Recur.jl")
include("Display.jl")

using .Engine, .Generator, .Recur, .Display

"""
    run_game(puzzle_dir; mask=nothing, io=stdin)

Interactive Sudoku game loop.

- `puzzle_dir`: directory containing sudoku.solution.txt and .recur/config.toml
- `mask`: Set of cell identifiers to blank out; defaults to a small demo set
- `io`: input stream (override for testing)

Each turn:
  1. Show the board.
  2. Prompt for row, col, value.
  3. Validate against solution (Engine).
  4. Write flow event (Generator).
  5. Classify cascade (Recur).
  6. Render cascade (Display).
  7. Repeat until solved or quit.
"""
function run_game(puzzle_dir::String; mask=nothing, io=stdin)
    solution_path = joinpath(puzzle_dir, "sudoku.solution.txt")
    solution = Engine.load_solution(solution_path)

    if mask === nothing
        # Default demo mask: a few cells in the easy-001 puzzle
        mask = Set([
            "sudoku.r3.c5",
            "sudoku.r1.c1",
            "sudoku.r5.c5",
            "sudoku.r7.c9",
            "sudoku.r9.c1",
        ])
    end

    grid = Engine.make_grid(solution, mask)
    remaining = copy(mask)

    println("\n  Sudoku — powered by recur trace-id")
    println("  Each placement externalises the cascade to the event log.\n")

    while !isempty(remaining)
        println(Display.render_grid(grid))
        println("  Empty cells: $(length(remaining))")
        print("  Enter row col value (e.g. 3 5 7), or 'q' to quit: ")

        line = readline(io)
        strip(line) == "q" && (println("  Goodbye."); break)

        parts = split(strip(line))
        if length(parts) != 3
            println("  Invalid input — expected: row col value")
            continue
        end

        row = tryparse(Int, parts[1])
        col = tryparse(Int, parts[2])
        val = tryparse(Int, parts[3])

        if row === nothing || col === nothing || val === nothing ||
           !(1 <= row <= 9) || !(1 <= col <= 9) || !(1 <= val <= 9)
            println("  Values must be integers 1–9.")
            continue
        end

        id = "sudoku.r$(row).c$(col)"
        if id ∉ remaining
            println("  That cell is already filled.")
            continue
        end

        if !Engine.is_valid_placement(solution, row, col, val)
            println("  Incorrect — try again.")
            continue
        end

        # Valid placement — apply locally
        Engine.apply_placement!(grid, row, col, val)
        delete!(remaining, id)

        # Generator writes flow event; Recur classifies it
        Generator.write_flow_event(row, col, val, puzzle_dir)
        cascade = Recur.trace_id(id; dir=puzzle_dir, scope="sudoku.**", ext=".txt")

        if cascade !== nothing
            println(Display.render_cascade(id, val, cascade))
        end
    end

    if Engine.is_solved(grid, solution)
        println(Display.render_grid(grid))
        println("\n  Puzzle solved! The cascade is complete.\n")
    end
end

end # module
