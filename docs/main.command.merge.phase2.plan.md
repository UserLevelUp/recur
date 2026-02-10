# Phase 2: Basic Merge Implementation

## Goal
Implement core `recur merge` command that can merge results from multiple pattern/separator pairs into a unified tree view.

## Scope
- Add `merge` subcommand to CLI
- Implement pattern/separator pairing
- Merge files from multiple searches
- Display unified tree (no markers yet)
- Handle basic 2-pattern merge

**Out of scope for Phase 2:**
- `--show-sep` markers (Phase 3)
- Normalization (Phase 3)
- Pipe mode (Phase 5)
- Files format (Phase 6)

## Implementation Steps

### Step 1: Add CLI Structure

**File:** `src/main.rs`

Add merge subcommand:
```rust
#[derive(Subcommand)]
enum Commands {
    // ... existing commands ...

    /// Merge hierarchical results from multiple naming conventions
    Merge {
        /// Patterns to merge (repeatable, paired with --sep)
        #[arg(long = "pattern", value_name = "PATTERN", required = true)]
        patterns: Vec<String>,

        /// Separators for each pattern (repeatable, paired with --pattern)
        #[arg(long = "sep", value_name = "CHAR", required = true)]
        sep: Vec<String>,

        /// Working directory
        #[arg(short = 'd', long = "dir", value_name = "DIR")]
        dir: Option<PathBuf>,

        /// Maximum depth to search
        #[arg(long = "max-depth", value_name = "N")]
        max_depth: Option<usize>,

        /// Use ASCII characters instead of Unicode
        #[arg(long = "ascii")]
        ascii: bool,

        /// Show file counts at each level
        #[arg(long = "count")]
        count: bool,

        /// Output as JSON
        #[arg(long = "json")]
        json: bool,
    },
}
```

Add handler in main():
```rust
Commands::Merge { patterns, sep, dir, max_depth, ascii, count, json } => {
    let separators: Vec<char> = sep.iter()
        .filter_map(|s| s.chars().next())
        .collect();

    if patterns.len() != separators.len() {
        eprintln!("Error: Number of --pattern and --sep arguments must match");
        std::process::exit(2);
    }

    main_command_merge_impl::execute(
        patterns,
        separators,
        dir.unwrap_or_else(|| PathBuf::from(".")),
        max_depth,
        !ascii,
        count,
        json,
    )?;
}
```

### Step 2: Create Implementation File

**File:** `src/main_command_merge_impl.rs`

```rust
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use crate::hierarchical_path::find_matching_files;
use crate::tree::TreeBuilder;

/// Execute merge command
pub fn execute(
    patterns: Vec<String>,
    separators: Vec<char>,
    dir: PathBuf,
    max_depth: Option<usize>,
    unicode: bool,
    show_count: bool,
    json: bool,
) -> Result<()> {
    // Step 1: Collect files from all pattern/separator pairs
    let mut all_files: Vec<PathBuf> = Vec::new();
    let mut seen: HashSet<PathBuf> = HashSet::new();

    for (pattern, separator) in patterns.iter().zip(separators.iter()) {
        let files = find_files_for_pattern(&pattern, *separator, &dir, max_depth)?;

        for file in files {
            if seen.insert(file.clone()) {
                all_files.push(file);
            }
        }
    }

    // Step 2: Build unified tree
    if all_files.is_empty() {
        println!("No files found");
        return Ok(());
    }

    // Use first separator as canonical form for display
    let canonical_separator = separators[0];

    // Build and display tree
    display_tree(&all_files, &patterns[0], canonical_separator, unicode, show_count, json)?;

    Ok(())
}

/// Find files matching a specific pattern with specific separator
fn find_files_for_pattern(
    pattern: &str,
    separator: char,
    dir: &PathBuf,
    max_depth: Option<usize>,
) -> Result<Vec<PathBuf>> {
    // Normalize pattern to use the separator
    let normalized_pattern = pattern.replace('.', &separator.to_string());

    // Use existing find_matching_files logic
    let files = find_matching_files(&normalized_pattern, separator, dir, max_depth)?;

    Ok(files)
}

/// Display merged tree
fn display_tree(
    files: &[PathBuf],
    base_pattern: &str,
    separator: char,
    unicode: bool,
    show_count: bool,
    json: bool,
) -> Result<()> {
    // Build tree structure
    let mut tree_builder = TreeBuilder::new(separator);

    for file in files {
        tree_builder.add_file(file);
    }

    // Display
    if json {
        tree_builder.print_json()?;
    } else {
        println!("{} (base)", base_pattern);
        tree_builder.print(unicode, show_count)?;
    }

    Ok(())
}
```

### Step 3: Add to Module System

**File:** `src/main.rs`

Add module declaration:
```rust
mod main_command_merge_impl;
```

### Step 4: Handle Pattern Normalization

Key insight: When user specifies:
- Pattern: `"main.command.tree"`
- Separator: `_`

We need to search for files like `main_command_tree_*.rs`, not `main.command.tree`.

The `find_files_for_pattern` function does:
```rust
let normalized_pattern = pattern.replace('.', &separator.to_string());
// "main.command.tree" with sep='_' → "main_command_tree"
```

### Step 5: Deduplication Strategy

```rust
let mut seen: HashSet<PathBuf> = HashSet::new();

for file in files {
    if seen.insert(file.clone()) {  // Returns false if already present
        all_files.push(file);
    }
}
```

## Testing Strategy

### Manual Test 1: Basic Two-Pattern Merge

```bash
# Setup
cd c:\src\recur

# Test merge
recur merge \
  --pattern "main.command.tree" --sep "." \
  --pattern "main_command_tree" --sep "_"

# Expected output:
# main.command.tree (base)
# ├── readme.md
# ├── test.jl
# └── impl.rs
```

### Manual Test 2: Three-Pattern Merge

```bash
recur merge \
  --pattern "main.trait.separator" --sep "." \
  --pattern "trait.separator" --sep "-" \
  --pattern "separator_merge" --sep "_"
```

### Edge Case Tests

1. **No files found**
   ```bash
   recur merge --pattern "nonexistent" --sep "."
   # Expected: "No files found"
   ```

2. **Mismatched pattern/sep count**
   ```bash
   recur merge --pattern "foo" --sep "." --sep "_"
   # Expected: Error message
   ```

3. **Single pattern (degenerate case)**
   ```bash
   recur merge --pattern "main" --sep "."
   # Expected: Works like normal tree
   ```

## Success Criteria

- [x] CLI accepts multiple --pattern and --sep arguments
- [ ] Patterns and separators are paired correctly
- [ ] Files found from all pattern/separator pairs
- [ ] Duplicate files removed (same path)
- [ ] Unified tree displayed
- [ ] Uses first separator as canonical form
- [ ] All tests pass
- [ ] Can merge recur's own docs + source

## Files to Create/Modify

### New Files
- `src/main_command_merge_impl.rs` - Core implementation

### Modified Files
- `src/main.rs` - Add merge subcommand and module

## Dependencies

Uses existing recur infrastructure:
- `hierarchical_path::find_matching_files()` - File discovery
- `tree::TreeBuilder` - Tree construction
- Existing separator parsing logic

## Known Issues / Limitations

1. **Pattern normalization is naive** - Just replaces dots
   - More sophisticated conversion needed later
   - Current approach: `"main.x.y"` → `"main_x_y"` when sep='_'

2. **No provenance tracking yet** - Can't tell which pattern found which file
   - That's Phase 3 (markers)

3. **Tree display uses first separator only** - Not necessarily best choice
   - Could be configurable later

4. **No validation of separator characters** - Could use invalid chars
   - Should validate separator is single ASCII char

## Next Phase Preview

**Phase 3: Provenance Tracking**
- Track which separator found each file
- Implement `--show-sep` markers
- Display like: `impl.rs [_]`
- Handle files found by multiple patterns

## Timeline Estimate

- Step 1-3: CLI structure (~30 min)
- Step 4: Pattern normalization (~30 min)
- Step 5: Deduplication (~15 min)
- Testing & debugging (~45 min)

**Total: ~2 hours**
