# Command: merge - Implementation Tracking

## Overview
Implement `recur merge` command for Unix-style composability of hierarchical search results across different naming conventions.

## Goals
1. Create `merge` command with pattern/separator pairing
2. Support multiple pattern specifications: `--pattern X --sep Y` (repeatable)
3. Implement pipe mode: read JSON from stdin
4. Add `--show-sep` markers for provenance tracking
5. Support normalization with `--sep-replace-default`
6. Output formats: tree and files

## Expected Behavior

### Basic merge
```bash
recur merge \
  --pattern "main.command.tree" --sep "." \
  --pattern "main_command_tree" --sep "_" \
  --show-sep
```

**Output:**
```
main.command.tree
├── readme.md [.]
├── test.jl [.]
└── impl.rs [_]
```

### Pipe mode
```bash
recur files "main.command.tree.**" --sep "." --json | \
recur files "main_command_tree_**" --sep "_" --json | \
recur merge --stdin --show-sep
```

## Implementation Phases

### Phase 1: Planning & Design
- [ ] Create eventness tracking files ✅
- [ ] Write comprehensive README
- [ ] Design CLI interface
- [ ] Plan merging algorithm
- [ ] Write test cases (Julia)
- [ ] Document pipe mode protocol

### Phase 2: Basic Merge (Pattern Mode)
- [ ] Add `merge` subcommand to CLI
- [ ] Implement pattern/separator pairing
- [ ] Build unified tree from multiple searches
- [ ] Basic output (no markers)
- [ ] Test with 2 patterns

### Phase 3: Provenance Tracking
- [ ] Track which separator found each file
- [ ] Implement `--show-sep` markers
- [ ] Test with 3+ patterns
- [ ] Handle deduplication

### Phase 4: Normalization
- [ ] Implement `--sep-replace-default`
- [ ] Normalize paths to target separator
- [ ] Preserve markers showing original separator
- [ ] Test normalized output

### Phase 5: Pipe Mode
- [ ] Design JSON protocol for stdin
- [ ] Parse JSON from multiple inputs
- [ ] Merge JSON hierarchies
- [ ] Test piping from files/tree commands

### Phase 6: Output Formats
- [ ] Tree format (default)
- [ ] Files format (flat list)
- [ ] JSON output mode
- [ ] ASCII vs Unicode tree

### Phase 7: Polish & Documentation
- [ ] Help text with examples
- [ ] Error messages
- [ ] Edge case handling
- [ ] Performance optimization
- [ ] Update main README.md

## Current Status
**Phase 1: Planning** 🎯

Branch: `merge-pipes`

## CLI Design

```rust
#[derive(Parser)]
pub struct MergeCommand {
    /// Patterns to merge (repeatable, paired with --sep)
    #[arg(long = "pattern", value_name = "PATTERN")]
    patterns: Vec<String>,

    /// Separators for each pattern (repeatable, paired with --pattern)
    #[arg(long = "sep", value_name = "CHAR")]
    separators: Vec<String>,

    /// Show separator markers [.] [_] etc.
    #[arg(long = "show-sep")]
    show_sep: bool,

    /// Normalize output to specific separator
    #[arg(long = "sep-replace-default", value_name = "CHAR")]
    sep_replace_default: Option<String>,

    /// Read JSON input from stdin (pipe mode)
    #[arg(long = "stdin")]
    stdin: bool,

    /// Output format
    #[arg(long = "format", default_value = "tree")]
    format: String,

    /// Output as JSON
    #[arg(long = "json")]
    json: bool,

    /// Use ASCII characters instead of Unicode
    #[arg(long = "ascii")]
    ascii: bool,

    /// Show file counts at each level
    #[arg(long = "count")]
    count: bool,
}
```

## Merging Algorithm

### Step 1: Collect Files
For each (pattern, separator) pair:
```rust
for (pattern, sep) in patterns.iter().zip(separators.iter()) {
    let files = find_files_with_separator(pattern, sep);
    for file in files {
        all_files.push((file, sep));
    }
}
```

### Step 2: Build Hierarchy
```rust
let mut tree = HierarchyTree::new();
for (file, sep) in all_files {
    tree.insert(file, sep);
}
```

### Step 3: Display
```rust
if show_sep {
    tree.display_with_markers();
} else {
    tree.display();
}
```

## Test Cases

### Test 1: Basic two-pattern merge
```julia
@test begin
    # Setup test files
    write("test/main.tree.readme.md", "")
    write("test/main_tree_impl.rs", "")

    # Run merge
    output = run(`recur merge --pattern "main.tree" --sep "." --pattern "main_tree" --sep "_"`)

    # Verify both files appear
    @test contains(output, "readme.md")
    @test contains(output, "impl.rs")
end
```

### Test 2: Three-pattern merge with markers
```julia
@test begin
    output = run(`recur merge
        --pattern "api.user" --sep "."
        --pattern "api_user" --sep "_"
        --pattern "api-user" --sep "-"
        --show-sep`)

    @test contains(output, "[.]")
    @test contains(output, "[_]")
    @test contains(output, "[-]")
end
```

### Test 3: Pipe mode
```julia
@test begin
    json1 = run(`recur files "api.user.**" --sep "." --json`)
    json2 = run(`recur files "api_user_**" --sep "_" --json`)

    output = pipeline(
        `echo $json1`,
        `echo $json2`,
        `recur merge --stdin --show-sep`
    )

    @test contains(output, "[.]")
    @test contains(output, "[_]")
end
```

### Test 4: Normalization
```julia
@test begin
    output = run(`recur merge
        --pattern "api.user" --sep "."
        --pattern "api_user" --sep "_"
        --sep-replace-default "."
        --show-sep`)

    # All paths use dots, but markers show origin
    @test contains(output, "api.user")
    @test !contains(output, "api_user")
    @test contains(output, "[_]")  # Marker preserved
end
```

## Files to Create/Modify

### New Files
- `src/main_command_merge.rs` - Command implementation
- `src/main_command_merge_impl.rs` - Core logic
- `julia-tests/main.command.merge.test.jl` - Test suite
- `docs/main.command.merge.readme.md` - Documentation ✅
- `docs/main.command.merge.todo.md` - This file ✅
- `docs/main.command.merge.todo.current.md` - Current work tracker
- `docs/main.command.merge.phase1.plan.md` - Phase 1 details

### Modified Files
- `src/main.rs` - Add merge subcommand
- `README.md` - Add merge command documentation

## Success Criteria

### Phase 1 Complete When:
- ✅ README written with comprehensive examples
- ✅ CLI interface designed
- ✅ Algorithm planned
- [ ] Test cases written
- [ ] Current work tracker created

### Feature Complete When:
- [ ] All 7 phases complete
- [ ] Can merge 2+ patterns with different separators
- [ ] Pipe mode working
- [ ] Markers show provenance
- [ ] Normalization working
- [ ] All tests passing
- [ ] Documentation updated
- [ ] Real-world usage validated

## Related Work
- Multi-separator merge: `docs/main.trait.separator-merge.*`
- Tree command: `src/main_command_tree_impl.rs`
- Files command: `src/main_command_files_impl.rs`
