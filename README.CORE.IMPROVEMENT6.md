# Core Improvement 6: Git Integration & Impact Analysis

## Overview

Make `recur` **Git-aware** and **pipeline-friendly** to provide impact analysis for code changes. Embrace Unix philosophy: do one thing well, compose with other tools.

**Philosophy**:
- Make Git decisions with full awareness of hierarchical dependencies
- Composable with standard Unix tools (`git`, `grep`, `sed`, `awk`, `xargs`)
- Pipelines over monolithic commands
- Each command accepts stdin, outputs stdout

## Motivation

### Current Pain Points

1. **Blind Refactoring**: Developers modify functions without knowing who calls them
2. **Merge Anxiety**: Unclear what changed files depend on or affect
3. **Review Overhead**: PR reviewers can't quickly see impact of changes
4. **Commit Scope Creep**: Hard to know if a commit touches unrelated hierarchies

### What This Enables

- **Pre-commit validation**: "Does this change affect critical paths?"
- **PR context**: "What depends on these modified functions?"
- **Refactor confidence**: "Who will break if I change this?"
- **Merge planning**: "What hierarchies are affected by this branch?"

## Core Enhancement: `--stdin` Flag

Add `--stdin` flag to ALL commands to accept file paths from stdin (one per line). This makes `recur` fully composable with Git and other Unix tools.

```bash
# Pattern: git diff | recur <command> --stdin

# Examples:
git diff --name-only | recur files --stdin
git diff --staged --name-only | recur tree --stdin
git ls-files "*.cs" | recur stats --stdin
```

**Implementation**: All commands check for `--stdin` flag and read paths from stdin if present.

## User Workflows

### Workflow 1: Impact Analysis Before Commit

**Scenario**: You modified `ValidateEmail()` in `UserService.Auth.cs`. What breaks?

```bash
# Unix pipeline approach (composable)
git diff --staged --name-only | \
  xargs grep -l "ValidateEmail" | \
  xargs -I {} recur callers "ValidateEmail" {} --depth 2

# Or find all modified functions and analyze each
git diff --staged --name-only "*.cs" | while read file; do
  echo "=== Impact of $file ==="
  # Extract function names, find callers
  grep -E "^\s*(public|private|protected)" "$file" | \
    sed 's/.* \(\w\+\)\s*(.*/\1/' | \
    xargs -I {} recur callers "{}" --scope "**" --count
done
```

**Output**:
```
=== Impact of UserService.Auth.cs ===

ValidateEmail:
  3 direct callers
  8 total callers (depth 2)

HashPassword:
  2 direct callers
  5 total callers (depth 2)
```

### Workflow 2: Hierarchical View of Changed Files

**Scenario**: You have 10 modified files. Which hierarchies are affected?

```bash
# Pipe git output to recur
git diff --name-only | recur files --stdin

# Or for staged changes
git diff --staged --name-only | recur files --stdin

# JSON output for tooling
git status --short | awk '{print $2}' | recur files --stdin --json
```

**Output**:
```
LevelController.** (5 files)
  LevelController.CreateWizard3.cs
  LevelController.CreateWizard3.Validation.cs
  LevelController.CreateWizard3.AI.cs
  LevelController.CreateWizard3.Persistence.cs

config.** (2 files)
  config.database.json
  config.api.json

Flat files (3 files):
  AuthService.cs
  Logger.cs
  README.md
```

### Workflow 3: Branch Comparison & PR Stats

**Scenario**: Compare current branch to main. What changed?

```bash
# Get changed files between branches
git diff main..HEAD --name-only

# Hierarchical breakdown
git diff main..HEAD --name-only | recur files --stdin

# Stats on changed files
git diff main..HEAD --name-only | recur stats --stdin

# Find what functions changed (using git diff + grep)
git diff main..HEAD "*.cs" | grep "^+.*public\|^+.*private" | \
  sed 's/.*[+ ] \(\w\+\)\s*(.*/\1/' | sort -u
```

**Output**:
```
# Hierarchies affected
UserService.** (7 files, 245 lines)
LevelController.** (3 files, 128 lines)
Flat files (2 files, 45 lines)

# New functions added
+ ValidateEmailFormat
+ HashPasswordWithSalt
+ CheckPermissionsV2
```

### Workflow 4: Scope Validation (Unix Style)

**Scenario**: Ensure commit only touches expected hierarchy.

```bash
# Check if staged files match pattern
git diff --staged --name-only | grep -E "^UserService\."

# Count files in scope vs out of scope
in_scope=$(git diff --staged --name-only | grep -c "^UserService\." || echo 0)
total=$(git diff --staged --name-only | wc -l)
out_scope=$((total - in_scope))

echo "In scope: $in_scope, Out of scope: $out_scope"

# Fail if files outside scope
if [ $out_scope -gt 0 ]; then
  echo "⚠️  Warning: Files outside UserService.** scope:"
  git diff --staged --name-only | grep -v "^UserService\."
  exit 1
fi
```

### Workflow 5: Pre-Commit Impact Check

**Scenario**: Git hook to analyze impact before commit.

```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "Analyzing impact of staged changes..."

# Get staged .cs files
staged=$(git diff --staged --name-only --diff-filter=ACM | grep "\.cs$")

if [ -z "$staged" ]; then
  exit 0  # No .cs files changed
fi

# Show hierarchical breakdown
echo "$staged" | recur files --stdin

# Count hierarchies touched
hierarchies=$(echo "$staged" | recur files --stdin --json | \
              jq -r 'keys[]' | wc -l)

echo "Hierarchies modified: $hierarchies"

# Warn if too many hierarchies
if [ "$hierarchies" -gt 3 ]; then
  echo "⚠️  Warning: Commit touches $hierarchies hierarchies"
  echo "Consider splitting into smaller commits"
  read -p "Continue? (y/n) " -n 1 -r
  echo
  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
  fi
fi
```

### Workflow 6: Find All Callers of Changed Functions

**Scenario**: What will break if I commit these changes?

```bash
# Extract all function definitions from staged files
git diff --staged --name-only "*.cs" | while read file; do
  # Find function names in file
  grep -E "^\s*(public|private|protected)" "$file" | \
    sed -E 's/.*\s+(\w+)\s*\(.*/\1/' | \
    while read func; do
      echo "=== Callers of $func (in $file) ==="
      recur callers "$func" --scope "**" --depth 1 | head -10
      echo
    done
done
```

## Core Implementation: `--stdin` Flag

Add `--stdin` flag to ALL existing commands. This is the key enhancement that makes `recur` Git-aware and pipeline-friendly.

### Updated Command Signatures

```bash
# All commands gain --stdin flag
recur files <PATTERN> [--stdin]
recur find <QUERY> [--stdin]
recur tree <PATTERN> [--stdin]
recur stats <PATTERN> [--stdin]
recur callers <FUNCTION> [--stdin]
recur callees <FUNCTION> [--stdin]
recur trace <FUNCTION> [--stdin]
```

### Behavior with `--stdin`

When `--stdin` is specified:
1. Read file paths from stdin (one per line)
2. Use those paths instead of searching filesystem
3. Apply all other options normally (scope, filters, etc.)

**Example Implementation** (in `src/main.rs`):

```rust
fn get_input_files(
    stdin_flag: bool,
    pattern: &HierarchyPattern,
    root: &Path
) -> Result<Vec<PathBuf>> {
    if stdin_flag {
        // Read from stdin
        let stdin = std::io::stdin();
        let mut files = Vec::new();
        for line in stdin.lock().lines() {
            let path = PathBuf::from(line?);
            if pattern.matches(&path) {
                files.push(path);
            }
        }
        Ok(files)
    } else {
        // Normal filesystem search
        let searcher = FileSearcher::new(SearchOptions {
            root: root.to_path_buf(),
            ..Default::default()
        });
        Ok(searcher.find(pattern))
    }
}
```

### Integration Examples

#### 1. Files Command
```bash
# Normal usage
recur files "UserService.**"

# With Git (stdin)
git diff --staged --name-only | recur files "**" --stdin

# Filter files from Git
git ls-files "*.cs" | recur files "UserService.**" --stdin
```

#### 2. Stats Command
```bash
# Normal usage
recur stats "**"

# Stats on changed files only
git diff main..HEAD --name-only | recur stats "**" --stdin

# Stats on specific file list
find src -name "*.cs" | recur stats "**" --stdin
```

#### 3. Callers/Callees Command
```bash
# Normal usage (searches all files)
recur callers "ValidateEmail" --scope "**"

# Search only in changed files
git diff --staged --name-only | \
  recur callers "ValidateEmail" --scope "**" --stdin

# Narrow scope to specific files
git diff --name-only | grep "UserService" | \
  recur callers "CreateUser" --stdin
```

#### 4. Trace Command
```bash
# Normal usage
recur trace "ProcessData" --depth 2

# Trace only within changed files
git diff --name-only | \
  recur trace "ProcessData" --depth 2 --stdin
```

## Optional Convenience Commands (Phase 2)

These are **optional** shortcuts for common workflows. The `--stdin` flag is the core feature.

### `recur git-files` (convenience wrapper)

Shortcut for: `git diff --name-only | recur files --stdin`

```bash
recur git-files [--staged|--unstaged|--all]

# Equivalent to:
git diff --staged --name-only | recur files "**" --stdin
```

### `recur git-stats` (convenience wrapper)

Shortcut for: `git diff --name-only | recur stats --stdin`

```bash
recur git-stats [--staged|--unstaged|--all] [--base BRANCH]

# Equivalent to:
git diff main..HEAD --name-only | recur stats "**" --stdin
```

**Decision**: Implement `--stdin` first (Phase 1). Add convenience commands only if users request them (Phase 2)

## Implementation Plan

### Phase 1: Add `--stdin` Flag (Core Feature)

**Estimated time**: 2-3 hours

**Files to modify**:
- `src/main.rs` - Add `--stdin` flag to all command definitions
- `src/search.rs` - Add helper function to read files from stdin

**Step 1.1**: Create stdin helper utility

```rust
// src/search.rs

use std::io::{BufRead, stdin};
use std::path::PathBuf;
use antml::{Result, Context};

/// Read file paths from stdin (one per line)
pub fn read_paths_from_stdin() -> Result<Vec<PathBuf>> {
    let stdin = stdin();
    let mut paths = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.context("Failed to read line from stdin")?;
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            paths.push(PathBuf::from(trimmed));
        }
    }

    Ok(paths)
}
```

**Step 1.2**: Add `--stdin` flag to command structs

```rust
// src/main.rs - Update all Commands variants

Commands::Files {
    pattern: String,
    // ... existing fields ...

    /// Read file paths from stdin instead of searching filesystem
    #[arg(long)]
    stdin: bool,
}

Commands::Stats {
    pattern: String,
    // ... existing fields ...

    #[arg(long)]
    stdin: bool,
}

Commands::Callers {
    function: String,
    // ... existing fields ...

    #[arg(long)]
    stdin: bool,
}

// Repeat for: Callees, Trace, Find, Tree, etc.
```

**Step 1.3**: Update command handlers to use stdin

```rust
// src/main.rs - Example for cmd_files

fn cmd_files(
    pattern: String,
    dir: PathBuf,
    stdin: bool,  // NEW parameter
    json: bool,
    color: bool,
) -> Result<()> {
    let scope_pattern = HierarchyPattern::parse(&pattern)?;

    // Get files from stdin or filesystem
    let files = if stdin {
        let all_paths = read_paths_from_stdin()?;
        // Filter by pattern
        all_paths.into_iter()
            .filter(|p| scope_pattern.matches(p))
            .collect()
    } else {
        // Normal filesystem search
        let searcher = FileSearcher::new(SearchOptions {
            root: dir,
            ..Default::default()
        });
        searcher.find(&scope_pattern)
    };

    // Rest of function unchanged...
    if json {
        println!("{}", JsonFormatter::format_file_list(&files));
    } else {
        let mut formatter = TerminalFormatter::new(color);
        formatter.print_file_list(&files);
    }

    Ok(())
}
```

**Step 1.4**: Apply same pattern to other commands

Repeat Step 1.3 for:
- `cmd_stats` - stats on stdin files
- `cmd_callers` - find callers only in stdin files
- `cmd_callees` - find callees only in stdin files
- `cmd_trace` - trace within stdin files
- `cmd_find` - search content in stdin files
- `cmd_tree` - show tree of stdin files

### Phase 2: Optional Convenience Wrappers (If Requested)

**Estimated time**: 2-3 hours (ONLY if users request it)

Add `git-files`, `git-stats`, `git-impact` as convenience wrappers that call `git` internally and pipe to existing commands.

**Implementation**:
```rust
// src/main.rs

Commands::GitFiles {
    #[arg(long)]
    staged: bool,
    // ... other options
}

fn cmd_git_files(staged: bool, ...) -> Result<()> {
    // Call git diff internally
    let output = Command::new("git")
        .args(&["diff", "--staged", "--name-only"])
        .output()?;

    let files_str = String::from_utf8(output.stdout)?;

    // Parse and display with existing file grouping logic
    // ...
}
```

**Decision**: Only implement Phase 2 if users find typing `git diff --name-only | recur files --stdin` too verbose. The `--stdin` flag provides all the core functionality.

## Testing Strategy

### Unit Tests

```bash
# Test stdin functionality
echo -e "file1.cs\nfile2.cs" | recur files "**" --stdin
echo -e "UserService.cs\nAuthService.cs" | recur stats "**" --stdin

# Test filtering with stdin
echo -e "UserService.Auth.cs\nLogger.cs" | recur files "UserService.**" --stdin
```

### Integration Tests with Git

```bash
# Real-world pipeline tests
git diff --staged --name-only | recur files "**" --stdin
git diff main..HEAD --name-only | recur stats "**" --stdin
git ls-files "*.cs" | recur files "**" --stdin --json

# Test with empty stdin
echo "" | recur files "**" --stdin

# Test with non-existent files
echo "nonexistent.cs" | recur files "**" --stdin
```

### Julia Tests

Add tests to existing `julia-tests/runtests.*.jl` files:

```julia
# In runtests.files.jl
@testset "files with stdin" begin
    # Create temp file list
    files = ["UserService.cs", "UserService.Auth.cs"]
    input = join(files, "\n")

    success, output, _ = run_recur_with_stdin("files \"**\" --stdin", input)

    @test success
    @test contains(output, "UserService.cs")
    @test contains(output, "UserService.Auth.cs")
end
```

## Success Metrics

- ✅ All `recur` commands accept `--stdin` flag
- ✅ Composable with standard Unix tools (`git`, `grep`, `awk`, `xargs`)
- ✅ Git workflows become one-liners
- ✅ Pre-commit hooks can use `recur` for validation
- ✅ No new dependencies required (pure stdin/stdout)

## Future Enhancements

### Phase 2: Integration with Modern Tools

**ripgrep (`rg`)** integration:
```bash
# Find all callers in recently modified files
rg -l "class.*Service" | recur callers "ProcessData" --stdin
```

**fd** integration:
```bash
# Stats on recently changed .cs files
fd --changed-within 1week "\.cs$" | recur stats "**" --stdin
```

**jq** for JSON pipelines:
```bash
# Get hierarchies with >100 lines changed
git diff main..HEAD --name-only | \
  recur stats "**" --stdin --json | \
  jq '.[] | select(.lines > 100) | .hierarchy'
```

### Phase 3: Advanced Git Features

1. **Branch comparison**: Compare caller/callee between branches
2. **Merge conflict prediction**: Based on function dependencies
3. **Test selection**: Suggest tests to run based on modified functions
4. **Coverage analysis**: Show test coverage for modified code

## Dependencies

**NO new dependencies required!**

The `--stdin` approach uses only:
- Standard library `std::io::stdin()`
- Existing `recur` functionality
- Unix pipes (provided by shell)

**Optional (Phase 2 only, if convenience commands needed)**:
- Could add `git2` for `recur git-files` convenience wrappers
- But pipelines are preferred

## Design Decisions (Finalized)

### 1. stdin Over Git Library
**Decision**: Use `--stdin` flag + Unix pipes instead of `git2` library.

**Rationale**:
- **Composability**: Works with any tool that outputs file paths (git, find, fd, etc.)
- **Simplicity**: No new dependencies, smaller binary
- **Unix philosophy**: Do one thing well, compose with pipes
- **Flexibility**: Users can use any git command, not just what we implement
- **Maintainability**: No need to keep up with git2 API changes

### 2. No Convenience Commands (Initially)
**Decision**: Implement ONLY `--stdin` flag in Phase 1. Skip `git-*` commands unless users request them.

**Rationale**:
- Users already know `git diff --name-only`
- Pipelines are more powerful and flexible
- Fewer commands to maintain
- Teaches Unix composition patterns

### 3. Filter After Reading
**Decision**: Apply pattern filtering AFTER reading from stdin.

**Rationale**:
- Let user control what goes into stdin
- `recur` filters based on pattern
- Composable: `git diff --name-only | grep UserService | recur files "**" --stdin`

### 4. One Path Per Line
**Decision**: stdin format is one file path per line (standard Unix convention).

**Rationale**:
- Matches `find` output, `git diff --name-only`, `ls`, etc.
- Easy to generate with any tool
- Easy to parse in any language

## Estimated Implementation Time

- **Phase 1** (`--stdin` flag):
  - Step 1.1 (stdin helper): 30 minutes
  - Step 1.2 (add flags): 30 minutes
  - Step 1.3-1.4 (update handlers): 1.5 hours
  - Testing: 1 hour
  - Documentation: 30 minutes
  - **Total**: ~4 hours

- **Phase 2** (convenience wrappers): 2-3 hours (ONLY if requested)

**Grand Total**: ~4 hours for full Unix-style Git integration!

## Comparison: Before vs After

### Before (IMPROVEMENT6 initial design)
- New `git2` dependency
- 5 new Git-specific commands
- ~15 hours implementation
- 500+ lines of Git integration code
- Maintenance burden

### After (Unix-friendly design)
- **Zero** new dependencies
- **One** new flag (`--stdin`)
- ~4 hours implementation
- ~50 lines of code
- Composable with ANY tool

---

**Status**: Design finalized - ready to implement after IMPROVEMENT5 (trace command) is complete.

**Philosophy**: Embrace Unix. Let `git` do Git, let `recur` do hierarchical analysis. Pipes connect them.
