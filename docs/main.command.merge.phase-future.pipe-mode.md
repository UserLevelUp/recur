# Future Phase: Pure Unix Pipe-Based Merge

## Vision

Instead of merge doing the searching, make it a pure pipe-based compositor that merges pre-computed results from other commands.

## Current Approach (Pattern-based)

```bash
# Merge does everything: search + merge
recur merge --pattern "main.command" --sep "." --pattern "main_command" --sep "_"
```

**Pros:** Convenient, all-in-one
**Cons:** Less composable, merge has to know how to search

## Future Approach (Pipe-based)

```bash
# Each command does one thing, compose with pipes
recur merge <(recur tree main --sep ".") <(recur tree main --sep "_")
```

**Pros:** Pure Unix, composable, flexible
**Cons:** More verbose, requires process substitution

## Design: Support BOTH

Make merge polymorphic - detect input mode:

```rust
recur merge [OPTIONS] [FILES...]

Modes:
  1. Pattern mode (convenience):
     recur merge --pattern X --sep Y --pattern Z --sep W

  2. File mode (pipes):
     recur merge file1.json file2.json

  3. Stdin mode (pure pipes):
     recur tree X | recur tree Y | recur merge --stdin

  4. Process substitution (Unix power):
     recur merge <(recur tree X) <(recur tree Y)
```

## CLI Design

```rust
#[derive(Parser)]
pub struct MergeCommand {
    /// Input files to merge (JSON format from tree/files commands)
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,

    /// Read JSON from stdin (pipe mode)
    #[arg(long = "stdin")]
    stdin: bool,

    /// Patterns to merge (convenience mode, repeatable)
    #[arg(long = "pattern", value_name = "PATTERN")]
    patterns: Vec<String>,

    /// Separators for patterns (convenience mode, repeatable)
    #[arg(long = "sep", value_name = "CHAR")]
    separators: Vec<String>,

    // ... other options
}
```

## Mode Detection

```rust
pub fn execute(cmd: MergeCommand) -> Result<()> {
    if !cmd.patterns.is_empty() {
        // Pattern mode (convenience)
        execute_pattern_mode(cmd.patterns, cmd.separators, ...)
    } else if cmd.stdin {
        // Stdin mode (pure pipes)
        execute_stdin_mode(...)
    } else if !cmd.files.is_empty() {
        // File mode (pipe to files)
        execute_file_mode(cmd.files, ...)
    } else {
        Err("No input specified. Use --pattern, --stdin, or provide files")?
    }
}
```

## Stdin Protocol

### Input Format (JSON Lines)

Each command outputs JSON, merge reads multiple JSON objects:

```json
{"type": "tree", "base": "main.command", "separator": ".", "files": [...]}
{"type": "tree", "base": "main_command", "separator": "_", "files": [...]}
```

### Parsing Strategy

```rust
fn execute_stdin_mode() -> Result<()> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);

    let mut all_trees = Vec::new();

    // Read JSON objects (one per line or as array)
    for line in reader.lines() {
        let json_obj: TreeJson = serde_json::from_str(&line?)?;
        all_trees.push(json_obj);
    }

    // Merge all trees
    merge_trees(all_trees)
}
```

## Usage Examples

### Pattern Mode (Convenience)
```bash
# Quick and easy
recur merge --pattern "main.command" --sep "." --pattern "main_command" --sep "_"
```

### File Mode (Cacheable)
```bash
# Save intermediate results
recur tree main --sep "." --json > docs.json
recur tree main --sep "_" --json > src.json

# Merge later
recur merge docs.json src.json
```

### Stdin Mode (Pure Pipes)
```bash
# Sequential pipe (requires multi-object stdin parsing)
{ recur tree main --sep "." --json; recur tree main --sep "_" --json; } | recur merge --stdin
```

### Process Substitution (Unix Power)
```bash
# Most Unix-y approach
recur merge <(recur tree main --sep "." --json) <(recur tree main --sep "_" --json)
```

## Advanced Compositions

### Filter Before Merge
```bash
# Merge only certain files
recur tree main --sep "." --json | jq 'filter by criteria' | \
recur tree main --sep "_" --json | jq 'filter by criteria' | \
recur merge --stdin
```

### Merge with External Data
```bash
# Combine recur output with external JSON
cat external-tree.json > combined.json
recur tree main --json >> combined.json
recur merge combined.json
```

### Three-Way Merge
```bash
recur merge \
  <(recur tree api --sep "." --json) \
  <(recur tree api --sep "_" --json) \
  <(recur tree api --sep "-" --json)
```

## Implementation Phases

### Phase 2: Pattern Mode (Current)
- ✅ Basic CLI structure
- ⏳ Path normalization
- ⏳ Pattern-based file discovery

### Phase 3: Provenance Tracking
- Add --show-sep markers
- Track which pattern found each file

### Phase 4: File Mode
- Read JSON from file arguments
- Parse tree JSON format
- Merge multiple file inputs

### Phase 5: Stdin Mode (Pipe Support)
- Read JSON from stdin
- Handle multiple JSON objects
- Support JSON Lines format

### Phase 6: Full Pipe Integration
- Test with process substitution
- Performance optimization
- Documentation & examples

## Benefits of Pipe Mode

### Composability
```bash
# Can pipe through standard tools
recur tree X --json | jq '.files | length' # Count files
recur tree X --json | tee debug.json | recur merge --stdin # Debug
```

### Caching
```bash
# Expensive searches cached
recur tree huge-project --sep "." --json > cache.json
# Later, merge with fresh results
recur merge cache.json <(recur tree new-module --sep "_" --json)
```

### Flexibility
```bash
# Merge ANY tree-like data, not just recur output
cat custom-tree.json | recur merge --stdin
```

### Testing
```bash
# Easy to test with fixtures
cat test-fixture.json | recur merge --stdin > output.txt
diff output.txt expected.txt
```

## JSON Format Specification

### Tree JSON Output
```json
{
  "command": "tree",
  "base": "main.command",
  "separator": ".",
  "files": [
    {"path": "docs/main.command.tree.readme.md", "separator": "."},
    {"path": "docs/main.command.files.readme.md", "separator": "."}
  ],
  "stats": {
    "total_files": 34,
    "total_dirs": 12
  }
}
```

### Files JSON Output
```json
{
  "command": "files",
  "pattern": "main.command.**",
  "separator": ".",
  "files": [
    "docs/main.command.tree.readme.md",
    "docs/main.command.files.readme.md"
  ]
}
```

### Merge Output
```json
{
  "command": "merge",
  "sources": [
    {"base": "main.command", "separator": ".", "count": 34},
    {"base": "main_command", "separator": "_", "count": 14}
  ],
  "merged": {
    "total_files": 48,
    "duplicates_removed": 0
  },
  "tree": { /* merged tree structure */ }
}
```

## Migration Path

1. **Phase 2-3:** Pattern mode (convenience) ✅
2. **Phase 4:** Add file mode, keep pattern mode
3. **Phase 5:** Add stdin mode, keep both previous modes
4. **Result:** Three modes, user chooses based on need

**No breaking changes** - each phase adds capability, doesn't remove.

## Documentation

### README Example
```bash
# Quick merge (pattern mode)
recur merge --pattern X --sep Y --pattern Z --sep W

# Power user merge (pipe mode)
recur merge <(recur tree X --json) <(recur tree Z --json)

# Choose based on your needs:
# - Pattern mode: Quick, convenient
# - Pipe mode: Flexible, composable, cacheable
```

### Man Page Section
```
MODES
    Pattern Mode (Convenience)
        Specify patterns directly. Merge does the searching.

    File Mode (Cacheable)
        Provide JSON files. Merge reads and combines them.

    Stdin Mode (Pure Unix)
        Pipe JSON data. Merge acts as pure compositor.
```

## Next Steps

1. Complete Phase 2: Pattern mode with path normalization
2. Complete Phase 3: Add --show-sep markers
3. Document JSON output format for tree/files commands
4. Implement Phase 4: File mode
5. Implement Phase 5: Stdin mode
6. Test with process substitution
7. Update README with all three modes

## Success Criteria

When pipe mode is complete:
- ✅ Pattern mode works (convenience)
- ✅ File mode works (caching)
- ✅ Stdin mode works (pipes)
- ✅ Process substitution works (Unix power)
- ✅ All modes produce identical output for same input
- ✅ No breaking changes to existing commands

## Estimated Timeline

- Pattern mode: 2-3 hours (Phase 2-3)
- File mode: 1-2 hours (Phase 4)
- Stdin mode: 2-3 hours (Phase 5)
- **Total: ~6-8 hours** for full pipe support

## Why This Matters

> "Write programs that do one thing well. Write programs to work together."
> - Doug McIlroy, Unix Philosophy

Pipe mode makes `recur merge` a true Unix citizen - composable, flexible, and predictable. Pattern mode provides convenience without sacrificing power.

**Both modes together = Best of both worlds.**
