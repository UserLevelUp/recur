"""
Tests for 'recur trace-stats' Command (IMPROVEMENT7)
====================================================

Statistical analysis of call graph complexity. Ranks functions by:
- Direct callees (depth 1)
- Transitive callees (all reachable functions)
- Circular reference patterns
- Maximum call depth
- Refactoring risk score

Phase 3 bootstrap currently covers command surface + validation.
Metric computation tests remain pending for later phase work.
"""

include("runtests.setup.jl")

@testset "recur trace-stats command" begin
    log_section("Testing: recur trace-stats")

    created_here = false
    if !isdir(TEST_DIR)
        setup_test_environment()
        created_here = true
    end

    try

    @testset "Contract tests" begin
        @testset "trace-stats --help output" begin
            success, output, _ = run_recur("trace-stats --help")

            passed = success &&
                     contains(output, "trace-stats") &&
                     contains(output, "--scope") &&
                     contains(output, "--sort-by") &&
                     contains(output, "--top") &&
                     contains(output, "--filter") &&
                     contains(output, "--depth") &&
                     contains(output, "--depth-guard") &&
                     contains(output, "--force")

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test contains(output, "trace-stats")
            @test contains(output, "--scope")
            @test contains(output, "--sort-by")
            @test contains(output, "--top")
            @test contains(output, "--filter")
            @test contains(output, "--depth")
            @test contains(output, "--depth-guard")
            @test contains(output, "--force")
            log_test("trace-stats help output works")
        end

        @testset "missing scope argument" begin
            success, _, error_output = run_recur("trace-stats")

            passed = !success &&
                     contains(error_output, "required arguments") &&
                     contains(error_output, "--scope")

            println(passed ? "  PASS" : "  FAIL")

            @test !success
            @test contains(error_output, "required arguments")
            @test contains(error_output, "--scope")
            log_test("trace-stats missing scope returns error")
        end

        @testset "invalid sort option" begin
            success, _, error_output = run_recur("trace-stats --scope \"**\" --sort-by latency")

            passed = !success &&
                     contains(error_output, "Invalid --sort-by")

            println(passed ? "  PASS" : "  FAIL")

            @test !success
            @test contains(error_output, "Invalid --sort-by")
            log_test("trace-stats invalid sort returns error")
        end

        @testset "invalid filter option" begin
            success, _, error_output = run_recur("trace-stats --scope \"**\" --filter urgent-only")

            passed = !success &&
                     contains(error_output, "Invalid --filter")

            println(passed ? "  PASS" : "  FAIL")

            @test !success
            @test contains(error_output, "Invalid --filter")
            log_test("trace-stats invalid filter returns error")
        end
    end

    @testset "Depth guardrails" begin
        @testset "hard-fail rejects depth above cap" begin
            success, _, error_output = run_recur("trace-stats --scope \"WideService\" --ext .cs --depth 6 --depth-guard hard-fail --json")

            passed = !success &&
                     contains(error_output, "Maximum depth is 5")

            println(passed ? "  PASS" : "  FAIL")

            @test !success
            @test contains(error_output, "Maximum depth is 5")
            log_test("trace-stats hard-fail depth guard enforced")
        end

        @testset "clamp mode reduces effective depth" begin
            success, output, error_output = run_recur("trace-stats --scope \"WideService\" --ext .cs --depth 6 --depth-guard clamp --json")

            data = JSON3.read(output)

            passed = success &&
                     Int(data["request"]["depth_requested"]) == 6 &&
                     Int(data["request"]["depth_effective"]) == 5 &&
                     String(data["request"]["depth_guard"]) == "clamp" &&
                     contains(lowercase(error_output), "clamping to 5")

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test Int(data["request"]["depth_requested"]) == 6
            @test Int(data["request"]["depth_effective"]) == 5
            @test String(data["request"]["depth_guard"]) == "clamp"
            @test contains(lowercase(error_output), "clamping to 5")
            log_test("trace-stats clamp depth guard works")
        end

        @testset "force bypasses cap even in hard-fail mode" begin
            success, output, _ = run_recur("trace-stats --scope \"WideService\" --ext .cs --depth 6 --depth-guard hard-fail --force --json")

            data = JSON3.read(output)

            passed = success &&
                     Int(data["request"]["depth_requested"]) == 6 &&
                     Int(data["request"]["depth_effective"]) == 6 &&
                     Bool(data["request"]["force"]) == true

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test Int(data["request"]["depth_requested"]) == 6
            @test Int(data["request"]["depth_effective"]) == 6
            @test Bool(data["request"]["force"]) == true
            log_test("trace-stats force bypasses depth cap")
        end
    end

    @testset "Basic trace-stats output" begin
        @testset "default sort by transitive" begin
            success, output, _ = run_recur("trace-stats --scope \"LevelController.CreateWizard3.**\" --ext .cs --top 5 --json")

            data = JSON3.read(output)
            functions = data["functions"]

            passed = success && length(functions) >= 2

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test length(functions) >= 2
            @test functions[1]["transitive"] >= functions[2]["transitive"]
            @test data["summary"]["total_functions"] == length(functions)
            log_test("trace-stats default sort by transitive works")
        end

        @testset "JSON output format" begin
            success, output, _ = run_recur("trace-stats --scope \"**\" --ext .cs --json")

            json_valid = false
            data = nothing
            try
                data = JSON3.read(output)
                json_valid = true
            catch
                json_valid = false
            end

            functions = json_valid ? data["functions"] : []
            function_names = json_valid ? [String(f["name"]) for f in functions] : String[]

            passed = success &&
                     json_valid &&
                     length(functions) > 0 &&
                     ("CreateWizard3" in function_names) &&
                     ("WideRoot" in function_names)

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test json_valid
            @test length(functions) > 0
            @test "CreateWizard3" in function_names
            @test "WideRoot" in function_names
            @test data["summary"]["total_functions"] == length(functions)
            @test data["summary"]["max_depth"] >= 1
            log_test("trace-stats JSON format contains computed metrics")
        end
    end

    @testset "Sorting options" begin
        @testset "sort by direct callees" begin
            success, output, _ = run_recur("trace-stats --scope \"WideService\" --ext .cs --sort-by direct --top 1 --json")

            data = JSON3.read(output)
            functions = data["functions"]

            passed = success &&
                     length(functions) == 1 &&
                     String(functions[1]["name"]) == "WideRoot" &&
                     functions[1]["direct"] >= 3

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test length(functions) == 1
            @test String(functions[1]["name"]) == "WideRoot"
            @test functions[1]["direct"] >= 3
            log_test("trace-stats sort by direct works")
        end

        @testset "sort by transitive callees" begin
            success, output, _ = run_recur("trace-stats --scope \"**\" --ext .cs --sort-by transitive --top 3 --json")

            data = JSON3.read(output)
            functions = data["functions"]

            passed = success &&
                     length(functions) >= 2 &&
                     functions[1]["transitive"] >= functions[2]["transitive"]

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test length(functions) >= 2
            @test functions[1]["transitive"] >= functions[2]["transitive"]
            log_test("trace-stats sort by transitive works")
        end

        @testset "sort by circular patterns" begin
            success, output, _ = run_recur("trace-stats --scope \"CycleService\" --ext .cs --sort-by circular --top 1 --json")

            data = JSON3.read(output)
            functions = data["functions"]

            name = length(functions) > 0 ? String(functions[1]["name"]) : ""

            passed = success &&
                     length(functions) == 1 &&
                     (name == "FunctionA" || name == "FunctionB") &&
                     functions[1]["circular"] >= 1

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test length(functions) == 1
            @test (name == "FunctionA" || name == "FunctionB")
            @test functions[1]["circular"] >= 1
            log_test("trace-stats sort by circular works")
        end

        @testset "sort by depth" begin
            success, output, _ = run_recur("trace-stats --scope \"**\" --ext .cs --sort-by depth --top 3 --json")

            data = JSON3.read(output)
            functions = data["functions"]

            passed = success &&
                     length(functions) >= 2 &&
                     Int(functions[1]["depth"]) >= Int(functions[2]["depth"])

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test length(functions) >= 2
            @test Int(functions[1]["depth"]) >= Int(functions[2]["depth"])
            log_test("trace-stats sort by depth works")
        end

        @testset "sort by risk score" begin
            success, output, _ = run_recur("trace-stats --scope \"**\" --ext .cs --sort-by risk --top 3 --json")

            data = JSON3.read(output)
            functions = data["functions"]

            risk_rank = Dict("Low" => 0, "Medium" => 1, "High" => 2)
            ranks = [get(risk_rank, String(f["risk"]), -1) for f in functions]
            non_increasing_risk = if length(ranks) <= 1
                true
            else
                all(i -> ranks[i] >= ranks[i + 1], 1:(length(ranks) - 1))
            end

            passed = success &&
                     length(functions) > 0 &&
                     String(data["request"]["sort_by"]) == "risk" &&
                     all(r -> r >= 0, ranks) &&
                     non_increasing_risk

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test length(functions) > 0
            @test String(data["request"]["sort_by"]) == "risk"
            @test all(r -> r >= 0, ranks)
            @test non_increasing_risk
            log_test("trace-stats sort by risk works")
        end
    end

    @testset "Filtering options" begin
        @testset "filter circular-only" begin
            success, output, _ = run_recur("trace-stats --scope \"**\" --ext .cs --filter circular-only --json")

            data = JSON3.read(output)
            functions = data["functions"]
            all_circular = length(functions) > 0 &&
                           all(f -> Int(f["circular"]) > 0, functions)

            names = [String(f["name"]) for f in functions]

            passed = success &&
                     all_circular &&
                     ("FunctionA" in names || "FunctionB" in names)

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test length(functions) > 0
            @test all(f -> Int(f["circular"]) > 0, functions)
            @test ("FunctionA" in names || "FunctionB" in names)
            log_test("trace-stats filter circular-only works")
        end

        @testset "filter low-risk only" begin
            success, output, _ = run_recur("trace-stats --scope \"**\" --ext .cs --filter low-risk --json")

            data = JSON3.read(output)
            functions = data["functions"]
            all_low = length(functions) > 0 &&
                      all(f -> String(f["risk"]) == "Low", functions)

            passed = success && all_low

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test length(functions) > 0
            @test all(f -> String(f["risk"]) == "Low", functions)
            log_test("trace-stats filter low-risk works")
        end
    end

    @testset "Git integration with stdin" begin
        @testset "trace-stats on changed files" begin
            stdin_input = joinpath(TEST_DIR, "LevelController.CreateWizard3.cs") * "\n" *
                          joinpath(TEST_DIR, "DynamicGameComponentService.Delete.cs")

            success, output, _ = run_recur_stdin(stdin_input, ["trace-stats", "--scope", "**", "--stdin", "--sort-by", "risk", "--json", "-d", TEST_DIR])

            data = success ? JSON3.read(output) : nothing
            functions = success ? data["functions"] : []

            passed = success && length(functions) > 0

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test length(functions) > 0
            log_test("trace-stats with stdin filters to provided files")
        end
    end

    @testset "Circular reference detection accuracy (PLACEHOLDER)" begin
        @testset "count distinct circular patterns" begin
            # Fixture: DistinctCycleService has two distinct back-edges through CycleRoot:
            #   CycleRoot → PathA → CycleRoot  [pattern 1]
            #   CycleRoot → PathB → CycleRoot  [pattern 2]
            # Expected: circular = 2 (distinct patterns, not just presence flag)
            # Requires: Rust change in main_command_trace_stats_impl.rs
            success, output, _ = run_recur("trace-stats --scope \"DistinctCycleService\" --ext .cs --json")

            data = success ? JSON3.read(output) : nothing
            functions = success ? data["functions"] : []
            cycle_root = filter(f -> String(f["name"]) == "CycleRoot", functions)

            @test_broken length(cycle_root) == 1 && Int(cycle_root[1]["circular"]) == 2
            log_test("circular pattern counting (PENDING RUST IMPLEMENTATION)")
        end

        @testset "no false positives" begin
            success, output, _ = run_recur("trace-stats --scope \"WideService\" --ext .cs --json")

            data = JSON3.read(output)
            functions = data["functions"]
            wide_root = filter(f -> String(f["name"]) == "WideRoot", functions)

            passed = success &&
                     length(wide_root) == 1 &&
                     Int(wide_root[1]["circular"]) == 0

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test length(wide_root) == 1
            @test Int(wide_root[1]["circular"]) == 0
            log_test("no false circular positives in linear call graph")
        end
    end

    @testset "Risk scoring accuracy" begin
        @testset "low risk (<10 transitive)" begin
            success, output, _ = run_recur("trace-stats --scope \"WideService\" --ext .cs --json")

            data = JSON3.read(output)
            functions = data["functions"]
            wide_root = filter(f -> String(f["name"]) == "WideRoot", functions)

            passed = success &&
                     length(wide_root) == 1 &&
                     Int(wide_root[1]["transitive"]) < 10 &&
                     String(wide_root[1]["risk"]) == "Low"

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test length(wide_root) == 1
            @test Int(wide_root[1]["transitive"]) < 10
            @test String(wide_root[1]["risk"]) == "Low"
            log_test("trace-stats low risk scoring works")
        end

        @testset "medium risk (10-30 transitive)" begin
            success, output, _ = run_recur("trace-stats --scope \"MediumService\" --ext .cs --json")

            data = JSON3.read(output)
            functions = data["functions"]
            medium_root = filter(f -> String(f["name"]) == "MediumRoot", functions)

            passed = success &&
                     length(medium_root) == 1 &&
                     Int(medium_root[1]["transitive"]) >= 10 &&
                     Int(medium_root[1]["transitive"]) <= 30 &&
                     String(medium_root[1]["risk"]) == "Medium"

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test length(medium_root) == 1
            @test Int(medium_root[1]["transitive"]) >= 10
            @test Int(medium_root[1]["transitive"]) <= 30
            @test String(medium_root[1]["risk"]) == "Medium"
            log_test("medium risk scoring works (10-30 transitive)")
        end

        @testset "high risk (>30 transitive)" begin
            success, output, _ = run_recur("trace-stats --scope \"HighService\" --ext .cs --json")

            data = JSON3.read(output)
            functions = data["functions"]
            high_root = filter(f -> String(f["name"]) == "HighRoot", functions)

            passed = success &&
                     length(high_root) == 1 &&
                     Int(high_root[1]["transitive"]) > 30 &&
                     String(high_root[1]["risk"]) == "High"

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test length(high_root) == 1
            @test Int(high_root[1]["transitive"]) > 30
            @test String(high_root[1]["risk"]) == "High"
            log_test("high risk scoring works (>30 transitive)")
        end
    end

    @testset "Top N limiting" begin
        @testset "limit to top 5" begin
            success, output, _ = run_recur("trace-stats --scope \"**\" --ext .cs --sort-by transitive --top 2 --json")

            data = JSON3.read(output)
            functions = data["functions"]

            passed = success && length(functions) <= 2 && length(functions) > 0

            println(passed ? "  PASS" : "  FAIL")

            @test success
            @test length(functions) > 0
            @test length(functions) <= 2
            log_test("trace-stats top-N limiting works")
        end

        @testset "no limit shows all" begin
            success_all, output_all, _ = run_recur("trace-stats --scope \"**\" --ext .cs --sort-by transitive --json")
            success_top, output_top, _ = run_recur("trace-stats --scope \"**\" --ext .cs --sort-by transitive --top 3 --json")

            data_all = JSON3.read(output_all)
            data_top = JSON3.read(output_top)

            functions_all = data_all["functions"]
            functions_top = data_top["functions"]

            passed = success_all &&
                     success_top &&
                     length(functions_all) >= length(functions_top)

            println(passed ? "  PASS" : "  FAIL")

            @test success_all
            @test success_top
            @test length(functions_all) >= length(functions_top)
            log_test("trace-stats no-top returns full result set")
        end
    end

    @testset "Performance on large codebases" begin
        @testset "handles 100+ functions" begin
            t_start = time()
            success, output, _ = run_recur("trace-stats --scope \"PerformanceService\" --ext .cs --json")
            elapsed = time() - t_start

            data = success ? JSON3.read(output) : nothing
            functions = success ? data["functions"] : []

            passed = success && length(functions) > 0 && elapsed < 30.0

            println(passed ? "  PASS ($(round(elapsed, digits=2))s)" : "  FAIL ($(round(elapsed, digits=2))s)")

            @test success
            @test length(functions) > 0
            @test elapsed < 30.0
            log_test("large codebase (100+ functions) completes within time budget")
        end
    end

    finally
        if created_here
            teardown_test_environment()
        end
    end
end

# Implementation Checklist for IMPROVEMENT7
# ==========================================
#
# Progress:
# - [x] Step 1: Add TraceStats command to src/main.rs CLI
# - [x] Activate contract tests (help + option validation)
# - [x] Add to main test suite via main.command.trace-stats.test.jl
# - [x] Add core metric assertions (sorting/filtering/top-N/basic risk)
#
# Remaining:
# - [ ] Upgrade circular metric to distinct cycle-pattern counting
# - [ ] Add stdin-focused trace-stats assertions
# - [ ] Add no-false-positive circular assertions
# - [ ] Add medium/high risk fixture assertions
# - [ ] Add larger-scope performance assertions
