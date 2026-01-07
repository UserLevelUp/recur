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

# View hierarchy as tree (recursive structure)
recur tree "ServiceName"

# Find files matching pattern (recursive)
recur files "Controller.*.Tests"

# Find related files (siblings in hierarchy)
recur related "UserService.Handlers.Create.cs"

# Search for hierarchical identifiers (recursive)
recur id "config.database.*"
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

| Feature | grep (1973) | rg | recur (2026) |
|---------|-------------|----|---------| 
| Fast | ? | ?? | ?? |
| Regex | ? | ? | ? |
| **Hierarchy-aware** | ? | ? | ?? |
| **Recursive tree** | ? | ? | ?? |
| **Scoped search** | ? | ? | ?? |

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

## Why Rust?

- Memory safety without garbage collection (successor to C)
- `ripgrep` proved Rust tools get adopted on Linux
- Excellent cross-platform support
- Cargo makes contributing easy

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md).

```bash
git clone https://github.com/userlevelup/recur
cd recur
cargo test
cargo run -- tree "src"
```

## License

MIT - Like Unix, open and free.

## Acknowledgments

- **Dennis M. Ritchie (1941-2011)** - For recursive hierarchies (1968)
- **Ken Thompson** - For grep and Unix (1973)
- **The Unix Philosophy** - Do one thing well
- **Rust Community** - For the tools and ecosystem

---

**recur**: *Recursive hierarchical search for the 21st century, honoring 58 years of innovation since Dennis Ritchie's 1968 thesis.*

*"UNIX is very simple, it just needs a genius to understand its simplicity."* — Dennis Ritchie
"# recur" 
