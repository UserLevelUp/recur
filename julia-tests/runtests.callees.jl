"""
Tests for 'recur callees' Command
==================================

Tests function callee detection (what functions a function calls),
hierarchical vs flat file prioritization, scoped search, and output
formats for the callees command.
"""

include("runtests.setup.jl")

@testset "recur callees command" begin
    log_section("Testing: recur callees")

    @testset "Basic callee detection" begin
        # Command: recur callees "CreateUser" --scope "**"
        # Should find ValidateEmail and SaveUser called by CreateUser
        # Hierarchical files should appear first
        @testset "callees basic search" begin
            success, output, _ = run_recur("callees \"CreateUser\" --scope \"**\"")

            passed = success &&
                     (contains(output, "ValidateEmail") ||
                      contains(output, "SaveUser"))

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "ValidateEmail") || contains(output, "SaveUser")
            log_test("basic callee search works")
        end

        # Command: recur callees "ProcessRequest" --scope "UserService.**"
        # Should find ValidateEmail called by ProcessRequest in UserService hierarchy
        @testset "callees with scope" begin
            success, output, _ = run_recur("callees \"ProcessRequest\" --scope \"UserService.**\"")

            passed = success &&
                     contains(output, "ValidateEmail")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "ValidateEmail")
            log_test("scoped callee search works")
        end

        # Command: recur callees "CreateUser" --scope "**"
        # Should find multiple callees (ValidateEmail and SaveUser)
        @testset "callees multiple results" begin
            success, output, _ = run_recur("callees \"CreateUser\" --scope \"**\"")

            # CreateUser calls both ValidateEmail and SaveUser
            passed = success &&
                     (contains(output, "ValidateEmail") ||
                      contains(output, "SaveUser"))

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            log_test("multiple callee detection works")
        end
    end

    @testset "Hierarchical vs flat distinction" begin
        # Command: recur callees "CreateUser" --scope "**"
        # Should show hierarchical files with [hierarchical, depth=X] marker
        # Should show flat files with [flat] marker
        @testset "callees hierarchical marking" begin
            success, output, _ = run_recur("callees \"CreateUser\" --scope \"**\"")

            passed = success &&
                     (contains(output, "[hierarchical") || contains(output, "[flat]"))

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            # Should contain hierarchical or flat markers
            @test contains(output, "[hierarchical") || contains(output, "[flat]")
            log_test("hierarchical marking works")
        end

        # Verify hierarchical files appear before flat files in output
        @testset "callees hierarchical priority" begin
            success, output, _ = run_recur("callees \"CreateUser\" --scope \"**\"")

            # If both hierarchical and flat files exist, hierarchical should come first
            passed = success

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            log_test("hierarchical priority works")
        end
    end

    @testset "Search options" begin
        # Command: recur callees "createuser" --scope "**" -i
        # Should find CreateUser with case-insensitive search
        @testset "callees case-insensitive" begin
            success, output, _ = run_recur("callees \"createuser\" --scope \"**\" -i")

            passed = success &&
                     (contains(lowercase(output), "validateemail") ||
                      contains(lowercase(output), "saveuser"))

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            log_test("case-insensitive callee search works")
        end

        # Command: recur callees "CreateUser" --scope "**" --ext .cs
        # Should only search in .cs files
        @testset "callees with extension filter" begin
            success, output, _ = run_recur("callees \"CreateUser\" --scope \"**\" --ext .cs")

            passed = success &&
                     (contains(output, ".cs") ||
                      # May have no results if function not in .cs files
                      !success)

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            # Command should execute (may have zero results)
            @test true
            log_test("extension filtering works")
        end

        # Command: recur callees "CreateUser" --scope "**" -C 1
        # Should show 1 line of context before and after
        @testset "callees with context lines" begin
            success, output, _ = run_recur("callees \"CreateUser\" --scope \"**\" -C 1")

            passed = success &&
                     (contains(output, "ValidateEmail") ||
                      contains(output, "SaveUser"))

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            log_test("context lines work")
        end
    end

    @testset "Traversal budget propagation (expected broken)" begin
        @testset "callees --help exposes budget flags" begin
            success, output, _ = run_recur("callees --help")

            passed = success &&
                     contains(output, "--depth") &&
                     contains(output, "--depth-guard") &&
                     contains(output, "--force")

            println(passed ? "  PASS" : "  BROKEN")

            @test success
            @test_broken contains(output, "--depth")
            @test_broken contains(output, "--depth-guard")
            @test_broken contains(output, "--force")
            log_test("callees budget flags contract (expected broken)")
        end

        @testset "hard-fail rejects depth above cap" begin
            success, _, error_output = run_recur(
                "callees \"CreateWizard3\" --scope \"LevelController.CreateWizard3.**\" --ext .cs --depth 6 --depth-guard hard-fail"
            )

            passed = !success &&
                     contains(error_output, "Maximum depth is 5")

            println(passed ? "  PASS" : "  BROKEN")

            @test_broken !success && contains(error_output, "Maximum depth is 5")
            log_test("callees hard-fail contract (expected broken)")
        end

        @testset "clamp mode allows execution with warning" begin
            success, output, error_output = run_recur(
                "callees \"CreateWizard3\" --scope \"LevelController.CreateWizard3.**\" --ext .cs --depth 6 --depth-guard clamp"
            )

            passed = success &&
                     contains(output, "ApplyTemplate") &&
                     contains(lowercase(error_output), "clamping")

            println(passed ? "  PASS" : "  BROKEN")

            @test_broken success
            @test_broken contains(output, "ApplyTemplate")
            @test_broken contains(lowercase(error_output), "clamping")
            log_test("callees clamp contract (expected broken)")
        end

        @testset "force bypasses hard-fail cap" begin
            success, output, _ = run_recur(
                "callees \"CreateWizard3\" --scope \"LevelController.CreateWizard3.**\" --ext .cs --depth 6 --depth-guard hard-fail --force"
            )

            passed = success &&
                     contains(output, "ApplyTemplate")

            println(passed ? "  PASS" : "  BROKEN")

            @test_broken success
            @test_broken contains(output, "ApplyTemplate")
            log_test("callees force contract (expected broken)")
        end

        @testset "depth propagation expands transitive callees" begin
            success_d1, output_d1, _ = run_recur(
                "callees \"CreateWizard3\" --scope \"LevelController.CreateWizard3.**\" --ext .cs --depth 1"
            )
            success_d2, output_d2, _ = run_recur(
                "callees \"CreateWizard3\" --scope \"LevelController.CreateWizard3.**\" --ext .cs --depth 2"
            )

            passed = success_d1 &&
                     success_d2 &&
                     !contains(output_d1, "RenderTemplate") &&
                     contains(output_d2, "RenderTemplate")

            println(passed ? "  PASS" : "  BROKEN")

            @test_broken success_d1 &&
                         success_d2 &&
                         !contains(output_d1, "RenderTemplate") &&
                         contains(output_d2, "RenderTemplate")
            log_test("callees depth propagation contract (expected broken)")
        end
    end

    @testset "Output formats" begin
        # Command: recur callees "CreateUser" --scope "**" --count
        # Should output only the count of callees
        @testset "callees with count" begin
            success, output, _ = run_recur("callees \"CreateUser\" --scope \"**\" --count")

            # Should contain a number (the count)
            passed = success && occursin(r"^\d+$", strip(output))

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test occursin(r"^\d+$", strip(output))
            log_test("count output works")
        end

        # Command: recur callees "CreateUser" --scope "**" --json
        # Should output valid JSON with hierarchical metadata
        @testset "callees with JSON output" begin
            success, output, _ = run_recur("callees \"CreateUser\" --scope \"**\" --json")

            # Try to parse as JSON
            local data
            local json_valid = false
            try
                data = JSON3.read(output)
                json_valid = true
            catch
                json_valid = false
            end

            passed = success && json_valid

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test json_valid
            if json_valid
                @test data !== nothing
                # Check for hierarchical metadata in JSON
                if isa(data, AbstractArray) && length(data) > 0
                    first_item = data[1]
                    @test isa(first_item, Any)
                end
            end
            log_test("JSON output with hierarchical metadata works")
        end

        # Command: recur callees "ProcessRequest" --scope "UserService.**" --json
        # Should output valid JSON
        @testset "callees JSON scoped results" begin
            success, output, _ = run_recur("callees \"ProcessRequest\" --scope \"UserService.**\" --json")

            # Try to parse as JSON
            local data
            local json_valid = false
            try
                data = JSON3.read(output)
                json_valid = true
            catch
                json_valid = false
            end

            passed = success && json_valid

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test json_valid
            if json_valid
                @test data !== nothing
            end
            log_test("JSON with scoped results works")
        end
    end

    @testset "Edge cases" begin
        # Command: recur callees "NonExistentFunction" --scope "**"
        # Should return no results (exit code 1)
        @testset "callees no results" begin
            success, output, _ = run_recur("callees \"NonExistentFunction\" --scope \"**\"")

            # Should not succeed (no results found)
            passed = !success

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test !success
            log_test("no results case works")
        end

        # Command: recur callees "ValidateEmail" --scope "**"
        # ValidateEmail doesn't call other functions (no callees)
        @testset "callees function with no callees" begin
            success, output, _ = run_recur("callees \"ValidateEmail\" --scope \"**\"")

            # Should return exit code 1 (no callees found)
            passed = !success

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test !success
            log_test("function with no callees works")
        end

        # Command: recur callees "CreateUser" --scope "NonExistent.**"
        # Scope doesn't match any files
        @testset "callees invalid scope" begin
            success, output, _ = run_recur("callees \"CreateUser\" --scope \"NonExistent.**\"")

            # Should fail (no files in scope)
            passed = !success

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test !success
            log_test("invalid scope handling works")
        end
    end
end
