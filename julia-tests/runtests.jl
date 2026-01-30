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
        include("runtests.files.jl")
        include("runtests.find.jl")
        include("runtests.tree.jl")
        include("runtests.related.jl")
        include("runtests.children.jl")
        include("runtests.id.jl")
        include("runtests.stats.jl")
        include("runtests.callers.jl")
        include("runtests.callees.jl")
        include("runtests.trace.jl")
        include("runtests.stdin.jl")       # IMPROVEMENT6 - Git integration with --stdin flag

        # TODO: Add more test modules as they are implemented
        # include("runtests.trace-stats.jl") # IMPROVEMENT7 - Statistical analysis of call graphs
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
