# Phase 3: --sep-replace-default Normalization

## Goal
Implement path normalization so output displays with a consistent separator, regardless of the actual file separator.

## Problem

When querying multiple separators, output shows mixed notation:
```bash
recur files "main.command.**" --sep "." --sep "_"

# Output (mixed):
main.command.files.readme.md      # dot separator
main_command_files_impl.rs        # underscore separator (confusing!)
```

## Solution

With `--sep-replace-default`, normalize all paths to use one separator:
```bash
recur files "main.command.**" --sep "." --sep "_" --sep-replace-default "."

# Output (normalized):
main.command.files.readme.md      # dot separator
main.command.files.impl.rs        # normalized from underscore!
```

## Implementation Strategy

### 1. Path Normalization Function

Create a helper that transforms path separators:

```rust
/// Normalize a file path's separator to a different character
fn normalize_path_separator(path: &Path, from_sep: char, to_sep: char) -> String {
    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
        // Get base name without extension
        let (base, ext) = filename
            .rsplit_once('.')
            .map(|(b, e)| (b, Some(e)))
            .unwrap_or((filename, None));

        // Replace separator in base name
        let normalized = base.replace(from_sep, &to_sep.to_string());

        // Reconstruct with extension
        if let Some(e) = ext {
            format!("{}.{}", normalized, e)
        } else {
            normalized
        }
    } else {
        path.display().to_string()
    }
}
```

### 2. Track Original Separator

When collecting files, remember which separator was used:

```rust
let mut file_separators: HashMap<PathBuf, char> = HashMap::new();

for sep in &separators {
    let files = find_files_for_separator(..., *sep)?;
    for file in files {
        file_separators.insert(file.clone(), *sep);
        all_files.push(file);
    }
}
```

### 3. Apply Normalization in Output

**For files command:**
```rust
if let Some(replace_sep) = replace_default {
    // Normalize paths before display
    let normalized_files: Vec<String> = all_files
        .iter()
        .map(|path| {
            let original_sep = file_separators.get(path).copied().unwrap_or('.');
            normalize_path_separator(path, original_sep, replace_sep)
        })
        .collect();

    formatter.print_file_list(&normalized_files);
} else {
    formatter.print_file_list(&all_files);
}
```

**For tree command:**
The tree structure is built from paths, so normalize paths before building:

```rust
if let Some(replace_sep) = replace_default {
    // Normalize paths before building tree
    let normalized_files: Vec<PathBuf> = all_files
        .iter()
        .map(|path| {
            let original_sep = file_separators.get(path).copied().unwrap_or('.');
            let normalized = normalize_path_separator(path, original_sep, replace_sep);
            PathBuf::from(normalized)
        })
        .collect();

    let tree = HierarchyTree::from_paths_with_separator(base, &normalized_files, replace_sep);
} else {
    let tree = HierarchyTree::from_paths_with_separator(base, &all_files, tree_separator);
}
```

## Edge Cases

### 1. Extension Preservation
```rust
main_command_files_impl.rs   // Input
main.command.files.impl.rs   // Output (extension preserved)
```

### 2. Multiple Extensions
```rust
test.data.json               // Input with multiple dots
test.data.json               // Should NOT change (extension dots preserved)
```

### 3. No Extension
```rust
main_command_files           // Input
main.command.files           # Output
```

### 4. Path Directories
Only normalize the filename, not directory parts:
```rust
./src/main_command_files_impl.rs
./src/main.command.files.impl.rs  // Only filename normalized
```

## Testing

### Test Cases

1. **Basic normalization**
   ```bash
   # Underscore to dot
   main_command_files.rs → main.command.files.rs
   ```

2. **Multiple files**
   ```bash
   recur files "main.command.**" --sep "." --sep "_" --sep-replace-default "."
   # All output uses dots
   ```

3. **Tree structure**
   ```bash
   recur tree main --sep "." --sep "_" --sep-replace-default "."
   # Tree nodes use normalized separator
   ```

4. **Different replacement separator**
   ```bash
   recur files "main.**" --sep "." --sep "_" --sep-replace-default "_"
   # All output uses underscores
   ```

## Files to Modify

1. **src/main_command_files_impl.rs**
   - Add normalization logic to `execute_with_separators()`
   - Apply before outputting file list

2. **src/main_command_tree_impl.rs**
   - Add normalization logic to `execute_with_separators()`
   - Apply before building tree

3. **Shared helper (optional)**
   - Could create `src/separator_utils.rs` for shared normalization function
   - Or keep inline in each command for now

## Success Criteria

- [ ] Normalization helper function implemented
- [ ] `files` command normalizes output when flag is used
- [ ] `tree` command normalizes output when flag is used
- [ ] Tests pass with normalized output
- [ ] Edge cases handled (extensions, no extension, etc.)
- [ ] All baseline tests still pass

## Example Output

### Before (Phase 2)
```bash
$ recur files "main.command.files.**" --sep "." --sep "_"
docs/main.command.files.readme.md
docs/main.command.files.test.jl
src/main_command_files_impl.rs       # Different separator!
src/main_command_files_stdin.rs      # Different separator!
```

### After (Phase 3)
```bash
$ recur files "main.command.files.**" --sep "." --sep "_" --sep-replace-default "."
docs/main.command.files.readme.md
docs/main.command.files.test.jl
src/main.command.files.impl.rs       # Normalized!
src/main.command.files.stdin.rs      # Normalized!
```

**Visual consistency achieved!** ✨
