"""
Tests for 'recur find' Command
===============================

Tests text search, scoped search, context lines, regex patterns,
and output formats for the find command.
"""

include("runtests.setup.jl")

@testset "recur find command" begin
    log_section("Testing: recur find")

    @testset "Basic text search" begin
        # Command: recur find "async" --scope "**"
        # Should find all files containing "async" keyword
        @testset "find basic text" begin
            success, output, _ = run_recur("find \"async\" --scope \"**\"")

            passed = success &&
                     contains(output, "ApiController.Auth.cs") &&
                     contains(output, "UserService.Handlers.Create.cs") &&
                     contains(output, "async")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "ApiController.Auth.cs")
            @test contains(output, "UserService.Handlers.Create.cs")
            @test contains(output, "async")
            log_test("basic text search works")
        end

        # Command: recur find "async" --scope "UserService.**"
        # Should only find matches within UserService hierarchy
        @testset "find with scope" begin
            success, output, _ = run_recur("find \"async\" --scope \"UserService.**\"")

            passed = success &&
                     contains(output, "UserService.Handlers.Create.cs") &&
                     contains(output, "UserService.Handlers.Update.cs") &&
                     !contains(output, "ApiController")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "UserService.Handlers.Create.cs")
            @test contains(output, "UserService.Handlers.Update.cs")
            @test !contains(output, "ApiController")
            log_test("scoped search works")
        end

        # Command: recur find "ASYNC" --scope "**" -i
        # Should find "async" even though search term is uppercase
        @testset "find with -i flag" begin
            success, output, _ = run_recur("find \"ASYNC\" --scope \"**\" -i")

            passed = success &&
                     contains(output, "async") &&
                     contains(output, "ApiController.Auth.cs")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "async")
            @test contains(output, "ApiController.Auth.cs")
            log_test("case-insensitive search works")
        end
    end

    @testset "Context lines" begin
        # Command: recur find "async" --scope "TestContext" -C 2
        # Should show 2 lines before and after the match
        @testset "find with context lines" begin
            success, output, _ = run_recur("find \"async\" --scope \"TestContext\" -C 2")

            passed = success &&
                     contains(output, "line 2:") &&
                     contains(output, "line 3:") &&
                     contains(output, "line 4:")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "line 2")
            @test contains(output, "line 3")
            @test contains(output, "line 4")
            log_test("context lines work")
        end

        # Command: recur find "async" --scope "TestContext" -C 0
        # Should show only the matching line, no context
        @testset "find with no context" begin
            success, output, _ = run_recur("find \"async\" --scope \"TestContext\" -C 0")

            passed = success &&
                     contains(output, "line 3") &&
                     !contains(output, "line 2") &&
                     !contains(output, "line 4")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "line 3")
            @test !contains(output, "line 2")
            @test !contains(output, "line 4")
            log_test("no context works")
        end

        # Command: recur find "async" --scope "TestContext" -C 5
        # Should show all 5 lines (entire file has 5 lines)
        @testset "find with multiple context" begin
            success, output, _ = run_recur("find \"async\" --scope \"TestContext\" -C 5")

            passed = success &&
                     contains(output, "line 1") &&
                     contains(output, "line 2") &&
                     contains(output, "line 3") &&
                     contains(output, "line 4") &&
                     contains(output, "line 5")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "line 1")
            @test contains(output, "line 2")
            @test contains(output, "line 3")
            @test contains(output, "line 4")
            @test contains(output, "line 5")
            log_test("multiple context lines work")
        end
    end

    @testset "Regex search" begin
        # Command: recur find "async.*Task" --scope "**" -E
        # Should match pattern with regex
        @testset "find with regex" begin
            success, output, _ = run_recur("find \"async.*Task\" --scope \"**\" -E")

            passed = success &&
                     contains(output, "ApiController.Auth.cs") &&
                     contains(output, "async")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "ApiController.Auth.cs")
            @test contains(output, "async")
            log_test("regex search works")
        end

        # Command: recur find "(Create|Update)User" --scope "UserService.**" -E
        # Should match either CreateUser or UpdateUser
        @testset "find with complex regex" begin
            success, output, _ = run_recur("find \"(Create|Update)User\" --scope \"UserService.**\" -E")

            passed = success &&
                     contains(output, "CreateUser") &&
                     contains(output, "UpdateUser") &&
                     !contains(output, "DeleteUser")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "CreateUser")
            @test contains(output, "UpdateUser")
            @test !contains(output, "DeleteUser")
            log_test("complex regex works")
        end
    end

    @testset "Output formats" begin
        # Command: recur find "async" --scope "UserService.**" --json
        # Should output valid JSON
        @testset "find with --json" begin
            success, output, _ = run_recur("find \"async\" --scope \"UserService.**\" --json")

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
            log_test("JSON output works")
        end

        # Command: recur find "async" --scope "TestContext" -C 2 --json
        # Should output valid JSON with context included
        @testset "find JSON context" begin
            success, output, _ = run_recur("find \"async\" --scope \"TestContext\" -C 2 --json")

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
            log_test("JSON with context works")
        end
    end
end
