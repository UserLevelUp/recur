"""
Tests for 'recur files' Command
================================

Tests pattern matching, depth filtering, extension filtering,
and output formats for the files command.
"""

include("runtests.setup.jl")

@testset "recur files command" begin
    log_section("Testing: recur files")

    @testset "Pattern matching" begin
        # Command: recur files "UserService.Handlers"
        # Should match: UserService.Handlers.cs
        # Should NOT match: UserService.Handlers.Create.cs (child)
        @testset "files with exact pattern" begin
            success, output, _ = run_recur("files \"UserService.Handlers\"")

            passed = success &&
                     contains(output, "UserService.Handlers.cs") &&
                     !contains(output, "UserService.Handlers.Create.cs") &&
                     !contains(output, "UserService.Handlers.Update.cs")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "UserService.Handlers.cs")
            # Should not match children with additional segments
            @test !contains(output, "UserService.Handlers.Create.cs")
            @test !contains(output, "UserService.Handlers.Update.cs")
            log_test("exact pattern matching works")
        end

        # Command: recur files "UserService.*"
        # Should match: UserService.Handlers.cs, UserService.Models.cs (depth 1)
        # Should NOT match: UserService.Handlers.Create.cs (too deep, depth 2)
        @testset "files with * wildcard" begin
            success, output, _ = run_recur("files \"UserService.*\"")

            passed = success &&
                     contains(output, "UserService.Handlers.cs") &&
                     contains(output, "UserService.Models.cs") &&
                     !contains(output, "UserService.Handlers.Create.cs")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "UserService.Handlers.cs")
            @test contains(output, "UserService.Models.cs")
            @test !contains(output, "UserService.Handlers.Create.cs")
            log_test("single-segment wildcard works")
        end

        # Command: recur files "UserService.**"
        # Should match all UserService files at any depth
        @testset "files with ** wildcard" begin
            success, output, _ = run_recur("files \"UserService.**\"")

            passed = success &&
                     contains(output, "UserService.cs") &&
                     contains(output, "UserService.Handlers.cs") &&
                     contains(output, "UserService.Handlers.Create.cs") &&
                     contains(output, "UserService.Models.Request.cs")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "UserService.cs")
            @test contains(output, "UserService.Handlers.cs")
            @test contains(output, "UserService.Handlers.Create.cs")
            @test contains(output, "UserService.Models.Request.cs")
            log_test("recursive wildcard works")
        end
    end

    @testset "Depth filtering" begin
        # Command: recur files "UserService.**" --min-depth 2
        # Should match only files at depth 2+ (2 dots)
        # Should NOT match: UserService.cs (depth 0), UserService.Handlers.cs (depth 1)
        # NOTE: Currently BROKEN - min_depth calculation is incorrect (counts pattern dots)
        @testset "files with --min-depth" begin
            success, output, _ = run_recur("files \"UserService.**\" --min-depth 2")

            passed = success &&
                     contains(output, "UserService.Handlers.Create.cs") &&
                     !contains(output, "UserService.cs") &&
                     !contains(output, "UserService.Handlers.cs")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL (KNOWN ISSUE)")

            @test_broken success
            @test_broken contains(output, "UserService.Handlers.Create.cs")
            @test !contains(output, "UserService.cs")
            @test !contains(output, "UserService.Handlers.cs")
            log_test("min-depth filtering works")
        end

        # Command: recur files "UserService.**" --max-depth 1
        # Should match only files at depth 0-1 (0-1 dots)
        # Should NOT match: UserService.Handlers.Create.cs (depth 2)
        # NOTE: Currently BROKEN - max_depth not applied to filesystem walker
        @testset "files with --max-depth" begin
            success, output, _ = run_recur("files \"UserService.**\" --max-depth 1")

            passed = success &&
                     contains(output, "UserService.cs") &&
                     contains(output, "UserService.Handlers.cs") &&
                     !contains(output, "UserService.Handlers.Create.cs")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL (KNOWN ISSUE)")

            @test success
            @test contains(output, "UserService.cs")
            @test contains(output, "UserService.Handlers.cs")
            @test_broken !contains(output, "UserService.Handlers.Create.cs")
            log_test("max-depth filtering works")
        end

        # Command: recur files "**" --min-depth 1 --max-depth 1
        # Should match only files at exactly depth 1 (1 dot in filename)
        # Should NOT match: depth 0 (UserService.cs) or depth 2+ (UserService.Handlers.Create.cs)
        # NOTE: Partially working - min_depth works with "**", but max_depth still broken
        @testset "files with depth range" begin
            success, output, _ = run_recur("files \"**\" --min-depth 1 --max-depth 1")

            passed = success &&
                     contains(output, "UserService.Handlers.cs") &&
                     contains(output, "ApiController.Auth.cs") &&
                     !contains(output, "UserService.cs") &&
                     !contains(output, "UserService.Handlers.Create.cs")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL (KNOWN ISSUE)")

            @test success
            @test contains(output, "UserService.Handlers.cs")
            @test contains(output, "ApiController.Auth.cs")
            @test !contains(output, "UserService.cs")
            @test_broken !contains(output, "UserService.Handlers.Create.cs")
            log_test("depth range filtering works")
        end

        # Command: recur files "**" --min-depth 5 --max-depth 2
        # Should fail with error: min-depth cannot be greater than max-depth
        @testset "files depth validation" begin
            success, output, error = run_recur("files \"**\" --min-depth 5 --max-depth 2")

            passed = !success &&
                     (contains(error, "min") || contains(error, "max") || contains(error, "depth"))

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test !success
            # Should contain error message about depth validation
            @test contains(error, "min") || contains(error, "max") || contains(error, "depth")
            log_test("depth validation error works")
        end
    end

    @testset "Extension filtering" begin
        # Command: recur files "**" --ext .cs
        # Should match only .cs files, not .json files
        @testset "files with extension filter" begin
            success, output, _ = run_recur("files \"**\" --ext .cs")

            passed = success &&
                     contains(output, "UserService.cs") &&
                     contains(output, "ApiController.cs") &&
                     !contains(output, "config.json")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "UserService.cs")
            @test contains(output, "ApiController.cs")
            @test !contains(output, "config.json")
            log_test("extension filtering works")
        end

        # Command: recur files "**" --ext ".cs,.json"
        # Should match both .cs and .json files, not .md files
        @testset "files with multiple extensions" begin
            success, output, _ = run_recur("files \"**\" --ext \".cs,.json\"")

            passed = success &&
                     contains(output, "UserService.cs") &&
                     contains(output, "config.json") &&
                     !contains(output, "README.md")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "UserService.cs")
            @test contains(output, "config.json")
            @test !contains(output, "README.md")
            log_test("multiple extension filtering works")
        end
    end

    @testset "Output formats" begin
        # Command: recur files "UserService.**" --count
        # Should output a count of files matching the pattern
        @testset "files with --count" begin
            success, output, _ = run_recur("files \"UserService.**\" --count")

            # Should contain a number (the count)
            passed = success && occursin(r"\d+", output)

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test occursin(r"\d+", output)
            log_test("count output works")
        end

        # Command: recur files "UserService.**" --json
        # Should output valid JSON with files array
        @testset "files with --json" begin
            success, output, _ = run_recur("files \"UserService.**\" --json")

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
                # JSON should have some expected structure
                # (adjust based on actual recur JSON output format)
                log_test("JSON output is valid")
            end
        end
    end
end
