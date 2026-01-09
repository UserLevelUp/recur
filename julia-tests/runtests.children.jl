"""
Tests for 'recur children' Command
===================================

Tests finding all child/descendant files of a hierarchy,
with count option and JSON output.
"""

include("runtests.setup.jl")

@testset "recur children command" begin
    log_section("Testing: recur children")

    @testset "Finding descendants" begin
        # Command: recur children "UserService"
        # Should find all files that are children/descendants of UserService
        # Including UserService itself and all nested files
        @testset "children basic" begin
            success, output, _ = run_recur("children \"UserService\"")

            passed = success &&
                     contains(output, "UserService.cs") &&
                     contains(output, "UserService.Handlers.cs") &&
                     contains(output, "UserService.Handlers.Create.cs") &&
                     contains(output, "UserService.Models.cs") &&
                     contains(output, "UserService.Models.Request.cs")

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test contains(output, "UserService.cs")
            @test contains(output, "UserService.Handlers.cs")
            @test contains(output, "UserService.Handlers.Create.cs")
            @test contains(output, "UserService.Models.cs")
            @test contains(output, "UserService.Models.Request.cs")
            log_test("basic children search works")
        end

        # Command: recur children "UserService.Handlers" --count
        # Should output only the count of children
        @testset "children with count" begin
            success, output, _ = run_recur("children \"UserService.Handlers\" --count")

            # Should contain a number (the count)
            # UserService.Handlers.cs + Create + Update + Delete = 4 files
            passed = success && occursin(r"\d+", output)

            println(passed ? "  ✓ PASS" : "  ✗ FAIL")

            @test success
            @test occursin(r"\d+", output)
            log_test("children count works")
        end
    end

    @testset "Output formats" begin
        # Command: recur children "config" --json
        # Should output valid JSON with list of child files
        @testset "children with JSON output" begin
            success, output, _ = run_recur("children \"config\" --json")

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
            log_test("JSON children output works")
        end
    end
end
