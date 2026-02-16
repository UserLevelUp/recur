#!/usr/bin/env julia
"""
Recur Integration Tests - Main Test Orchestrator
=================================================

Comprehensive test suite for the recur hierarchical search tool.
Tests are organized into hierarchical modules that can be run
individually or as a complete suite.

Usage:
    julia runtests.jl                    # Run all tests
    julia runtests.jl --verbose          # Run with verbose output
    julia runtests.files.jl             # Run only files command tests
    julia runtests.find.jl              # Run only find command tests

Test Structure:
    runtests.jl           - Main orchestrator (this file)
    runtests.setup.jl     - Test environment and utilities
    runtests.files.jl     - Tests for 'recur files' command
    runtests.find.jl      - Tests for 'recur find' command
    [more test files to come...]
"""

using Test

println("""
╔═══════════════════════════════════════════════════════════╗
║         Recur Integration Test Suite                     ║
║         Hierarchical Search Tool Testing                 ║
╚═══════════════════════════════════════════════════════════╝
""")

# Include setup utilities
include("runtests.setup.jl")

# Setup test environment once for all tests
setup_test_environment()

try
    # Run all test modules
    @testset "Recur Complete Test Suite" begin

        # Include and run test files
        include("main.command.files.test.jl")
        include("main.command.find.test.jl")
        include("main.command.tree.test.jl")
        include("main.command.related.test.jl")
        include("main.command.children.test.jl")
        include("main.command.id.test.jl")
        include("main.command.stats.test.jl")
        include("main.command.merge.test.jl")
        include("main.command.callers.test.jl")
        include("main.command.callees.test.jl")
        include("main.command.trace.test.jl")
        include("main.meta.dogfooding.test.jl")  # Dogfooding hierarchy: tree + separator precedence
        include("main.command.stdin.test.jl")    # IMPROVEMENT6 - Git integration with --stdin flag

        # TODO: Add more test modules as they are implemented
        include("main.command.trace-stats.test.jl")  # IMPROVEMENT7 - Statistical analysis of call graphs (phase3 bootstrap)
        # include("runtests.gaps.jl")        # Needs feature implementation

    end
finally
    # Always cleanup test environment
    teardown_test_environment()
end

println("""
╔═══════════════════════════════════════════════════════════╗
║         Test Suite Complete                              ║
╚═══════════════════════════════════════════════════════════╝
""")
