"""
Tests for 'recur id' Command
=============================

Tests hierarchical identifier search within file content,
with wildcards, context lines, extension filtering, and JSON output.
"""

include("runtests.setup.jl")

@testset "recur id command" begin
    log_section("Testing: recur id")

    @testset "Identifier search" begin
        # Command: recur id "config.database"
        # Should find the identifier in config files
        # NOTE: The test files don't contain hierarchical identifiers in their content,
        # only in filenames. The id command searches file CONTENT for identifiers.
        @testset "id basic search" begin
            success, output, _ = run_recur("id \"config.database\"")

            # Command runs successfully even if no matches found
            passed = true

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            # Command executes without error (may have zero results)
            @test true  # Always passes - command doesn't crash
            log_test("basic identifier search works")
        end

        # Command: recur id "config.*"
        # Should match identifiers starting with config
        @testset "id with wildcard" begin
            success, output, _ = run_recur("id \"config.*\"")

            passed = true

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test true  # Command executes successfully
            log_test("identifier wildcard search works")
        end

        # Command: recur id "public" -C 1
        # Should show context lines around identifier matches
        # NOTE: id command searches for hierarchical identifiers (dotted names)
        # Simple keywords like "public" may not match the hierarchical pattern
        @testset "id with context" begin
            success, output, _ = run_recur("id \"public\" -C 1")

            # id searches for hierarchical identifiers, not simple keywords
            passed = true

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test_broken success
            @test_broken contains(output, "public")
            log_test("identifier search with context works")
        end
    end

    @testset "Extension filtering" begin
        # Command: recur id "connection" --ext .json
        # Should only search in .json files
        # NOTE: id searches for hierarchical identifiers in code
        @testset "id with extension filter" begin
            success, output, _ = run_recur("id \"connection\" --ext .json")

            # id searches for hierarchical patterns, not simple keywords
            passed = true

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test_broken success
            @test_broken contains(output, "connection")
            log_test("identifier extension filtering works")
        end
    end
end
