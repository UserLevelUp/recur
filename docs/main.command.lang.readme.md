# recur lang

Status: `language design contract`
Date: 2026-07-23

`recur lang` is the pure query surface for Recur language sources, symbols,
contracts, lanes, slices, Warps, and runtime Eventness.

`recur-lang` is the companion coordination implementation. It parses
coordination programs, maintains lane state, validates external receipts, and
writes ACK/NAK state that core Recur can inspect. The current Julia algorithm
interpreter remains a design spike rather than a requirement that the
production companion compile arbitrary target languages.

## Core split

```text
recur lang   = query, validate, expand, contract, trace, explain, exit
recur-lang   = coordinate, await Eventness, validate receipts, write ACK/NAK
```

This follows the repository-wide companion rule:

```text
recur <topic>        = pure query / inspection / explanation
recur-<topic>        = opinionated runner / writer / async actor
```

Expansion and contraction in `recur lang` are read-only views over one
canonical parsed model. They do not rewrite source files.

## Relationship to `recur trace-id`

These surfaces complement one another rather than duplicate one another:

| Surface | Primary question | World model |
|---|---|---|
| `recur trace-id` | Where does an identifier appear, and which relationships were declared near it? | Open-world repository scan |
| `recur lang` | Is this coordination program valid, and what does it mean? | Closed-world formal model |
| `recur-lang` | How should this valid model advance through its lanes? | Stateful coordination actor |

`recur trace-id` remains language-independent. It finds identifier lineage
across source, documentation, tests, Eventness, and receipts without needing
to understand their complete semantics. An absent match is not necessarily an
error.

`recur lang` parses a declared coordination boundary. It understands exact
`i(...)` and `o(...)` contract identity, block dependencies, joins, waits,
bounded feedback, lane write scopes, and required receipts. Within that
closed-world boundary, a missing producer, mismatched input, invalid join, or
undeclared feedback edge can be a validation error.

Recur Lang should give blocks, contracts, lanes, work orders, and receipts
stable trace IDs. `trace-id` can then follow those IDs through implementation
files that are intentionally outside the Recur Lang parser:

```text
recur lang show game.path-monkey.f
recur trace-id "game.pathing.route" --scope "**" -d .
```

The first query explains one formal block and its contract. The second finds
the associated specification, target-language code, fixtures, tests, reviews,
and receipts across the repository. A future `recur lang trace` command would
trace formal edges inside the parsed model; it would not replace the broader
textual lineage scan.

## Proposed query commands

```powershell
recur lang list
recur lang list --filter "main.lang.**"
recur lang show gcd.f
recur lang show game.pathing@1
recur lang show "merge.i(b)" --json
recur lang contract AlgorithmLab
recur lang expand gcd.f
recur lang expand game.pathing@1
recur lang inputs merge.f
recur lang outputs bubble.f
recur lang refs "bubble.o(b)"
recur lang refs "game.pathing@1"
recur lang lanes AlgorithmLab
recur lang warps AlgorithmLab
recur lang check demos/main.lang/main.lang.algorithm-lab.recur
recur lang status algorithm-lab
recur lang explain algorithm-lab
```

The query surface may parse and validate source, but it must not execute a
function, schedule a lane, update Eventness, or write an artifact.

## Proposed language-runner commands

The shipped bounded Warp command is:

```powershell
recur-lang warp demos/main.lang/main.lang.algorithm-lab.recur gcd --json

recur-lang warp <source> <scope> `
  --dir <project-root> `
  --eventness <exact-E0-file> `
  --receipt <external-receipt> `
  --confirm
```

The first form is always a dry run. It validates the scope's compact function,
body flow, Warp, and final `state` event, then reports the exact transition and
an FNV-1a content hash of the parsed source. The hash detects stale receipts;
it is not a signature or a trust proof.

The confirmed form accepts only an exact file path beneath `--dir`. Its file
stem must equal the declared E0 identity. The external receipt must use this
shape:

```toml
schema = "recur-lang-warp-receipt-v1"
scope = "gcd"
current = "demo.algorithm.gcd.todo.current"
slice = "gcd.f"
desired = "demo.algorithm.gcd.complete"
source_hash = "fnv1a64:..."
ack = "accepted"
attempt = 1
artifact = "commit:abc123"
test_receipt = "ci:test-42"
```

After validation, `recur-lang` renames only that E0 artifact to the declared Ef
name in the same directory. It then writes
`.recur/lang/recur-lang.<id>.status.current.md` using
`recur-lang-warp-status-v1`. A stale or rejected receipt writes NAK and leaves
the E0 artifact unchanged.

The broader runner commands remain proposed:

```powershell
recur-lang coordinate demos/main.lang/main.lang.skippy-watch-coordination.recur
recur-lang assign csharp-monkey
recur-lang accept-receipt .recur/runs/change-104/lanes/csharp_monkey/worker.csharp-monkey.001.receipt.current.md
recur-lang status change-104
```

Those four commands are proposed Improvement 30 shapes, not shipped commands.

The current Julia spike separately demonstrates parser and algorithm behavior:

```powershell
julia --startup-file=no demos/main.lang/main.lang.cli.jl list
julia --startup-file=no demos/main.lang/main.lang.cli.jl run all
```

That Julia surface supplies parser, contract, and reference-algorithm evidence.
It does not commit the eventual `recur-lang` companion to an embedded compiler
or target-language runtime.

## Responsibility boundary

### `recur lang`

- discover language files through hierarchy;
- list scopes, symbols, bundle contracts, lanes, slices, and Warps;
- render compact or expanded views;
- contract an accepted subsystem into one versioned public block and preserve
  exact drill-down into its internal model;
- verify that a parent imports the exact child contract version or content hash
  associated with the child's acceptance receipt;
- verify that `o(b)` and `i(b)` resolve to one exact contract;
- show fan-out and fan-in references;
- inspect source spans, event identifiers, and saved runtime state;
- explain ACK and NAK results;
- exit without changing project state.

### `recur-lang`

- plan and confirm one exact `E0 -> dE -> Ef` file transition;
- require explicit confirmation and a source-hash-bound external receipt;
- keep the action inside one declared project root and one named E0 artifact;
- parse the coordination model and resolve exact contract references;
- maintain synchronous and asynchronous lane state;
- await filesystem Eventness and declared fan-in joins;
- validate changed-file lists and required external receipts;
- publish child integration readiness and record each parent's acceptance as
  separate Eventness facts;
- emit consume, trigger, produce, and state events;
- write coordination receipts and ACK/NAK Eventness;
- enforce declared attempt, cancellation, and failure policy;
- leave compilers, linters, test runners, and Git operations to explicitly
  declared external workers.

`recur-watch` may provide the blocking filesystem subscription used by a lane
controller. Core `recur watch` only inspects watcher status. The formal
watch/work protocol is captured in
`docs/main.improvement.30.contract.watch-coordination-v0.todo.future-plan.md`.

## Runtime state

The companion should write small records beneath:

```text
.recur/lang/recur-lang.<id>.status.current.md
```

The shipped Warp status shape extends the suggested fields with schema,
source hash, exact before/after evidence, external artifact/test references,
attempt, and timestamps:

```text
schema = "recur-lang-warp-status-v1"
id = "gcd-001"
language = "main.lang"
source = "demos/main.lang/main.lang.algorithm-lab.recur"
source_hash = "fnv1a64:..."
state = "complete"
ack = "accepted"
nak_reason = ""
scope = "gcd"
warp = "gcd"
lane = "gcd"
current = "demo.algorithm.gcd.todo.current"
slice = "gcd.f"
desired = "demo.algorithm.gcd.complete"
before_evidence = "docs/demo.algorithm.gcd.todo.current.md"
after_evidence = "docs/demo.algorithm.gcd.complete.md"
receipt = "receipts/gcd.001.md"
artifact = "commit:abc123"
test_receipt = "ci:test-42"
attempt = 1
started_at = "unix:1784899200"
completed_at = "unix:1784899201"
status_receipt = ".recur/lang/recur-lang.gcd-001.status.current.md"
```

Rejected or partial runs use the same record:

```text
state = stopped
ack = rejected
nak_reason = "merge.i(b) does not match bubble.o(b)"
```

Core `recur lang status algorithm-lab` reads these records and explains the
result. It never repairs or reruns them.

## Related

- `docs/main.lang.readme.md`
- `docs/main.command.trace-id.readme.md`
- `docs/main.recur.purity.decision.md`
- `docs/main.command.watch.readme.md`
- `docs/main.command.version.readme.md`
- `recur_language_start.md`
