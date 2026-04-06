# main.command.trace-id.readme

Command reference for `recur trace-id`.

## What It Does

`trace-id` traces a hierarchical identifier through a codebase (or any text
files) and classifies every occurrence into one of four roles:

| Role | Meaning | Example keywords |
|---|---|---|
| `define` | Where the identifier is declared or assigned | `const`, `=`, `static readonly` |
| `produce` | Where the identifier's value is emitted/sent | `publish`, `send`, `emit` |
| `consume` | Where the identifier is received/bound | `subscribe`, `queuebind`, `routingkey` |
| `trigger` | Where the identifier causes a side-effect | `trigger`, `register`, `solve` |

recur does not know what the identifier means. It sees keywords near the
identifier and classifies the line. The vocabulary is configurable via traits.

## Usage

```bash
recur trace-id "<identifier>" [flags]
```

## Flags

| Flag | Description | Default |
|---|---|---|
| `--scope <GLOB>` | Hierarchical glob to restrict file set | required |
| `--sep <CHAR>` | Separator character (`.` `_` `-`) | `.` |
| `--ext <EXT>` | File extension filter (`.cs`, `.rs`, `.jl`) | all |
| `--format <FMT>` | Output style: `summary` or `full` | `summary` |
| `--json` | Machine-readable JSON output | false |
| `--stdin` | Read file list from stdin pipe | false |
| `--depth <N>` | Max traversal depth | 2 |
| `--depth-guard <POLICY>` | On depth cap hit: `hard-fail`, `clamp`, `warn` | `hard-fail` |
| `--force` | Bypass depth cap | false |
| `--save-run` | Persist latest JSON artifact under `.recur/trace-id/runs/<name>/` | false |
| `--reuse-if-fresh` | Reuse saved JSON when query/config/input files still match | false |
| `--check-run` | Report whether a saved run is `fresh`, `stale`, or `missing` | false |
| `--run-name <NAME>` | Stable name for saved run artifacts | none |
| `-d <DIR>` | Working directory | `.` |

## Examples

```bash
# Basic: find all roles of an identifier
recur trace-id "my.event.id" --scope "**"

# Full output: show every site
recur trace-id "my.event.id" --scope "**" --format full

# JSON: machine-readable for downstream tools
recur trace-id "my.event.id" --scope "**" --json

# Scope to a subtree only
recur trace-id "my.event.id" --scope "my.service.**"

# Glob: trace a whole identifier family at once
recur trace-id "my.event.**" --scope "**"

# Depth control: how far does flow expand?
recur trace-id "my.event.id" --scope "**" --depth 1
recur trace-id "my.event.id" --scope "**" --depth 5

# Depth guardrail: clamp instead of fail on deep graphs
recur trace-id "my.event.id" --scope "**" --depth 5 --depth-guard clamp

# Force: bypass cap for a full trace
recur trace-id "my.event.id" --scope "**" --depth 9 --force

# Stdin: limit file set via pipe
cat changed-files.txt | recur trace-id "my.event.id" --scope "**" --stdin

# Custom separator (underscore convention)
recur trace-id "my_event_id" --scope "**" --sep _

# Save a reusable run artifact
recur trace-id "my.event.id" --scope "**" --json --save-run --run-name my.event.primary

# Reuse the saved run if nothing relevant changed
recur trace-id "my.event.id" --scope "**" --json --reuse-if-fresh --run-name my.event.primary

# Check freshness only
recur trace-id "my.event.id" --scope "**" --check-run --run-name my.event.primary
```

## JSON Output Shape

```json
{
  "identifier": "my.event.id",
  "define": [
    { "path": "src/Topics.cs", "line_number": 4, "line": "  const string MyEvent = \"my.event.id\";", "edge_type": "define" }
  ],
  "produce": [
    { "path": "src/Publisher.cs", "line_number": 12, "line": "  await bus.PublishAsync(Topics.MyEvent);", "edge_type": "produce" }
  ],
  "consume": [
    { "path": "src/Subscriber.cs", "line_number": 8, "line": "  channel.QueueBind(\"q\", \"x\", routingKey: Topics.MyEvent);", "edge_type": "consume" }
  ],
  "trigger": [],
  "request": {
    "pattern": "my.event.id",
    "scope": "**",
    "depth_requested": 2,
    "depth_effective": 2,
    "depth_cap": 5,
    "depth_guard": "hard-fail",
    "force": false,
    "format": "summary",
    "json": true
  }
}
```

Each site object includes `edge_type`: `define`, `produce`, `consume`, or `trigger`.

## Trait Configuration

Keywords for each role are configurable per project via `.recur/config.toml`:

```toml
[traits.trace_id]
producer_keywords = "publish,send,emit"
consumer_keywords = "subscribe,queuebind,routingkey,bind,consume"
trigger_keywords = "trigger,register,solve"
```

Tune at runtime:
```bash
recur trait set trace_id.producer_keywords "publish,emit,propagate"
recur trait set trace_id.consumer_keywords "subscribe,bind,consume"
recur trait set trace_id.trigger_keywords "trigger,register,solve"
```

## Pipeline

`trace-id --json` output pipes into `recur merge`, and merge JSON retains `edge_type`
on leaf nodes:

```bash
recur trace-id "my.event.id" --scope "**" --json | recur merge --stdin --base trace --sep . --json
```

## Saved Runs

`trace-id` can persist metadata-only run artifacts under `.recur/trace-id/runs/<name>/`.

Files:

- `manifest.toml` - query identity, config fingerprint, file fingerprint, file count
- `latest.json` - last saved `trace-id` JSON result

Current persistence is `latest`-only: saved runs keep `manifest.toml` plus
`latest.json`, and do not create timestamped `history/` artifacts yet.

Freshness is based on:

- query shape (`pattern`, `scope`, separator, depth, stdin/ext flags)
- nearest `.recur/config.toml` content when present
- scoped file set metadata (path, size, modified time)

This keeps the feature inside recur's metadata boundary: it stores the discovered
trace result and freshness signals, not duplicated source files.

Examples:

```bash
# Save a run
recur trace-id "my.event.id" --scope "**" --json --save-run --run-name my.event.primary

# Reuse only when fresh; otherwise fall back to a live trace
recur trace-id "my.event.id" --scope "**" --json --reuse-if-fresh --run-name my.event.primary

# Inspect freshness for scripting or eventness workflows
recur trace-id "my.event.id" --scope "**" --check-run --run-name my.event.primary --json
```

## Transition Audit Pattern

`trace-id` is a strong fit for auditable eventness transitions when the same
identifier is carried through forward and backward state changes.

Pattern:

- canonical eventness or mirror state remains the source of truth
- transition files carry the shared identifier as reusable evidence
- `--save-run` captures one semantic snapshot of those transition files
- `--check-run` turns `stale` when the transition evidence changes
- rerunning `trace-id` refreshes the saved evidence for the new state

Example eventness lines:

```text
transition.audit.order.42 = todo.current
transition.audit.order.42 publish review.current
review.current subscribe transition.audit.order.42
transition.audit.order.42 trigger advance
transition.audit.order.42 publish todo.current
todo.current subscribe transition.audit.order.42
transition.audit.order.42 trigger rollback
```

That pattern does not make the saved run canonical by itself. It makes the saved
run auditable evidence that the semantic transition path can be rediscovered and
rechecked over time.

## Works On Any Text Files

`trace-id` is not a code parser. It matches the identifier string in any text
file and classifies lines by keyword proximity. This means it works equally on:

- Source code (`.cs`, `.rs`, `.jl`, `.ts`)
- Eventness files (`sudoku.flow.r3c5`, `sudoku.solution`)
- Configuration files (`.toml`, `.json`, `.yaml`)
- Documentation (`.md`)
- Any file where identifiers appear as text

## Sudoku Demo Context

In a Sudoku game, the game engine writes eventness files like:

```
sudoku.r3.c5 = 7
sudoku.r3.c1 publish 7
sudoku.r7.c5 subscribe sudoku.r3.c5
sudoku.r7.c5 trigger solve
```

Then:
```bash
# Configure Sudoku vocabulary
recur trait set trace_id.producer_keywords "publish"
recur trait set trace_id.consumer_keywords "subscribe"
recur trait set trace_id.trigger_keywords "trigger"

# Trace what happens when 7 is placed at r3.c5
recur trace-id "sudoku.r3.c5" --scope "sudoku.**" --format full -d games/active/

# Machine-readable for Julia to parse and visualize
recur trace-id "sudoku.r3.c5" --scope "sudoku.**" --json -d games/active/

# Trace the whole row at once
recur trace-id "sudoku.row.3.**" --scope "sudoku.**" -d games/active/
```

recur does not know it is playing Sudoku.

## References

- `src/main_command_trace_id_impl.rs` — implementation
- `src/trait/trace_id.rs` — trait policy resolver
- `docs/main.command.trait.readme.md` — trait configuration
- `docs/main.command.trace-id.edge-type.complete.md` — edge_type field record
- `docs/main.demo.sudoku.trace-id.todo.current.md` — Sudoku demo context
