<div align="center">
  <p>
    <img src="choco/recur-icon.svg" alt="recur icon" width="128">
  </p>
  <h1>recur</h1>
  <div class="version">v0.2.8</div>
  <p>
    <a href="https://opensource.org/licenses/MIT"><img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg"></a>
    <a href="https://www.rust-lang.org/"><img alt="Rust 1.70+" src="https://img.shields.io/badge/rust-1.70%2B-blue.svg"></a>
  </p>
</div>

**Hierarchy-aware code search for modern codebases.**

Development is mainly using ChatGPT 5.2+ Codex and Claude Sonnet & Opus 4.5+ tied to vscode.  I run out of tokens on one I usually switch to the other one.  I use Julia for tests as I'm more familiar with that to verify work.   I actually don't understand Rust at all, but it seems pretty easy to vibe code as long as I can verify and control that direction with the tests.  I also test recur in a private Github Copilot project which will remain nameless so as not to reek havoc. 

`recur` is a command-line tool for working with *hierarchically named code*: files, modules, and identifiers that encode structure using dot-separated or “C#-style” naming conventions. While tools like `grep` and `ripgrep` excel at fast text matching, they treat results as flat lists. `recur` complements them by understanding hierarchy—so you can search, navigate, and analyze related code as a structured system.

*Named in quiet tribute to Dennis Ritchie's early work on recursive program structure.*

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

> **Important:** `recur` searches directories recursively.
> Run it from your project/solution root (or pass `-d <path>`) so you only scan relevant files.
> Avoid running from `C:\`, `C:\Windows`, `/`, or other system roots unless you intentionally want a full-machine scan.
> For safer defaults, run `recur init` to create `.recur/config.toml` with project-local settings and excludes.

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
- **Multi-separator merge** — unify results across naming conventions (`.` and `_`)
- **Unix pipe composability** — stdin mode for pure pipe workflows (auto-JSON detection)
- **Scoped text search** within a hierarchy (grep-like, but structure-aware)
- **Context lines** via `-C` (show surrounding lines like `grep -C`)
- **Tree visualization** using Unicode box-drawing characters
- **Related file discovery** (siblings within the hierarchy)
- **Identifier search** (dot-notation identifiers in code)
- **Identifier flow tracing** (`recur trace-id` for define/produce/consume/trigger lanes)
- **Hierarchy statistics** with depth analysis and pagination
- **Project-aware config** via `.recur/config.toml` (`recur init`, `recur init --analyze`)
- **Trait config management** via `recur trait` (list/get/set trait keys in `.recur/config.toml`)
- **Structured flattening** (`recur flatten`) for XML/JSON/TOML/YAML/CSV to `path = value` records
- **Cross-domain gap analysis** — verify completeness across representations
- **Multiple output formats** (human-friendly terminal output, plus JSON for tooling)
- **Proper exit codes** (0=success, 1=no results, 2=error)

### Grep-like options
- `-C N` - Show N lines of context around matches
- `-i` - Case-insensitive search
- `-E` - Use regular expressions
- `--json` - Output results as JSON
- `--color` - Colorized output (auto-detected)

### Multi-separator merge (Cross-domain entity tracking)
**Problem:** Documentation uses dots (`api.user.service.md`), source code uses underscores (`api_user_service.rs`). Traditional tools can't unify them.

**Solution:** Use multiple `--sep` flags to merge results across naming conventions:
```bash
# Merge documentation (dots) + source code (underscores)
recur tree main --sep "." --sep "_"

# Show which domain each file comes from (gap analysis)
recur tree main --sep "." --sep "_" --show-sep

# Normalize output to consistent separator
recur files "main.command.**" --sep "." --sep "_" --sep-replace-default "."
```

**Use cases:**
- **Living documentation** — verify code has matching docs/tests
- **Polyglot projects** — navigate TypeScript (`.`) + Python (`_`) + Go (`/`) as one
- **Configuration management** — track configs across file (`.`) and env-var (`_`) conventions
- **Build pipelines** — verify artifact completeness across naming conventions
- **Gap analysis** — `--show-sep` marks each file's origin: `[.]` or `[_]`

See [`docs/main.trait.separator-merge.readme.md`](docs/main.trait.separator-merge.readme.md) for details.

## Commands

### `recur init` — initialize or analyze project config
```bash
recur init                                  # Create .recur/config.toml from detected lanes
recur init --analyze                        # Suggest lane/separator updates
recur init -d ../another-project --analyze  # Analyze a different project root
recur init --force                          # Overwrite existing .recur/config.toml
recur init --analyze --json                 # Machine-readable report
```

```bash
# Typical workflow
recur init
recur files "main_command_**" -d src/ --count   # No --sep needed when .recur/config.toml maps src/ => _
recur init --analyze                             # Re-check lane/separator drift after refactors
```


### `recur trace-id` - trace hierarchical identifier flow
```bash
recur trace-id "ulu.topic.dot.ownership.create" --scope "**" --ext ".cs"
recur trace-id "ulu.topic.dot.ownership.create" --scope "**" --ext ".cs" --json
recur trace-id "ulu.topic.dot.**" --scope "**" --ext ".cs" --json
git diff --name-only origin/main...HEAD | recur trace-id "ulu.topic.dot.**" --scope "**" --stdin --ext ".cs" --json
```

### `recur trait` - manage trait settings in project config
`recur trait` reads/writes trait settings in `.recur/config.toml`.
Run `recur init` first so the config file exists.

```bash
recur trait list
recur trait get traversal_budget.max_depth
recur trait set traversal_budget.max_depth 3
recur trait set traversal_budget.depth_guard clamp
recur trait set trace_id.producer_keywords "\"publish,send,emit,enqueue\""
```

### `recur files` — find files by hierarchical pattern
```bash
recur files "Controller.*"                    # All direct children
recur files "Controller.**"                   # All descendants (recursive)
recur files "*.Tests" --ext .cs              # Test files only
recur files "Module.*" --count               # Show count only
recur files **.AutoSave.** -i -e cs          # No quotes needed with stdin stdout

# Multi-separator merge (cross-domain)
recur files "main.**" --sep "." --sep "_"                        # Merge docs + src
recur files "api.user.**" --sep "." --sep "_" --show-sep         # With markers
recur files "config.**" --sep "." --sep "_" --sep-replace-default "."  # Normalized
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

# Multi-separator merge (cross-domain)
recur tree "main" --sep "." --sep "_"                      # Merge docs + src
recur tree "main" --sep "." --sep "_" --show-sep           # With domain markers
recur tree "api" --sep "." --sep "_" --sep-replace-default "."  # Normalized
```

### `recur merge` — merge results across separators
```bash
# Pattern mode: Merge docs (.) and source (_) into one view
recur merge --pattern "main.command" --sep "." --pattern "main_command" --sep "_"

# Show provenance markers in the merged tree
recur merge --pattern "api.user" --sep "." --pattern "api_user" --sep "_" --show-sep

# Normalize output to dots for display
recur merge --pattern "main.command.tree" --sep "." --pattern "main_command_tree" --sep "_" --sep-replace-default "."

# File mode: merge cached JSON outputs (useful for repeated analysis)
recur tree "main.command" --sep "." --json > main.command.json
recur tree "main_command" --sep "_" --json > main_command.json
recur merge main.command.json --sep "." main_command.json --sep "_" --base "main.command" --show-sep

# Stdin mode: Pure Unix pipe composition (no --json flags needed!)
bash -c "{ recur tree main.command --sep .; recur tree main_command --sep _; }" | \
  recur merge --stdin --base "main.command" --sep . --sep _ --show-sep

# PowerShell stdin mode
@(
  (recur tree main.command --sep . | Out-String),
  (recur tree main_command --sep _ | Out-String)
) -join "`n" | recur merge --stdin --base "main.command" --sep . --sep _ --show-sep

# Single source via stdin (simpler case)
recur tree main --sep . | recur merge --stdin --base "main" --sep .
```

`merge` can read `flatten --json` output (it accepts arrays of objects with a `path` field), but there is a current limitation:
- With dot-separated flattened paths (for example `config.db.host`), merge's tree model treats the last segment as a file extension, so leaf shape is lossy.
- Merge also discards flattened `value`/`kind`; it merges paths only.

Use this workaround today when merging flattened datasets:

```bash
# Use an underscore hierarchy for flatten inputs you plan to merge
recur --sep _ flatten config.xml --json > xml.flat.json
recur --sep _ flatten config.json --json > json.flat.json
recur merge xml.flat.json --sep _ json.flat.json --sep _ --base config
```

```powershell
# PowerShell stdin mode (avoid temp-file encoding issues)
@(
  (recur --sep _ flatten config.xml --json | Out-String),
  (recur --sep _ flatten config.json --json | Out-String)
) -join "`n" | recur merge --stdin --base config --sep _ --sep _
```

See `docs/main.command.merge.readme.md` for merge internals and `docs/main.command.flatten.readme.md` for flatten details.

### `recur flatten` — flatten XML/JSON/TOML/YAML/CSV into hierarchical records
```bash
recur flatten config.xml                          # path = value text output
recur flatten config.json --json                  # JSON array output
recur flatten .recur/config.toml --format toml    # TOML support
recur flatten appsettings.yaml --format yaml      # YAML support
recur flatten levels.csv --format csv             # CSV support
recur flatten config.xml --filter "config.db"     # Prefix filter
recur flatten config.json --max-depth 2           # Depth cap (0 = unlimited)
cat pom.xml | recur flatten --stdin --format xml  # Stdin mode
recur --sep _ flatten config.json --json          # Use custom hierarchy separator
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

## Advanced Composability

**Hold my beer...** 🍺 These examples showcase the full power of Unix pipe composition with multi-source merging, cross-separator unification, and provenance tracking.

### Multi-Domain Gap Analysis
```bash
# Bash: Find docs that exist but have no corresponding source files
bash -c "{
  recur tree api -d docs/ --sep .;
  recur tree api -d src/ --sep _;
}" | \
recur merge --stdin --base api --sep . --sep _ --show-sep | \
grep "\[.\]" | grep -v "\[_\]"  # Docs without src = gap!
```

```powershell
# PowerShell: Same gap analysis
@(
  (recur tree api -d docs/ --sep . | Out-String),
  (recur tree api -d src/ --sep _ | Out-String)
) -join "`n" | recur merge --stdin --base api --sep . --sep _ --show-sep |
  Select-String "\[.\]" | Select-String "\[_\]" -NotMatch  # Docs without src = gap!
```

### Cross-Project Entity Tracking
```bash
# Bash: Merge 3 different naming conventions into unified view
bash -c "{
  recur tree config -d docs/ --sep .;
  recur tree config -d src/ --sep _;
  recur tree config -d scripts/ --sep -;
}" | \
recur merge --stdin --base config --sep . --sep _ --sep - --show-sep | \
tee merged-view.txt | grep "\\[.\\].*\\[_\\].*\\[-\\]"  # Files in all 3 domains!
```

```powershell
# PowerShell: Same 3-way merge
@(
  (recur tree config -d docs/ --sep . | Out-String),
  (recur tree config -d src/ --sep _ | Out-String),
  (recur tree config -d scripts/ --sep - | Out-String)
) -join "`n" | recur merge --stdin --base config --sep . --sep _ --sep - --show-sep |
  Tee-Object merged-view.txt | Select-String "\[\.\].*\[_\].*\[-\]"  # Files in all 3 domains!
```

### DevOps Pipeline Validation
```bash
# Bash: Verify all services have docs, src, and tests
for service in auth billing payments; do
  bash -c "{
    recur tree $service -d docs/ --sep .;
    recur tree $service -d src/ --sep _;
    recur tree $service -d tests/ --sep -;
  }" | \
  recur merge --stdin --base $service --sep . --sep _ --sep - --show-sep | \
  tee "reports/$service-coverage.txt" | \
  grep -c "\\[.\\].*\\[_\\].*\\[-\\]" || echo "$service: incomplete coverage!"
done
```

```powershell
# PowerShell: Same service validation loop
foreach ($service in @('auth', 'billing', 'payments')) {
  @(
    (recur tree $service -d docs/ --sep . | Out-String),
    (recur tree $service -d src/ --sep _ | Out-String),
    (recur tree $service -d tests/ --sep - | Out-String)
  ) -join "`n" | recur merge --stdin --base $service --sep . --sep _ --sep - --show-sep |
    Tee-Object "reports\$service-coverage.txt" |
    Select-String "\[\.\].*\[_\].*\[-\]" |
    Measure-Object | Select-Object -ExpandProperty Count |
    ForEach-Object { if ($_ -eq 0) { "$service: incomplete coverage!" } }
}
```

### Configuration Drift Detection
```bash
# Bash: Compare expected vs deployed configs, find drift
bash -c "{
  recur tree app.config -d expected/ --sep .;
  recur tree app.config -d deployed/ --sep _;
}" | \
recur merge --stdin --base app.config --sep . --sep _ --show-sep | \
grep "\[.\]" | grep -v "\[_\]" | \
awk '{print "DRIFT: "$0}' | tee config-drift-report.txt
```

```powershell
# PowerShell: Same drift detection
@(
  (recur tree app.config -d expected/ --sep . | Out-String),
  (recur tree app.config -d deployed/ --sep _ | Out-String)
) -join "`n" | recur merge --stdin --base app.config --sep . --sep _ --show-sep |
  Select-String "\[.\]" | Select-String "\[_\]" -NotMatch |
  ForEach-Object { "DRIFT: $_" } | Tee-Object config-drift-report.txt
```

### Polyglot Codebase Navigation
```bash
# Bash: Merge TypeScript (.), Python (_), Go (/), Rust (-) into one hierarchy
bash -c "{
  recur tree api -d typescript/ --sep .;
  recur tree api -d python/ --sep _;
  recur tree api -d go/ --sep /;
  recur tree api -d rust/ --sep -;
}" | \
recur merge --stdin --base api --sep . --sep _ --sep / --sep - --show-sep | \
jq '.files[] | select(.path | contains("auth"))' | \
less  # Unified cross-language view!
```

```powershell
# PowerShell: Same 4-language merge
@(
  (recur tree api -d typescript/ --sep . | Out-String),
  (recur tree api -d python/ --sep _ | Out-String),
  (recur tree api -d go/ --sep / | Out-String),
  (recur tree api -d rust/ --sep - | Out-String)
) -join "`n" | recur merge --stdin --base api --sep . --sep _ --sep / --sep - --show-sep |
  jq '.files[] | select(.path | contains(\"auth\"))' | more  # Unified cross-language view!
```

### Continuous Integration Workflow
```bash
# Bash: Cache expensive tree operations, merge on-demand
recur tree main.command -d docs/ --sep . > docs.json
recur tree main_command -d src/ --sep _ > src.json
recur tree main-command -d tests/ --sep - > tests.json

# Fast merge from cached JSON (no re-scanning!)
recur merge docs.json src.json tests.json \
  --base "main.command" --sep . --sep _ --sep - \
  --show-sep | tee ci-report.txt | \
  grep -c "\\[.\\].*\\[_\\].*\\[-\\]" > coverage-count.txt
```

```powershell
# PowerShell: Same CI workflow (cross-platform!)
recur tree main.command -d docs/ --sep . > docs.json
recur tree main_command -d src/ --sep _ > src.json
recur tree main-command -d tests/ --sep - > tests.json

# Fast merge from cached JSON (no re-scanning!)
recur merge docs.json src.json tests.json `
  --base "main.command" --sep . --sep _ --sep - `
  --show-sep | Tee-Object ci-report.txt |
  Select-String "\[\.\].*\[_\].*\[-\]" |
  Measure-Object | Select-Object -ExpandProperty Count > coverage-count.txt
```

**Performance tip:** Use file mode (cached JSON) for repeated analysis, stdin mode for dynamic exploration.

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
| **JSON output for piping** | ❌ | ❌ | ✅ |
| **Multi-source merge** | ❌ | ❌ | ✅ |
| **Cross-separator merge** | ❌ | ❌ | ✅ |
| **Stdin mode (pipe composition)** | ❌ | ❌ | ✅ |
| **Auto-JSON detection** | ❌ | ❌ | ✅ |
| **Provenance tracking** | ❌ | ❌ | ✅ |

**Note:** `recur` does not replace `rg` or `grep`—it complements them. For raw text-search throughput, `ripgrep` is hard to beat. `recur` focuses on a different (and increasingly common) problem: *working with structure encoded in names*.

**Unique to recur:** Full Unix pipe composability with stdin mode, multi-source merging, and automatic JSON detection. Example: `recur tree docs --sep . | recur merge --stdin --base config --sep .` — no manual JSON flags needed, pure pipe workflows.

## The Tribute

> *"Program Structure and Computational Complexity"*  
> — Dennis Ritchie, PhD Thesis, Harvard, 1968

Ritchie's thesis explored **recursive functions and hierarchical program structures**. Those ideas later shaped Unix and C. `recur` borrows a small part of that spirit by treating structure as something worth seeing directly.

The tribute is also personal. I had the chance to speak with Dennis Ritchie briefly around 2004 or 2005, and what stayed with me was how generous and soft-spoken he was.

## Why the name “recur”?

1. **Recursive** — searches hierarchies recursively  
2. **Recur** — short and memorable, in the tradition of `grep`, `awk`, and `sed`  
3. **Ritchie** — a quiet nod to Dennis Ritchie's early work on program structure  
4. **Unix philosophy** — aim for a focused tool that composes well

## Documentation

- [Full Tribute](RECUR-TRIBUTE.md) — the Dennis Ritchie note  
- [White Paper](docs/recur.white.paper.docx) — design and philosophy
- [Proposal](RECUR-PROPOSAL.md) — technical design  
- [Contributing](CONTRIBUTING.md) — how to contribute  
- [Implementation](IMPLEMENTATION-COMPLETE.md) — code walkthrough  
- [Recur Expert Playbook](julia-expert/references/recur-playbook.md) — canonical agent workflow guide
- [Init Command Notes](docs/main.command.init.readme.md) — project-aware `.recur/config.toml` bootstrap
- [Flatten Command Notes](docs/main.command.flatten.readme.md) — structured data flattening and merge interop caveats

`AGENT.PROMPT.recur-expert.md` and `docs/AGENT.PROMPT.recur-expert.md` are pointer files only.

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
cargo run --profile release-safe --bin recur -- tree "main" -d src
cargo install --path . --profile release-safe --locked --force --offline
```

## License

MIT — permissive, simple, and broadly compatible.

## Acknowledgments

`recur` is possible because of the tools and ideas that came before:

- **Dennis M. Ritchie (1941–2011)** — for foundational work on recursion, program structure, and the systems ideas underneath so much of modern computing  
- **Ken Thompson** — for `grep` and the standard it set for developer tooling  
- **Andrew Gallant (BurntSushi)** — for `ripgrep` and modern Rust CLI excellence  
- **The Unix philosophy** — “Do one thing well”  
- **The Rust community** — for the ecosystem that makes tools like this practical  

---

**recur**: *Hierarchy-aware search for the 21st century—built for modern naming conventions, inspired by foundational ideas in program structure.*

## Demo

![skippy & Joe's Bar](demos/ascii-drinks/bar.svg)
**Try it locally:** `powershell -ExecutionPolicy Bypass -File demos/ascii-drinks/bar.ps1`
for the interactive version, or see [`demos/ascii-drinks/`](demos/ascii-drinks/) for the full demo.
