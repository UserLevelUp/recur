# Reference: Stdin Implementation Patterns

This file points to working stdin implementations to use as references.

## Two Patterns Available

### Pattern 1: Separate Stdin Module (Recommended for find)

**files command** - Best reference for find:
- ✅ `src/main_command_files_impl.rs` - Main implementation
- ✅ `src/main_command_files_stdin.rs` - Separate stdin module
- ✅ Stdin tests passing

**stats command** - Alternative example:
- ✅ `src/main_command_stats_impl.rs` - Main implementation
- ✅ `src/main_command_stats_stdin.rs` - Separate stdin module
- ✅ Stdin tests passing

### Pattern 2: Integrated Stdin (Reference for logic)

**tree command** - Integrated approach:
- ✅ `src/main_command_tree_impl.rs` - Stdin integrated in impl
- ✅ Has `read_resolved_paths_from_stdin()` helper
- ✅ Stdin tests passing

**related command** - Another integrated example:
- ✅ `src/main_command_related_impl.rs` - Stdin integrated in impl
- ✅ Has `read_resolved_paths_from_stdin()` helper
- ✅ Stdin tests passing

## How to Study References

```bash
# View all stdin implementations
recur files "main_command_*_stdin" -d src/ --sep _

# Read the separate module pattern (RECOMMENDED)
cat src/main_command_files_stdin.rs
cat src/main_command_stats_stdin.rs

# Read the integrated pattern (for comparison)
grep -A 20 "stdin" src/main_command_tree_impl.rs
grep -A 20 "stdin" src/main_command_related_impl.rs

# Check test coverage
cd julia-tests && julia runtests.jl 2>&1 | grep "stdin.*PASS"
```

## Recommended Approach for Find

**Use Pattern 1 (Separate Module)** because:
1. Find is a **content search command** (more complex)
2. Keeps stdin logic separate and testable
3. Follows files/stats precedent
4. Cleaner code organization

**Implementation steps:**
1. Create `src/main_command_find_stdin.rs`
2. Add helper function like `collect_files_from_stdin()`
3. Use `read_paths_from_stdin()` from `recur::r#trait`
4. Filter paths by scope pattern
5. Return filtered paths to search

## Key Helper Function (from recur::search)

All implementations use:
```rust
use recur::search::read_paths_from_stdin;
// or
use recur::r#trait::read_paths_from_stdin;
```

This reads file paths from stdin (one per line) and returns `Vec<PathBuf>`.

## Test Files

- `julia-tests/main.command.find.test.jl` - Command tests
- `julia-tests/main.command.stdin.test.jl` - Stdin capability tests
