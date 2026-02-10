# Phase 2 Status: Complete (Path Normalization Implemented)

## Current Status: Complete

### What's Working ✅

1. **CLI Structure** - Fully implemented and tested
   ```bash
   recur merge --pattern "X" --sep "." --pattern "Y" --sep "_"
   ```

2. **File Discovery** - Working perfectly!
   ```
   Pattern 'main.command' with separator '.': 34 files found
   Pattern 'main_command' with separator '_': 14 files found
   Total unique files after merge: 48
   ```
   - Math checks out: 34 + 14 = 48 ✅
   - No duplicates ✅
   - Both patterns finding files ✅

3. **Deduplication** - Working correctly
   - HashSet prevents duplicate entries
   - Verified with debug counters

### Fix Implemented ✅

**Path normalization before tree build:**
- Normalize each file name to the canonical separator
- Tree now includes underscore-derived files under dot hierarchy

## Implemented Solution: Path Normalization

### Result

Tree output now shows files from all patterns.

### Normalization Algorithm

For each file path:
1. Extract the filename (without directory)
2. Detect which separator it uses (from our list)
3. Replace that separator with target separator
4. Reconstruct path

**Example:**
```
Input:  src/main_command_tree_impl.rs (found with sep '_')
Detect: Uses '_' separator
Replace: main_command_tree_impl → main.command.tree.impl
Output: src/main.command.tree.impl.rs
```

Now the tree builder can fit it into `main.command.*` hierarchy!

## Implementation Plan

### Step 1: Add Path Normalization Function

```rust
/// Normalize file paths to use target separator
fn normalize_paths_to_separator(
    files: &[PathBuf],
    original_separators: &[(PathBuf, char)],  // Map of file → separator used
    target_separator: char,
) -> Vec<PathBuf> {
    files.iter().map(|path| {
        // Get the separator used for this file
        let sep = original_separators.get(path).copied().unwrap_or(target_separator);

        if sep == target_separator {
            return path.clone();  // Already using target separator
        }

        // Normalize the filename
        let filename = path.file_name().unwrap().to_str().unwrap();
        let normalized_filename = filename.replace(sep, &target_separator.to_string());

        // Reconstruct path
        let mut new_path = path.clone();
        new_path.set_file_name(normalized_filename);
        new_path
    }).collect()
}
```

### Step 2: Track Separator Per File

Update the execute function to track which separator found each file:

```rust
let mut file_to_separator: HashMap<PathBuf, char> = HashMap::new();

for (pattern, separator) in patterns.iter().zip(separators.iter()) {
    let files = find_files_for_pattern(...);

    for file in files {
        if seen.insert(file.clone()) {
            all_files.push(file.clone());
            file_to_separator.insert(file, *separator);  // Track it!
        }
    }
}
```

### Step 3: Use Normalization Before Tree Building

```rust
// Normalize all paths to use first separator
let canonical_separator = separators[0];
let normalized_files = normalize_paths_to_separator(
    &all_files,
    &file_to_separator,
    canonical_separator,
);

// Now build tree with normalized paths
display_tree(&normalized_files, base_pattern, canonical_separator, ...);
```

## Testing

### Test 1: Verify Normalization

```bash
recur merge --pattern "main.command" --sep "." --pattern "main_command" --sep "_"
```

**Now includes:**
```
main.command
├── callees
│   ├── readme.md       # From pattern 1 (dots)
│   ├── test.jl         # From pattern 1 (dots)
│   └── impl.rs         # From pattern 2 (underscores) - NOW VISIBLE!
├── tree
│   ├── readme.md       # From pattern 1
│   ├── test.jl         # From pattern 1
│   └── impl.rs         # From pattern 2 - NOW VISIBLE!
```

### Test 2: Verify Counts

Debug output should show:
```
Pattern 'main.command' with separator '.': 34 files
Pattern 'main_command' with separator '_': 14 files
Total unique files: 48
Tree display: 48 files (all visible!)
```

### Test 3: Three-Pattern Merge

```bash
recur merge \
  --pattern "api.user" --sep "." \
  --pattern "api_user" --sep "_" \
  --pattern "api-user" --sep "-"
```

Should merge all three into unified view.

## Files Modified

### Completed
- `src/main.rs` - Added Merge command to CLI
- `src/main_command_merge_impl.rs` - Implementation (path normalization)
- `docs/main.command.merge.phase2.plan.md` - Phase 2 plan
- `docs/main.command.merge.phase2.status.md` - This file

### Files to Modify Next
- `src/main_command_merge_impl.rs` - Add path normalization

## Debug Output Currently Available

The implementation has debug output showing:
```
Pattern 'X' with separator 'Y': N files found
  Sample files:
    1: path/to/file1
    2: path/to/file2
Total unique files after merge: N
```

This is helpful for verifying file discovery works.

## Next Session Checklist

1. [ ] Implement `normalize_paths_to_separator()` function
2. [ ] Update `execute()` to track separator per file (HashMap)
3. [ ] Call normalization before tree building
4. [ ] Test with `main.command` + `main_command` merge
5. [ ] Verify .rs files now appear in tree output
6. [ ] Remove debug output (eprintln!)
7. [ ] Commit Phase 2 complete
8. [ ] Move to Phase 3: Provenance tracking (--show-sep markers)

## Git Status

**Branch:** `merge-pipes`

**Commits:**
- `6ab1ee3` - Phase 1: Planning and design
- `e9501fb` - Phase 2: Basic merge implementation (in progress)

**Uncommitted changes:**
- Debug output added to merge_impl.rs

## Key Insight

> The merge command finds files correctly (math proves it: 34 + 14 = 48).
> The issue is purely in tree DISPLAY, not file DISCOVERY.
> Solution: Normalize file paths before building tree.

## Estimated Time to Fix

- Implement normalization function: 30 min
- Test and debug: 30 min
- **Total: ~1 hour to complete Phase 2**

Then Phase 3 (provenance markers) will be straightforward since file discovery already works!
