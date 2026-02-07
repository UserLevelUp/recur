# IMPROVEMENT6 Dogfooding: Using Recur on Itself

## 🍽️ Eating Our Own Dog Food

**Core Idea**: Use recur's hierarchical naming conventions on recur's own source code to make the codebase self-documenting and queryable.

## ⚡ BREAKTHROUGH: Multi-Separator Support

**The Challenge**: Rust doesn't allow dots in filenames (e.g., `main.command.files.stdin.rs` won't compile).

**The Solution**: Recur now supports **multiple hierarchy separators** with the `--sep` flag!

```bash
# Use underscores for Rust modules
recur files "main_command_*_stdin" -d src/ --sep _

# Use dots for documentation
recur files "command.*.doc" -d docs/ --sep .

# Use dashes for kebab-case
recur files "ui-component-*" -d src/ --sep -
```

**Key Features:**
- ✅ **No mixing**: Each query uses ONE separator (no ambiguity)
- ✅ **Language-agnostic**: Works with any naming convention
- ✅ **Rust-friendly**: Use `_` for modules, dots preserved in other chars
- ✅ **Gap detection**: Missing files = missing capabilities (visible!)

### Real Example:

```bash
# Files: main_command_files_impl.rs, main_command_files_stdin.rs, main_command_stats_impl.rs

# Find all command implementations
$ recur files "main_command_*_impl" -d src/ --sep _
✓ main_command_files_impl.rs
✓ main_command_stats_impl.rs

# Find commands WITH stdin support
$ recur files "main_command_*_stdin" -d src/ --sep _
✓ main_command_files_stdin.rs
✓ main_command_stats_stdin.rs

# Gap analysis: which commands DON'T have stdin?
# (commands with _impl but no matching _stdin file)
```

---

## Why Dogfooding?

Instead of maintaining separate documentation about which commands support which features, we encode this directly in the file names. Then we can use **recur itself** to discover:

- Which commands support stdin?
- Which commands use content search?
- Which commands have tests?
- Which features have trait implementations?
- What's the overall architecture?

**Example Queries** (with `--sep _` for Rust):
```bash
# See all commands and their types
recur files "main_command_**" -d src/ --sep _

# Find all standard file-list commands
recur files "main_command_*_impl" -d src/ --sep _

# Find all stdin-capable commands
recur files "main_command_*_stdin" -d src/ --sep _

# See what traits exist
recur files "trait_**" -d src/trait/ --sep _

# Count stdin support coverage
recur files "main_command_*_stdin" -d src/ --sep _ --count

# Check test coverage (later, with Julia tests named similarly)
recur files "main.command.**.test" -d julia-tests/
```

---

## PRIORITY-0: Run `recur` First (Before `rg` or PowerShell)

Before claiming dogfooding is implemented, run the real CLI queries from repository root:

```bash
# 1) Global dot-hierarchy view (docs/tests/metadata branches)
recur tree "main"

# 2) Source hierarchy view with Rust-safe separator
recur tree "main" -d src/ --sep _
recur files "main_command_*_impl" -d src/ --sep _ --count
recur files "main_command_*_stdin" -d src/ --sep _ --count

# 3) Coverage comparison across folders
recur files "main.command.**.test" -d julia-tests/ --count
recur files "main.command.**.readme" -d docs/ --count
```

How to read this:
- If `recur tree "main"` looks rich but `src/` only has a small subset (for example only `files` and `stats`), Phase 2 is not complete yet.
- Dogfooding is only "real" when source command branches, test branches, and doc branches are all visible and coherent under the shared `main` model.

Why `rg` or PowerShell file listing is not enough:
- They only show file presence, not whether `recur` hierarchy parsing and wildcard semantics (`*`, `**`) behave correctly.
- They do not validate separator behavior (`--sep _`) in the actual search/match engine.
- They do not validate stdin execution paths (`--stdin`) and command-level runtime behavior.
- Dogfooding is an end-to-end contract of the tool itself; only `recur` queries prove that contract.

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

### Proposed Hierarchical Structure (Using Underscores for Rust)

```
src/
├── main.rs                       // CLI entry point only
│
├── trait/                        // Traits (directory structure)
│   ├── mod.rs
│   ├── stdin.rs                  // StdinCapable trait
│   └── content_search.rs         // ContentSearchCapable trait
│
├── main_command_files_impl.rs         // Implementation: main.command.files.impl
├── main_command_files_stdin.rs        // Stdin capability: main.command.files.stdin
├── main_command_files_doc.md     // Documentation: main.command.files.doc
│
├── main_command_stats_impl.rs         // Implementation: main.command.stats.impl
├── main_command_stats_stdin.rs        // Stdin capability: main.command.stats.stdin
│
├── main_command_find_impl.rs     // Implementation: main.command.find.impl
├── command_find_search.rs        // Search capability: main.command.find.search
├── command_find_stdin.rs         // Stdin capability: main.command.find.stdin
│
├── parser.rs                     // HierarchyPattern, HierarchicalName
├── tree.rs                       // HierarchyTree
└── output.rs                     // Formatters
```

### File Naming Convention

**Pattern**: `<category>_<name>_<capability>.rs`

Query with: `recur files "<category>_<name>_<capability>" --sep _`

**Categories:**
- `trait_*` - Trait definitions (or use trait/ directory)
- `command_*` - Command implementations and capabilities
- `test_*` (future) - Test modules

**Capabilities:**
- `_impl` - Core implementation
- `_stdin` - Stdin support (trait impl or marker)
- `_search` - Content search support
- `_doc` - Documentation (in docs/ directory)

**Examples:**
- `main_command_files_impl.rs` → query: `main.command.files.impl` (with --sep _)
- `command_find_search.rs` → query: `main.command.find.search` (with --sep _)
- `trait/stdin.rs` → dir hierarchy, query: `trait/*` in trait/

**Visibility of Gaps:**
```bash
# All commands
recur files "main_command_*_impl" -d src/ --sep _

# Commands with stdin
recur files "main_command_*_stdin" -d src/ --sep _

# Missing stdin? Compare the two lists!
```

---

## 🧪 Dogfooding in Action: Real Examples

### Current Test Files

We've created demo files showing the multi-separator approach:

```
src/
├── main_command_files_impl.rs      # main.command.files.impl
├── main_command_files_stdin.rs     # main.command.files.stdin
├── main_command_stats_impl.rs      # main.command.stats.impl
└── main_command_stats_stdin.rs     # main.command.stats.stdin
```

### Query Examples

```bash
# 1. Find ALL commands (implementations)
$ recur files "main_command_*_impl" -d src/ --sep _
src/main_command_files_impl.rs
src/main_command_stats_impl.rs

# 2. Find commands WITH stdin support
$ recur files "main_command_*_stdin" -d src/ --sep _
src/main_command_files_stdin.rs
src/main_command_stats_stdin.rs

# 3. Find everything under 'command' hierarchy
$ recur files "main_command_**" -d src/ --sep _
src/main_command_files_impl.rs
src/main_command_files_stdin.rs
src/main_command_stats_impl.rs
src/main_command_stats_stdin.rs

# 4. COUNT stdin-capable commands
$ recur files "main_command_*_stdin" -d src/ --sep _ --count
2 files

# 5. Find specific command's files
$ recur files "main_command_files_*" -d src/ --sep _
src/main_command_files_impl.rs
src/main_command_files_stdin.rs
```

### Gap Analysis (Future)

Once all commands are refactored:

```bash
# Step 1: Get all command names
ALL_COMMANDS=$(recur files "main_command_*_impl" -d src/ --sep _ | sed 's/_impl.rs//')

# Step 2: Check which ones have stdin
for cmd in $ALL_COMMANDS; do
  if ! recur files "${cmd}_stdin" -d src/ --sep _ > /dev/null 2>&1; then
    echo "❌ Missing stdin: $cmd"
  fi
done

# Step 3: Check which ones have tests
for cmd in $ALL_COMMANDS; do
  test_name="test_${cmd#command_}"
  if ! recur files "${test_name}" -d julia-tests/ --sep _ > /dev/null 2>&1; then
    echo "❌ Missing test: $cmd"
  fi
done
```

### Benefits Demonstrated

✅ **Self-Documenting**: File structure shows capabilities
✅ **Queryable**: Use recur to analyze codebase
✅ **Gap Detection**: Missing files = missing capabilities
✅ **No External Docs**: The code IS the documentation
✅ **Language-Agnostic**: Works with Rust, Python, Julia, etc.

---

## 📝 Extended Dogfooding: TODO and Task Management

### Hierarchical TODO Files

Use the hierarchy to track TODOs, priorities, and blockers alongside your code:

```
src/
├── main_command_files_impl.rs
├── main_command_files_todo.md           # General TODOs for files command
├── main_command_files_todo_priority.md  # High-priority tasks
├── main_command_files_todo_blocker.md   # Blocking issues
│
├── main_command_find_impl.rs
├── main_command_find_todo.md
├── main_command_find_todo_priority.md
```

### TODO Query Examples

```bash
# Find ALL TODOs in the codebase
recur files "main_command_*_todo" -d src/ --sep _

# Find HIGH-PRIORITY TODOs only
recur files "main_command_*_todo_priority" -d src/ --sep _

# Find blockers
recur files "main_command_*_todo_blocker" -d src/ --sep _

# Which commands have NO TODOs? (gap analysis)
# Compare: main_command_*_impl.rs vs main_command_*_todo.md

# TODOs for a specific subsystem
recur files "main_command_files_todo*" -d src/ --sep _
```

### Benefits

✅ **Co-located**: TODOs live with the code they describe
✅ **Queryable**: Use recur to filter by priority/type
✅ **Visible**: Missing TODO files = no known issues (or undocumented)
✅ **Hierarchical**: Organize TODOs by component
✅ **Trackable**: Changes in TODO files = visible in git

### Example TODO File

**src/main_command_files_todo_priority.md**:
```markdown
# High-Priority TODOs: Files Command

## 🔥 P0: Implement stdin filtering for extension
- [ ] Support multiple extensions in stdin mode
- [ ] Add tests for stdin + extension combo

## 🔥 P0: Performance optimization
- [ ] Profile large directory scans
- [ ] Consider parallel file reading

## Related
- See: main_command_files_todo.md for lower-priority tasks
- Blocked by: main_parser_todo_blocker.md (pattern parsing issue)
```

### Future: Automated TODO Reports

```bash
# Generate priority matrix
recur stats "main_command_*_todo" -d src/ --sep _
recur stats "main_command_*_todo_priority" -d src/ --sep _

# Find stale TODOs (no git changes in 90 days)
find_stale_todos() {
  for todo in $(recur files "main_command_*_todo*" -d src/ --sep _); do
    last_change=$(git log -1 --format=%cr "$todo")
    echo "$todo: $last_change"
  done
}
```

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

**File**: `main.command.files.std.rs`

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

**File**: `main.command.find.search.rs`

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
# ├── main.command.files.std.rs
# ├── main.command.stats.std.rs
# ├── main.command.find.search.rs
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
recur related "main.command.find.search.rs" -d src/
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
   - `main.command.files.std.rs` ✅ Already has stdin
   - `main.command.stats.std.rs` ✅ Already has stdin
   - `command.tree.std.rs` ✅ Already has stdin
   - `command.related.std.rs` ✅ Already has stdin
   - `command.children.std.rs` ✅ Already has stdin

Each file:
- Implements `StdinCapable` trait
- Uses `filter_stdin_paths()` helper
- Self-contained command logic

### Phase 3: Refactor Content-Search Commands
4. Extract each content-search command:
   - `main.command.find.search.rs` ⏳ Needs stdin implementation
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
   - `julia-tests/test.main.command.files.std.jl`
   - `julia-tests/test.main.command.find.search.jl`
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
#   - main.command.files.std.rs
#   - main.command.stats.std.rs
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
recur related "main.command.find.search.rs" -d src/
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
- Pick `main.command.files.std.rs`
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
├── test.main.command.files.std.jl
├── test.main.command.find.search.jl
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
├── guide.main.command.files.md
├── guide.main.command.find.md
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

**Chosen approach**: Use dots like `src/main.command.files.std.rs`

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

**Option 2**: Dot-only everywhere (`main.command.files.std.rs`)  ❌ **REJECTED**
- ❌ Invalid for Rust source/module file naming
- ❌ Conflicts with practical compiler/module constraints
- ❌ Creates friction where implementation work actually happens (`src/`)

**Option 3**: Underscore-only everywhere (`main_command_files_std`)  ❌ **REJECTED**
- ❌ Works for Rust, but weakens natural semantic readability in docs/tests
- ❌ Makes cross-folder intent less obvious for humans and LLMs
- ❌ Reduces expressiveness for chain-like metadata (`todo.priority`, etc.)

**Option 4**: Split-by-domain separators  ✅ **CHOSEN**
- ✅ `src/` uses underscore with Rust-safe names (`main_command_*`) + `--sep _`
- ✅ `julia-tests/` and `docs/` use dot-based semantic names (`main.command.*`)
- ✅ Shared `main` root enables consistent cross-folder queries
- ✅ Missing branches (test/readme/todo/priority) pop by visible absence

---

## Root-Level Dogfooding Workflows

Run these from repository root (`c:/src/recur`) to inspect multiple folders with one command set.

```bash
# 1) Source command modules in src/
recur files "main_command_*_impl" -d src/ --sep _
recur files "main_command_*_stdin" -d src/ --sep _

# 2) Julia command tests (main-prefixed dot hierarchy)
recur tree "main" -d julia-tests/
recur files "main.command.**.test" -d julia-tests/

# 3) Docs hierarchy (main-prefixed dot hierarchy)
recur tree "main" -d docs/
recur files "main.command.**.readme" -d docs/
recur files "main.command.**.todo" -d docs/
recur files "main.command.**.todo.priority" -d docs/

# 4) Cross-folder matrix from root with stdin (mixed paths)
printf "src/main_command_files_impl.rs\nsrc/main_command_stats_impl.rs\njulia-tests/main.command.files.test.jl\ndocs/main.command.files.readme.md\n" \
  | recur stats "**" --stdin

# 5) Content search only in piped src files
printf "src/main_command_files_impl.rs\nsrc/main_command_stats_impl.rs\n" \
  | recur find "Stdin" --scope "main_command_**" --stdin --sep _

# 6) Improvement roadmap status visibility (docs branch)
recur tree "main.improvement" -d docs/
recur files "main.improvement.*.complete" -d docs/ --count
recur files "main.improvement.**.todo" -d docs/ --count
recur files "main.improvement.**.todo.future-plan" -d docs/ --count
recur files "main.improvement.**.current" -d docs/
recur files "main.improvement.**.current" -d docs/ --count
```

What this validates:
- Root execution works.
- Folder-to-folder querying works (`src/`, `julia-tests/`, and `docs/`).
- `--stdin` now scopes to the piped file list for file, stats, find, callers, callees, and trace flows.
- `--sep _` works consistently for Rust-friendly source naming.
- Dot-separated `main.*` trees expose missing tests/docs/todo priority by visible absence.
- Improvement lifecycle status (complete vs todo vs future-plan) is visible and queryable.
- Current active cursor (`*.current`) is visible and queryable.

---

## Anti-Thesis vs Thesis

### Anti-Thesis (Wrong)

"Dogfooding means one universal naming style across all folders."

Why this is wrong:
1. `src/` has language constraints (Rust does not allow dot-separated module filenames).
2. Docs/tests have different optimization goals than compiled source (semantic clarity over compiler compatibility).
3. Forcing one separator globally causes either invalid source names or less expressive docs/tests.

### New Thesis (Adopted)

"Dogfooding means one shared hierarchy model with domain-appropriate separators."

Model:
1. Shared root: `main`
2. Rust source (`src/`): `main_command_*` queried with `--sep _`
3. Tests/docs (`julia-tests/`, `docs/`): `main.command.*` queried with dot separator
4. Suffix chains encode status/intent (`readme`, `test`, `todo`, `todo.priority`, `todo.current`)

Why this is better:
1. Valid and practical for Rust implementation.
2. Highly readable and expressive for docs/tests.
3. Queryable across folders with one conceptual root.
4. Works for both humans and LLMs as a deterministic coordination contract.

---

## Appendix A: Phase 2 (Execution)

### Phase Number

**Phase 2: Structural Execution and Coverage**

Phase 1 (concept and naming strategy) is complete.  
Phase 2 focuses on making real Rust command code align with the structure.

### Entry Criteria

- `main` naming contract is defined and accepted.
- `julia-tests/` and `docs/` are using `main.command.*` structure.
- Source placeholders exist under `src/main_command_*`.

### Goals

1. Move command implementations out of `src/main.rs` into `src/main_command_*_impl.rs`.
2. Move stdin-specific behavior into `src/main_command_*_stdin.rs` where applicable.
3. Keep CLI behavior and outputs stable while refactoring internals.
4. Use `recur` queries to verify structural coverage after each command migration.

### Initial Command Order (Recommended)

1. `main.command.stats`
2. `main.command.files`
3. `main.command.children`
4. `main.command.related`
5. `main.command.id`
6. `main.command.find`
7. `main.command.callers`
8. `main.command.callees`
9. `main.command.trace`

### Deliverables

- Real implementation modules replace placeholders for each migrated command.
- Matching tests/docs branches remain visible under `main.command.*`.
- Updated TODO/priority files reflect actual migration state.

### Priority-0 Execution Gate (Mandatory First Check)

Run these before per-command migration work:

```bash
# Global hierarchy view (docs/tests/metadata)
recur tree "main"

# Source hierarchy view (Rust-safe separator)
recur tree "main" -d src/ --sep _
recur files "main_command_*_impl" -d src/ --sep _ --count
recur files "main_command_*_stdin" -d src/ --sep _ --count

# Cross-folder coverage snapshot
recur files "main.command.**.test" -d julia-tests/ --count
recur files "main.command.**.readme" -d docs/ --count
```

Interpretation rule:
- Do not infer Phase 2 progress from file listings alone (`rg`, PowerShell, `ls`).
- Treat `recur` output as source of truth for hierarchy semantics (`*`, `**`, `--sep`, and `--stdin` behavior).
- If `recur tree "main"` appears complete but `src/` branch counts are low, Phase 2 extraction is still incomplete.

### Validation Loop (Per Command)

```bash
# 0) Re-run Priority-0 gate snapshot when needed
recur tree "main"
recur files "main_command_*_impl" -d src/ --sep _ --count
recur files "main_command_*_stdin" -d src/ --sep _ --count

# 1) Confirm source branch exists
recur files "main_command_<name>_*" -d src/ --sep _

# 2) Confirm test branch exists
recur files "main.command.<name>.test" -d julia-tests/

# 3) Confirm docs branch exists
recur files "main.command.<name>.readme" -d docs/

# 4) Run test suites
cargo test
julia julia-tests/runtests.jl
```

### Done Criteria for Phase 2

1. All targeted command logic is extracted from `src/main.rs` into `src/main_command_*` modules.
2. Julia integration suite passes with no new regressions.
3. Rust tests pass.
4. `recur tree "main"` and `src/ --sep _` counts both show coherent, aligned structure for migrated commands.
5. TODO priorities in docs accurately reflect remaining work.

---

## Conclusion

By using recur's naming conventions on itself with a shared `main` root and domain-appropriate separators, we get a **self-documenting, queryable codebase** that is practical and consistent.

This is the ultimate dogfooding: **recur understanding recur**.

The hierarchical names are functional metadata, not just organization. They make structure, coverage, and planning state discoverable using the same toolchain we are building.

**Concept Status:** ✅ Complete

What remains is implementation and incremental refinement, not thesis definition:
1. Expand source/test/doc coverage under the naming contract.
2. Add automated gap checks in Julia tests/CI.
3. Continue evolving command capabilities (IMPROVEMENT7+ / IMPROVEMENT9 planning).

---

*"The best way to validate a design is to use it yourself."*
— Dennis Ritchie (probably)



