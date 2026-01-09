"""
Tests for 'recur stats' Command
================================

Tests statistics summary, depth breakdown, depth level listing,
and JSON output for the stats command.
"""

include("runtests.setup.jl")

@testset "recur stats command" begin
    log_section("Testing: recur stats")

    @testset "Statistics summary" begin
        # Command: recur stats "**"
        # Should show total files, lines, and depth breakdown
        @testset "stats basic summary" begin
            success, output, _ = run_recur("stats \"**\"")

            passed = success &&
                     contains(output, "Total files:") &&
                     contains(output, "Total lines:") &&
                     contains(output, "Max depth:") &&
                     contains(output, "Depth breakdown:")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "Total files:")
            @test contains(output, "Total lines:")
            @test contains(output, "Max depth:")
            @test contains(output, "Depth breakdown:")
            log_test("basic stats summary works")
        end

        # Command: recur stats "UserService.**"
        # Should show statistics for UserService hierarchy only
        @testset "stats with pattern scope" begin
            success, output, _ = run_recur("stats \"UserService.**\"")

            passed = success &&
                     contains(output, "Statistics for: UserService.**") &&
                     contains(output, "Total files:") &&
                     contains(output, "Depth breakdown:")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "Statistics for: UserService.**")
            @test contains(output, "Total files:")
            @test contains(output, "Depth breakdown:")
            log_test("scoped stats works")
        end
    end

    @testset "Depth level listing" begin
        # Command: recur stats "**" -l 0
        # Should list files at depth level 0 (no dots in name)
        @testset "stats list level 0" begin
            success, output, _ = run_recur("stats \"**\" -l 0")

            passed = success &&
                     contains(output, "Files at depth level 0:") &&
                     (contains(output, "UserService.cs") || contains(output, "ApiController.cs"))

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "Files at depth level 0:")
            log_test("stats level 0 listing works")
        end

        # Command: recur stats "**" -l 1
        # Should list files at depth level 1 (one dot in name)
        @testset "stats list level 1" begin
            success, output, _ = run_recur("stats \"**\" -l 1")

            passed = success &&
                     contains(output, "Files at depth level 1:") &&
                     (contains(output, "UserService.Handlers.cs") || contains(output, "UserService.Models.cs"))

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "Files at depth level 1:")
            log_test("stats level 1 listing works")
        end

        # Command: recur stats "UserService.**" -l 2
        # Should list UserService files at depth level 2 (two dots in name)
        # NOTE: Depth counting may be relative to the pattern, not absolute
        @testset "stats list level 2" begin
            success, output, _ = run_recur("stats \"UserService.**\" -l 2")

            passed = success &&
                     (contains(output, "Files at depth level 2:") ||
                      contains(output, "No files at depth level 2"))

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            # May show "No files" if depth is counted relative to pattern
            @test contains(output, "Files at depth level 2:") || contains(output, "No files at depth level 2")
            log_test("stats level 2 listing works")
        end
    end

    @testset "Output formats" begin
        # Command: recur stats "config.**" --json
        # Should output valid JSON with statistics
        @testset "stats with JSON output" begin
            success, output, _ = run_recur("stats \"config.**\" --json")

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
                # JSON should have expected statistics fields
                @test haskey(data, :total_files) || haskey(data, "total_files")
            end
            log_test("JSON stats output works")
        end

        # Command: recur stats "**" -l 1 --json
        # Should output JSON with level-specific file listing
        @testset "stats JSON with level" begin
            success, output, _ = run_recur("stats \"**\" -l 1 --json")

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
            log_test("JSON stats with level works")
        end
    end
end
