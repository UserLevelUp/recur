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
        include("main.command.tree.wildcard-current.test.jl")  # standalone tree wildcard receipt target
        include("main.command.related.test.jl")
        include("main.command.children.test.jl")
        include("main.command.id.test.jl")
        include("main.command.stats.test.jl")
        include("main.command.merge.test.jl")
        include("main.command.unflatten.test.jl")  # IMPROVEMENT15 - frozen contract tests (expected broken)
        include("main.command.callers.test.jl")
        include("main.command.callees.test.jl")
        include("main.command.trace.test.jl")
        include("main.meta.dogfooding.test.jl")  # Dogfooding hierarchy: tree + separator precedence
        include("main.command.stdin.test.jl")    # IMPROVEMENT6 - Git integration with --stdin flag
        include("main.command.init.test.jl")     # Init command: config generation, analyze mode, lane collision dedupe
        include("main.command.reveal.test.jl")   # IMPROVEMENT22 - lane-local reveal helpers
        include("main.command.watch.test.jl")    # IMPROVEMENT23 - pure watcher-state query surface
        include("main.command.version.test.jl")  # IMPROVEMENT26 - pure version query + recur-version writer
        include("main.command.capability.test.jl")  # IMPROVEMENT28 - capability-card query surface
        include("main.improvement.27.warp.contract.test.jl")  # IMPROVEMENT27 - frozen warp-status-v1 fixtures
        include("main.command.warp.test.jl")  # IMPROVEMENT27 - read-only warp status
        include("main.command.warp.structure.test.jl")  # IMPROVEMENT27 - lane boundaries, collapse plan, and config
        include("main.command.lane.test.jl")    # IMPROVEMENT21 - named lane scaffolding (recur lane)
        include("main.lane.coordination.trace-id.test.jl")  # IMPROVEMENT21+22 - trace-id as lane handoff contract
        include("main.recur-git.checkpoint.lanes.test.jl")  # Checkpoint snapshot lane coverage for active agent vaults
        include("main.recur-git.test-receipt.test.jl")  # Immutable passed/failed test-event receipts
        include("main.recur.watch.test.jl")  # IMPROVEMENT23 - recur watch pub/sub subscription contract (expected red)
        include("main.recur.psyche.test.jl")  # IMPROVEMENT23 - recur psyche v1 red-first spec lock

        # TODO: Add more test modules as they are implemented
        include("main.command.trace-stats.test.jl")  # IMPROVEMENT7 - Statistical analysis of call graphs (phase3 bootstrap)
        include("main.command.trace-id.test.jl")     # IMPROVEMENT8 - trace-id MVP contract tests (expected broken)
        include("main.command.trait.test.jl")        # Trait config command + traversal budget placeholders
        include("main.demo.skippy.trace-id.test.jl") # Demo: Skippy adaptive comms + trace-id protocol
        include("runtests.demo.sudoku.jl")           # Demo: Sudoku + trace-id Phase 1+2 (file protocol + Recur.jl)
        include("runtests.demo.sudoku.phase3.jl")    # Demo: Sudoku Phase 3 (Generator.jl — flow files + cascades)
        include("runtests.demo.sudoku.phase4.jl")    # Demo: Sudoku Phase 4 (Engine.jl + Display.jl + Game.jl)
        include("main.demo.sudoku.watch.test.jl")    # Lane L cross-loop demo regression wrapper
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
