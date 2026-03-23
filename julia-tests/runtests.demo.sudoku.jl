"""
Demo: Sudoku + trace-id (Phase 1 — File Protocol Proof)
========================================================

Proves that recur trace-id works as a pure discoverability engine
on plain-text eventness files — not just source code.

Key architectural principle: recur is data-driven, not biased.
The vocabulary (produce/consume/trigger keywords) is configured
per-project. recur sees identifiers and keywords in text — it
does not know what Sudoku is.

Phase 1 goals:
  1a. Sudoku eventness files are visible to recur file discovery.
  1b. trace-id finds define sites in plain .txt files (no code needed).
  1c. Default vocab (publish, subscribe) classifies produce/consume
      without any project config — defaults already include these.
  1d. 'trigger' keyword requires project-local config (not in defaults).
  1e. Full cascade (define/produce/consume/trigger) fires with config.
  1f. JSON site schema includes edge_type field per role.
"""

include("runtests.setup.jl")

const SUDOKU_DIR = "test_sudoku"

function seed_sudoku_fixture_no_config()
    isdir(SUDOKU_DIR) && rm(SUDOKU_DIR, recursive=true)
    mkdir(SUDOKU_DIR)

    # Explicit config with trace_id disabled — isolates Phase 1d from repo config.
    # Without this, load_nearest walks up to c:/src/recur/.recur/config.toml.
    # That repo config happens to not include 'trigger', but we don't want to
    # rely on that. Disabled = use pure defaults, not inherited repo vocabulary.
    mkdir(joinpath(SUDOKU_DIR, ".recur"))
    write(joinpath(SUDOKU_DIR, ".recur", "config.toml"), """
[traits.trace_id]
enabled = false
notes = "Phase 1a-1d: test default TraceIdPolicy, not repo config inheritance"
""")

    # sudoku.solution.txt — canonical cell values (define sites for trace-id)
    # Each line: identifier = value  →  trace-id classifies as 'define'
    write(joinpath(SUDOKU_DIR, "sudoku.solution.txt"), """
sudoku.r1.c1 = 5
sudoku.r1.c2 = 3
sudoku.r3.c5 = 7
sudoku.r9.c9 = 9
""")

    # sudoku.flow.r3c5.txt — move event: human places 7 at r3.c5
    # Every line that should be classified must reference the identifier.
    # publish  → produce  (default keyword — no config needed)
    # subscribe → consume (default keyword — no config needed)
    # trigger  → trigger  (NOT in defaults — requires project config)
    write(joinpath(SUDOKU_DIR, "sudoku.flow.r3c5.txt"), """
sudoku.r3.c5 = 7
sudoku.r3.c5 publish row.3.constraint
sudoku.r3.c5 publish col.5.constraint
sudoku.r7.c5 subscribe sudoku.r3.c5
sudoku.r1.c5 subscribe sudoku.r3.c5
sudoku.r3.c5 trigger solve
""")
end

function add_sudoku_config()
    mkpath(joinpath(SUDOKU_DIR, ".recur"))
    # Project-local trait config — adds 'trigger' to trigger_keywords.
    # publish/subscribe are redundant here (already defaults) but explicit
    # for portability: the demo ships its own vocabulary contract.
    write(joinpath(SUDOKU_DIR, ".recur", "config.toml"), """
[traits.trace_id]
enabled = true
producer_keywords = "publish,emit,propagate"
consumer_keywords = "subscribe,bind,consume"
trigger_keywords = "trigger,register,solve"
notes = "Sudoku demo: classify move events by role (publish/subscribe/trigger)"
""")
end

@testset "Demo: Sudoku + trace-id (Phase 1 — File Protocol)" begin
    log_section("Testing: Sudoku demo Phase 1")

    try
        # --- Phase 1a + 1b + 1c + 1d: no project config ---
        seed_sudoku_fixture_no_config()

        @testset "Phase 1a: Sudoku files are recur-visible" begin
            success, output, _ = run_recur(["files", "sudoku.**", "-d", SUDOKU_DIR])
            @test success
            parsed = try JSON3.read(output); catch; nothing end
            @test parsed !== nothing
            if parsed !== nothing
                paths = [String(p) for p in parsed]
                @test length(paths) >= 2
                @test any(p -> contains(p, "solution"), paths)
                @test any(p -> contains(p, "flow"), paths)
            end
        end

        @testset "Phase 1b: trace-id finds define sites in plain .txt files" begin
            # Proves: recur trace-id works on non-code files.
            # 'sudoku.r3.c5 = 7' in both .txt files → define sites.
            success, output, _ = run_recur([
                "trace-id", "sudoku.r3.c5",
                "--scope", "sudoku.**",
                "--ext", ".txt",
                "--json",
                "-d", SUDOKU_DIR,
            ])
            @test success
            parsed = try JSON3.read(output); catch; nothing end
            @test parsed !== nothing
            if parsed !== nothing
                define = get(parsed, :define, get(parsed, "define", []))
                @test length(define) >= 1
            end
        end

        @testset "Phase 1c: Default vocab classifies publish (produce) and subscribe (consume)" begin
            # Proves: 'publish' and 'subscribe' are in default TraceIdPolicy —
            # no project config required. recur classifies Sudoku flow events
            # using built-in vocabulary.
            success, output, _ = run_recur([
                "trace-id", "sudoku.r3.c5",
                "--scope", "sudoku.**",
                "--ext", ".txt",
                "--json",
                "-d", SUDOKU_DIR,
            ])
            @test success
            parsed = try JSON3.read(output); catch; nothing end
            @test parsed !== nothing
            if parsed !== nothing
                produce = get(parsed, :produce, get(parsed, "produce", []))
                consume = get(parsed, :consume, get(parsed, "consume", []))
                @test length(produce) >= 1   # "sudoku.r3.c5 publish ..." lines
                @test length(consume) >= 1   # "... subscribe sudoku.r3.c5" lines
            end
        end

        @testset "Phase 1d: 'trigger' keyword NOT classified without project config" begin
            # Proves: the default TraceIdPolicy does not include 'trigger' as a
            # trigger keyword. Project config is required to classify Sudoku's
            # 'trigger solve' lines. This is the data-driven contract — recur
            # is unbiased. Vocabulary is per-project, not hardcoded.
            success, output, _ = run_recur([
                "trace-id", "sudoku.r3.c5",
                "--scope", "sudoku.**",
                "--ext", ".txt",
                "--json",
                "-d", SUDOKU_DIR,
            ])
            @test success
            parsed = try JSON3.read(output); catch; nothing end
            @test parsed !== nothing
            if parsed !== nothing
                trigger = get(parsed, :trigger, get(parsed, "trigger", []))
                # Without config, 'trigger' is not a keyword → 0 trigger sites
                @test length(trigger) == 0
            end
        end

        # --- Phase 1e + 1f: add project-local config with Sudoku vocab ---
        add_sudoku_config()

        @testset "Phase 1e: Project config enables full cascade (all 4 roles)" begin
            # Proves: with .recur/config.toml in the demo dir, trace-id picks up
            # the Sudoku vocabulary and classifies all 4 roles correctly.
            # recur's -d directory drives config discovery — the demo ships its
            # own keyword contract, fully portable and project-scoped.
            success, output, _ = run_recur([
                "trace-id", "sudoku.r3.c5",
                "--scope", "sudoku.**",
                "--ext", ".txt",
                "--json",
                "-d", SUDOKU_DIR,
            ])
            @test success
            parsed = try JSON3.read(output); catch; nothing end
            @test parsed !== nothing
            if parsed !== nothing
                define  = get(parsed, :define,  get(parsed, "define",  []))
                produce = get(parsed, :produce, get(parsed, "produce", []))
                consume = get(parsed, :consume, get(parsed, "consume", []))
                trigger = get(parsed, :trigger, get(parsed, "trigger", []))

                @test length(define)  >= 1   # = 7 in solution + flow files
                @test length(produce) >= 1   # publish lines
                @test length(consume) >= 1   # subscribe lines
                @test length(trigger) >= 1   # trigger solve line (config enables this)
            end
        end

        @testset "Phase 1f: JSON site schema includes edge_type per role" begin
            success, output, _ = run_recur([
                "trace-id", "sudoku.r3.c5",
                "--scope", "sudoku.**",
                "--ext", ".txt",
                "--json",
                "-d", SUDOKU_DIR,
            ])
            @test success
            parsed = try JSON3.read(output); catch; nothing end
            @test parsed !== nothing
            if parsed !== nothing
                # Top-level shape
                @test haskey(parsed, :identifier) || haskey(parsed, "identifier")

                # Each non-empty role array has edge_type on first site
                for (role_sym, role_str) in [
                    (:define, "define"), (:produce, "produce"),
                    (:consume, "consume"), (:trigger, "trigger")
                ]
                    sites = get(parsed, role_sym, get(parsed, role_str, []))
                    if length(sites) > 0
                        site = sites[1]
                        has_et = haskey(site, :edge_type) || haskey(site, "edge_type")
                        @test has_et
                        if has_et
                            edge_val = String(get(site, :edge_type, get(site, "edge_type", "")))
                            @test edge_val == role_str
                        end
                    end
                end
            end
        end

    finally
        isdir(SUDOKU_DIR) && rm(SUDOKU_DIR, recursive=true)
    end
end

# ---------------------------------------------------------------------------
# Phase 2: Julia Recur.jl wrapper + hardcoded puzzle fixture
# ---------------------------------------------------------------------------
# Proves: the demo's Recur.jl module correctly wraps recur subprocess calls
# and returns parsed JSON matching the Phase 1 contract.
# The puzzle fixture lives in demos/sudoku/puzzles/easy-001/ — permanent,
# not torn down. Tests run recur against those files directly.
# ---------------------------------------------------------------------------

const DEMO_PUZZLE_DIR = joinpath(@__DIR__, "..", "demos", "sudoku", "puzzles", "easy-001")
const RECUR_JL_PATH   = joinpath(@__DIR__, "..", "demos", "sudoku", "julia", "Recur.jl")

@testset "Demo: Sudoku Phase 2 — Recur.jl wrapper + puzzle fixture" begin
    log_section("Testing: Sudoku demo Phase 2")

    @testset "Phase 2a: Recur.jl module exists and loads" begin
        @test isfile(RECUR_JL_PATH)
        # Verify it parses as valid Julia (include should not throw)
        loaded = try
            include(RECUR_JL_PATH)
            true
        catch e
            @error "Recur.jl failed to load" exception=e
            false
        end
        @test loaded
    end

    @testset "Phase 2b: Puzzle fixture files exist on disk" begin
        @test isdir(DEMO_PUZZLE_DIR)
        @test isfile(joinpath(DEMO_PUZZLE_DIR, "sudoku.solution.txt"))
        @test isfile(joinpath(DEMO_PUZZLE_DIR, "sudoku.flow.r3c5.txt"))
        @test isfile(joinpath(DEMO_PUZZLE_DIR, "..", "..", ".recur", "config.toml"))
    end

    @testset "Phase 2c: recur files sees puzzle files as sudoku.** hierarchy" begin
        success, output, _ = run_recur(["files", "sudoku.**", "-d", DEMO_PUZZLE_DIR])
        @test success
        parsed = try JSON3.read(output); catch; nothing end
        @test parsed !== nothing
        if parsed !== nothing
            paths = [String(p) for p in parsed]
            @test length(paths) >= 2
            @test any(p -> contains(p, "solution"), paths)
            @test any(p -> contains(p, "flow"), paths)
        end
    end

    @testset "Phase 2d: Recur.jl trace_id returns valid cascade for r3.c5" begin
        # Load Recur.jl and call trace_id — the core Phase 2 proof.
        # This is the full loop: Julia → recur subprocess → JSON parse → struct.
        include(RECUR_JL_PATH)

        demo_dir = abspath(joinpath(DEMO_PUZZLE_DIR, "..", ".."))
        puzzle_dir = abspath(DEMO_PUZZLE_DIR)

        result = Recur.trace_id(
            "sudoku.r3.c5";
            dir = puzzle_dir,
            scope = "sudoku.**",
            ext = ".txt",
        )

        @test result !== nothing
        if result !== nothing
            @test haskey(result, :identifier) || haskey(result, "identifier")
            define  = get(result, :define,  get(result, "define",  []))
            produce = get(result, :produce, get(result, "produce", []))
            consume = get(result, :consume, get(result, "consume", []))
            trigger = get(result, :trigger, get(result, "trigger", []))

            @test length(define)  >= 1   # solution + flow files both define r3.c5
            @test length(produce) >= 1   # publish lines → produce
            @test length(consume) >= 1   # subscribe lines → consume
            # trigger requires demo .recur/config.toml — puzzle_dir is inside demo root
            @test length(trigger) >= 1
        end
    end

    @testset "Phase 2e: Recur.jl files returns puzzle paths" begin
        include(RECUR_JL_PATH)
        puzzle_dir = abspath(DEMO_PUZZLE_DIR)
        paths = Recur.files("sudoku.**"; dir = puzzle_dir)
        @test length(paths) >= 2
        @test any(p -> contains(p, "solution"), paths)
        @test any(p -> contains(p, "flow"), paths)
    end
end
