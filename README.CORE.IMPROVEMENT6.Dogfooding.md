# IMPROVEMENT6 Dogfooding: Using Recur on Itself

## 🍽️ Eating Our Own Dog Food

**Core Idea**: Use recur's hierarchical naming conventions on recur's own source code to make the codebase self-documenting and queryable.

## Why Dogfooding?

Instead of maintaining separate documentation about which commands support which features, we encode this directly in the file names. Then we can use **recur itself** to discover:

- Which commands support stdin?
- Which commands use content search?
- Which commands have tests?
- Which features have trait implementations?
- What's the overall architecture?

**Example Queries:**
```bash
# See all commands and their types
recur tree "command" -d src/

# Find all standard file-list commands
recur files "command.**.std" -d src/

# Find all content-search commands
recur files "command.**.search" -d src/

# See what traits exist
recur files "trait.**" -d src/

# Find all stdin-capable implementations
recur files "**.stdin.**" -d src/

# Check test coverage (later, with Julia tests named similarly)
recur files "test.command.**" -d julia-tests/
```

---

## Proposed File Hierarchy

### Current State
```
src/
├── main.rs          // All commands in one massive file (~1200 lines)
├── search.rs        // All searchers mixed together
├── parser.rs
├── tree.rs
└── output.rs
```

### Proposed Hierarchical Structure

```
src/
├── main.rs                       // CLI entry point only
│
├── trait.stdin.rs                // Stdin capability trait
├── trait.content_search.rs       // Content search trait
│
├── command.files.std.rs          // Standard file-list command
├── command.stats.std.rs          // Standard file-list command
├── command.tree.std.rs           // Standard file-list command
├── command.related.std.rs        // Standard file-list command
├── command.children.std.rs       // Standard file-list command
│
├── command.find.search.rs        // Content-search command
├── command.id.search.rs          // Content-search command
├── command.callers.search.rs     // Content-search command
├── command.callees.search.rs     // Content-search command
├── command.trace.search.rs       // Content-search command
│
├── parser.rs                     // HierarchyPattern, HierarchicalName
├── tree.rs                       // HierarchyTree
└── output.rs                     // Formatters
```

### File Naming Convention

**Pattern**: `<category>.<name>.<type>.rs`

**Categories:**
- `trait.*` - Trait definitions and shared implementations
- `command.*` - Command implementations
- `test.*` (future) - Test modules

**Types for Commands:**
- `.std` - Standard file-list operations (uses FileSearcher)
- `.search` - Content-search operations (uses ContentSearcher, CallerSearcher, etc.)

**Examples:**
- `command.files.std.rs` - Files command (standard)
- `command.find.search.rs` - Find command (content search)
- `trait.stdin.rs` - Stdin capability trait

---

## Trait Definitions

### `trait.stdin.rs`

Contains the `StdinCapable` trait and helper functions:

```rust
use std::path::PathBuf;
use anyhow::Result;
use crate::parser::{HierarchyPattern, HierarchicalName};

/// Read file paths from stdin (one per line)
pub fn read_paths_from_stdin() -> Result<Vec<PathBuf>> {
    // Implementation
}

/// Trait for commands that can read from stdin
pub trait StdinCapable {
    /// Filter stdin paths by a hierarchical pattern
    fn filter_stdin_paths(
        paths: Vec<PathBuf>,
        pattern: &HierarchyPattern,
        extensions: Option<&[String]>,
    ) -> Vec<PathBuf> {
        paths
            .into_iter()
            .filter(|p| {
                // Extract hierarchical name and match
                if let Some(filename) = p.file_name().and_then(|n| n.to_str()) {
                    let name_without_ext = filename.rsplit_once('.')
                        .map(|(name, _)| name)
                        .unwrap_or(filename);
                    let hier_name = HierarchicalName::new(name_without_ext);
                    pattern.matches(&hier_name)
                } else {
                    false
                }
            })
            .filter(|p| {
                // Apply extension filter if specified
                if let Some(exts) = extensions {
                    if let Some(file_ext) = p.extension().and_then(|e| e.to_str()) {
                        exts.iter().any(|e| {
                            let e = e.trim_start_matches('.');
                            file_ext == e
                        })
                    } else {
                        false
                    }
                } else {
                    true
                }
            })
            .collect()
    }
}
```

### `trait.content_search.rs`

Contains the `ContentSearchCapable` trait for searching within files:

```rust
use std::path::PathBuf;
use anyhow::Result;
use crate::search::{SearchResult, SearchOptions};

/// Trait for commands that search file contents
pub trait ContentSearchCapable {
    /// Search specific files for a pattern (instead of scanning filesystem)
    fn search_files(
        files: Vec<PathBuf>,
        query: &str,
        options: &SearchOptions,
    ) -> Result<Vec<SearchResult>>;

    /// Search specific files with regex
    fn search_files_regex(
        files: Vec<PathBuf>,
        regex: &regex::Regex,
        options: &SearchOptions,
    ) -> Result<Vec<SearchResult>>;
}
```

---

## Command Implementation Pattern

### Standard Command (File-List Based)

**File**: `command.files.std.rs`

```rust
use crate::trait_stdin::{read_paths_from_stdin, StdinCapable};
use crate::parser::HierarchyPattern;
use crate::search::{FileSearcher, SearchOptions};

pub struct FilesCommand;

impl StdinCapable for FilesCommand {}

impl FilesCommand {
    pub fn execute(
        pattern: String,
        dir: PathBuf,
        ext: Option<String>,
        stdin: bool,
        // ... other params
    ) -> anyhow::Result<()> {
        let pattern = HierarchyPattern::parse(&pattern)?;

        let files = if stdin {
            // Use trait helper
            let stdin_paths = read_paths_from_stdin()?;
            let extensions = ext.map(|s|
                s.split(',').map(|x| x.trim().to_string()).collect::<Vec<_>>()
            );
            Self::filter_stdin_paths(
                stdin_paths,
                &pattern,
                extensions.as_deref(),
            )
        } else {
            // Use filesystem search
            let mut options = SearchOptions {
                root: dir,
                ..Default::default()
            };
            if let Some(ext_str) = ext {
                options.extensions = ext_str.split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
            }
            let searcher = FileSearcher::new(options);
            searcher.find(&pattern)
        };

        // Output files...
        Ok(())
    }
}
```

### Content-Search Command

**File**: `command.find.search.rs`

```rust
use crate::trait_stdin::{read_paths_from_stdin, StdinCapable};
use crate::trait_content_search::ContentSearchCapable;
use crate::search::{ContentSearcher, SearchOptions};

pub struct FindCommand;

impl StdinCapable for FindCommand {}
impl ContentSearchCapable for FindCommand {}

impl FindCommand {
    pub fn execute(
        query: String,
        scope: String,
        dir: PathBuf,
        stdin: bool,
        // ... other params
    ) -> anyhow::Result<()> {
        let scope_pattern = HierarchyPattern::parse(&scope)?;

        let results = if stdin {
            // 1. Filter stdin paths by scope
            let stdin_paths = read_paths_from_stdin()?;
            let files = Self::filter_stdin_paths(stdin_paths, &scope_pattern, None);

            // 2. Search within those specific files
            let mut options = SearchOptions {
                root: dir,
                ..Default::default()
            };
            Self::search_files(files, &query, &options)?
        } else {
            // Use filesystem search
            let searcher = ContentSearcher::new(options);
            searcher.search(&query, &scope_pattern)
        };

        // Output results...
        Ok(())
    }
}
```

---

## Benefits of This Approach

### 1. **Self-Documentation**
```bash
# Instantly see architecture
recur tree "command" -d src/
# Output:
# command
# ├── command.files.std.rs
# ├── command.stats.std.rs
# ├── command.find.search.rs
# └── command.callers.search.rs
```

### 2. **Queryable Codebase**
```bash
# How many standard commands?
recur files "command.**.std" -d src/ --count
# Output: 5 files

# How many content-search commands?
recur files "command.**.search" -d src/ --count
# Output: 5 files

# Which traits exist?
recur files "trait.**" -d src/
# Output:
# trait.stdin.rs
# trait.content_search.rs
```

### 3. **Feature Matrix Discovery**
```bash
# Find all stdin-capable code
recur files "**.stdin.**" -d src/

# See command-trait relationships
recur related "command.find.search.rs" -d src/
# Shows: trait.stdin.rs, trait.content_search.rs
```

### 4. **Test Coverage Mapping** (Future)

Once Julia tests follow the same pattern:

```bash
# Which commands have tests?
recur files "test.command.**" -d julia-tests/

# Which commands don't have tests?
# (files in src/command.** but not in julia-tests/test.command.**)

# Do all stdin commands have stdin tests?
recur files "test.**.stdin.**" -d julia-tests/
```

### 5. **Documentation Auto-Generation**

Generate docs from structure:

```bash
# Build feature matrix
recur stats "command.**" -d src/ -l 1
# Outputs commands grouped by type (std vs search)

# Generate trait usage report
recur callers "StdinCapable" --scope "command.**"
```

---

## Implementation Plan

### Phase 1: Extract Traits ✅ Ready to implement
1. Create `trait.stdin.rs`
   - Move `read_paths_from_stdin()` from search.rs
   - Define `StdinCapable` trait with `filter_stdin_paths()` helper
   - Export publicly

2. Create `trait.content_search.rs`
   - Define `ContentSearchCapable` trait
   - Add methods for searching specific file lists
   - Extract common search logic

### Phase 2: Refactor Standard Commands
3. Extract each standard command to its own file:
   - `command.files.std.rs` ✅ Already has stdin
   - `command.stats.std.rs` ✅ Already has stdin
   - `command.tree.std.rs` ✅ Already has stdin
   - `command.related.std.rs` ✅ Already has stdin
   - `command.children.std.rs` ✅ Already has stdin

Each file:
- Implements `StdinCapable` trait
- Uses `filter_stdin_paths()` helper
- Self-contained command logic

### Phase 3: Refactor Content-Search Commands
4. Extract each content-search command:
   - `command.find.search.rs` ⏳ Needs stdin implementation
   - `command.id.search.rs` ⏳ Needs stdin implementation
   - `command.callers.search.rs` ⏳ Needs stdin implementation
   - `command.callees.search.rs` ⏳ Needs stdin implementation
   - `command.trace.search.rs` ⏳ Needs stdin implementation

Each file:
- Implements both `StdinCapable` and `ContentSearchCapable`
- Uses trait helpers for stdin + content search
- Can search specific files from stdin

### Phase 4: Cleanup
5. Update `main.rs`
   - Import all command modules
   - Route CLI commands to respective modules
   - Keep thin, just dispatch logic

6. Update `search.rs`
   - Remove `read_paths_from_stdin` (moved to trait)
   - Keep core searchers: FileSearcher, ContentSearcher, etc.
   - Add methods for searching specific files (used by trait)

### Phase 5: Test Alignment (Future)
7. Rename Julia tests to match:
   - `julia-tests/test.command.files.std.jl`
   - `julia-tests/test.command.find.search.jl`
   - `julia-tests/test.trait.stdin.jl`

8. Create test discovery script:
   ```bash
   # Show test coverage matrix
   ./scripts/test_coverage_matrix.sh
   ```

---

## Example Queries After Refactoring

### Architecture Discovery
```bash
# See full hierarchy
recur tree "" -d src/

# Group by type
recur stats "command.**" -d src/ -l 1
# Output:
# Depth 0: 0 files
# Depth 1: 10 files (all commands)
#   - command.files.std.rs
#   - command.stats.std.rs
#   - ...

# Find all traits
recur files "trait.**" -d src/
```

### Feature Matrix
```bash
# Standard commands (5)
recur files "command.**.std" -d src/ --count

# Content-search commands (5)
recur files "command.**.search" -d src/ --count

# Commands with stdin support (10 - all of them)
recur files "command.**" -d src/ --count
```

### Cross-References
```bash
# Which files use the stdin trait?
recur callers "StdinCapable" --scope "**" -d src/

# Which files implement content search?
recur callers "ContentSearchCapable" --scope "**" -d src/

# Related files to find command
recur related "command.find.search.rs" -d src/
```

### Gap Analysis
```bash
# Commands without tests (when tests are named hierarchically)
# Compare: src/command.**.rs vs julia-tests/test.command.**.jl

# Features without documentation
# Compare: src/trait.**.rs vs docs/trait.**.md
```

---

## Migration Strategy

### Step 1: Create Trait Files (No Breaking Changes)
- Extract `trait.stdin.rs` with helpers
- Extract `trait.content_search.rs` with helpers
- Keep old code in place, add new trait code alongside
- Run tests to verify nothing breaks

### Step 2: Migrate One Command (Validate Pattern)
- Pick `command.files.std.rs`
- Extract to new file, implement traits
- Update main.rs to use new module
- Run tests to verify
- **Checkpoint**: If this works, proceed. If not, adjust pattern.

### Step 3: Migrate Remaining Standard Commands
- Extract all std commands one by one
- Each is self-contained and implements StdinCapable
- Test after each extraction

### Step 4: Migrate Content-Search Commands
- Extract all search commands one by one
- Implement both StdinCapable and ContentSearchCapable
- Add stdin support using traits
- Test after each extraction

### Step 5: Final Cleanup
- Remove old code from main.rs
- Update imports and module structure
- Run full test suite
- Document new structure in CONTRIBUTING.md

---

## Success Criteria

✅ **Code Organization**
- Each command in its own file
- Clear trait boundaries
- No duplicated stdin logic

✅ **Discoverability**
- Can run `recur tree "command"` to see all commands
- Can run `recur files "**.std"` to find standard commands
- Can run `recur files "trait.**"` to see all traits

✅ **Test Coverage**
- All tests pass after refactoring
- No functionality lost
- New commands easier to add

✅ **Documentation**
- File structure is self-documenting
- Can generate feature matrix from files
- Easy to onboard new contributors

---

## Future Extensions

### 1. **Test Naming Alignment**
```
julia-tests/
├── test.command.files.std.jl
├── test.command.find.search.jl
├── test.trait.stdin.jl
└── test.integration.git_workflows.jl
```

Then query test coverage:
```bash
# Commands with tests
recur files "test.command.**" -d julia-tests/

# Traits with tests
recur files "test.trait.**" -d julia-tests/
```

### 2. **Documentation Hierarchy**
```
docs/
├── guide.command.files.md
├── guide.command.find.md
├── guide.trait.stdin.md
└── guide.architecture.overview.md
```

Query docs:
```bash
# All command guides
recur files "guide.command.**" -d docs/
```

### 3. **Auto-Generated Feature Matrix**

Script that runs recur on itself:
```bash
#!/bin/bash
# scripts/feature_matrix.sh

echo "Command Feature Matrix"
echo "======================"
echo ""

echo "Standard Commands (File-List):"
recur files "command.**.std" -d src/

echo ""
echo "Content-Search Commands:"
recur files "command.**.search" -d src/

echo ""
echo "Traits:"
recur files "trait.**" -d src/

echo ""
echo "Test Coverage:"
for cmd in src/command.*.rs; do
    base=$(basename "$cmd" .rs)
    test_file="julia-tests/test.${base}.jl"
    if [ -f "$test_file" ]; then
        echo "✅ $base"
    else
        echo "❌ $base (no test)"
    fi
done
```

---

## Notes

### Why Not Directories?

**Alternative**: Use directories like `src/command/files/std.rs`

**Chosen approach**: Use dots like `src/command.files.std.rs`

**Reasoning**:
1. **Flat is better than nested** (for this use case)
2. **Easier to grep/search** - one `src/` directory
3. **Matches recur's naming** - dots are natural delimiters
4. **Works with recur's patterns** - `command.**.std` is clean
5. **Fewer navigation clicks** - no deep directory diving

### Naming Alternatives Considered

**Option 1**: `command-files-std.rs` (hyphens)
- ❌ Hyphens harder to parse hierarchically
- ❌ Doesn't match recur's dot-notation philosophy

**Option 2**: `files_std_command.rs` (underscores)
- ❌ Rust convention but not hierarchical
- ❌ Can't use recur patterns on it

**Option 3**: `command.files.std.rs` (dots) ✅ **CHOSEN**
- ✅ Matches recur's hierarchy pattern
- ✅ Works with `**` and `*` patterns
- ✅ Self-documenting structure
- ✅ Queryable with recur itself

---

## Conclusion

By using recur's own naming conventions on itself, we create a **self-documenting, queryable codebase** where the structure is discoverable through the tool itself.

This is the ultimate dogfooding: **recur understanding recur**.

The hierarchical file names aren't just organizational—they're functional metadata that makes the codebase navigable and analyzable using the very tool we're building.

**Next Steps:**
1. ✅ Document approach (this file)
2. ⏳ Create `trait.stdin.rs`
3. ⏳ Create `trait.content_search.rs`
4. ⏳ Extract standard commands
5. ⏳ Extract content-search commands
6. ⏳ Verify with tests
7. ⏳ Update documentation

---

*"The best way to validate a design is to use it yourself."*
— Dennis Ritchie (probably)
