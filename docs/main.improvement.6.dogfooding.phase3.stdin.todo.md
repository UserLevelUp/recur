# Phase 3: Complete Stdin Support for All Commands

Status: `todo` (next phase after extraction)

## Goal

Add stdin support to the 6 remaining commands that don't have it yet.

## Commands Needing Stdin

1. ⏳ **find** - Content search command
2. ⏳ **children** - List immediate children
3. ⏳ **id** - Content search for identifiers
4. ⏳ **callers** - Content search for callers
5. ⏳ **callees** - Content search for callees
6. ⏳ **trace** - Transitive call graph

## Pattern to Follow

Look at working examples:
- `src/main_command_files_stdin.rs` (standard command)
- `src/main_command_stats_stdin.rs` (standard command)

## Test Validation

Run Julia tests to verify:
```bash
cd julia-tests && julia runtests.jl 2>&1 | grep -A 20 "stdin"
```

All stdin tests should pass when complete.

## References

- `README.CORE.IMPROVEMENT6.md` - Main stdin implementation guide
- `README.CORE.IMPROVEMENT6.Dogfooding.md` - Dogfooding structure
- `julia-tests/main.command.stdin.test.jl` - Stdin tests
