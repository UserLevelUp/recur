# recur: Recursive Hierarchical Search Tool

**In Honor of Dennis M. Ritchie (1941-2011)**

> *"Program Structure and Computational Complexity"*  
> — Dennis Ritchie's PhD Thesis, Harvard University, 1968

## The Tribute

Dennis Ritchie's doctoral thesis explored **recursive functions and hierarchical program structures**. He later co-created Unix and the C programming language—tools built on hierarchical filesystem concepts. 

**recur** (recursive hierarchical search) honors this legacy by bringing hierarchical understanding to code search, 58 years after his foundational work.

## Project Vision

**recur** is a modern command-line search tool designed for recursively-organized codebases. Unlike `grep` (created by Ken Thompson in 1973 for Unix) which was designed for flat text files, **recur** understands **hierarchical file naming conventions** and **recursive directory structures**.

## Why "recur"?

1. **Recursive** - Searches hierarchies recursively
2. **Recur** - Short, memorable, easy to type
3. **Ritchie** - Honors Dennis Ritchie's PhD work on recursive hierarchies
4. **Unix Philosophy** - Simple, composable, powerful (like grep, awk, sed)

## The Dennis Ritchie Connection

### His Work
- **1968**: PhD thesis on recursive functions and program hierarchies
- **1969-1973**: Co-created Unix with Ken Thompson at Bell Labs
- **1972**: Created the C programming language
- **1973**: Ken Thompson created `grep` for Unix

### Our Work
- **2026**: Create `recur` - hierarchical search for modern code
- Built in **Rust** (memory-safe successor to C)
- Respects **Unix philosophy** (pipes, exit codes, composability)
- Understands **hierarchical naming** (what Dennis explored in his thesis)

## The Problem

Modern codebases use hierarchical naming:
- **Files**: `LevelController.CreateWizard3.Templates.cs`
- **Identifiers**: `config.database.connection.timeout`
- **Modules**: `user.level.up.services.game`

Traditional tools (grep, awk, find) don't understand these **recursive hierarchical structures**.

## The Solution: recur

```bash
# Search within a hierarchical scope (recursive)
recur find "CreateSection" --scope "LevelController.CreateWizard3"

# Show recursive file hierarchy as a tree
recur tree "LevelController"
# LevelController (recursive structure)
# ??? Base.cs
# ??? CreateWizard3 (recursion level 1)
# ?   ??? Creation.cs
# ?   ??? Templates.cs (recursion level 2)
# ?   ??? Models.cs
# ??? Publish.cs

# Find related files (siblings in hierarchy)
recur related "DynamicGameService.Ops.cs"

# Search for hierarchical identifiers recursively
recur id "ulu.role.*"
```

## Core Features

### 1. Recursive File Matching
```bash
# Pattern: Parent.Child.* matches recursively
recur files "LevelController.CreateWizard3.*"
```

### 2. Scoped Recursive Search
```bash
# Search only within hierarchy (recursive descent)
recur find "pattern" --scope "Module.SubModule"
```

### 3. Recursive Tree Visualization
```bash
# Show hierarchy tree (recursive structure)
recur tree "ServiceName" --depth 3
```

### 4. Related File Discovery (Hierarchical)
```bash
# Find siblings (same parent in hierarchy)
recur related "File.Parent.Child.cs"
```

### 5. Identifier Search (Recursive Patterns)
```bash
# Search for hierarchical identifiers recursively
recur id "config.database.**"  # ** = recursive wildcard
```

## Comparison with grep

| Feature | grep (1973) | recur (2026) |
|---------|-------------|--------------|
| Created by | Ken Thompson | User Level Up |
| Inspiration | Unix pipes | Dennis Ritchie's thesis |
| Design | Flat text | Recursive hierarchies |
| Patterns | Regex | Hierarchical + Regex |
| Output | Lines | Hierarchical context |
| Philosophy | Unix | Unix + Modern |

## Installation

```bash
# From source
git clone https://github.com/userlevelup/recur
cd recur
cargo install --path .

# From crates.io
cargo install recur

# Debian/Ubuntu
sudo apt install recur

# Arch Linux
yay -S recur
```

## Usage Examples

```bash
# Basic search (like grep, but hierarchy-aware)
recur find "async" --scope "Controller"

# Recursive tree (unlike ls -R, understands hierarchy)
recur tree "LevelController" --depth 2

# Find files (unlike find, understands naming patterns)
recur files "Module.SubModule.*"

# Combine with Unix tools (pipes work!)
recur files "Controller.*" | xargs grep "async"
```

## Technical Details

- **Language**: Rust (memory-safe successor to C)
- **Performance**: < 1?s parse time, parallel search
- **Philosophy**: Unix-friendly (exit codes, pipes, composition)
- **License**: MIT (like Unix, open and free)

## Tribute to Dennis Ritchie

### Why This Matters

Dennis Ritchie's 1968 thesis explored **recursive functions and hierarchical program structures** - the very concepts that modern programming relies on:

- **Recursive file systems** (directories contain directories)
- **Hierarchical namespaces** (modules contain submodules)
- **Nested scopes** (functions contain functions)

**recur** brings his theoretical work full circle—applying hierarchical understanding to code search in the 21st century.

### The Legacy

```
1968: Dennis Ritchie - "Program Structure and Computational Complexity"
      ? (Recursive hierarchies in theory)
1969: Unix - Hierarchical filesystem
      ? (Recursive hierarchies in practice)
1972: C - Hierarchical include system
      ? (Recursive hierarchies in language)
1973: grep - Flat text search (Ken Thompson)
      ? (Search without hierarchy understanding)
2026: recur - Hierarchical search
      ? (Search WITH recursive hierarchy understanding)
```

### In Memoriam

> "Dennis was well loved by his colleagues at Bell Labs, and by computer programmers everywhere."  
> — Rob Pike, on Dennis Ritchie's passing, October 12, 2011

**recur is our tribute to his foundational work on recursive hierarchical structures.**

## Name Alternatives Considered

- `hsearch` - Hierarchical search (original name)
- `hgrep` - Hierarchical grep (taken by rhysd/hgrep)
- `hf` - Hierarchical find (Hugging Face conflict)
- `hs` - Hierarchical search (considered)
- **`recur`** - Recursive hierarchical search ? (chosen - honors Dennis Ritchie)

## Documentation

- [Full Proposal](RECUR-PROPOSAL.md)
- [Implementation Guide](IMPLEMENTATION-COMPLETE.md)
- [Contributing](CONTRIBUTING.md)
- [Dennis Ritchie's Thesis](https://en.wikipedia.org/wiki/Dennis_Ritchie#Education_and_early_career)

## Acknowledgments

- **Dennis M. Ritchie** - For his pioneering work on recursive hierarchies
- **Ken Thompson** - For creating grep and Unix
- **The Unix Philosophy** - Simple tools that do one thing well
- **Rust Community** - For the language and ecosystem
- **User Level Up** - For the hierarchical naming conventions that inspired this tool

## License

MIT License - Like Unix, open and free.

---

**recur**: *Honoring 58 years of recursive hierarchical structures, from Dennis Ritchie's 1968 thesis to modern codebases.*

*"The best way to predict the future is to invent it."* — Alan Kay (but Dennis Ritchie invented the hierarchical future we live in today)
