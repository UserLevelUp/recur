# Phase 4: --show-sep Display Markers

## Goal
Add visual markers showing which separator (domain) each file came from.

## Use Case: Gap Analysis

When merging multiple separators, users want to see:
- Which files exist in docs?
- Which files exist in source?
- What's missing where?

**Solution:** Append separator markers to output.

## Expected Behavior

### Files Command
```bash
recur files "main.command.**" --sep "." --sep "_" --show-sep

# Output:
main.command.files.readme.md [.]
main.command.files.test.jl [.]
main.command.files.impl.rs [_]
main.command.files.stdin.rs [_]
```

**Markers show domain:**
- `[.]` = From docs/tests (dot separator)
- `[_]` = From source (underscore separator)

### Tree Command
```bash
recur tree main.command.files --sep "." --sep "_" --show-sep

# Output:
main.command.files
├── readme.md [.]
├── test.jl [.]
├── impl.rs [_]
└── stdin.rs [_]
```

### With Normalization
```bash
recur tree main.command.files --sep "." --sep "_" --sep-replace-default "." --show-sep

# Output (normalized paths + markers):
main.command.files
├── readme.md [.]
├── test.jl [.]
├── impl.rs [_]        # Normalized from main_command_files_impl.rs
└── stdin.rs [_]       # Normalized from main_command_files_stdin.rs
```

**Key insight:** Even with normalized paths, markers show original domain!

## Implementation Strategy

### 1. Data Structure (Already Have It!)

We already track which separator was used:
```rust
let mut file_separators: HashMap<PathBuf, char> = HashMap::new();

for sep in &separators {
    let files = find_files_for_separator(..., *sep)?;
    for file in files {
        file_separators.insert(file.clone(), *sep);  // ✅ Already tracking!
    }
}
```

### 2. Format Marker String

Create helper function:
```rust
fn format_with_separator_marker(path: &str, separator: char) -> String {
    format!("{} [{}]", path, separator)
}
```

### 3. Apply to Files Command

Modify display logic:
```rust
if show_sep {
    let marked_files: Vec<String> = display_files
        .iter()
        .map(|path| {
            let sep = file_separators.get(path).copied().unwrap_or(separators[0]);
            let path_str = path.display().to_string();
            format!("{} [{}]", path_str, sep)
        })
        .collect();

    formatter.print_file_list_as_strings(&marked_files);
} else {
    formatter.print_file_list(&display_files);
}
```

### 4. Apply to Tree Command

**Challenge:** Tree is built from paths, not printed line-by-line.

**Solution:** Modify filenames before building tree:
```rust
if show_sep {
    let marked_files: Vec<PathBuf> = display_files
        .iter()
        .map(|path| {
            let sep = file_separators.get(path).copied().unwrap_or(separators[0]);

            // Append marker to filename
            if let Some(filename) = path.file_name() {
                let marked_filename = format!("{} [{}]", filename.to_string_lossy(), sep);
                let mut marked_path = path.clone();
                marked_path.set_file_name(marked_filename);
                marked_path
            } else {
                path.clone()
            }
        })
        .collect();

    let tree = HierarchyTree::from_paths_with_separator(base, &marked_files, tree_separator);
} else {
    let tree = HierarchyTree::from_paths_with_separator(base, &display_files, tree_separator);
}
```

## Edge Cases

### Single Separator (No Multi-Sep)

```bash
recur files "main.**" --sep "." --show-sep
```

**Behavior:** Should NOT show markers (only one domain, so marker is redundant).

**Implementation:**
```rust
let show_markers = show_sep && separators.len() > 1;
```

### Missing Separator Data

If we can't determine which separator was used:
```rust
let sep = file_separators.get(path).copied().unwrap_or(separators[0]);
// Defaults to first separator if unknown
```

### Marker Format

**Chosen:** `[.]` with square brackets
- Clear visual separation
- Common pattern (like git markers)
- Easy to parse

**Alternatives considered:**
- `(.)` - Too subtle
- `{.}` - Less common
- `<.>` - Looks like HTML
- `[.]` - ✅ Best choice

## Gap Analysis Use Case

**Find what's documented but not implemented:**
```bash
recur files "main.command.**" --sep "." --sep "_" --show-sep | grep -v "\[_\]"
```

Shows files that only have `[.]` marker (docs) but no `[_]` marker (source).

**Find what's implemented but not documented:**
```bash
recur files "main.command.**" --sep "." --sep "_" --show-sep | grep -v "\[.\]"
```

Shows files that only have `[_]` marker (source) but no `[.]` marker (docs).

## Testing

### Test Cases

1. **Basic markers**
   ```bash
   recur files "test.**" --sep "." --sep "_" --show-sep
   # Should show markers for each file
   ```

2. **Single separator (no markers)**
   ```bash
   recur files "test.**" --sep "." --show-sep
   # Should NOT show markers (redundant)
   ```

3. **With normalization**
   ```bash
   recur files "test.**" --sep "." --sep "_" --sep-replace-default "." --show-sep
   # Normalized paths + original separator markers
   ```

4. **Tree output**
   ```bash
   recur tree test --sep "." --sep "_" --show-sep
   # Tree structure with markers on leaves
   ```

## Implementation Files

1. **src/main_command_files_impl.rs**
   - Add marker formatting in display logic
   - Apply when show_sep is true

2. **src/main_command_tree_impl.rs**
   - Append markers to filenames before tree building
   - Apply when show_sep is true

## Success Criteria

- [ ] Files command shows markers when --show-sep is used
- [ ] Tree command shows markers when --show-sep is used
- [ ] No markers shown when only one separator (redundant)
- [ ] Markers show ORIGINAL separator even with normalization
- [ ] Format is clear and parseable: `filename [sep]`

## Example Workflow

**1. Check feature completeness:**
```bash
recur tree main.command.files --sep "." --sep "_" --show-sep
```

**Output:**
```
main.command.files
├── readme.md [.]     ✅ Documented
├── test.jl [.]       ✅ Tested
├── impl.rs [_]       ✅ Implemented
└── stdin.rs [_]      ✅ Has stdin support
```

**2. Find gaps:**
```bash
recur files "main.command.**" --sep "." --sep "_" --show-sep | \
  awk '{print $NF}' | sort | uniq -c
```

Shows count of files by separator (how many docs vs source files).

## Summary

`--show-sep` enables **domain visibility** - users can see at a glance which files exist in which domain, making gap analysis trivial.

Combined with normalization, you get:
- Unified visual presentation (normalized paths)
- Domain attribution (separator markers)
- Best of both worlds!
