# Phase 5: Enhanced Help & Examples for Multi-Separator

## Goal
Add rich examples to help text showing multi-separator usage patterns, different separator types, and expected outputs.

## Where to Add Examples

### 1. Command Help Text (in main.rs)

Update the doc comments for `tree` and `files` commands to include multi-separator examples:

```rust
/// Show recursive hierarchy tree for files
///
/// Examples:
///   # Single separator (default)
///   recur tree "LevelController"
///
///   # Multiple separators - merge domains
///   recur tree "main" --sep "." --sep "_"
///
///   # With normalized output
///   recur tree "main" --sep "." --sep "_" --sep-replace-default "."
///
///   # Show which domain each file is from
///   recur tree "main" --sep "." --sep "_" --show-sep
///
///   # Git integration with multi-separator
///   git diff --name-only | recur tree "**" --stdin --sep "." --sep "_"
Tree {
    // ...
}
```

### 2. Global `--sep` Flag Help

Update the `sep` field documentation in the `Cli` struct:

```rust
/// Hierarchy separator character (default: '.')
///
/// Common separators:
///   '.'  - Dot (docs, tests, most codebases)
///   '_'  - Underscore (Rust modules, Python)
///   '-'  - Dash (kebab-case)
///   ':'  - Colon (namespaces, C++)
///
/// Multi-separator usage:
///   --sep "." --sep "_"  # Merge dot and underscore hierarchies
///
/// This enables cross-domain queries:
///   recur tree main --sep "." --sep "_"
///   # Shows docs (main.command.files.md) + src (main_command_files.rs)
#[arg(long, global = true, value_name = "CHAR")]
sep: Vec<String>,
```

### 3. Examples Section in README/Documentation

Create examples showing:

#### Example 1: Rust Project (Docs + Source)
```bash
# Problem: Rust modules use _, but docs use .
# Solution: Query both at once!

recur tree main --sep "." --sep "_"
```

**Output:**
```
main (base)
├── command
│   ├── files
│   │   ├── readme.md       # docs/ (dot separator)
│   │   ├── test.jl         # tests/ (dot separator)
│   │   ├── impl.rs         # src/ (underscore separator)
│   │   └── stdin.rs        # src/ (underscore separator)
```

#### Example 2: Python Project (Modules + Docs)
```bash
# Python uses _ for modules, . for docs
recur tree myproject --sep "." --sep "_"
```

#### Example 3: C++ Project (Namespaces + Files)
```bash
# C++ uses :: for namespaces, . for file hierarchy
recur tree Engine --sep "." --sep ":"
```

#### Example 4: Gap Analysis
```bash
# Find which commands have docs but no implementation
recur files "main.command.**" --sep "." --sep "_" --show-sep | grep -v "\[_\]"
```

Shows files that only exist in docs (no underscore = no src implementation).

#### Example 5: Normalized View
```bash
# Show everything with consistent dot notation
recur files "main.command.**" --sep "." --sep "_" --sep-replace-default "."
```

**Before (mixed):**
```
docs/main.command.files.readme.md
src/main_command_files_impl.rs
```

**After (normalized):**
```
main.command.files.readme.md
main.command.files.impl.rs
```

### 4. Interactive Examples (when --help is used)

Add a tip at the bottom of help text:

```
TIP: Use multiple --sep flags to merge hierarchies from different domains:
  recur tree main --sep "." --sep "_"    # Merge docs + source

See examples at: https://github.com/userlevelup/recur#multi-separator
```

## Implementation Tasks

1. **Update main.rs doc comments** for `tree` and `files` commands
2. **Enhance `sep` field documentation** with multi-separator examples
3. **Add after_help for multi-separator** in command definitions
4. **Create visual comparison** in README showing before/after
5. **Add cookbook section** with common multi-separator patterns

## Separator Cheat Sheet (for docs)

| Separator | Common Use | Example Hierarchy |
|-----------|------------|-------------------|
| `.` (dot) | Docs, tests, general | `main.command.files` |
| `_` (underscore) | Rust, Python, C | `main_command_files` |
| `-` (dash) | Kebab-case, URLs | `main-command-files` |
| `:` (colon) | C++ namespaces | `main:command:files` |
| `/` (slash) | Path-like | `main/command/files` |

## Expected Help Output Examples

### `recur tree --help` (excerpt)

```
Show recursive hierarchy tree for files

Usage: recur tree [OPTIONS] <BASE>

Arguments:
  <BASE>  Base name to build tree from

Options:
      --sep <CHAR>
          Hierarchy separator (default: '.')

          Use multiple times to merge different hierarchies:
            --sep "." --sep "_"  # Merge docs + source

          Common separators: . _ - : /

      --sep-replace-default <CHAR>
          Normalize output to this separator

          Example:
            --sep "." --sep "_" --sep-replace-default "."
            Shows all paths with dots, even if originally underscores

      --show-sep
          Show which separator was used for each file

          Output format:
            readme.md [.]     # From dot hierarchy
            impl.rs [_]       # From underscore hierarchy

Examples:
  # Single hierarchy
  recur tree main

  # Merge docs + source
  recur tree main --sep "." --sep "_"

  # Normalized view
  recur tree main --sep "." --sep "_" --sep-replace-default "."

  # See domain for each file
  recur tree main --sep "." --sep "_" --show-sep

  # Git integration
  git diff --name-only | recur tree "**" --stdin --sep "." --sep "_"
```

## Visual Diagrams (for README)

### Before Multi-Separator (Separate Views)

```
# View docs
recur tree main
main
└── command
    └── files
        ├── readme.md
        └── test.jl

# View source (separate command)
recur tree main --sep "_"
main
└── command
    └── files
        ├── impl.rs
        └── stdin.rs
```

### After Multi-Separator (Unified View)

```
# One command, complete view
recur tree main --sep "." --sep "_"
main
└── command
    └── files
        ├── readme.md    # docs
        ├── test.jl      # tests
        ├── impl.rs      # source
        └── stdin.rs     # source
```

**The complete picture in one query!**

## Success Criteria

- [ ] Help text includes 3+ multi-separator examples per command
- [ ] Separator cheat sheet in documentation
- [ ] Visual before/after comparisons in README
- [ ] Common use cases documented (Rust, Python, C++)
- [ ] Gap analysis example showing practical usage
- [ ] Tip/hint shown when using `--help`

## Benefits

1. **Discoverability** - Users see multi-separator in help text
2. **Learning** - Examples show actual usage patterns
3. **Adoption** - Clear use cases encourage feature usage
4. **Debugging** - Examples help users understand output format
