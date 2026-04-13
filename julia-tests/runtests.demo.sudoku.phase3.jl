"""
Demo: Sudoku + trace-id (Phase 3 — Generator.jl)
=================================================

Proves Generator.jl correctly:
  - Computes Sudoku peer groups (row/col/box) from rules alone
  - Writes properly-formed flow event files
  - Flow files classify correctly under recur trace-id
  - generate_cascade returns valid JSON for one placement
  - generate_cascades writes sudoku.cascades.json for a subset
  - stable saved runs persist and can be reused when inputs stay fresh

Generator knows Sudoku rules. recur knows nothing about Sudoku.
The separation is enforced by the file protocol: Generator writes,
recur classifies what it finds.
"""

include("runtests.setup.jl")

const GENERATOR_JL_PATH = joinpath(@__DIR__, "..", "demos", "sudoku", "julia", "Generator.jl")
const DEMO_PUZZLE_DIR   = joinpath(@__DIR__, "..", "demos", "sudoku", "puzzles", "easy-001")
const RECUR_JL_PATH     = joinpath(@__DIR__, "..", "demos", "sudoku", "julia", "Recur.jl")

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

function make_generator_test_dir()
    dir = mktempdir()
    # Config: Sudoku vocabulary
    mkdir(joinpath(dir, ".recur"))
    write(joinpath(dir, ".recur", "config.toml"), """
[traits.trace_id]
enabled = true
producer_keywords = "publish,emit,propagate"
consumer_keywords = "subscribe,bind,consume"
trigger_keywords = "trigger,register,solve"
notes = "Sudoku demo: classify move events by role"
""")
    # Solution: define sites for trace-id
    cp(joinpath(DEMO_PUZZLE_DIR, "sudoku.solution.txt"), joinpath(dir, "sudoku.solution.txt"))
    return dir
end

# ---------------------------------------------------------------------------
# Phase 3 tests
# ---------------------------------------------------------------------------

@testset "Demo: Sudoku Phase 3 — Generator.jl" begin
    log_section("Testing: Sudoku demo Phase 3")

    @testset "Phase 3a: Generator.jl exists and loads" begin
        @test isfile(GENERATOR_JL_PATH)
        loaded = try include(GENERATOR_JL_PATH); true catch e; @error "Generator.jl failed" exception=e; false end
        @test loaded
    end

    @testset "Phase 3b: box_of computes correct box numbers" begin
        include(GENERATOR_JL_PATH)
        # Standard Sudoku box layout (1-indexed, left-to-right, top-to-bottom):
        #   Box 1: r1-r3, c1-c3 | Box 2: r1-r3, c4-c6 | Box 3: r1-r3, c7-c9
        #   Box 4: r4-r6, c1-c3 | Box 5: r4-r6, c4-c6 | Box 6: r4-r6, c7-c9
        #   Box 7: r7-r9, c1-c3 | Box 8: r7-r9, c4-c6 | Box 9: r7-r9, c7-c9
        @test Generator.box_of(1, 1) == 1
        @test Generator.box_of(1, 4) == 2
        @test Generator.box_of(1, 7) == 3
        @test Generator.box_of(3, 5) == 2   # the handwritten fixture had box.5 — this is the fix
        @test Generator.box_of(5, 5) == 5
        @test Generator.box_of(9, 9) == 9
    end

    @testset "Phase 3c: peer groups are correct and exclude self" begin
        include(GENERATOR_JL_PATH)
        row_p = Generator.row_peers(3, 5)
        col_p = Generator.col_peers(3, 5)
        box_p = Generator.box_peers(3, 5)
        all_p = Generator.all_peers(3, 5)

        @test length(row_p) == 8
        @test length(col_p) == 8
        @test length(box_p) == 8
        @test !any(p -> p == (3, 5), row_p)
        @test !any(p -> p == (3, 5), col_p)
        @test !any(p -> p == (3, 5), box_p)
        @test !any(p -> p == (3, 5), all_p)
        # r3.c5 has 20 unique peers (8 row + 8 col + 4 box-only, no overlap between row+col)
        @test length(all_p) == 20
    end

    @testset "Phase 3d: write_flow_event writes correctly-formed file" begin
        include(GENERATOR_JL_PATH)
        dir = make_generator_test_dir()
        try
            path = Generator.write_flow_event(3, 5, 7, dir)
            @test isfile(path)

            content = read(path, String)
            lines = filter(!isempty, strip.(split(content, "\n")))

            # define line
            @test any(l -> l == "sudoku.r3.c5 = 7", lines)
            # publish lines — row, col, box (box 2 for r3.c5)
            @test any(l -> l == "sudoku.r3.c5 publish row.3.constraint", lines)
            @test any(l -> l == "sudoku.r3.c5 publish col.5.constraint", lines)
            @test any(l -> l == "sudoku.r3.c5 publish box.2.constraint", lines)  # box 2, not 5
            # subscribe lines — every peer references the identifier
            @test any(l -> contains(l, "subscribe sudoku.r3.c5"), lines)
            # trigger line
            @test any(l -> l == "sudoku.r3.c5 trigger solve", lines)

            # Every subscribe line must reference the identifier (so trace-id classifies it)
            subscribe_lines = filter(l -> contains(l, "subscribe"), lines)
            @test all(l -> contains(l, "sudoku.r3.c5"), subscribe_lines)
            @test length(subscribe_lines) == 20  # all unique peers
        finally
            rm(dir, recursive=true)
        end
    end

    @testset "Phase 3e: Generator flow file classifies correctly with recur trace-id" begin
        include(GENERATOR_JL_PATH)
        include(RECUR_JL_PATH)
        dir = make_generator_test_dir()
        try
            Generator.write_flow_event(3, 5, 7, dir)

            result = Recur.trace_id(
                "sudoku.r3.c5";
                dir = dir,
                scope = "sudoku.**",
                ext = ".txt",
            )

            @test result !== nothing
            if result !== nothing
                define  = get(result, :define,  get(result, "define",  []))
                produce = get(result, :produce, get(result, "produce", []))
                consume = get(result, :consume, get(result, "consume", []))
                trigger = get(result, :trigger, get(result, "trigger", []))

                @test length(define)  >= 1   # = 7 in solution + flow
                @test length(produce) >= 1   # publish lines
                @test length(consume) == 20  # all 20 peers subscribe
                @test length(trigger) >= 1   # trigger solve
            end
        finally
            rm(dir, recursive=true)
        end
    end

    @testset "Phase 3f: generate_cascade returns valid cascade JSON" begin
        include(GENERATOR_JL_PATH)
        include(RECUR_JL_PATH)
        dir = make_generator_test_dir()
        try
            result = Generator.generate_cascade(3, 5, 7, dir, Recur)

            @test result !== nothing
            if result !== nothing
                @test haskey(result, :cell)   || haskey(result, "cell")
                @test haskey(result, :value)  || haskey(result, "value")
                @test haskey(result, :cascade) || haskey(result, "cascade")

                cascade = get(result, :cascade, get(result, "cascade", nothing))
                @test cascade !== nothing
                if cascade !== nothing
                    @test haskey(cascade, :define)  || haskey(cascade, "define")
                    @test haskey(cascade, :produce) || haskey(cascade, "produce")
                    @test haskey(cascade, :consume) || haskey(cascade, "consume")
                    @test haskey(cascade, :trigger) || haskey(cascade, "trigger")
                end
            end
        finally
            rm(dir, recursive=true)
        end
    end

    @testset "Phase 3g: generate_cascades writes sudoku.cascades.json" begin
        include(GENERATOR_JL_PATH)
        include(RECUR_JL_PATH)
        dir = make_generator_test_dir()
        try
            # Two-cell mini solution for speed
            mini_solution = [
                ("sudoku.r1.c1", 5),
                ("sudoku.r3.c5", 7),
            ]
            out_path = Generator.generate_cascades(mini_solution, dir, Recur)

            @test isfile(out_path)
            parsed = try JSON3.read(read(out_path, String)); catch; nothing end
            @test parsed !== nothing
            if parsed !== nothing
                @test parsed isa AbstractVector
                @test length(parsed) == 2
                first_entry = parsed[1]
                @test haskey(first_entry, :cell)   || haskey(first_entry, "cell")
                @test haskey(first_entry, :value)  || haskey(first_entry, "value")
                @test haskey(first_entry, :cascade) || haskey(first_entry, "cascade")
            end
        finally
            rm(dir, recursive=true)
        end
    end

    @testset "Phase 3h: generate_cascade persists and reuses fresh saved runs" begin
        include(GENERATOR_JL_PATH)
        include(RECUR_JL_PATH)
        dir = make_generator_test_dir()
        try
            first = Generator.generate_cascade(3, 5, 7, dir, Recur)
            @test first !== nothing

            run_dir = joinpath(dir, ".recur", "trace-id", "runs", "sudoku.r3.c5")
            manifest_path = joinpath(run_dir, "manifest.toml")
            latest_json_path = joinpath(run_dir, "latest.json")

            @test isfile(manifest_path)
            @test isfile(latest_json_path)
            @test contains(read(manifest_path, String), "name = \"sudoku.r3.c5\"")

            latest_text = read(latest_json_path, String)
            cached_text = replace(
                replace(
                    latest_text,
                    "\"pattern\":\"sudoku.r3.c5\"" => "\"pattern\":\"cached.reuse.marker\"",
                ),
                "\"pattern\": \"sudoku.r3.c5\"" => "\"pattern\": \"cached.reuse.marker\"",
            )
            @test cached_text != latest_text
            write(latest_json_path, cached_text)

            reused = Generator.generate_cascade(3, 5, 7, dir, Recur)
            @test reused !== nothing

            if reused !== nothing
                cascade = get(reused, :cascade, get(reused, "cascade", nothing))
                request = cascade === nothing ? nothing : get(cascade, :request, get(cascade, "request", nothing))
                pattern = request === nothing ? nothing : String(get(request, :pattern, get(request, "pattern", "")))
                @test pattern == "cached.reuse.marker"
            end
        finally
            rm(dir, recursive=true)
        end
    end
end
