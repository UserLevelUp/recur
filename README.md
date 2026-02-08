<div align="center">
  <h1>recur</h1>
  <div class="version">v0.1.12</div>
  <p>
    <a href="https://opensource.org/licenses/MIT"><img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg"></a>
    <a href="https://www.rust-lang.org/"><img alt="Rust 1.70+" src="https://img.shields.io/badge/rust-1.70%2B-blue.svg"></a>
  </p>
</div>

**Recursive, hierarchy-aware search for modern codebases.**

Development is mainly using ChatGPT 5.2+ Codex and Claude Sonnet & Opus 4.5+ tied to vscode.  I run out of tokens on one I usually switch to the other one.  I use Julia for tests as I'm more familiar with that to verify work.   I actually don't understand Rust at all, but it seems pretty easy to vibe code as long as I can verify and control that direction with the tests.  I also test recur in a private Github Copilot project which will remain nameless so as not to reek havoc. 

`recur` is a command-line tool for working with *hierarchically named code*: files, modules, and identifiers that encode structure using dot-separated or “C#-style” naming conventions. While tools like `grep` and `ripgrep` excel at fast text matching, they treat results as flat lists. `recur` complements them by understanding hierarchy—so you can search, navigate, and analyze related code as a structured system.

*Inspired by Dennis M. Ritchie’s 1968 work on recursive hierarchies and program structure.*

```bash
# Search within a hierarchical scope
recur find "CreateSection" --scope "LevelController.CreateWizard3"

# Visualize a hierarchy as a tree
recur tree "LevelController"

# Discover related (sibling) files in the hierarchy
recur related "Service.Module.Feature.cs"
```

## Why recur?

Many modern codebases encode structure directly into names:

```
LevelController.CreateWizard3.Templates.cs
config.database.connection.timeout
user.level.up.services.game
```

Traditional tools (e.g., `grep`, `awk`, `find`) do not interpret these as *recursive hierarchies*. `recur` does—making it easier to:

- search *within* a subsystem (scope),
- visualize nested structure (tree),
- find related files (siblings/children),
- and locate hierarchical identifiers in code.

## Installation

**What you get:** Two binaries for a complete toolset:
- **`recur`** - Core hierarchy tool (pure, no Git dependencies)
- **`recur-git`** - Git workflow extension (checkpoint tracking, dogfooding)
  - recur-git depends on existing git binaries to already be installed
### From Cargo (crates.io)
```bash
cargo install recur
```

### From source (local checkout)
```bash
cargo build --profile release-safe --locked
cargo install --path . --profile release-safe --locked --force --offline
```

Both binaries will be installed to Cargo's bin folder. If `recur` is not found, add it to PATH:
- Windows: `%USERPROFILE%\.cargo\bin`
- macOS/Linux: `~/.cargo/bin`

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
# Search within a hierarchy scope (recursive)
recur find "async" --scope "Controller.Api"

# Search with context lines (similar to grep -C)
recur find "CreateSection" --scope "LevelController.**" -C 2

# View hierarchy as a tree (recursive structure)
recur tree "ServiceName"

# Find files matching a hierarchical pattern (recursive)
recur files "Controller.*.Tests"

# Find related files (siblings in hierarchy)
recur related "UserService.Handlers.Create.cs"

# Find related files excluding the input file
recur related "UserService.Handlers.Create.cs" --exclude-self

# Search for hierarchical identifiers (recursive)
recur id "config.database.*" -C 1

# Analyze hierarchy statistics by depth (with listing at depth level)
recur stats "ServiceName" -l 1
```

## Features

### Core capabilities
- **Hierarchy-aware pattern matching** with `*` and `**` wildcards
- **Scoped text search** within a hierarchy (grep-like, but structure-aware)
- **Context lines** via `-C` (show surrounding lines like `grep -C`)
- **Tree visualization** using Unicode box-drawing characters
- **Related file discovery** (siblings within the hierarchy)
- **Identifier search** (dot-notation identifiers in code)
- **Hierarchy statistics** with depth analysis and pagination
- **Multiple output formats** (human-friendly terminal output, plus JSON for tooling)
- **Proper exit codes** (0=success, 1=no results, 2=error)

### Grep-like options
- `-C N` - Show N lines of context around matches
- `-i` - Case-insensitive search
- `-E` - Use regular expressions
- `--json` - Output results as JSON
- `--color` - Colorized output (auto-detected)

## Commands

### `recur files` — find files by hierarchical pattern
```bash
recur files "Controller.*"                    # All direct children
recur files "Controller.**"                   # All descendants (recursive)
recur files "*.Tests" --ext .cs              # Test files only
recur files "Module.*" --count               # Show count only
recur files **.AutoSave.** -i -e cs          # No quotes needed with stdin stdout
```

### `recur find` — search text within a hierarchy scope
```bash
recur find "async" --scope "Controller.Api"          # Search in scope
recur find "TODO" --scope "Service.**" -C 2          # With context
recur find "pattern" --scope "Module" -i             # Case-insensitive
recur find "async.*Task" --scope "**" -E             # Regex search
```

### `recur tree` — visualize hierarchy as a tree
```bash
recur tree "ServiceName"                     # Unicode tree view
recur tree "ServiceName" --count             # With file counts
recur tree "ServiceName" --ascii             # ASCII-only (no Unicode)
recur tree "ServiceName" --json              # JSON output
```

### `recur related` — find sibling files
```bash
recur related "Service.Module.Feature.cs"                 # Include self
recur related "Service.Module.Feature.cs" --exclude-self  # Exclude self
```

### `recur children` — find child files
```bash
recur children "Service.Module"              # All children
recur children "Service.Module" --count      # Show count only
```

### `recur id` — search for hierarchical identifiers
```bash
recur id "config.database.*"                 # Find identifiers
recur id "ulu.role.**" -C 2                  # With context lines
recur id "config.*" --ext .json              # JSON files only
```

### `recur stats` — analyze hierarchy statistics
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
| `Module.*` | One level (depth = 1) | `Module.Feature.cs` |
| `Module.**` | Any depth (recursive) | `Module.A.B.C.cs` |
| `*.Tests` | Prefix wildcard | `Module.Tests.cs` |
| `Module.**.Tests` | Deep + suffix | `Module.Sub.Feature.Tests.cs` |

## Comparison

| Feature | grep (1973) | rg (ripgrep) | recur (2026) |
|---------|-------------|--------------|-------------|
| Fast text search | ✅ | ✅ | ✅ |
| Regex support | ✅ | ✅ | ✅ |
| Context lines (`-C`) | ✅ | ✅ | ✅ |
| **Hierarchy-aware** | ❌ | ❌ | ✅ |
| **Tree view** | ❌ | ❌ | ✅ |
| **Scoped search** | ❌ | ❌ | ✅ |
| **Related file discovery** | ❌ | ❌ | ✅ |
| **Hierarchical patterns** | ❌ | ❌ | ✅ |

**Note:** `recur` does not replace `rg` or `grep`—it complements them. For raw text-search throughput, `ripgrep` is hard to beat. `recur` focuses on a different (and increasingly common) problem: *working with structure encoded in names*.

## The Tribute

> *"Program Structure and Computational Complexity"*  
> — Dennis Ritchie, PhD Thesis, Harvard, 1968

Ritchie’s thesis explored **recursive functions and hierarchical program structures**—ideas that later shaped Unix and C. `recur` is a small tribute to that legacy: bringing hierarchy-aware understanding to everyday developer search workflows.

## Why the name “recur”?

1. **Recursive** — searches hierarchies recursively  
2. **Recur** — short and memorable (in the tradition of `grep`, `awk`, `sed`)  
3. **Ritchie** — honors Dennis Ritchie’s early work on hierarchical program structure  
4. **Unix philosophy** — aim for a focused tool that composes well

## Documentation

- [Full Tribute](RECUR-TRIBUTE.md) — the Dennis Ritchie connection  
- [Proposal](RECUR-PROPOSAL.md) — technical design  
- [Contributing](CONTRIBUTING.md) — how to contribute  
- [Implementation](IMPLEMENTATION-COMPLETE.md) — code walkthrough  

## recur-git: Git Workflow Extension

`recur-git` is a separate binary for Git-aware workflows. It keeps `recur` pure (focused on hierarchies) while adding checkpoint tracking for dogfooding.

**Checkpoint tracking:**
```bash
# Snapshot current state (git + active todo leaves)
recur-git checkpoint --snapshot

# Append to checkpoint log (requires --file)
recur-git checkpoint --append-parallel --checkpoint-id ck-phase3-01 -f checkpoints.md
```

See [`docs/main.recur-git.artifact.md`](docs/main.recur-git.artifact.md) for details.

## Contributing

Contributions are welcome. Please see [CONTRIBUTING.md](CONTRIBUTING.md).

For dogfooding state transitions and commit checkpoints, use:
- `docs/main.git.checkpoint.readme.md`
- `scripts/dogfooding_checkpoint.ps1`
- `recur-git checkpoint` commands (see above)

### Local git hook note

This repo uses a **local-only** pre-commit hook on the maintainer's machine to bump `VERSION`, update `Cargo.toml`, and refresh `Cargo.lock` when committing on `main`. Git hooks live in `.git/hooks` and are not shared with other clones. If you need to update or remove it later, check `.git/hooks/pre-commit`.

```bash
git clone https://github.com/userlevelup/recur
cd recur
cargo test
cargo build --profile release-safe --locked
cargo run --profile release-safe -- tree "main" -d src
cargo install --path . --profile release-safe --locked --force --offline
```

## License

MIT — permissive, simple, and broadly compatible.

## Acknowledgments

`recur` is possible because of the tools and ideas that came before:

- **Dennis M. Ritchie (1941–2011)** — for foundational work on recursion and program structure  
- **Ken Thompson** — for `grep` and the standard it set for developer tooling  
- **Andrew Gallant (BurntSushi)** — for `ripgrep` and modern Rust CLI excellence  
- **The Unix philosophy** — “Do one thing well”  
- **The Rust community** — for the ecosystem that makes tools like this practical  

---

**recur**: *Hierarchy-aware search for the 21st century—built for modern naming conventions, inspired by foundational ideas in program structure.*
