"""
Tests for 'recur related' Command
==================================

Tests finding sibling files in the same hierarchical level,
with and without self-exclusion, and JSON output.
"""

include("runtests.setup.jl")

@testset "recur related command" begin
    log_section("Testing: recur related")

    @testset "Finding siblings" begin
        # Command: recur related "UserService.Handlers.Create.cs"
        # Should find all files at the same level (siblings)
        # Including self: Create, Update, Delete
        @testset "related basic" begin
            success, output, _ = run_recur("related \"UserService.Handlers.Create.cs\"")

            passed = success &&
                     contains(output, "UserService.Handlers.Create.cs") &&
                     contains(output, "UserService.Handlers.Update.cs") &&
                     contains(output, "UserService.Handlers.Delete.cs")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "UserService.Handlers.Create.cs")
            @test contains(output, "UserService.Handlers.Update.cs")
            @test contains(output, "UserService.Handlers.Delete.cs")
            log_test("basic related search works")
        end

        # Command: recur related "UserService.Handlers.Create.cs" --exclude-self
        # Should find siblings but NOT include the input file itself
        @testset "related with exclude-self" begin
            success, output, _ = run_recur("related \"UserService.Handlers.Create.cs\" --exclude-self")

            passed = success &&
                     !contains(output, "UserService.Handlers.Create.cs") &&
                     contains(output, "UserService.Handlers.Update.cs") &&
                     contains(output, "UserService.Handlers.Delete.cs")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test !contains(output, "UserService.Handlers.Create.cs")
            @test contains(output, "UserService.Handlers.Update.cs")
            @test contains(output, "UserService.Handlers.Delete.cs")
            log_test("exclude-self flag works")
        end
    end

    @testset "Output formats" begin
        # Command: recur related "ApiController.Auth.cs" --json
        # Should output valid JSON with list of related files
        @testset "related with JSON output" begin
            success, output, _ = run_recur("related \"ApiController.Auth.cs\" --json")

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
            log_test("JSON related output works")
        end
    end
end
