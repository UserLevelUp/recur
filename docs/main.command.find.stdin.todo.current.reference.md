# Reference: Files Command Stdin Implementation

This file points to the working reference implementation for stdin support.

## Reference Command: files

**Status:** ✅ Working (stdin tests passing)

**Source files:**
- `src/main_command_files_impl.rs` - Main implementation
- `src/main_command_files_stdin.rs` - Stdin-specific logic

**Test files:**
- `julia-tests/main.command.files.test.jl` - Command tests
- `julia-tests/main.command.stdin.test.jl` - Stdin capability tests

## How to Study Reference

```bash
# View the implementation
recur files "main_command_files_*" -d src/ --sep _

# Read the stdin module
cat src/main_command_files_stdin.rs

# Check test coverage
cd julia-tests && julia runtests.jl 2>&1 | grep "files.*stdin"
```

## Pattern to Follow

The `files` command shows the pattern for **standard file-list commands**.

For **content search commands** like `find`, adapt the pattern to:
1. Filter stdin paths by scope pattern (same as files)
2. Search content in filtered paths (different - search instead of list)

## Alternative Reference: stats

`stats` is another working example:
- `src/main_command_stats_impl.rs`
- `src/main_command_stats_stdin.rs`
