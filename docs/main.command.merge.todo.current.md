# Current Work: merge Command - Phase 2 (90% Complete)

## Status
**Phase 2: Basic Merge - One issue remaining**

Path normalization needed to display files from all patterns in unified tree.

## What Works ✅

### File Discovery
```bash
recur merge --pattern "main.command" --sep "." --pattern "main_command" --sep "_"

# Debug output shows:
Pattern 'main.command' with separator '.': 34 files found
Pattern 'main_command' with separator '_': 14 files found
Total unique files after merge: 48
```

**Verification:** 34 + 14 = 48 ✅ (Math checks out!)

### CLI & Deduplication
- Multiple `--pattern` and `--sep` flags work
- Deduplication prevents duplicates
- All files being discovered correctly

## The Problem ❌

**Tree display doesn't show underscore-separated files:**

Files ARE found but not displayed:
```
Sample files from underscore pattern:
  .\src\main_command_callees_impl.rs
  .\src\main_command_tree_impl.rs
```

**Root Cause:**
Tree builder uses first separator (`.`) as canonical form, tries to build tree rooted at `main.command`. But files named `main_command_*.rs` don't fit into dot-based hierarchy!

**Why:**
- File: `src/main_command_tree_impl.rs`
- Tree base: `main.command`
- Tree can't match `main_command_*` to `main.command.*`

## The Fix: Path Normalization

**Before building tree, normalize ALL paths to use canonical separator.**

### Algorithm
```rust
// For each file:
// 1. Detect which separator it uses
// 2. Replace with target separator in filename
// 3. Rebuild path

// Example:
// Input:  src/main_command_tree_impl.rs (sep='_')
// Output: src/main.command.tree.impl.rs (sep='.')
```

### Implementation Steps

1. **Track separator per file**
   ```rust
   let mut file_to_separator: HashMap<PathBuf, char> = HashMap::new();
   // Store which separator found each file
   ```

2. **Add normalization function**
   ```rust
   fn normalize_paths_to_separator(
       files: &[PathBuf],
       file_seps: &HashMap<PathBuf, char>,
       target_sep: char,
   ) -> Vec<PathBuf>
   ```

3. **Use before tree building**
   ```rust
   let normalized = normalize_paths_to_separator(&all_files, &file_to_separator, separators[0]);
   display_tree(&normalized, ...);
   ```

## Files Status

### Created This Session
- `docs/main.command.merge.readme.md` ✅
- `docs/main.command.merge.todo.md` ✅
- `docs/main.command.merge.phase2.plan.md` ✅
- `docs/main.command.merge.phase2.status.md` ✅ (This session's findings)
- `src/main_command_merge_impl.rs` ✅ (needs path normalization)

### Modified
- `src/main.rs` - Added Merge command ✅

## Next Session Tasks

1. [ ] Read `main.command.merge.phase2.status.md` for context
2. [ ] Implement path normalization function
3. [ ] Update execute() to track separator per file
4. [ ] Test merge shows ALL files (including .rs from underscore pattern)
5. [ ] Remove debug output (eprintln!)
6. [ ] Commit Phase 2 complete
7. [ ] Start Phase 3: Provenance markers (--show-sep)

## Testing

### Current Test
```bash
./target/debug/recur.exe merge \
  --pattern "main.command" --sep "." \
  --pattern "main_command" --sep "_"
```

**Current output:** Shows 34 docs/test files, missing 14 source files

**After fix:** Should show all 48 files in unified tree

## Branch & Commits

**Branch:** `merge-pipes`

**Commits:**
- `6ab1ee3` - Phase 1: Planning
- `e9501fb` - Phase 2: Basic implementation (in progress)

**Next commit:** Phase 2 complete with path normalization

## Key Files for Next Session

1. **READ FIRST:**
   - `docs/main.command.merge.phase2.status.md` - Complete analysis of the issue

2. **MODIFY:**
   - `src/main_command_merge_impl.rs` - Add path normalization

3. **TEST WITH:**
   ```bash
   recur merge --pattern "main.command" --sep "." --pattern "main_command" --sep "_"
   ```

## Estimated Remaining Work

- **Time:** ~1 hour
- **Complexity:** Medium (straightforward path manipulation)
- **Files to change:** 1 (main_command_merge_impl.rs)

## Success Criteria

When Phase 2 is complete:
- ✅ Tree output includes files from ALL patterns
- ✅ Source .rs files visible alongside docs .md files
- ✅ Counts match: input files = tree display files
- ✅ No duplicates
- ✅ Debug output removed

Then move to Phase 3: `--show-sep` markers for provenance tracking.
