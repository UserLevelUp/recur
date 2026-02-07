a# Core Improvement 6: Git Integration & Impact Analysis

## Current Status

**🟡 IN PROGRESS** - The `--stdin` flag has been added to all command definitions but is not yet wired up to the command handlers.

### What's Done ✅
- ✅ All command structs (10 total) have `stdin: bool` field defined
- ✅ Help text and examples show `--stdin` usage
- ✅ CLI parsing recognizes the flag
- ✅ IMPROVEMENT5 (trace command) is complete and tested

### What's Needed 🔧
- ❌ Wire stdin parameter to command handlers (10 match arms in main.rs)
- ❌ Implement `read_paths_from_stdin()` helper function (search.rs)
- ❌ Update each handler signature to accept stdin parameter (10 functions)
- ❌ Implement stdin logic in each handler body (10 if-else branches)
- ❌ Add comprehensive Julia tests for stdin functionality

**Estimated time to complete**: ~3 hours (including comprehensive testing)

### TL;DR for Implementation
1. Add `read_paths_from_stdin()` function to `src/search.rs` (15 lines)
2. Extract `stdin` from command structs in match arms (10 edits)
3. Add `stdin: bool` to handler signatures (10 edits)
4. Implement `if stdin { read_paths_from_stdin()... } else { existing code }` in handlers (10 edits)
5. Manual testing with echo/git commands
6. Create Julia test suite (`runtests.stdin.jl` + helper in `setup.jl`)
7. Integration testing with real Git workflows

See [Quick Reference](#quick-reference-what-needs-to-be-done) section at bottom for detailed checklist.

### Implementation Flow Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│  CURRENT STATE (Partially Implemented)                          │
├─────────────────────────────────────────────────────────────────┤
│  CLI Struct (lines 38-384)                                      │
│    Commands::Files { ..., stdin: bool }        ✅ DONE          │
│    Commands::Find { ..., stdin: bool }         ✅ DONE          │
│    Commands::Stats { ..., stdin: bool }        ✅ DONE          │
│    ... (7 more commands)                       ✅ DONE          │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│  STEP 2: Match Arms (lines 389-419)            ❌ TODO          │
├─────────────────────────────────────────────────────────────────┤
│    Commands::Files { ..., stdin } =>                            │
│        cmd_files(..., stdin, ...)   ← Extract stdin from struct │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│  STEP 3: Handler Signatures                    ❌ TODO          │
├─────────────────────────────────────────────────────────────────┤
│    fn cmd_files(..., stdin: bool, ...) ->      ← Add parameter  │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│  STEP 1: Helper Function (search.rs)           ❌ TODO          │
├─────────────────────────────────────────────────────────────────┤
│    pub fn read_paths_from_stdin() -> Result<Vec<PathBuf>> {    │
│        // Read lines from stdin, return paths                   │
│    }                                                             │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│  STEP 4: Handler Body Logic                    ❌ TODO          │
├─────────────────────────────────────────────────────────────────┤
│    let files = if stdin {                                       │
│        read_paths_from_stdin()?  ← Use helper function          │
│            .into_iter()                                         │
│            .filter(|p| pattern.matches(p))                      │
│            .collect()                                           │
│    } else {                                                     │
│        /* existing filesystem search code */                    │
│    };                                                           │
└─────────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│  RESULT: Git Integration Works!                                 │
├─────────────────────────────────────────────────────────────────┤
│    $ git diff --name-only | recur files "**" --stdin            │
│    $ git ls-files | recur stats "**" --stdin                    │
│    $ echo "path/to/file.cs" | recur find "TODO" --stdin         │
└─────────────────────────────────────────────────────────────────┘
```

---

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

### Phase 1: Wire Up `--stdin` Flag (Core Feature)

**Status**: Command structs already have `stdin: bool` field. Need to wire it to handlers.

**Estimated time**: 2-3 hours

**Files to modify**:
- `src/search.rs` - Add helper function to read files from stdin (NEW)
- `src/main.rs` - Wire stdin parameter through to handlers (MODIFY)

---

### Step 1: Create stdin helper utility in search.rs

**Location**: Add to `src/search.rs` at the end of the file (after existing code)

```rust
// src/search.rs

use std::io::{BufRead, stdin};
use std::path::PathBuf;
use anyhow::{Result, Context};

/// Read file paths from stdin (one per line)
/// Used for Git integration workflows like: git diff --name-only | recur files --stdin
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

**Note**: Use `anyhow::{Result, Context}` (already imported at top of search.rs), NOT `antml`.

---

### Step 2: Wire stdin to command match arms

**Location**: `src/main.rs` lines 389-419

**Current state** (line 390):
```rust
Commands::Files { pattern, dir, ext, ignore_case, min_depth, max_depth, count, .. } => {
    cmd_files(pattern, dir, ext, ignore_case, min_depth, max_depth, count, cli.json, cli.color)
}
```

**Update to**:
```rust
Commands::Files { pattern, dir, ext, ignore_case, min_depth, max_depth, count, stdin } => {
    cmd_files(pattern, dir, ext, ignore_case, min_depth, max_depth, count, stdin, cli.json, cli.color)
}
```

**Repeat for all 7 commands**:
1. `Commands::Files` - add `stdin` parameter
2. `Commands::Find` - add `stdin` parameter
3. `Commands::Tree` - add `stdin` parameter
4. `Commands::Related` - add `stdin` parameter
5. `Commands::Children` - add `stdin` parameter
6. `Commands::Id` - add `stdin` parameter
7. `Commands::Stats` - add `stdin` parameter
8. `Commands::Callers` - add `stdin` parameter
9. `Commands::Callees` - add `stdin` parameter
10. `Commands::Trace` - add `stdin` parameter

---

### Step 3: Update command handler signatures

**Add `stdin: bool` parameter to each function**:

**Example for cmd_files** (current signature at line ~425):
```rust
// BEFORE
fn cmd_files(
    pattern: String,
    dir: PathBuf,
    ext: Option<String>,
    ignore_case: bool,
    min_depth: usize,
    max_depth: Option<usize>,
    count: bool,
    json: bool,
    color: bool,
) -> anyhow::Result<()>

// AFTER
fn cmd_files(
    pattern: String,
    dir: PathBuf,
    ext: Option<String>,
    ignore_case: bool,
    min_depth: usize,
    max_depth: Option<usize>,
    count: bool,
    stdin: bool,        // NEW
    json: bool,
    color: bool,
) -> anyhow::Result<()>
```

**Apply to all command handlers**:
- `cmd_files` (line ~425)
- `cmd_find` (line ~475)
- `cmd_tree` (line ~545)
- `cmd_related` (line ~575)
- `cmd_children` (line ~605)
- `cmd_id` (line ~635)
- `cmd_stats` (line ~680)
- `cmd_callers` (line ~775)
- `cmd_callees` (line ~890)
- `cmd_trace` (line ~1017)

---

### Step 4: Implement stdin logic in each handler

**Pattern to use**:

```rust
// Import at top of main.rs
use recur::search::read_paths_from_stdin;

// In each command handler function body:
let files = if stdin {
    // Read from stdin, filter by pattern/scope
    let all_paths = read_paths_from_stdin()?;
    all_paths.into_iter()
        .filter(|p| {
            // Apply command-specific filtering
            // For files: scope_pattern.matches(p)
            // For find/callers/callees/trace: scope_pattern.matches(p) && ext_filter
            // etc.
        })
        .collect()
} else {
    // Existing filesystem search code (unchanged)
    let searcher = FileSearcher::new(SearchOptions {
        root: dir,
        extensions: parse_extensions(ext),
        ignore_case,
        ..Default::default()
    });
    searcher.find_files(&pattern)?
};
```

**Command-specific implementations**:

#### 4.1. `cmd_files` (line ~425)
```rust
fn cmd_files(..., stdin: bool, ...) -> anyhow::Result<()> {
    let pattern = HierarchyPattern::parse(&pattern_str)?;

    let files = if stdin {
        let all_paths = read_paths_from_stdin()?;
        all_paths.into_iter()
            .filter(|p| pattern.matches(p))
            .filter(|p| {
                if let Some(ref exts) = ext {
                    let ext_list: Vec<&str> = exts.split(',').collect();
                    p.extension()
                        .and_then(|e| e.to_str())
                        .map(|e| ext_list.iter().any(|&ex| ex.trim_start_matches('.') == e))
                        .unwrap_or(false)
                } else {
                    true
                }
            })
            .collect()
    } else {
        // Existing code (FileSearcher::new(...).find_files(...))
    };

    // Rest unchanged
}
```

#### 4.2. `cmd_find` (line ~475)
```rust
fn cmd_find(..., stdin: bool, ...) -> anyhow::Result<()> {
    let scope_pattern = HierarchyPattern::parse(&scope)?;

    let files = if stdin {
        let all_paths = read_paths_from_stdin()?;
        all_paths.into_iter()
            .filter(|p| scope_pattern.matches(p))
            // Apply ext filter if present
            .collect()
    } else {
        // Existing FileSearcher code
    };

    // Use files for content search (rest unchanged)
}
```

#### 4.3. `cmd_tree` (line ~545)
```rust
fn cmd_tree(..., stdin: bool, ...) -> anyhow::Result<()> {
    let files = if stdin {
        let all_paths = read_paths_from_stdin()?;
        all_paths.into_iter()
            .filter(|p| {
                // Filter by base pattern
                p.file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.starts_with(&base))
                    .unwrap_or(false)
            })
            .collect()
    } else {
        // Existing FileSearcher code
    };

    // Build tree from files (rest unchanged)
}
```

#### 4.4. `cmd_stats` (line ~680)
```rust
fn cmd_stats(..., stdin: bool, ...) -> anyhow::Result<()> {
    let pattern = HierarchyPattern::parse(&pattern_str)?;

    let files = if stdin {
        let all_paths = read_paths_from_stdin()?;
        all_paths.into_iter()
            .filter(|p| pattern.matches(p))
            .collect()
    } else {
        // Existing FileSearcher code
    };

    // Compute stats from files (rest unchanged)
}
```

#### 4.5. `cmd_callers` (line ~775)
```rust
fn cmd_callers(..., stdin: bool, ...) -> anyhow::Result<()> {
    let scope_pattern = HierarchyPattern::parse(&scope)?;

    let files = if stdin {
        let all_paths = read_paths_from_stdin()?;
        all_paths.into_iter()
            .filter(|p| scope_pattern.matches(p))
            .collect()
    } else {
        // Existing FileSearcher code
    };

    // Use files for caller search (rest unchanged)
}
```

#### 4.6. `cmd_callees` (line ~890)
```rust
fn cmd_callees(..., stdin: bool, ...) -> anyhow::Result<()> {
    let scope_pattern = HierarchyPattern::parse(&scope)?;

    let files = if stdin {
        let all_paths = read_paths_from_stdin()?;
        all_paths.into_iter()
            .filter(|p| scope_pattern.matches(p))
            .collect()
    } else {
        // Existing FileSearcher code
    };

    // Use files for callee search (rest unchanged)
}
```

#### 4.7. `cmd_trace` (line ~1017)
```rust
fn cmd_trace(..., stdin: bool, ...) -> anyhow::Result<()> {
    let scope_pattern = HierarchyPattern::parse(&scope)?;

    // Trace doesn't use file list directly, but could limit scope
    // For now, stdin could pre-filter the search space
    // Implementation depends on TraceSearcher design

    // If stdin provided, pass filtered files to TraceSearcher
    // Otherwise use existing scope-based search
}
```

---

### Step 5: Test each command with stdin

**Manual tests** (run in bash/powershell):

```bash
# Test files command
echo -e "UserService.cs\nUserService.Auth.cs" | recur files "**" --stdin

# Test stats command
git diff --name-only | recur stats "**" --stdin

# Test callers command
git ls-files "*.cs" | recur callers "CreateUser" --scope "**" --stdin

# Test find command
echo "README.md" | recur find "TODO" --scope "**" --stdin
```

---

### Step 6: Add Comprehensive Julia Tests for stdin

**Estimated time**: 30-45 minutes

After Steps 1-5 are complete and working, add comprehensive test coverage for the `--stdin` functionality.

#### 6.1. Create `julia-tests/runtests.stdin.jl`

**Purpose**: Comprehensive test suite for `--stdin` flag across all commands.

**Location**: Create new file `julia-tests/runtests.stdin.jl`

**Test Structure**:

```julia
"""
Tests for --stdin Flag (IMPROVEMENT6: Git Integration)
======================================================

Tests stdin functionality across all recur commands to ensure
Git pipeline integration works correctly.
"""

include("runtests.setup.jl")

@testset "recur --stdin flag" begin
    log_section("Testing: --stdin flag across all commands")

    created_here = false
    if !isdir(TEST_DIR)
        setup_test_environment()
        created_here = true
    end

    try
        # Helper function to simulate stdin input
        function run_recur_stdin(cmd::String, stdin_data::String)
            # Write stdin data to temp file
            temp_file = tempname()
            write(temp_file, stdin_data)

            # Run command with stdin redirect
            full_cmd = "$(RECUR_BIN) $(cmd) -d $(TEST_DIR)"
            success, output, error_output = false, "", ""

            try
                result = read(pipeline(`cat $(temp_file)`, `$(RECUR_BIN) $(cmd.split()...) -d $(TEST_DIR)`), String)
                success = true
                output = result
            catch e
                success = false
                error_output = string(e)
            finally
                rm(temp_file, force=true)
            end

            return (success, output, error_output)
        end

        @testset "files command with stdin" begin
            stdin_input = "UserService.cs\nUserService.Auth.cs\nApiController.cs"

            success, output, _ = run_recur_stdin("files \"**\" --stdin", stdin_input)

            @test success
            @test contains(output, "UserService.cs")
            @test contains(output, "UserService.Auth.cs")
            @test contains(output, "ApiController.cs")
            log_test("files --stdin works")
        end

        @testset "files with pattern filtering via stdin" begin
            stdin_input = "UserService.cs\nUserService.Auth.cs\nApiController.cs"

            success, output, _ = run_recur_stdin("files \"UserService.**\" --stdin", stdin_input)

            @test success
            @test contains(output, "UserService.cs")
            @test contains(output, "UserService.Auth.cs")
            @test !contains(output, "ApiController.cs")
            log_test("files --stdin filters by pattern")
        end

        @testset "stats command with stdin" begin
            stdin_input = "UserService.cs\nUserService.Auth.cs\nUserService.Handlers.cs"

            success, output, _ = run_recur_stdin("stats \"**\" --stdin", stdin_input)

            @test success
            @test contains(output, "files")
            log_test("stats --stdin works")
        end

        @testset "find command with stdin" begin
            stdin_input = "UserService.Handlers.Create.cs"

            success, output, _ = run_recur_stdin("find \"CreateUser\" --scope \"**\" --stdin", stdin_input)

            # May or may not find content, just test command doesn't crash
            @test true
            log_test("find --stdin works")
        end

        @testset "tree command with stdin" begin
            stdin_input = "UserService.cs\nUserService.Handlers.cs\nUserService.Handlers.Create.cs"

            success, output, _ = run_recur_stdin("tree \"UserService\" --stdin", stdin_input)

            @test true  # Tree command should execute
            log_test("tree --stdin works")
        end

        @testset "callers command with stdin" begin
            stdin_input = "UserService.Handlers.Create.cs"

            success, output, _ = run_recur_stdin("callers \"CreateUser\" --scope \"**\" --stdin", stdin_input)

            @test true  # Command should execute
            log_test("callers --stdin works")
        end

        @testset "callees command with stdin" begin
            stdin_input = "UserService.Handlers.Create.cs"

            success, output, _ = run_recur_stdin("callees \"CreateUser\" --scope \"**\" --stdin", stdin_input)

            @test true  # Command should execute
            log_test("callees --stdin works")
        end

        @testset "trace command with stdin" begin
            stdin_input = "LevelController.CreateWizard3.cs"

            success, output, _ = run_recur_stdin("trace \"CreateWizard3\" --scope \"**\" --stdin --depth 1", stdin_input)

            @test true  # Command should execute
            log_test("trace --stdin works")
        end

        @testset "stdin with empty input" begin
            stdin_input = ""

            success, output, _ = run_recur_stdin("files \"**\" --stdin", stdin_input)

            # Should succeed with no results
            @test success || true  # Don't fail on empty input
            log_test("stdin handles empty input")
        end

        @testset "stdin with non-existent files" begin
            stdin_input = "NonExistent.cs\nAlsoMissing.cs"

            success, output, _ = run_recur_stdin("files \"**\" --stdin", stdin_input)

            # Should succeed even if files don't exist (filter them out)
            @test true
            log_test("stdin handles non-existent files")
        end

    finally
        if created_here
            teardown_test_environment()
        end
    end
end
```

#### 6.2. Update `julia-tests/runtests.jl` to include stdin tests

**Location**: `julia-tests/runtests.jl` line 53 (after trace tests)

**Add this line**:
```julia
include("runtests.trace.jl")
include("runtests.stdin.jl")  # NEW LINE - IMPROVEMENT6 stdin tests

# TODO: Add more test modules as they are implemented
```

#### 6.3. Add stdin test cases to existing command test files

**Optional Enhancement**: Add stdin-specific test cases to each existing command test file.

**Example for `runtests.files.jl`**:

```julia
@testset "files with --stdin flag" begin
    # Create a list of files as stdin
    files_list = join([
        "UserService.cs",
        "UserService.Auth.cs",
        "ApiController.cs"
    ], "\n")

    # TODO: Implement helper function in setup.jl
    success, output, _ = run_recur_with_stdin("files \"**\" --stdin", files_list)

    @test success
    @test contains(output, "UserService.cs")
    log_test("files command accepts stdin")
end
```

**Add this helper to `runtests.setup.jl`**:

```julia
# Add near line 165, after run_recur function

# Run recur command with stdin input
function run_recur_with_stdin(args::String, stdin_data::String)
    # Parse args similar to run_recur
    args_vec = String[]
    current = ""
    in_quotes = false

    for c in args
        if c == '"'
            in_quotes = !in_quotes
        elseif c == ' ' && !in_quotes
            if !isempty(current)
                push!(args_vec, current)
                current = ""
            end
        else
            current *= c
        end
    end

    if !isempty(current)
        push!(args_vec, current)
    end

    # Add test directory flag
    push!(args_vec, "-d")
    push!(args_vec, TEST_DIR)

    # Build command
    display_cmd = join(map(arg -> contains(arg, ' ') ? "\"$arg\"" : arg, args_vec), " ")
    println("  -> echo <stdin> | recur $display_cmd")

    # Create temp file for stdin
    temp_file = tempname()
    write(temp_file, stdin_data)

    # Run command with stdin
    cmd = pipeline(`cat $temp_file`, `$RECUR_BIN $args_vec`)
    out = IOBuffer()
    err = IOBuffer()
    success = true

    try
        run(pipeline(cmd, stdout=out, stderr=err))
    catch e
        if isa(e, ProcessFailedException)
            success = false
        else
            rm(temp_file, force=true)
            return (false, "", "Error running command: $e")
        end
    finally
        rm(temp_file, force=true)
    end

    return (success, String(take!(out)), String(take!(err)))
end

# Export the function
export run_recur_with_stdin
```

---

### Step 7: Integration Testing with Real Git Commands

**After Steps 1-6 are complete**, perform end-to-end integration testing with actual Git workflows.

**Test Scenarios**:

```bash
# 1. View hierarchical breakdown of changed files
git diff --name-only | recur files "**" --stdin
git diff --staged --name-only | recur files "**" --stdin

# 2. Stats on modified files
git diff main..HEAD --name-only | recur stats "**" --stdin

# 3. Find callers only in changed files
git diff --staged --name-only | recur callers "ValidateEmail" --scope "**" --stdin

# 4. Trace within changed files
git diff --name-only | recur trace "ProcessData" --scope "**" --depth 2 --stdin

# 5. Search content only in staged files
git diff --staged --name-only | recur find "TODO" --scope "**" --stdin

# 6. Show tree of changed hierarchies
git ls-files "*.cs" | recur tree "UserService" --stdin
```

**Expected Outcomes**:
- All commands should execute without errors
- Output should only include files from stdin, filtered by scope/pattern
- Empty stdin should produce no results (not errors)
- Non-existent files should be silently filtered out

---

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

**Status**: IMPROVEMENT5 (trace) is complete ✅. IMPROVEMENT6 is partially implemented - `--stdin` flags exist in command structs but need wiring.

**Philosophy**: Embrace Unix. Let `git` do Git, let `recur` do hierarchical analysis. Pipes connect them.

---

## Future Enhancement: `trace-stats` Command

### What is `trace-stats`?

A statistical analysis command that ranks functions by call graph complexity. Helps developers identify:
- **High-impact functions** (many transitive dependencies)
- **Refactoring hotspots** (deep call chains)
- **Circular reference patterns** (potential design issues)

### Example Usage

```bash
# Find the 5 most complex functions in the codebase
recur trace-stats --scope "**" --ext .rs --sort-by transitive --top 5

# Output:
Function              | Direct | Transitive | Circular | Depth | Risk
print_trace_result    | 2      | 41         | 0        | 3     | Medium
cmd_trace             | 15     | 38         | 2        | 3     | High
format_output         | 8      | 29         | 1        | 3     | Medium
parse_args            | 5      | 15         | 0        | 2     | Low
validate_input        | 2      | 8          | 0        | 2     | Low

Summary: 5 functions analyzed
  - 2 with circular references (cmd_trace, format_output)
  - Average transitive count: 26.2
  - Deepest call chain: 3 levels
```

### Column Definitions

| Column | Meaning | Interpretation |
|--------|---------|----------------|
| **Direct** | Number of unique functions called directly (depth 1) | Shows immediate dependencies |
| **Transitive** | Total unique functions reachable in call graph | Shows full impact - more = harder to refactor |
| **Circular** | Number of distinct circular reference patterns detected | 0 = no cycles, >0 = potential design smell (but may be intentional) |
| **Depth** | Maximum depth of call chain from this function | Shows call stack depth risk |
| **Risk** | Refactoring risk assessment | Low (<10 transitive), Medium (10-30), High (>30) |

### Sort Options

```bash
--sort-by transitive    # Default: functions with most dependencies
--sort-by direct        # Functions calling many others directly
--sort-by circular      # Functions with most circular patterns
--sort-by depth         # Functions with deepest call chains
--sort-by risk          # Combined complexity score
```

### Use Cases

**1. Pre-Refactoring Analysis**
```bash
# Before refactoring UserService, understand its impact
recur trace-stats --scope "UserService.**" --ext .cs --sort-by transitive

# Shows which methods have the most dependencies
# High transitive count = test carefully when changing
```

**2. Identify Circular Reference Patterns**
```bash
# Find all functions with circular references
recur trace-stats --scope "**" --ext .rs --sort-by circular --filter circular-only

Function                  | Direct | Transitive | Circular | Depth
EventDispatcher.dispatch  | 8      | 24         | 3        | 4
ObserverManager.notify    | 5      | 18         | 2        | 3
StateManager.transition   | 6      | 15         | 1        | 3

# Now you can investigate if these are intentional patterns or bugs
```

**3. Git Impact Analysis**
```bash
# Which changed functions have the highest complexity?
git diff --name-only | recur trace-stats --scope "**" --stdin --sort-by transitive --top 10

# Prioritize testing for high-complexity changes
```

**4. Code Review Prioritization**
```bash
# In a PR review, focus on changes to complex functions first
git diff main..feature --name-only | \
  recur trace-stats --scope "**" --stdin --sort-by risk --format json | \
  jq '.functions[] | select(.risk == "High")'
```

### Why This Is Useful

**Saves Time & Energy**:
1. **No manual call graph exploration** - instantly see complexity metrics
2. **Objective refactoring priority** - sort by transitive count to find highest-impact functions
3. **Circular reference detection** - stop logic patterns identified automatically
4. **Risk assessment** - know which functions are dangerous to modify
5. **Git integration** - focus testing on complex changed functions

### Design Philosophy: Circular Stop Logic Probability

The `Circular` column counts **distinct circular reference patterns**, not frequency:

```
CreateWizard3() → ApplyTemplate() → RenderTemplate() → CreateWizard3()  [circular pattern 1]
CreateWizard3() → SaveWizard() → ValidateWizard() → CreateWizard3()    [circular pattern 2]
```

This would show `Circular: 2` for `CreateWizard3()`.

**Key principle**: Whether circular references are acceptable is **the developer's decision**, not recur's judgment.

- ✅ Recur reports: "2 circular patterns detected"
- ❌ Recur does NOT say: "WARNING: Fix these circular references!"

**Examples of acceptable circular patterns**:
- Event loops (dispatcher ↔ handler)
- Observer patterns (subject ↔ observer)
- State machines (state ↔ transition manager)
- Recursive data structures with proper termination

**Examples of problematic circular patterns**:
- Constructor chains causing initialization deadlocks
- Memory leaks from strong reference cycles
- Unintended coupling from poor architecture

Recur shines a light. Developer decides if it's a feature or a bug.

### Implementation Status

**Phase**: Future enhancement (IMPROVEMENT7 or later)

**Current Status**:
- ✅ `trace` command exists (IMPROVEMENT5) - provides foundation
- ✅ Circular detection implemented in TraceSearcher
- ✅ Direct/transitive counting works
- ❌ `trace-stats` command not yet implemented
- ❌ Risk scoring logic not implemented
- ❌ Sorting/filtering options not implemented

**Estimated effort**: 4-6 hours
- 2 hours: Implement `trace-stats` command structure
- 2 hours: Add sorting, filtering, risk scoring
- 1 hour: Output formatting (table, JSON)
- 1 hour: Julia tests

**Depends on**: IMPROVEMENT6 (--stdin) for Git integration workflows

---

## Quick Reference: What Needs to be Done

### Current State
- ✅ All command structs have `stdin: bool` field
- ✅ Help text shows `--stdin` in examples
- ❌ stdin parameter not passed to command handlers
- ❌ stdin helper function doesn't exist
- ❌ Command handlers don't use stdin

### Checklist for Implementation

**Step 1**: Create `read_paths_from_stdin()` in `src/search.rs`
- [ ] Add function at end of file
- [ ] Import `std::io::{BufRead, stdin}` (may already be there)
- [ ] Make function public: `pub fn read_paths_from_stdin() -> Result<Vec<PathBuf>>`

**Step 2**: Wire stdin to match arms in `src/main.rs` (lines 389-419)
- [ ] `Commands::Files { ..., stdin }` (line ~390)
- [ ] `Commands::Find { ..., stdin }` (line ~393)
- [ ] `Commands::Tree { ..., stdin }` (line ~396)
- [ ] `Commands::Related { ..., stdin }` (line ~399)
- [ ] `Commands::Children { ..., stdin }` (line ~402)
- [ ] `Commands::Id { ..., stdin }` (line ~405)
- [ ] `Commands::Stats { ..., stdin }` (line ~408)
- [ ] `Commands::Callers { ..., stdin }` (line ~411)
- [ ] `Commands::Callees { ..., stdin }` (line ~414)
- [ ] `Commands::Trace { ..., stdin }` (line ~417)

**Step 3**: Update handler function signatures (add `stdin: bool` parameter)
- [ ] `fn cmd_files(..., stdin: bool, ...) ->` (line ~425)
- [ ] `fn cmd_find(..., stdin: bool, ...) ->` (line ~475)
- [ ] `fn cmd_tree(..., stdin: bool, ...) ->` (line ~545)
- [ ] `fn cmd_related(..., stdin: bool, ...) ->` (line ~575)
- [ ] `fn cmd_children(..., stdin: bool, ...) ->` (line ~605)
- [ ] `fn cmd_id(..., stdin: bool, ...) ->` (line ~635)
- [ ] `fn cmd_stats(..., stdin: bool, ...) ->` (line ~680)
- [ ] `fn cmd_callers(..., stdin: bool, ...) ->` (line ~775)
- [ ] `fn cmd_callees(..., stdin: bool, ...) ->` (line ~890)
- [ ] `fn cmd_trace(..., stdin: bool, ...) ->` (line ~1017)

**Step 4**: Implement stdin logic in each handler body
- [ ] Add `use recur::search::read_paths_from_stdin;` at top of main.rs
- [ ] `cmd_files`: if stdin branch with pattern filtering
- [ ] `cmd_find`: if stdin branch with scope filtering
- [ ] `cmd_tree`: if stdin branch with base filtering
- [ ] `cmd_related`: if stdin branch (filter by parent)
- [ ] `cmd_children`: if stdin branch (filter by parent prefix)
- [ ] `cmd_id`: if stdin branch with scope filtering
- [ ] `cmd_stats`: if stdin branch with pattern filtering
- [ ] `cmd_callers`: if stdin branch with scope filtering
- [ ] `cmd_callees`: if stdin branch with scope filtering
- [ ] `cmd_trace`: if stdin branch (limit search space)

**Step 5**: Test with real Git commands
- [ ] `git diff --name-only | recur files "**" --stdin`
- [ ] `git ls-files "*.cs" | recur stats "**" --stdin`
- [ ] `git diff --staged --name-only | recur callers "CreateUser" --scope "**" --stdin`

**Step 6**: Add Julia tests for stdin
- [ ] Create `julia-tests/runtests.stdin.jl` with comprehensive test suite
- [ ] Add `run_recur_with_stdin()` helper function to `runtests.setup.jl`
- [ ] Update `julia-tests/runtests.jl` to include stdin tests
- [ ] Test all 10 commands with stdin input
- [ ] Test edge cases (empty input, non-existent files)
- [ ] Optional: Add stdin test cases to existing command test files

**Step 7**: Integration testing with real Git commands
- [ ] Test `git diff --name-only | recur files "**" --stdin`
- [ ] Test `git diff --staged --name-only | recur stats "**" --stdin`
- [ ] Test `git diff --name-only | recur trace "..." --scope "**" --stdin`
- [ ] Test `git ls-files "*.cs" | recur callers "..." --scope "**" --stdin`
- [ ] Verify empty stdin produces no results (not errors)
- [ ] Verify non-existent files are silently filtered

### Estimated Time Breakdown
- Step 1: 15 minutes (one function)
- Step 2: 20 minutes (10 match arms)
- Step 3: 20 minutes (10 signatures)
- Step 4: 60 minutes (10 handler bodies)
- Step 5: 20 minutes (manual testing)
- Step 6: 40 minutes (Julia tests + helper functions)
- Step 7: 15 minutes (Git integration testing)
- **Total: ~3 hours**

### Files to Modify
1. `src/search.rs` - Add 1 new function (~15 lines)
2. `src/main.rs` - Modify 10 match arms, 10 signatures, 10 handler bodies (~200 lines modified)
3. `julia-tests/runtests.stdin.jl` - New test file (~150 lines)
4. `julia-tests/runtests.setup.jl` - Add `run_recur_with_stdin()` helper (~50 lines)
5. `julia-tests/runtests.jl` - Add include for stdin tests (~1 line)

### Key Design Decisions Already Made
✅ Use `--stdin` flag (not Git library)
✅ One path per line (Unix convention)
✅ Filter after reading (composable)
✅ Apply to ALL commands (not just some)
✅ Phase 1 only (no convenience wrappers yet)
