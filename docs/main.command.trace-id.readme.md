# main.command.trace-id.readme

Command reference for `recur trace-id`.

## What It Does

Use `trace-id PATTERN -d ROOT` to locate matching folders and files. Supply
`--scope` to inspect relationships in the chosen file hierarchy. The existing
scoped mode classifies heuristic occurrences into four roles:

| Role | Meaning | Example keywords |
|---|---|---|
| `define` | Where the identifier is declared or assigned | `const`, `=`, `static readonly` |
| `produce` | Where the identifier's value is emitted/sent | `publish`, `send`, `emit` |
| `consume` | Where the identifier is received/bound | `subscribe`, `queuebind`, `routingkey` |
| `trigger` | Where the identifier causes a side-effect | `trigger`, `register`, `solve` |

recur does not know what the identifier means. It sees keywords near the
identifier and classifies the line. The vocabulary is configurable via traits.

## Boundary With Recur Lang

`recur trace-id`, `recur lang`, and `recur-lang` answer different questions:

| Surface | Primary question |
|---|---|
| `recur trace-id` | Where does this identifier appear, and which relationships were declared near it? |
| `recur lang` | Is this formal coordination program valid, and what does it mean? |
| `recur-lang` | How should a valid coordination program advance through its lanes? |

`trace-id` is an open-world, language-independent repository scan. It can
connect a durable identifier across Markdown, Rust, C#, Julia, tests, receipts,
and Eventness without parsing those artifacts as one programming language. A
missing occurrence is not automatically an error because other relationships
may exist outside the selected directory, scope, or extensions.

Recur Lang is a closed-world coordination model. Its parser knows about exact
`i(...)` and `o(...)` contract identities, blocks, lanes, joins, waits, bounded
feedback, write scopes, and required receipts. A missing producer, incompatible
input, invalid join, undeclared feedback edge, or absent required receipt can
therefore invalidate the model.

For example:

```text
defines: game.pathing.route A-star route contract
consumes: game.pathing.route enemy movement lane
publish: game.pathing.route.receipt accepted pathing verification
consumer: game.pathing.route accepted route consumption
trigger: game.pathing.route coordinator releases integration lane
```

`recur trace-id "game.pathing.route"` can find that lineage across the project,
but it does not decide whether the route contract matches a lane input or
whether the lanes can deadlock. The proposed `recur lang` surface performs
those formal checks. Recur Lang artifacts should publish stable trace IDs so
`trace-id` can discover their lineage outside the parsed coordination model.

`recur trace-id` is implemented. Core `recur lang` now provides WIR1/CIR1
queries and bounded static checks; `recur-lang` provides companion planning
and checked transitions. See `main.command.lang.query.readme.md` and
`main.command.lang.plan.readme.md` for their current coverage and limits.

## Usage

```bash
recur trace-id "<identifier>" -d <root>
recur trace-id "<identifier>" --scope <scope> -d <root>
```

## Path discovery and explicit declarations

Without `--scope`, the command prints deduplicated matching folders and file
paths. `--json` uses `recur-trace-id-paths-v1` with `folders`, `files` and
`coverage`; there are no line bodies or role lists. An explicit `--scope "**"`
still requests legacy detail. A complete path search with no hits exits 1;
an incomplete search reports omissions, including when no hit was found.

Path, declared and explicit inspection modes bound traversal and bytes. Defaults are 50,000 entries
and 32 MiB total; `--max-entries` and `--max-bytes` may select a smaller or
larger finite budget up to 250,000 entries and 128 MiB. Hidden entries and common
build output directories are excluded. `--ext` and the configured content
search policy also apply. Links are not traversed, and reads must stay in `-d`.
Unreadable, oversized and non-UTF-8 inputs can make coverage incomplete.
Binary samples and artifact digest reads share that byte allowance. Depth,
format, stdin and saved-run options remain for legacy scoped queries.

To inspect exact declarations, add `--declared` to a scoped query. It reads an
optional metadata block on the first line of a Markdown file, with exact
delimiters and one record per line. For example:

```markdown
<!-- recur-trace-v1
defines: capability.baseline
publish: capability.baseline
artifact: {"id":"capability.baseline","context":"dock","revision":"R04","path":"r04.txt","sha256":"<64 lowercase hex digits>"}
requires: {"id":"capability.baseline","context":"dock","expected":"R06","actual":"R04"}
-->
```

`defines`, `publish`, `consumer` and `trigger` accept a single dotted ID.
Segments contain ASCII letters, digits or underscores; invalid tokens get a
diagnostic with a source span. Prose, fenced examples and generated source
literals remain searchable through the other modes. They do not become
declarations. `--unique-defines` opts into checking duplicate `defines` entries
in the chosen scope. Filtering can ignore case; authored identities used for
binding stay case-sensitive.

The `recur-trace-id-declarations-v1` report contains declarations, artifacts,
pins, diagnostics and coverage. Its source spans use one-based lines and
one-based, end-exclusive byte columns. Artifact bindings use exact
`(id, context, revision)` tuples from the selected scope. Paths are root-relative
and SHA-256 checks are bounded to 4 MiB per artifact and the remaining aggregate
byte budget. The report distinguishes matching bytes, the expected revision,
and missing, ambiguous or unavailable inputs. In the example, a matching R04
digest can coexist with an R06 revision mismatch. Successful report generation
does not mean the reported pins passed. This mode does not assess tests or accept
Warp gates.

## ID sets and selected context

First locate files, then explicitly inspect the observed IDs or nearby text:

```powershell
recur trace-id 'capability.**' -d ROOT
recur trace-id 'capability.**' -d ROOT --scope '**' --list-ids
recur trace-id 'capability.**' -d ROOT --scope 'lab.**' --context 3 --select capability.baseline --select capability.delivery
```

`--list-ids` returns stable distinct tokens, their parent namespace, matching files,
file counts and occurrence counts. JSON uses `recur-trace-id-inventory-v1` with
`ids`. Repeated occurrences count separately but files are deduplicated. Tokens
in prose, filenames or generated source literals are occurrences, not inferred
declarations; both new views explicitly report `provenance: token-occurrences`.

`--context N` returns matching lines and N neighboring lines (0 through 20).
JSON uses `recur-trace-id-context-v1` with `excerpts`: each has `path`, one-based
`start_line`/`end_line`, selected `ids`, and the original `lines` without line
terminators. Overlapping or adjacent windows merge within each file. Text output
prints the same passages with paths and line numbers. This is extracted context,
not a semantic summary; instructions found inside a file are never executed.

Repeat `--select ID_OR_PATTERN` to form a union within the primary pattern.
No selector means all primary matches. Existing hierarchy matching applies:
`capability.baseline` is exact, while `capability.baseline.**` also includes that
ID and its descendants. Up to 64 selectors are supported. Case-insensitive
filtering preserves the original spelling. `-d`, `--scope`, `--ext` and scan
policy still bound which files are read. No project configuration is required.

Both views require scope and cannot be combined with each other or `--declared`.
`--max-results` defaults to 200 (maximum 10,000 ID entries or excerpts).
`--max-output-bytes` defaults to 32 KiB (maximum 1 MiB) and bounds the sum of
compact serialized result-item sizes; the surrounding report and presentation
overhead are additional. Whole items that do not fit are omitted, so Unicode
and source lines are not cut mid-character. Selection and output-limit flags
require one of these views.

JSON separates scan `coverage` from `output` coverage; top-level `complete`
requires both. `output.returned_bytes` measures the serialized returned items.
Its omission counts describe skipped candidate updates/windows, not unique
missing IDs. Counts apply to returned data and may be lower bounds when
coverage is incomplete. Limited results are selected deterministically during
file traversal, then inventory IDs are sorted; they are not a global ranking.
A complete empty view exits 1. Incomplete results remain readable (exit 0),
with diagnostics on stderr in text mode. Existing scoped role queries keep
their legacy JSON and behavior.

## Companion inspection preferences

`recur-trace-id init -d ROOT --dry-run --json` previews the missing preferences.
Omit `--dry-run` to add them to the nearest `.recur/config.toml`. In a fresh root
it creates that config. Existing settings, comments, and trait policy are
preserved. Use `cargo install --path . --locked --bin recur-trace-id` from the
Recur source checkout when installing this companion from source.

```toml
[recur-trace-id]
scope = "**"
extensions = ".md,.recur,.jl,.rs"

[recur-trace-id.inventory]
max_results = 100
max_output_bytes = 16384

[recur-trace-id.context]
lines = 2
max_results = 20
max_output_bytes = 8192
```

```powershell
recur-trace-id inventory 'demo.blackjack.web.split.**' -d C:/src/recur/demos/blackjack-web/split --json
recur-trace-id context 'demo.blackjack.web.split.**' -d C:/src/recur/demos/blackjack-web/split --select 'demo.blackjack.web.split.contract'
```

The companion passes the selected preferences to the existing `recur trace-id`
inventory and context views. Override with `--scope`, `--ext`, `--max-results`,
`--max-output-bytes`, or `--lines` for context. The core query keeps its own CLI
defaults, output schemas, and shared trait policy. See the [configuration guide](main.command.config.readme.md)
for the other companions.

## Flags

| Flag | Description | Default |
|---|---|---|
| `--scope <GLOB>` | Hierarchical file scope for relationship detail | optional; paths only if omitted |
| `--declared` | Read explicit Markdown declarations and revision pins | false; requires scope |
| `--list-ids` | Distinct observed IDs, files and counts | false; requires scope |
| `--context <N>` | Source excerpts with N neighboring lines | absent; requires scope |
| `--select <PATTERN>` | Repeatable selection within the primary pattern | all primary matches |
| `--max-results <N>` | Maximum inventory entries or excerpts | 200 |
| `--max-output-bytes <N>` | Maximum serialized result-item bytes | 32768 |
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

## Evidence Policy

Saved runs are reusable derived evidence, not canonical eventness. The stable
policy is `latest` only and metadata freshness only (query shape, config text,
and scoped path/size/modified-time records). Timestamped history and content
hashing are deferred rather than implied by the current flags.

A consumer such as a future `recur warp` may use a run only when
`--check-run` reports `fresh`. A `stale` or `missing` run is a visible residual:
the consumer must request a live trace or report the evidence gap, never score
the cached payload as current truth.

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
- Persona/protocol files (`skippy.case.separator.correction`)
- Configuration files (`.toml`, `.json`, `.yaml`)
- Documentation (`.md`)
- Any file where identifiers appear as text

## Adaptive Comms Demo Context

The Skippy adaptive-comms demo uses plain `.txt` protocol files to encode:

- active relationship phase
- cue-specific case identifiers
- `publish` lines for jab-family, boast, and lament recommendations
- `subscribe` lines for downstream selections
- `trigger` lines for tone activation

That keeps the selection logic auditable in text while leaving final response
synthesis with the LLM.

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
- `docs/main.command.lang.readme.md` — proposed formal coordination query
- `docs/main.command.trace-id.edge-type.complete.md` — edge_type field record
- `docs/main.demo.sudoku.trace-id.todo.current.md` — Sudoku demo context
