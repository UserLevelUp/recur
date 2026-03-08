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
```

## JSON Output Shape

```json
{
  "identifier": "my.event.id",
  "define": [
    { "path": "src/Topics.cs", "line_number": 4, "line": "  const string MyEvent = \"my.event.id\";" }
  ],
  "produce": [
    { "path": "src/Publisher.cs", "line_number": 12, "line": "  await bus.PublishAsync(Topics.MyEvent);" }
  ],
  "consume": [
    { "path": "src/Subscriber.cs", "line_number": 8, "line": "  channel.QueueBind(\"q\", \"x\", routingKey: Topics.MyEvent);" }
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

Note: `edge_type` field on each site is pending (see `main.command.trace-id.edge-type.todo.current.md`).

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

`trace-id --json` output pipes into `recur merge`:

```bash
recur trace-id "my.event.id" --scope "**" --json | recur merge --stdin --base trace --sep .
```

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
- `docs/main.command.trace-id.edge-type.todo.current.md` — edge_type pending
- `docs/main.demo.sudoku.trace-id.todo.current.md` — Sudoku demo context
