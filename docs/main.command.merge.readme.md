# Command: merge

## Overview
The `recur merge` command provides Unix-style composability for combining hierarchical search results from different naming conventions. Instead of automatic pattern conversion, users explicitly control what gets merged through pipes or multiple pattern specifications.

## Motivation

**Problem:** Different parts of a codebase use different naming conventions:
- Documentation: `main.command.tree.readme.md` (dots)
- Source code: `main_command_tree_impl.rs` (underscores)
- Tests: `main.command.tree.test.jl` (dots)

**Previous approach:** Required automatic pattern normalization (surprising behavior)

**New approach:** Explicit merging via composition (Unix philosophy)

## Design Philosophy

1. **Explicit over implicit** - User controls what gets merged
2. **Composable** - Works with pipes, follows Unix philosophy
3. **Focused** - Does one thing well (merging hierarchies)
4. **Flexible** - Can merge ANY searches, not just separator variants

## Usage

### Basic Usage

```bash
# Merge two tree views
recur merge \
  --pattern "main.command.tree" --sep "." \
  --pattern "main_command_tree" --sep "_"
```

**Output:**
```
main.command.tree
├── readme.md [.]
├── test.jl [.]
└── impl.rs [_]
```

### Pipe Mode (Ultimate Unix Flexibility)

```bash
# Pipe multiple searches into merge
recur files "main.command.tree.**" --sep "." --json | \
recur files "main_command_tree_**" --sep "_" --json | \
recur merge --stdin --show-sep
```

### File Mode (Phase 4)

```bash
# Merge cached JSON outputs from disk
recur merge \
  main.command.tree.json --sep "." \
  main_command_tree.json --sep "_" \
  --base "main.command.tree" \
  --show-sep
```

### Multi-Language Merge

```bash
# Merge TypeScript + Python + Go
recur merge \
  --pattern "service.user" --sep "." \
  --pattern "service_user" --sep "_" \
  --pattern "service/user" --sep "/" \
  --show-sep

# Output shows:
service.user
├── index.ts [.]        # TypeScript
├── handler.py [_]      # Python
└── service.go [/]      # Go
```

### Gap Analysis

```bash
# Find what's documented vs what's implemented
recur merge \
  --pattern "api.user" --sep "." \
  --pattern "api_user" --sep "_" \
  --show-sep | grep -E "\[.\].*readme"

# Shows which implementations lack documentation
```

## Command-Line Interface

```
recur merge [OPTIONS]

OPTIONS:
    --pattern <PATTERN>        Pattern to search (can be repeated)
    --sep <CHAR>               Separator for corresponding pattern (can be repeated)
  --base <BASE>               Base name for tree output (file mode)
  <FILE>...                   JSON input files (file mode)
    --show-sep                 Show separator markers [.] [_] etc.
    --sep-replace-default <CHAR>  Normalize output to specific separator
  --stdin                    Read JSON input from stdin (pipe mode, Phase 5)
    --json                     Output as JSON
    --format <tree|files>      Output format (default: tree)
    --ascii                    Use ASCII characters instead of Unicode
    --count                    Show file counts at each level
```

## Pattern Ordering

Patterns and separators are paired in order:
```bash
recur merge \
  --pattern "api.user" --sep "." \      # Pair 1
  --pattern "api_user" --sep "_" \      # Pair 2
  --pattern "api-user" --sep "-"        # Pair 3
```

## Pipe Mode Details

### Input Format

Pipe mode accepts JSON from other recur commands:
```bash
# Each command outputs JSON
recur files "api.user.**" --sep "." --json
recur files "api_user_**" --sep "_" --json
```

### File Mode Input Format

File inputs accept any of the following JSON formats:

```json
["path/one.ext", "path/two.ext"]
```

```json
{ "files": ["path/one.ext", "path/two.ext"] }
```

```json
{ "name": "root", "children": [{ "path": "path/one.ext" }] }
```

### Merging Strategy

1. Parse all JSON inputs
2. Extract hierarchical paths
3. Merge into unified tree structure
4. Track which separator found each file
5. Display with markers if `--show-sep` specified

## Use Cases

### 1. Documentation Completeness Check

```bash
# Merge docs and source
recur merge \
  --pattern "main.command" --sep "." \
  --pattern "main_command" --sep "_" \
  --show-sep > completeness.txt

# Analyze gaps
grep "\[_\]" completeness.txt | while read line; do
  impl=$(echo "$line" | awk '{print $1}')
  doc="${impl//_/.}.readme.md"
  [ ! -f "docs/$doc" ] && echo "Missing docs: $impl"
done
```

### 2. Polyglot Project Navigation

```bash
# One view of multi-language service
recur merge \
  --pattern "user.service" --sep "." \    # TypeScript
  --pattern "user_service" --sep "_" \    # Python
  --pattern "user/service" --sep "/"      # Go
```

### 3. Configuration Environment Parity

```bash
# Verify prod has all configs that dev has
recur merge \
  --pattern "prod.database" --sep "." \
  --pattern "prod_redis" --sep "_" \
  --show-sep > prod.txt

recur merge \
  --pattern "dev.database" --sep "." \
  --pattern "dev_redis" --sep "_" \
  --show-sep > dev.txt

diff prod.txt dev.txt
```

### 4. Build Pipeline Verification

```bash
# Check all artifacts exist
recur merge \
  --pattern "component.widget" --sep "." \     # Source
  --pattern "component-widget" --sep "-" \     # Library
  --pattern "component/widget" --sep "/" \     # Docs
  --show-sep
```

## Implementation Notes

### Hierarchical Merging Algorithm

1. **Collect all paths** from each pattern/separator pair
2. **Build unified tree** using first separator as canonical form
3. **Track provenance** - which separator found each file
4. **Display with markers** if requested

### Normalization

When `--sep-replace-default` is specified:
- All paths normalized to target separator
- Markers still show original separator
- Example: `api_user.py [_]` → displayed as `api.user.py [_]`

### Deduplication

If same file found with multiple separators:
- Keep first occurrence
- Mark with first separator that found it
- Avoid duplicate entries

## Examples

### Example 1: Simple Merge

**Input:**
```bash
recur merge \
  --pattern "main.tree" --sep "." \
  --pattern "main_tree" --sep "_"
```

**Output:**
```
main.tree
├── readme.md
├── test.jl
└── impl.rs
```

### Example 2: With Markers

**Input:**
```bash
recur merge \
  --pattern "main.tree" --sep "." \
  --pattern "main_tree" --sep "_" \
  --show-sep
```

**Output:**
```
main.tree
├── readme.md [.]
├── test.jl [.]
└── impl.rs [_]
```

### Example 3: Three-Way Merge

**Input:**
```bash
recur merge \
  --pattern "api.user" --sep "." \
  --pattern "api_user" --sep "_" \
  --pattern "api-user" --sep "-" \
  --show-sep
```

**Output:**
```
api.user
├── service.ts [.]
├── handler.py [_]
└── config.yaml [-]
```

### Example 4: Pipe Mode

**Input:**
```bash
recur files "api.user.**" --sep "." --json | \
recur files "api_user_**" --sep "_" --json | \
recur merge --stdin --format tree --show-sep
```

**Output:**
```
api.user
├── service.ts [.]
├── handler.py [_]
└── test.js [.]
```

## Comparison with Multi-Separator Flags

### Old Approach (Limited)
```bash
# Multiple --sep flags (same pattern, different separators)
recur tree "main" --sep "." --sep "_"
# Only works if files share exact same base name
```

### New Approach (Flexible)
```bash
# Explicit merge (different patterns, different separators)
recur merge \
  --pattern "main.command" --sep "." \
  --pattern "main_command" --sep "_"
# Works with different naming schemes
```

## Future Enhancements

- [ ] Support for regex patterns in merge
- [ ] Diff mode: show only differences between domains
- [ ] Interactive mode: choose which domain to prioritize
- [ ] Smart pattern conversion: `main.x` → `main_x` automatically
- [ ] Merge from file inputs (not just patterns)
- [ ] Visual diff highlighting

## See Also

- `recur tree` - Tree visualization
- `recur files` - File listing
- Multi-separator merge: `docs/main.trait.separator-merge.readme.md`
