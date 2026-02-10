# Current Work: merge Command - Phase 3 Complete

## Status
**Phase 2 + Phase 3 complete**

Path normalization and provenance markers now working in pattern mode.

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

## Fixed ✅

**Tree display now includes underscore-separated files:**

Example:
```
recur merge --pattern "main.command.tree" --sep "." --pattern "main_command_tree" --sep "_" --show-sep --sep-replace-default "."
```
Output:
```
main.command.tree
├── readme.md [.]
├── test.jl [.]
└── impl.rs [_]
```

## Implemented Fixes

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

1. [ ] Phase 4: File mode (merge JSON files)
2. [ ] Phase 5: Stdin mode (pipe merge)
3. [ ] Phase 6: Full pipe integration and docs

## Testing

### Current Test
```bash
./target/debug/recur.exe merge \
  --pattern "main.command" --sep "." \
  --pattern "main_command" --sep "_"
```

**Current output:** Shows all files in unified tree

## Branch & Commits

**Branch:** `merge-pipes`

**Commits:**
- `6ab1ee3` - Phase 1: Planning
- `e9501fb` - Phase 2: Basic implementation (in progress)

**Next commit:** Phase 3 complete (markers + normalization)

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

- **Time:** ~2-4 hours (pipe modes + docs)
- **Complexity:** Medium-high (JSON parsing + IO)
- **Files to change:** merge impl + CLI help + docs

## Success Criteria

Phase 2 + Phase 3 complete:
- ✅ Tree output includes files from ALL patterns
- ✅ Source .rs files visible alongside docs .md files
- ✅ `--show-sep` markers display provenance
- ✅ No duplicates
- ✅ Debug output removed

Next: Phase 4 pipe/file inputs.
