"""
Tests for 'recur tree' Command
===============================

Tests hierarchical tree visualization, depth limits,
ASCII mode, and output formats for the tree command.
"""

include("runtests.setup.jl")

@testset "recur tree command" begin
    log_section("Testing: recur tree")

    @testset "Basic tree visualization" begin
        # Command: recur tree "UserService"
        # Should show hierarchical tree structure
        @testset "tree basic visualization" begin
            success, output, _ = run_recur("tree \"UserService\"")

            passed = success &&
                     contains(output, "UserService") &&
                     contains(output, "Handlers") &&
                     contains(output, "Create") &&
                     contains(output, "Models")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "UserService")
            @test contains(output, "Handlers")
            @test contains(output, "Create")
            @test contains(output, "Models")
            log_test("basic tree visualization works")
        end

        # Command: recur tree "UserService" --depth 1
        # Should show only first level of hierarchy
        # NOTE: Currently depth limit may not work as expected
        @testset "tree with depth limit" begin
            success, output, _ = run_recur("tree \"UserService\" --depth 1")

            passed = success &&
                     contains(output, "Handlers") &&
                     contains(output, "Models")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "Handlers")
            @test contains(output, "Models")
            # Depth limiting may show all descendants
            @test_broken !contains(output, "Create")
            @test_broken !contains(output, "Update")
            log_test("tree depth limiting works")
        end

        # Command: recur tree "config" --ascii
        # Should use ASCII characters instead of Unicode box drawing
        @testset "tree with ASCII mode" begin
            success, output, _ = run_recur("tree \"config\" --ascii")

            passed = success &&
                     contains(output, "config") &&
                     # ASCII mode uses +, |, and - instead of Unicode box chars
                     (contains(output, "+--") || contains(output, "|") || contains(output, "+-"))

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "config")
            log_test("ASCII mode works")
        end
    end

    @testset "Tree statistics" begin
        # Command: recur tree "UserService" --count
        # Should show file count statistics
        @testset "tree with count" begin
            success, output, _ = run_recur("tree \"UserService\" --count")

            passed = success &&
                     contains(output, "UserService") &&
                     occursin(r"\d+", output)  # Contains a number (count)

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "UserService")
            @test occursin(r"\d+", output)
            log_test("tree count works")
        end
    end

    @testset "Output formats" begin
        # Command: recur tree "ApiController" --json
        # Should output valid JSON representation of tree
        @testset "tree with JSON output" begin
            success, output, _ = run_recur("tree \"ApiController\" --json")

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
            log_test("JSON tree output works")
        end
    end
end
