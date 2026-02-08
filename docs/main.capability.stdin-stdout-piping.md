# Capability: stdin/stdout Piping

## What stdin/stdout Enables

The `--stdin` flag transforms recur into a **composable Unix tool** that can be chained with other commands.

## Core Capabilities

### 1. Pipe Recur → Recur (Multi-stage Filtering)

**Pattern**: Use one recur command to find files, pipe to another to filter further.

```bash
# Find all docs, then filter to just stdin-related
recur files "**" -d docs/ | recur files "**.stdin.**" --stdin

# Find all command implementations, filter to just those with stdin
recur files "main_command_**" -d src/ --sep _ | recur files "**_stdin**" --stdin --sep _
```

**Why this works**: Each path is on its own line. No quoting needed, even with spaces!

### 2. Pipe Recur → Unix Tools (Count, Filter, Process)

**Pattern**: Use standard Unix tools to process recur output.

```bash
# Count readme files
recur files "**.readme" -d docs/ | wc -l

# Extract file extensions
recur files "**" -d src/ | grep -o '\.[^.]*$' | sort -u

# Find longest filename
recur files "**" -d docs/ | awk '{ print length, $0 }' | sort -n | tail -1
```

**Why this matters**: Full Unix toolchain compatibility. Use `grep`, `sed`, `awk`, `xargs`, etc.

### 3. Pipe Git → Recur (Git Integration!)

**Pattern**: Git outputs file paths, recur filters by hierarchy.

```bash
# View changed files by hierarchy
git diff --name-only | recur files "**" --stdin

# Stats on staged files
git diff --staged --name-only | recur stats "**" --stdin

# Find which UserService files changed
git diff --name-only | recur files "UserService.**" --stdin

# Search for TODOs only in changed files
git diff --name-only | recur find "TODO" --scope "**" --stdin
```

**Why this is powerful**:
- Focus analysis on changed files only
- Understand hierarchical impact of commits
- Pre-commit validation by hierarchy

### 4. Pipe rg/grep → Recur (Fast Search + Hierarchy)

**Pattern**: Use ripgrep for speed, recur for hierarchy filtering.

```bash
# Files containing "async", filtered by hierarchy
rg -l "async" src/ | recur files "main_command_**" --stdin --sep _

# Files with TODO comments in UserService hierarchy
rg -l "TODO" | recur files "UserService.**" --stdin

# Find all Rust files with specific trait, filter by module
rg -l "impl StdinCapable" | recur files "**_stdin**" --stdin --sep _
```

**Why combine them**:
- `rg` is faster for content search
- `recur` better for hierarchical filtering
- Best of both worlds!

### 5. Pipe fd/find → Recur (File Discovery + Hierarchy)

**Pattern**: Use fd for file discovery, recur for hierarchy.

```bash
# Recently modified files, filtered by hierarchy
fd --changed-within 1week | recur files "UserService.**" --stdin

# Large files in specific hierarchy
fd --size +1m | recur files "main.command.**" --stdin
```

## Space Handling: No Quoting Needed!

**The killer feature**: Paths with spaces work seamlessly.

```bash
# This just works - no quoting needed!
recur files "**" -d "path with spaces/" | recur files "**" --stdin
```

**Why**:
- Paths are separated by newlines (one per line)
- No shell word splitting happens
- Spaces in paths are preserved exactly

## Advanced Patterns

### Chain Multiple Filters

```bash
# Three-stage filter
recur files "**" -d src/ --sep _ |     # All src files
  recur files "main_command_**" --stdin --sep _ |  # Just commands
  recur files "**_stdin**" --stdin --sep _         # Just stdin modules
```

### Combine with xargs

```bash
# Run command on each file
recur files "main_command_*_impl" -d src/ --sep _ | xargs wc -l

# Delete matching files (careful!)
recur files "**.tmp" -d /tmp/ | xargs rm
```

### Complex Git Workflows

```bash
# Pre-commit: Check which hierarchies are affected
git diff --staged --name-only | \
  recur files "**" --stdin | \
  cut -d. -f1 | \
  sort -u

# Find callers of functions in changed files
git diff --name-only "*.cs" | \
  recur callers "ValidateEmail" --scope "**" --stdin
```

### JSON Pipelines

```bash
# Get JSON data, process with jq
git diff --name-only | \
  recur stats "**" --stdin --json | \
  jq '.[] | select(.files > 5)'
```

## Avoiding String Quoting

**Before stdin (string hell):**
```bash
# Paths with spaces need escaping
recur files "some pattern" -d "path with spaces"  # Must quote

# Passing paths to other commands
for file in $(recur files "**"); do  # Breaks on spaces!
  recur find "TODO" --scope "$file"  # Need quotes everywhere
done
```

**After stdin (clean pipes):**
```bash
# No quoting needed - paths flow through pipes
git ls-files | recur files "**" --stdin  # Just works!

# Spaces handled automatically
recur files "**" -d "path with spaces/" | \
  recur files "pattern" --stdin  # No quotes needed
```

## When to Use stdin vs Direct Invocation

**Use stdin when:**
- ✅ Filtering output from another tool (git, rg, fd)
- ✅ Chaining recur commands together
- ✅ Working with dynamic file lists
- ✅ Handling paths with spaces
- ✅ Integrating into complex pipelines

**Use direct invocation when:**
- ✅ Simple one-off query
- ✅ Exploring codebase interactively
- ✅ Known file patterns

## Performance Notes

**stdin is FAST** because:
- No filesystem scanning needed
- Direct path processing
- Filtered input from upstream commands

**Example speedup:**
```bash
# Slow: Scan entire codebase for pattern
recur find "TODO" --scope "**"

# Fast: Only search changed files
git diff --name-only | recur find "TODO" --scope "**" --stdin
```

## Philosophy

**stdin/stdout enables the Unix philosophy:**
1. Do one thing well (recur does hierarchical analysis)
2. Compose with other tools (git, rg, fd, grep, awk)
3. Text streams as universal interface (paths on stdout)
4. Pipelines over monoliths (chain commands)

**Result**: Flexible, composable, powerful workflows without building everything into recur!
