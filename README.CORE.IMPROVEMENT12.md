# Core Improvement 12: recur-git Hierarchy-Aware Git Commands

## Current Status

**🔴 NOT STARTED** - Future enhancement for recur-git binary.

### Dependencies
- ✅ **recur-git binary** - **COMPLETE** - Provides foundation and checkpoint command
- ✅ **IMPROVEMENT6** (stdin support) - **COMPLETE** - Enables piping Git output

### What This Adds

Three new Git-aware commands for `recur-git` that combine Git operations with recur's hierarchical understanding:

1. **`recur-git diff`** - Show hierarchical diff of changed files
2. **`recur-git blame`** - Hierarchical blame view
3. **`recur-git log`** - Hierarchical commit history

**Key Value Proposition**: Navigate Git history through the lens of your codebase's hierarchical structure, making it easier to understand changes in context.

---

## Philosophy: Git + Hierarchy = Better Context

**Core Principle**: Keep hierarchy semantics in recur library, compose with Git in recur-git.

**Why separate from core recur?**
- Recur stays pure (no Git dependencies)
- recur-git handles Git integration
- Clean separation of concerns
- Each tool does one thing well

**Why add to recur-git?**
- Git already provides excellent tools (`git diff`, `git blame`, `git log`)
- But they show flat file lists
- recur-git adds hierarchical grouping and visualization
- Better context for understanding changes

---

## Command 1: `recur-git diff`

### Overview

Show Git diff results grouped and visualized by hierarchical structure.

**Traditional `git diff`:**
```
UserService.cs
UserService.Handlers.cs
UserService.Handlers.Create.cs
UserService.Handlers.Update.cs
UserService.Models.cs
ApiController.cs
```

**With `recur-git diff`:**
```
UserService (5 files changed)
├── UserService.cs
├── Handlers (3 files)
│   ├── Handlers.cs
│   ├── Create.cs
│   └── Update.cs
└── Models.cs

ApiController (1 file changed)
└── ApiController.cs
```

### Use Cases

#### 1. PR Review: Understand Change Scope

**Scenario**: Reviewing a PR that claims "just a small UserService change"

```bash
# Show hierarchical view of changes
recur-git diff main..feature-branch

# Output shows true scope:
UserService (12 files changed)
├── UserService.cs
├── Handlers (8 files)
│   ├── Create.cs
│   ├── Update.cs
│   ├── Delete.cs
│   └── Validation (5 files)
└── Models (3 files)

# "Small change" actually touched 12 files across 3 subsystems!
```

#### 2. Focus on Specific Hierarchy

**Scenario**: Only care about changes to UserService subsystem

```bash
# Show only UserService changes
recur-git diff main..feature --scope "UserService.**"

# Equivalent to filtering manually:
git diff main..feature --name-only | recur files "UserService.**" --stdin
```

#### 3. Compare Branches with Hierarchy Stats

**Scenario**: Which subsystems differ between branches?

```bash
# Show stats grouped by hierarchy
recur-git diff main..feature --stats

# Output:
Hierarchy              | Files Changed | Lines Added | Lines Deleted
UserService.**         | 12            | 234         | 89
ApiController.**       | 3             | 45          | 12
Tests.UserService.**   | 8             | 156         | 23
```

### Command-Line Interface

```bash
recur-git diff [<commit>] [<commit>] [OPTIONS]

# Compare commits/branches
recur-git diff main..feature
recur-git diff HEAD~5..HEAD
recur-git diff abc123..def456

# Scope to hierarchy
recur-git diff main..feature --scope "UserService.**"

# Output modes
recur-git diff main..feature --tree          # Tree visualization (default)
recur-git diff main..feature --stats         # Stats table
recur-git diff main..feature --files-only    # Just file list (grouped)
recur-git diff main..feature --json          # JSON output

# Git diff options (pass-through)
recur-git diff main..feature --name-only     # File names only
recur-git diff main..feature --name-status   # With status (M/A/D)

# Hierarchical filtering
recur-git diff main..feature --ext .cs       # Only .cs files
recur-git diff main..feature --min-depth 2   # Exclude top-level files
```

### Implementation Approach

**File**: `src/recur_git_main.rs`

```rust
Commands::Diff {
    commits,
    scope,
    tree,
    stats,
    files_only,
    json,
    ext,
    min_depth,
} => {
    // 1. Run git diff --name-status <commits>
    let git_output = run_git_diff(&commits)?;

    // 2. Parse into (status, path) pairs
    let changes = parse_git_diff_output(&git_output)?;

    // 3. Apply hierarchical filtering (scope, ext, depth)
    let filtered = filter_by_hierarchy(&changes, &scope, &ext, min_depth)?;

    // 4. Build hierarchical tree structure
    let tree = build_hierarchy_tree(&filtered)?;

    // 5. Output in requested format
    if stats {
        print_hierarchy_stats(&tree)?;
    } else if tree {
        print_hierarchy_tree(&tree)?;
    } else if files_only {
        print_grouped_files(&tree)?;
    } else if json {
        print_json(&tree)?;
    }
}
```

### Example Outputs

#### Tree Mode (Default)

```bash
recur-git diff main..feature

UserService (12 files changed)
├── M UserService.cs
├── Handlers (8 files)
│   ├── M Create.cs
│   ├── M Update.cs
│   ├── A Delete.cs
│   └── Validation (5 files)
│       ├── M EmailValidator.cs
│       ├── M PasswordValidator.cs
│       └── A PhoneValidator.cs
└── Models (3 files)
    ├── M UserModel.cs
    └── A UserPreferences.cs

Legend: M=Modified, A=Added, D=Deleted
```

#### Stats Mode

```bash
recur-git diff main..feature --stats

Hierarchy              | Files | Modified | Added | Deleted
UserService            | 12    | 9        | 3     | 0
UserService.Handlers   | 8     | 6        | 2     | 0
UserService.Models     | 3     | 2        | 1     | 0
ApiController          | 3     | 3        | 0     | 0
Tests                  | 8     | 5        | 3     | 0
```

---

## Command 2: `recur-git blame`

### Overview

Show blame information grouped by hierarchical structure, making it easier to understand who owns which subsystems.

**Traditional `git blame`:**
- Shows line-by-line authorship for a single file
- No context about related files
- No subsystem ownership view

**With `recur-git blame`:**
- Shows authorship across hierarchies
- Aggregates ownership by subsystem
- Identifies "owners" of hierarchical components

### Use Cases

#### 1. Find Subsystem Owners

**Scenario**: Who should review changes to UserService?

```bash
# Show ownership of UserService hierarchy
recur-git blame --scope "UserService.**"

# Output:
UserService (12 files)
  Primary authors:
    alice@example.com: 65% (487 lines)
    bob@example.com: 25% (187 lines)
    carol@example.com: 10% (75 lines)

  By component:
    UserService.Handlers: alice@example.com (78%)
    UserService.Models: bob@example.com (62%)
    UserService.Validation: carol@example.com (85%)
```

**Decision**: Alice should be the primary reviewer (owns Handlers subsystem).

#### 2. Identify Stale Code

**Scenario**: Find code that hasn't been touched in years

```bash
# Show last-modified dates by hierarchy
recur-git blame --scope "**" --oldest

# Output:
Hierarchy                   | Last Modified | Author
LegacyService.**            | 2019-03-15    | old-dev@example.com
DeprecatedController.**     | 2020-06-22    | former-dev@example.com
UserService.**              | 2026-01-30    | current-dev@example.com
```

**Decision**: LegacyService is a candidate for refactoring or removal.

#### 3. Track Code Ownership Over Time

**Scenario**: Did ownership of this subsystem change recently?

```bash
# Show authorship distribution for commits in last 6 months
recur-git blame --scope "PaymentService.**" --since "6 months ago"

# Output:
PaymentService (15 files)
  Authors (last 6 months):
    new-dev@example.com: 45% (recent changes)
    old-dev@example.com: 55% (legacy code)

  Recent activity:
    Last 30 days: 23 commits by new-dev@example.com
    Last 90 days: 8 commits by old-dev@example.com
```

**Insight**: Ownership is transitioning from old-dev to new-dev.

### Command-Line Interface

```bash
recur-git blame [OPTIONS]

# Scope to hierarchy
recur-git blame --scope "UserService.**"
recur-git blame --scope "**.Handlers.**"

# Time filtering
recur-git blame --scope "**" --since "6 months ago"
recur-git blame --scope "**" --until "2023-01-01"

# Output modes
recur-git blame --scope "**" --summary          # Aggregate by hierarchy (default)
recur-git blame --scope "**" --by-author        # Group by author
recur-git blame --scope "**" --oldest           # Show oldest code first
recur-git blame --scope "**" --json             # JSON output

# Filtering
recur-git blame --scope "**" --ext .cs          # Only .cs files
recur-git blame --scope "**" --exclude "**.Tests.**"  # Exclude tests
```

### Implementation Approach

```rust
Commands::Blame {
    scope,
    since,
    until,
    summary,
    by_author,
    oldest,
    json,
    ext,
} => {
    // 1. Find all files in scope
    let files = find_files_in_scope(&scope, &ext)?;

    // 2. Run git blame on each file
    let blame_data = run_git_blame_batch(&files, since, until)?;

    // 3. Aggregate by hierarchy
    let hierarchy_stats = aggregate_by_hierarchy(&blame_data)?;

    // 4. Output in requested format
    if by_author {
        print_by_author(&hierarchy_stats)?;
    } else if oldest {
        print_oldest_first(&hierarchy_stats)?;
    } else if summary {
        print_summary(&hierarchy_stats)?;
    } else if json {
        print_json(&hierarchy_stats)?;
    }
}
```

### Example Output

```bash
recur-git blame --scope "UserService.**" --summary

UserService (12 files, 1,247 lines)
├── Primary authors:
│   ├── alice@example.com: 65% (811 lines)
│   ├── bob@example.com: 25% (312 lines)
│   └── carol@example.com: 10% (124 lines)
│
├── Handlers (8 files, 623 lines)
│   ├── alice@example.com: 78% (486 lines)
│   └── bob@example.com: 22% (137 lines)
│
├── Models (3 files, 312 lines)
│   ├── bob@example.com: 62% (193 lines)
│   └── alice@example.com: 38% (119 lines)
│
└── Validation (2 files, 156 lines)
    └── carol@example.com: 85% (133 lines)

Last modified: 2026-01-30 14:23:45
Most active contributor (last 30 days): alice@example.com (15 commits)
```

---

## Command 3: `recur-git log`

### Overview

Show Git commit history grouped by affected hierarchies, making it easier to track subsystem evolution.

**Traditional `git log`:**
- Linear list of commits
- No grouping by affected subsystems
- Hard to see "UserService history" vs "ApiController history"

**With `recur-git log`:**
- Group commits by affected hierarchies
- Track subsystem evolution
- Identify cross-cutting changes

### Use Cases

#### 1. Subsystem History

**Scenario**: What's the recent history of UserService changes?

```bash
# Show commits affecting UserService
recur-git log --scope "UserService.**" --since "3 months ago"

# Output:
UserService (23 commits in last 3 months)

2026-02-07 alice   feat: add email validation
  Files: UserService.Validation.EmailValidator.cs

2026-02-05 bob     fix: handle null users in handlers
  Files: UserService.Handlers.Create.cs
         UserService.Handlers.Update.cs

2026-02-01 alice   refactor: extract validation logic
  Files: UserService.Validation.* (5 files)
         UserService.Handlers.Create.cs
         UserService.Handlers.Update.cs
```

#### 2. Cross-Hierarchy Changes

**Scenario**: Which commits touched both UserService AND ApiController?

```bash
# Find commits affecting multiple hierarchies
recur-git log --affects "UserService.**,ApiController.**" --cross-hierarchy

# Output:
Cross-hierarchy commits (affecting 2+ subsystems):

2026-02-03 carol   refactor: unify authentication flow
  Affected:
    - UserService.Handlers.** (3 files)
    - ApiController.Auth.** (2 files)

2026-01-28 alice   feat: add user roles
  Affected:
    - UserService.Models.** (2 files)
    - ApiController.Middleware.** (1 file)
```

**Insight**: These commits touched multiple subsystems - potential architectural changes.

#### 3. Release Notes by Hierarchy

**Scenario**: Generate release notes grouped by subsystem

```bash
# Show commits between releases, grouped by hierarchy
recur-git log v1.0.0..v2.0.0 --group-by-hierarchy --format release-notes

# Output:
Release Notes: v1.0.0 → v2.0.0

## UserService
- feat: add email validation (alice, 2026-02-07)
- fix: handle null users (bob, 2026-02-05)
- refactor: extract validation logic (alice, 2026-02-01)

## ApiController
- feat: add rate limiting (carol, 2026-02-04)
- fix: auth token expiration (bob, 2026-01-29)

## Tests
- test: add validation tests (alice, 2026-02-06)
- test: improve coverage (carol, 2026-02-02)
```

#### 4. Find Last Change to Subsystem

**Scenario**: When was the last time LegacyService was touched?

```bash
# Show most recent commit for each hierarchy
recur-git log --scope "**" --last-change

# Output:
Hierarchy                   | Last Commit | Date       | Author
UserService.**              | abc123f     | 2026-02-07 | alice
ApiController.**            | def456a     | 2026-02-04 | carol
LegacyService.**            | 789bcd2     | 2019-03-15 | old-dev
Tests.**                    | 456efg8     | 2026-02-06 | alice
```

**Decision**: LegacyService hasn't been touched in 7 years!

### Command-Line Interface

```bash
recur-git log [<revision-range>] [OPTIONS]

# Scope to hierarchy
recur-git log --scope "UserService.**"
recur-git log --scope "**.Handlers.**"

# Time filtering
recur-git log --since "3 months ago"
recur-git log --until "2025-01-01"
recur-git log v1.0.0..v2.0.0

# Grouping options
recur-git log --group-by-hierarchy           # Group by affected hierarchy (default)
recur-git log --cross-hierarchy              # Only show commits affecting 2+ hierarchies
recur-git log --last-change                  # Show last commit for each hierarchy

# Output formats
recur-git log --format oneline               # One line per commit
recur-git log --format detailed              # Full commit messages
recur-git log --format release-notes         # Release notes format
recur-git log --json                         # JSON output

# Filtering
recur-git log --ext .cs                      # Only commits touching .cs files
recur-git log --author alice                 # Only alice's commits
recur-git log --affects "User**,Api**" --cross-hierarchy  # Cross-hierarchy
```

### Implementation Approach

```rust
Commands::Log {
    revision_range,
    scope,
    since,
    until,
    group_by_hierarchy,
    cross_hierarchy,
    last_change,
    format,
    json,
    ext,
    author,
} => {
    // 1. Run git log with filters
    let commits = run_git_log(&revision_range, since, until, author)?;

    // 2. For each commit, get affected files
    let commits_with_files = enrich_with_file_lists(&commits)?;

    // 3. Apply hierarchical filtering (scope, ext)
    let filtered = filter_by_hierarchy(&commits_with_files, &scope, &ext)?;

    // 4. Group by hierarchy
    if group_by_hierarchy {
        let grouped = group_commits_by_hierarchy(&filtered)?;
        print_grouped_log(&grouped, &format)?;
    } else if cross_hierarchy {
        let cross = find_cross_hierarchy_commits(&filtered)?;
        print_cross_hierarchy_log(&cross, &format)?;
    } else if last_change {
        let last = find_last_change_per_hierarchy(&filtered)?;
        print_last_change_table(&last)?;
    }
}
```

### Example Outputs

#### Default: Grouped by Hierarchy

```bash
recur-git log --scope "UserService.**" --since "1 month ago"

UserService (15 commits in last month)

Handlers (8 commits)
├── 2026-02-07 abc123f alice   feat: add email validation
├── 2026-02-05 def456a bob     fix: handle null users
└── 2026-02-01 789bcd2 alice   refactor: extract validation

Models (4 commits)
├── 2026-02-03 456efg8 bob     feat: add user preferences model
└── 2026-01-28 123abc4 alice   refactor: rename fields

Validation (3 commits)
└── 2026-02-06 789def0 carol   test: add validation tests
```

#### Cross-Hierarchy Mode

```bash
recur-git log --cross-hierarchy

Cross-hierarchy commits (affecting 2+ subsystems):

2026-02-03 789abc1 carol   refactor: unify authentication flow
  Affected hierarchies:
    - UserService.Handlers (3 files)
    - ApiController.Auth (2 files)
    - Tests.Integration (4 files)

2026-01-28 456def2 alice   feat: add user roles
  Affected hierarchies:
    - UserService.Models (2 files)
    - ApiController.Middleware (1 file)
```

#### Release Notes Format

```bash
recur-git log v1.0.0..v2.0.0 --format release-notes

Release Notes: v1.0.0 → v2.0.0 (45 commits)

## Features

### UserService
- Add email validation (alice, 2026-02-07)
- Add user preferences (bob, 2026-02-03)
- Add role management (alice, 2026-01-28)

### ApiController
- Add rate limiting (carol, 2026-02-04)
- Add JWT authentication (bob, 2026-01-30)

## Bug Fixes

### UserService
- Handle null users in handlers (bob, 2026-02-05)
- Fix validation edge cases (carol, 2026-02-02)

### ApiController
- Fix auth token expiration (bob, 2026-01-29)

## Refactoring

### UserService
- Extract validation logic (alice, 2026-02-01)
- Unify authentication flow (carol, 2026-02-03)
```

---

## Implementation Plan

### Phase 1: `recur-git diff` (2-3 hours)

1. Add `Diff` command to `Commands` enum
2. Implement `run_git_diff()` to call `git diff --name-status`
3. Parse Git output into (status, path) tuples
4. Build hierarchical tree from paths
5. Implement tree visualization output
6. Add stats mode
7. Add JSON output

### Phase 2: `recur-git blame` (3-4 hours)

1. Add `Blame` command to `Commands` enum
2. Implement `run_git_blame_batch()` to call `git blame` on files
3. Parse blame output (author, date, line count)
4. Aggregate by hierarchy
5. Implement summary output
6. Add by-author grouping
7. Add JSON output

### Phase 3: `recur-git log` (3-4 hours)

1. Add `Log` command to `Commands` enum
2. Implement `run_git_log()` to call `git log --name-status`
3. Parse commit metadata + affected files
4. Group commits by hierarchy
5. Implement grouped log output
6. Add cross-hierarchy detection
7. Add release notes format
8. Add JSON output

---

## Testing Strategy

### Unit Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_git_diff_output() {
        let output = "M\tUserService.cs\nA\tUserService.Handlers.cs";
        let changes = parse_git_diff_output(output).unwrap();
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0].status, "M");
    }

    #[test]
    fn test_build_hierarchy_tree() {
        let paths = vec!["UserService.cs", "UserService.Handlers.cs"];
        let tree = build_hierarchy_tree(&paths).unwrap();
        assert_eq!(tree.children.len(), 1); // UserService
    }
}
```

### Integration Tests (Julia)

```julia
# julia-tests/runtests.recur-git.jl

@testset "recur-git diff" begin
    # Setup: Create test repo with commits
    run(`git init test-repo`)

    # Test: Basic diff
    output = read(`recur-git diff HEAD~1..HEAD`, String)
    @test contains(output, "UserService")

    # Test: Scoped diff
    output = read(`recur-git diff HEAD~1..HEAD --scope "User**"`, String)
    @test contains(output, "UserService")
    @test !contains(output, "ApiController")
end

@testset "recur-git blame" begin
    # Test: Ownership summary
    output = read(`recur-git blame --scope "UserService.**"`, String)
    @test contains(output, "Primary authors")
end

@testset "recur-git log" begin
    # Test: Grouped log
    output = read(`recur-git log --scope "UserService.**"`, String)
    @test contains(output, "commits")
end
```

---

## Success Criteria

### recur-git diff
- ✅ Groups changed files by hierarchy
- ✅ Shows tree visualization
- ✅ Provides stats by hierarchy
- ✅ Supports scoping to hierarchy patterns
- ✅ JSON output works
- ✅ Pass-through Git diff options work

### recur-git blame
- ✅ Aggregates authorship by hierarchy
- ✅ Shows primary authors for subsystems
- ✅ Supports time filtering (--since, --until)
- ✅ Identifies oldest code
- ✅ JSON output works

### recur-git log
- ✅ Groups commits by affected hierarchy
- ✅ Identifies cross-hierarchy commits
- ✅ Shows last change per hierarchy
- ✅ Generates release notes format
- ✅ JSON output works
- ✅ Supports time and author filtering

---

## Estimated Effort

**Total**: 8-11 hours

| Command | Implementation | Testing | Total |
|---------|---------------|---------|-------|
| recur-git diff | 2-3 hours | 1 hour | 3-4 hours |
| recur-git blame | 3-4 hours | 1 hour | 4-5 hours |
| recur-git log | 3-4 hours | 1 hour | 4-5 hours |

---

## Why This Matters

### Developer Pain Points

**Before recur-git:**
- "Which subsystem did this PR really touch?" → Manual file review
- "Who owns UserService?" → Check git blame on each file individually
- "What's the history of this subsystem?" → Filter git log manually

**With recur-git:**
- `recur-git diff` → Instant hierarchical view of changes
- `recur-git blame` → Aggregated ownership by subsystem
- `recur-git log` → Subsystem-focused commit history

**Time saved**: 5-10 minutes per code review, 15-20 minutes per subsystem investigation

---

## Integration with Existing Tools

### Compose with recur

```bash
# Find complex functions in changed files
recur-git diff main..feature --files-only | \
  recur trace-stats --scope "**" --stdin --sort-by risk

# Search changed files for TODOs
recur-git diff main..feature --files-only | \
  recur find "TODO" --scope "**" --stdin
```

### Compose with standard Git

```bash
# recur-git adds hierarchy, but you can still use git for details
recur-git diff main..feature --scope "UserService.**"  # Overview
git diff main..feature UserService.cs                  # Detailed diff
```

---

## Future Enhancements

### Phase 2 (Optional):

1. **Interactive mode**: Navigate hierarchy interactively
   ```bash
   recur-git diff main..feature --interactive
   # Arrow keys to navigate tree, Enter to show detailed diff
   ```

2. **Visualization**: Generate graphs
   ```bash
   recur-git log --scope "**" --visualize activity.svg
   # Heatmap of subsystem activity over time
   ```

3. **AI summaries**: Generate change summaries
   ```bash
   recur-git diff main..feature --summarize
   # "This PR primarily touches UserService authentication logic..."
   ```

---

## Conclusion

**recur-git** becomes a complete Git workflow companion:
- `checkpoint` - Track dogfooding state
- `diff` - Understand change scope hierarchically
- `blame` - Identify subsystem ownership
- `log` - Track subsystem evolution

**Philosophy**: Git provides the data, recur-git adds hierarchical context.

---

**Status**: Ready for implementation. Incremental approach recommended (implement `diff` first, then `blame`, then `log`).

**Dependencies**: All dependencies complete (recur-git binary exists, stdin support works).

**Next Steps**: Implement `recur-git diff` as proof of concept, validate approach, then add `blame` and `log`.
