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
            # PLACEHOLDER
            # success, output, _ = run_recur("trace-stats --scope \"**\" --ext .cs --sort-by depth --top 3")

            # Functions with deepest call chains first
            # Shows stack depth risk

            @test_skip true
            log_test("trace-stats sort by depth (PENDING IMPLEMENTATION)")
        end

        @testset "sort by risk score" begin
            # PLACEHOLDER
            # success, output, _ = run_recur("trace-stats --scope \"**\" --ext .cs --sort-by risk --top 3")

            # Combined complexity score
            # Risk levels: Low (<10 transitive), Medium (10-30), High (>30)

            @test_skip true
            log_test("trace-stats sort by risk (PENDING IMPLEMENTATION)")
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

    @testset "Git integration with stdin (PLACEHOLDER)" begin
        @testset "trace-stats on changed files" begin
            # PLACEHOLDER
            # This depends on IMPROVEMENT6 (--stdin flag)
            # stdin_input = "LevelController.CreateWizard3.cs\nDynamicGameComponentService.Delete.cs"
            # success, output, _ = run_recur_with_stdin("trace-stats --scope \"**\" --stdin --sort-by risk", stdin_input)

            # Show complexity stats only for changed files
            # Helps prioritize testing in PR reviews

            @test_skip true
            log_test("trace-stats with stdin (PENDING IMPROVEMENT6 + IMPROVEMENT7)")
        end
    end

    @testset "Circular reference detection accuracy (PLACEHOLDER)" begin
        @testset "count distinct circular patterns" begin
            # PLACEHOLDER
            # Test setup:
            #   CreateWizard3() → ApplyTemplate() → RenderTemplate() → CreateWizard3()  [pattern 1]
            #   CreateWizard3() → SaveWizard() → ValidateWizard() → CreateWizard3()    [pattern 2]
            #
            # Expected:
            #   CreateWizard3: Circular = 2 (two distinct patterns)

            @test_skip true
            log_test("circular pattern counting (PENDING IMPLEMENTATION)")
        end

        @testset "no false positives" begin
            # PLACEHOLDER
            # Functions without circular references should show Circular = 0
            # Even if they have complex call graphs

            @test_skip true
            log_test("no false circular positives (PENDING IMPLEMENTATION)")
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
            # PLACEHOLDER
            # Functions with 10-30 transitive callees should show Risk = Medium

            @test_skip true
            log_test("medium risk scoring (PENDING IMPLEMENTATION)")
        end

        @testset "high risk (>30 transitive)" begin
            # PLACEHOLDER
            # Functions with > 30 transitive callees should show Risk = High

            @test_skip true
            log_test("high risk scoring (PENDING IMPLEMENTATION)")
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

    @testset "Performance on large codebases (PLACEHOLDER)" begin
        @testset "handles 100+ functions" begin
            # PLACEHOLDER
            # Create test environment with 100+ functions
            # Verify trace-stats completes in reasonable time

            @test_skip true
            log_test("large codebase performance (PENDING IMPLEMENTATION)")
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
# - [ ] Activate skipped depth/risk ordering assertions
# - [ ] Add stdin-focused trace-stats assertions
# - [ ] Add larger-scope performance assertions
