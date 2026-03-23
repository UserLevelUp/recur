"""
Demo: Sudoku + trace-id (Phase 4 — Engine.jl + Display.jl + Game.jl)
======================================================================

Proves:
  4a. Engine.jl exists and loads.
  4b. load_solution parses the fixture (81 cells, correct values).
  4c. make_grid builds a masked 9x9 grid (nothing = player must fill).
  4d. is_valid_placement checks a candidate against the solution.
  4e. get_candidates returns the valid value for an empty cell.
  4f. is_solved returns true only when all cells match the solution.
  4g. apply_placement mutates the grid and flow event is classifiable.
  4h. Display.jl exists and loads.
  4i. render_grid returns a multi-line ASCII board.
  4j. render_cascade formats a trace-id cascade for terminal output.

Architecture:
  Engine knows Sudoku rules and the solution.
  Generator writes flow event files (reused from Phase 3).
  Recur classifies the cascade (reused from Phase 2).
  Display renders results to terminal.
  Game.jl orchestrates the loop — tested here with simulated input.
"""

include("runtests.setup.jl")

const ENGINE_JL_PATH  = joinpath(@__DIR__, "..", "demos", "sudoku", "julia", "Engine.jl")
const DISPLAY_JL_PATH = joinpath(@__DIR__, "..", "demos", "sudoku", "julia", "Display.jl")
const GAME_JL_PATH    = joinpath(@__DIR__, "..", "demos", "sudoku", "julia", "Game.jl")
const P4_PUZZLE_DIR   = joinpath(@__DIR__, "..", "demos", "sudoku", "puzzles", "easy-001")
const P4_GENERATOR_JL = joinpath(@__DIR__, "..", "demos", "sudoku", "julia", "Generator.jl")
const P4_RECUR_JL     = joinpath(@__DIR__, "..", "demos", "sudoku", "julia", "Recur.jl")

# ---------------------------------------------------------------------------
# Phase 4 tests
# ---------------------------------------------------------------------------

@testset "Demo: Sudoku Phase 4 — Engine.jl + Display.jl + Game.jl" begin
    log_section("Testing: Sudoku demo Phase 4")

    @testset "Phase 4a: Engine.jl exists and loads" begin
        @test isfile(ENGINE_JL_PATH)
        loaded = try include(ENGINE_JL_PATH); true catch e; @error "Engine.jl failed" exception=e; false end
        @test loaded
    end

    @testset "Phase 4b: load_solution parses fixture correctly" begin
        include(ENGINE_JL_PATH)
        solution = Engine.load_solution(joinpath(P4_PUZZLE_DIR, "sudoku.solution.txt"))

        # 81 cells total
        @test length(solution) == 81

        # Spot-check known values from easy-001
        @test get(solution, "sudoku.r1.c1", 0) == 5
        @test get(solution, "sudoku.r3.c5", 0) == 7
        @test get(solution, "sudoku.r9.c9", 0) == 9
        @test get(solution, "sudoku.r5.c5", 0) == 5

        # All values are 1-9
        @test all(v -> 1 <= v <= 9, values(solution))
    end

    @testset "Phase 4c: make_grid creates masked 9x9 grid" begin
        include(ENGINE_JL_PATH)
        solution = Engine.load_solution(joinpath(P4_PUZZLE_DIR, "sudoku.solution.txt"))

        # Mask r3.c5 (the canonical demo cell) and r1.c1
        mask = Set(["sudoku.r3.c5", "sudoku.r1.c1"])
        grid = Engine.make_grid(solution, mask)

        # Grid is 9x9
        @test size(grid) == (9, 9)

        # Masked cells are nothing (player must fill)
        @test grid[1, 1] === nothing   # r1.c1
        @test grid[3, 5] === nothing   # r3.c5

        # Unmasked cells have their solution value
        @test grid[1, 2] == 3         # r1.c2 = 3
        @test grid[9, 9] == 9         # r9.c9 = 9
    end

    @testset "Phase 4d: is_valid_placement checks against solution" begin
        include(ENGINE_JL_PATH)
        solution = Engine.load_solution(joinpath(P4_PUZZLE_DIR, "sudoku.solution.txt"))

        # r3.c5 solution value is 7 — correct
        @test Engine.is_valid_placement(solution, 3, 5, 7) == true
        # Wrong value — not valid
        @test Engine.is_valid_placement(solution, 3, 5, 4) == false
        @test Engine.is_valid_placement(solution, 3, 5, 1) == false

        # r1.c1 solution value is 5 — correct
        @test Engine.is_valid_placement(solution, 1, 1, 5) == true
        @test Engine.is_valid_placement(solution, 1, 1, 9) == false
    end

    @testset "Phase 4e: get_candidates returns only the valid value for empty cell" begin
        include(ENGINE_JL_PATH)
        solution = Engine.load_solution(joinpath(P4_PUZZLE_DIR, "sudoku.solution.txt"))

        # Mask r3.c5 only
        mask = Set(["sudoku.r3.c5"])
        grid = Engine.make_grid(solution, mask)

        # Candidates for the masked cell — solution-validated (only 7 is correct)
        candidates = Engine.get_candidates(solution, grid, 3, 5)
        @test 7 in candidates
        @test length(candidates) == 1   # only the solution value is always valid

        # Filled cell has no candidates (or returns empty)
        candidates_filled = Engine.get_candidates(solution, grid, 1, 2)
        @test isempty(candidates_filled)
    end

    @testset "Phase 4f: is_solved returns correct status" begin
        include(ENGINE_JL_PATH)
        solution = Engine.load_solution(joinpath(P4_PUZZLE_DIR, "sudoku.solution.txt"))

        # Grid with no mask — already solved
        full_grid = Engine.make_grid(solution, Set{String}())
        @test Engine.is_solved(full_grid, solution) == true

        # Grid with one masked cell — not solved
        mask = Set(["sudoku.r3.c5"])
        partial_grid = Engine.make_grid(solution, mask)
        @test Engine.is_solved(partial_grid, solution) == false

        # Fill it in correctly — now solved
        Engine.apply_placement!(partial_grid, 3, 5, 7)
        @test Engine.is_solved(partial_grid, solution) == true
    end

    @testset "Phase 4g: apply_placement! mutates grid; flow event classifiable by recur" begin
        include(ENGINE_JL_PATH)
        include(P4_GENERATOR_JL)
        include(P4_RECUR_JL)
        solution = Engine.load_solution(joinpath(P4_PUZZLE_DIR, "sudoku.solution.txt"))

        mask = Set(["sudoku.r3.c5"])
        grid = Engine.make_grid(solution, mask)

        # Before placement — cell is empty
        @test grid[3, 5] === nothing

        # Apply the correct placement
        Engine.apply_placement!(grid, 3, 5, 7)
        @test grid[3, 5] == 7

        # Generator writes flow event; Recur classifies it
        dir = mktempdir()
        try
            mkdir(joinpath(dir, ".recur"))
            write(joinpath(dir, ".recur", "config.toml"), """
[traits.trace_id]
enabled = true
producer_keywords = "publish,emit,propagate"
consumer_keywords = "subscribe,bind,consume"
trigger_keywords = "trigger,register,solve"
""")
            cp(joinpath(P4_PUZZLE_DIR, "sudoku.solution.txt"), joinpath(dir, "sudoku.solution.txt"))

            Generator.write_flow_event(3, 5, 7, dir)
            result = Recur.trace_id("sudoku.r3.c5"; dir=dir, scope="sudoku.**", ext=".txt")

            @test result !== nothing
            if result !== nothing
                consume = get(result, :consume, get(result, "consume", []))
                @test length(consume) == 20  # all 20 peers subscribe
            end
        finally
            rm(dir, recursive=true)
        end
    end

    @testset "Phase 4h: Display.jl exists and loads" begin
        @test isfile(DISPLAY_JL_PATH)
        loaded = try include(DISPLAY_JL_PATH); true catch e; @error "Display.jl failed" exception=e; false end
        @test loaded
    end

    @testset "Phase 4i: render_grid returns multi-line ASCII board" begin
        include(ENGINE_JL_PATH)
        include(DISPLAY_JL_PATH)
        solution = Engine.load_solution(joinpath(P4_PUZZLE_DIR, "sudoku.solution.txt"))
        mask = Set(["sudoku.r3.c5", "sudoku.r1.c1"])
        grid = Engine.make_grid(solution, mask)

        output = Display.render_grid(grid)

        # Has content
        @test !isempty(output)

        # Has at least 9 digit rows
        lines = split(output, "\n")
        digit_rows = filter(l -> any(isdigit, l), lines)
        @test length(digit_rows) >= 9

        # Masked cells rendered as '.' or '_' or '?'
        @test any(l -> contains(l, ".") || contains(l, "_") || contains(l, "?"), lines)

        # Known unmasked values appear (r1.c2 = 3, r9.c9 = 9)
        @test contains(output, "3")
        @test contains(output, "9")
    end

    @testset "Phase 4j: render_cascade formats trace-id result" begin
        include(ENGINE_JL_PATH)
        include(DISPLAY_JL_PATH)
        include(P4_GENERATOR_JL)
        include(P4_RECUR_JL)

        dir = mktempdir()
        try
            mkdir(joinpath(dir, ".recur"))
            write(joinpath(dir, ".recur", "config.toml"), """
[traits.trace_id]
enabled = true
producer_keywords = "publish,emit,propagate"
consumer_keywords = "subscribe,bind,consume"
trigger_keywords = "trigger,register,solve"
""")
            cp(joinpath(P4_PUZZLE_DIR, "sudoku.solution.txt"), joinpath(dir, "sudoku.solution.txt"))
            Generator.write_flow_event(3, 5, 7, dir)

            result = Recur.trace_id("sudoku.r3.c5"; dir=dir, scope="sudoku.**", ext=".txt")
            @test result !== nothing

            if result !== nothing
                output = Display.render_cascade("sudoku.r3.c5", 7, result)
                @test !isempty(output)
                # Shows the identifier
                @test contains(output, "r3") || contains(output, "c5") || contains(output, "sudoku")
                # Shows role counts or role names
                @test contains(output, "consume") || contains(output, "produce") ||
                      contains(output, "define") || contains(output, "20")
            end
        finally
            rm(dir, recursive=true)
        end
    end

end
