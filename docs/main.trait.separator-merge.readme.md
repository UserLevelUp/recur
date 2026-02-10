# Trait: MultiSeparatorCapable

## What It Does

The `MultiSeparatorCapable` trait allows commands to accept multiple `--sep` flags and merge results from different separator domains into a unified view.

## Problem

Recur uses different separators in different domains:
- **Docs/Tests**: Use `.` (dots) - `main.command.files.readme.md`
- **Source code**: Use `_` (underscores) - `main_command_files_impl.rs`

Currently, you must query each domain separately:
```bash
recur tree "main" -d docs/         # Only sees docs
recur tree "main" -d src/ --sep _  # Only sees source
```

This means you can't see the **complete picture** - where docs, tests, and implementation all exist together.

## Solution

Multiple separators merge the logical hierarchy:
```bash
recur tree "main" --sep "." --sep "_"
```

Output shows ALL files under their logical nodes:
```
main.command.files/
  readme.md      # from docs (. separator)
  test.jl        # from tests (. separator)
  impl.rs        # from src (_ separator)
  stdin.rs       # from src (_ separator)
```

## Use Cases

### 1. Complete View of a Feature

See all artifacts for a command:
```bash
recur tree "main.command.files" --sep "." --sep "_"
```

Shows:
- Documentation (`.md` files)
- Tests (`.jl` files)
- Implementation (`.rs` files)

### 2. Cross-Domain File Listing

List all files for a pattern:
```bash
recur files "main.command.**" --sep "." --sep "_"
```

Gets every file across docs, tests, and source.

### 3. Normalized Path Display

Use `--sep-replace-default` to align paths visually:
```bash
recur files "main.command.**" --sep "." --sep "_" --sep-replace-default "."
```

Output (normalized to dots):
```
main.command.files.readme.md
main.command.files.test.jl
main.command.files.impl.rs       # actually main_command_files_impl.rs
main.command.files.stdin.rs      # actually main_command_files_stdin.rs
```

### 4. Show Which Domain

Use `--show-sep` to see which separator (domain) each file uses:
```bash
recur tree "main.command.files" --sep "." --sep "_" --show-sep
```

Output:
```
main.command.files/
  readme.md [.]
  test.jl [.]
  impl.rs [_]
  stdin.rs [_]
```

This shows **completeness** - which domains have files for this feature.

## Supported Commands

### tree
Primary use case - shows merged hierarchy:
```bash
recur tree "main" --sep "." --sep "_"
recur tree "main" --sep "." --sep "_" --sep-replace-default "."
recur tree "main" --sep "." --sep "_" --show-sep
```

### files
Lists all matching paths from all separator domains:
```bash
recur files "main.command.**" --sep "." --sep "_"
recur files "main.command.**" --sep "." --sep "_" --sep-replace-default "." --show-sep
```

## Flags

### `--sep <SEPARATOR>` (repeatable)
Specify multiple separators to query. Each separator scans the hierarchy independently, then results are merged.

```bash
--sep "." --sep "_"        # Query both dot and underscore hierarchies
```

### `--sep-replace-default <SEPARATOR>`
Normalize all output paths to use this separator, regardless of actual file separator.

```bash
--sep-replace-default "."  # Show all paths with dots
```

Useful for:
- Visual alignment
- Piping to tools expecting consistent format
- Understanding logical hierarchy without separator noise

### `--show-sep`
Append the actual separator used after each file/path.

```bash
--show-sep                 # Show [.] or [_] after each file
```

Useful for:
- Seeing which domain (docs vs src) a file belongs to
- Understanding completeness across domains
- Debugging multi-separator queries

## Implementation Pattern

Commands implement the trait:
```rust
pub trait MultiSeparatorCapable {
    fn execute_with_separators(
        &self,
        separators: Vec<char>,
        replace_default: Option<char>,
        show_sep: bool,
    ) -> anyhow::Result<()>;
}
```

The trait handles:
1. Running queries with each separator
2. Merging results by logical hierarchy
3. Normalizing separators if requested
4. Annotating with separator markers if requested

## Merging Strategy

Results are merged by **logical path**:
1. Query with separator `.` finds `main.command.files.readme.md`
2. Query with separator `_` finds `main_command_files_impl.rs`
3. Both map to logical path `main → command → files`
4. Merged output shows both under `main.command.files/`

## Edge Cases

### Same Logical Path, Different Files
Both domains can have files with the same logical name:
```
docs/main.command.files.readme.md
src/main_command_files_readme.rs
```

Both appear in merged output - no conflict.

### No Separator Replacement
Without `--sep-replace-default`, paths display with their actual separator:
```
main.command.files.readme.md
main_command_files_impl.rs
```

With `--sep-replace-default "."`:
```
main.command.files.readme.md
main.command.files.impl.rs      # normalized!
```

## Philosophy

**The logical hierarchy is the source of truth.**

Different domains (docs, tests, source) use different physical separators due to technical constraints (Rust identifiers can't have dots). But they represent the **same logical structure**.

Multi-separator support unifies this - you see the complete feature across all domains.

## Example Workflow

**Find what exists for the `files` command:**
```bash
recur tree "main.command.files" --sep "." --sep "_" --show-sep
```

Output:
```
main.command.files/
  readme.md [.]        # ✅ Documented
  test.jl [.]          # ✅ Tested
  impl.rs [_]          # ✅ Implemented
  stdin.rs [_]         # ✅ Has stdin support
```

**Gap analysis** - see which commands lack implementation:
```bash
recur files "main.command.**" --sep "." --sep "_" --show-sep | grep -v "\[_\]"
```

Shows files that only exist in docs/tests but not in src.

## See Also

- [main.trait.stdin](../../src/trait/stdin.rs) - StdinCapable trait pattern
- [main.trait.content_search](../../src/trait/content_search.rs) - ContentSearchCapable trait pattern
- [main.separator.history](main.separator.history.md) - Separator design decisions
