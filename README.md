# recur

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)

I'm new to Rust so this is a good project to learn Rust and also perform a useful task grep wasn't designed for which is search hierarchical file in c#, go, javascript, and even Rust codebases.  I'm totally vibing it so enjoy.

**Recursive hierarchical search tool for modern codebases.**

*In honor of Dennis M. Ritchie's 1968 PhD thesis on recursive hierarchies.*

```bash
# Instead of grep returning 500 unrelated matches...
recur find "CreateSection" --scope "LevelController.CreateWizard3"

# See your code hierarchy recursively
recur tree "LevelController"

# Find related files in the hierarchy  
recur related "Service.Module.Feature.cs"
```

## The Tribute

> *"Program Structure and Computational Complexity"*  
> — Dennis Ritchie, PhD Thesis, Harvard, 1968

Dennis Ritchie's thesis explored **recursive functions and hierarchical program structures**. He later co-created Unix and C—tools built on hierarchical concepts. 

**recur honors this 58-year legacy** by bringing recursive hierarchical understanding to code search.

## The Problem

Modern codebases use hierarchical naming:

```
LevelController.CreateWizard3.Templates.cs
config.database.connection.timeout
user.level.up.services.game
```

Traditional tools (grep, awk, find) don't understand these **recursive hierarchical structures**.

## The Solution

```bash
# Recursive search within hierarchy scope
recur find "async" --scope "Controller.Api"

# Recursive tree view
recur tree "ServiceName"
# ServiceName
# ??? Core.cs
# ??? Handlers (recursive level 1)
# ?   ??? Create.cs
# ?   ??? Update.cs (recursive level 2)
# ?   ??? Delete.cs
# ??? Models.cs

# Find files matching recursive pattern
recur files "Controller.*.Tests"

# Find related (sibling) files
recur related "UserService.Handlers.Create.cs"
```

## Installation

### From Source (Rust)
```bash
cargo install recur
```

### Debian/Ubuntu
```bash
# Coming soon
sudo apt install recur
```

### Arch Linux
```bash
# Coming soon (AUR)
yay -S recur
```

## Quick Start

```bash
# Search within hierarchy scope (recursive)
recur find "async" --scope "Controller.Api"

# Search with context lines (like grep -C)
recur find "CreateSection" --scope "LevelController.**" -C 2

# View hierarchy as tree (recursive structure)
recur tree "ServiceName"

# Find files matching pattern (recursive)
recur files "Controller.*.Tests"

# Find related files (siblings in hierarchy)
recur related "UserService.Handlers.Create.cs"

# Find related files excluding self
recur related "UserService.Handlers.Create.cs" --exclude-self

# Search for hierarchical identifiers (recursive)
recur id "config.database.*" -C 1

# Analyze hierarchy statistics by depth
recur stats "ServiceName" -l 1
```

## Features

### Core Capabilities
- **Hierarchical pattern matching** with `*` and `**` wildcards
- **Scoped text search** within hierarchy (like grep but hierarchy-aware)
- **Context lines** with `-C` flag (shows surrounding lines like grep)
- **Tree visualization** with Unicode box-drawing characters
- **Related file discovery** (find siblings in hierarchy)
- **Identifier search** (find dot-notation identifiers in code)
- **Hierarchy statistics** with depth-level analysis and pagination
- **Multiple output formats** (terminal with colors, JSON for tooling)
- **Proper exit codes** (0=success, 1=no results, 2=error)

### Grep-like Options
- `-C N` - Show N lines of context around matches
- `-i` - Case-insensitive search
- `-E` - Use regular expressions
- `--json` - Output results as JSON
- `--color` - Colorized output (auto-detected)

## Commands

### `recur files` - Find files by hierarchical pattern
```bash
recur files "Controller.*"                    # All direct children
recur files "Controller.**"                   # All descendants (recursive)
recur files "*.Tests" --ext .cs              # Test files only
recur files "Module.*" --count               # Show count only
```

### `recur find` - Search text within hierarchy scope
```bash
recur find "async" --scope "Controller.Api"          # Search in scope
recur find "TODO" --scope "Service.**" -C 2          # With context
recur find "pattern" --scope "Module" -i             # Case-insensitive
recur find "async.*Task" --scope "**" -E             # Regex search
```

### `recur tree` - Visualize hierarchy as tree
```bash
recur tree "ServiceName"                     # Unicode tree view
recur tree "ServiceName" --count             # With file counts
recur tree "ServiceName" --ascii             # ASCII-only (no Unicode)
recur tree "ServiceName" --json              # JSON output
```

### `recur related` - Find sibling files
```bash
recur related "Service.Module.Feature.cs"           # Include self
recur related "Service.Module.Feature.cs" --exclude-self  # Exclude self
```

### `recur children` - Find child files
```bash
recur children "Service.Module"              # All children
recur children "Service.Module" --count      # Show count only
```

### `recur id` - Search for identifiers
```bash
recur id "config.database.*"                 # Find identifiers
recur id "ulu.role.**" -C 2                  # With context lines
recur id "config.*" --ext .json              # JSON files only
```

### `recur stats` - Analyze hierarchy statistics
```bash
recur stats "ServiceName"                    # Summary with depth breakdown
recur stats "ServiceName" -l 0               # List files at depth 0 (base)
recur stats "ServiceName" -l 1               # List files at depth 1 (children)
recur stats "Controller.**" --ext .cs        # Stats for .cs files only
recur stats "**" --json                      # JSON output
```

## Pattern Syntax (Recursive)

| Pattern | Matches | Example |
|---------|---------|---------|
| `Module.Sub` | Exact | `Module.Sub.cs` |
| `Module.*` | One level (recursive 1) | `Module.Feature.cs` |
| `Module.**` | Any depth (fully recursive) | `Module.A.B.C.cs` |
| `*.Tests` | Prefix wildcard | `Module.Tests.cs` |
| `Module.**.Tests` | Deep + suffix | `Module.Sub.Feature.Tests.cs` |

## Comparison

| Feature | grep (1973) | rg (ripgrep) | recur (2026) |
|---------|-------------|--------------|-------------|
| Fast text search | ✅ | ✅ | ✅ |
| Regex support | ✅ | ✅ | ✅ |
| Context lines (`-C`) | ✅ | ✅ | ✅ |
| **Hierarchy-aware** | ❌ | ❌ | ✅ |
| **Recursive tree view** | ❌ | ❌ | ✅ |
| **Scoped search** | ❌ | ❌ | ✅ |
| **Related file discovery** | ❌ | ❌ | ✅ |
| **Pattern matching** | ❌ | ❌ | ✅ |

**Note:** `recur` doesn't replace `rg` or `grep` - it complements them. For pure text search speed, `ripgrep` is unmatched. `recur` fills a different niche: understanding hierarchical file naming conventions. See [README.rg.md](README.rg.md) for a detailed comparison of when to use each tool.

## Why "recur"?

1. **Recursive** - Searches hierarchies recursively
2. **Recur** - Short, memorable (like grep, awk, sed)
3. **Ritchie** - Honors Dennis Ritchie's recursive hierarchy work
4. **Unix Philosophy** - Simple, composable, powerful

## The Legacy

```
1968: Dennis Ritchie - PhD on recursive hierarchies
      ?
1969: Unix - Hierarchical filesystem (recursive)
      ?
1972: C - Hierarchical includes (recursive)
      ?
1973: grep - Flat text search (Ken Thompson)
      ?
2026: recur - Hierarchical search (recursive)
```

## Documentation

- [Full Tribute](RECUR-TRIBUTE.md) - The Dennis Ritchie connection
- [Proposal](RECUR-PROPOSAL.md) - Complete technical design
- [Contributing](CONTRIBUTING.md) - How to contribute
- [Implementation](IMPLEMENTATION-COMPLETE.md) - Complete code walkthrough

## Standing on the Shoulders of Giants

`recur` exists because of the incredible work that came before:

- **grep (1973)** - Ken Thompson's revolutionary pattern matching tool set the standard for text search
- **ripgrep (2016)** - Andrew Gallant's blazingly fast rewrite proved Rust could improve on C's performance
- **Unix philosophy** - Do one thing well, compose tools together

`recur` doesn't aim to replace these tools. Instead, it addresses a specific need they weren't designed for: **understanding hierarchical file naming patterns** common in modern codebases.

Use `rg` for fast text search. Use `recur` when your files are named `Service.Module.Feature.cs`.

## Why Rust?

- Memory safety without garbage collection (like C, but safer)
- `ripgrep` proved Rust tools can match or exceed C performance
- Excellent cross-platform support
- Cargo makes contributing easy
- Standing on ripgrep's shoulders for lessons learned

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md).

```bash
git clone https://github.com/userlevelup/recur
cd recur
cargo test
cargo run -- tree "src"
cargo install --path .   <-- installs globally
```

## License

MIT - Like Unix, open and free.

## Acknowledgments

This tool exists because of those who came before:

- **Dennis M. Ritchie (1941-2011)** - For his 1968 PhD thesis on recursive hierarchies, and for co-creating Unix and C
- **Ken Thompson** - For grep (1973), which defined what a search tool should be
- **Andrew Gallant (BurntSushi)** - For ripgrep, which showed how Rust could honor and improve Unix tools
- **The Unix Philosophy** - "Do one thing well" - a principle we humbly attempt to follow
- **The Rust Community** - For creating an ecosystem that makes tools like this possible

We stand on the shoulders of 58 years of innovation. `recur` is just the next small step.

---

**recur**: *Recursive hierarchical search for the 21st century, honoring 58 years of innovation since Dennis Ritchie's 1968 thesis.*

*"UNIX is very simple, it just needs a genius to understand its simplicity."* — Dennis Ritchie
"# recur" 
