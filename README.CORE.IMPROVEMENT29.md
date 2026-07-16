RECUR IMPROVEMENT 29
recur-reveal next Orientation Packet
====================================
Date: July 3, 2026
Status: Proposal / future direction
Author: Captured from BGE/kicad eventness orchestration work

INTENT
------
Add a first-class future companion executable for the thing reveal is growing
toward: producing the next coherent state of attention for a human or agent,
with any command execution kept out of core `recur reveal`.

The preferred command shape is:

```text
recur-reveal next
recur-reveal next <domain-or-lane>
recur-reveal next <domain-or-lane> --json
```

The boundary is:

```text
recur reveal        = pure capsule listing/showing inside core recur
recur-reveal next   = companion orientation packet and optional ACK/NAK runner
```

This follows the same split already used by the project:

```text
recur <topic>       = pure query / inspection / explanation
recur-<topic>       = opinionated runner / writer / async actor
```

WHY `recur-reveal next`
-----------------------
The `next` command should live in the `recur-reveal` executable, not under core
`recur reveal`.

Core `recur reveal` should stay boring and pure: list reveal capsules, show one
capsule, print context, exit. It should not grow into an agent runner or a
command dispatcher.

The `recur-reveal` companion can own the higher-level "next" behavior:

- compose persona / role / agenda from files;
- inspect north-star, active index, paused lanes, trace edges, and receipts;
- print the next orientation packet;
- optionally execute declared packet fields such as `pull.first` or `verify`
  only with explicit policy / confirmation;
- write ACK/NAK state that core recur can inspect later.

This keeps the mental model crisp:

```text
recur reveal      = show me the capsule
recur-reveal next = tell me the next safe pull, and optionally run declared work
```

PROBLEM
-------
Current reveal capsules are good at lane-local rehydration, but agents still
need to stitch together several surfaces by hand:

- `.recur/config.toml` reveal policy;
- root `.recur-*` capability cards;
- lane-local `*.recur.md` ignition capsules;
- active index or focus-gate files;
- north-star files;
- paused / complete / current eventness states;
- trace-id relationships;
- verification receipts.

That manual stitching is where LLM assistants drift. They can find files, but
they need a compact, evidence-backed answer to:

```text
What should I read now, what should I ignore, and what is the first safe pull?
```

RECUR-REVEAL NEXT SURFACE
-------------------------
`recur-reveal next` should be the only initial command in the `recur-reveal`
executable.

By default, it should be read-only. It may:

- read nearest `.recur/config.toml` reveal policy;
- inspect root `.recur-*` capability cards;
- discover lane-local reveal capsules;
- find active index / focus-gate files by configured or conventional names;
- identify north-star files by configured or conventional names;
- summarize current / constructing / paused / complete / strange state counts;
- parse trace-id relationship lines;
- surface verification receipts and trusted evidence;
- print text or JSON;
- suggest the first pull command as text.

By default, it must not:

- execute `pull.first`, `verify`, or any shell command;
- edit, rename, delete, stage, commit, or approve files;
- start watchers, daemons, or background workers;
- impersonate a role beyond printing the declared persona/agent fields;
- treat paused lanes as active unless the request explicitly names them;
- invent a backlog from old eventness.

Execution belongs behind explicit flags on the same `next` command, for example:

```text
recur-reveal next --run pull.first --confirm
recur-reveal next bge.engine --run verify --confirm
```

ORIENTATION PACKET
------------------
A useful text packet should be short enough to paste into an agent prompt.

Suggested fields:

```text
root: <project-root>
query: <domain-or-lane-or-default>
persona: <from reveal capsule or policy>
agent: <declared role>
north_star: <path or none>
active_index: <path or none>
current_next_action: <short text>
read_now:
  - <small set of files>
exclude_from_planning:
  - <paused lane or pattern>
first_pull: <command text only>
verify: <command text only>
trusted_receipts:
  - <commit/test/replay/receipt summary>
trace_edges:
  - <short define/consume/produce/trigger/block/supersede/verify edge>
ready_state: <one-line statement>
```

JSON shape should mirror the text packet so agents and wrappers can consume it
without scraping prose.

DOMAIN / LANE RESOLUTION
------------------------
Default behavior:

```text
recur-reveal next
```

Resolution order:

1. Use configured current thread / default reveal target when present.
2. Use the single active index if exactly one is visible in scope.
3. If multiple domains are visible, list candidates instead of guessing.
4. If no active focus exists, fall back to ordinary `recur reveal` listing.

Explicit domain or lane:

```text
recur-reveal next bge.engine
recur-reveal next main.improvement.29
```

Resolution order:

1. Match a reveal capsule for the exact lane or nearest parent.
2. Find matching active index / focus-gate files.
3. Find matching north-star files.
4. Summarize relevant eventness states under the same membrane.
5. Exclude paused lanes unless the explicit query names a paused lane.

TRACE-ID EXPECTATIONS
---------------------
`recur-reveal next` should prefer trace-id relationship edges over prose.

Recommended verbs:

```text
defines
consumes
produces
triggers
blocks
supersedes
verifies
```

The command should not require every repo to use those exact verbs at first, but
it should warn when an eventness tree claims trace-id discipline while using
unknown relationship verbs.

COMMAND EXECUTION BOUNDARY
--------------------------
The companion executable should not sprout a whole command garden. The initial
shape is one command, `next`, with read-only default behavior and explicit
execution flags.

Possible command forms:

```text
recur-reveal next --dry-run
recur-reveal next --run pull.first --confirm
recur-reveal next bge.engine --run verify --confirm
```

`recur-reveal next` may:

- produce its own JSON packet;
- execute only explicitly declared fields such as `pull.first` or `verify`;
- require operator confirmation unless policy says otherwise;
- preserve command, scope, exit status, timestamp, and output summary;
- write ACK/NAK state under `.recur/reveal/`;
- leave eventness that core `recur` can inspect later.

`recur-reveal next` must not:

- expand the packet into an unbounded plan;
- run undeclared commands;
- silently approve state transitions;
- mutate source files except through an explicitly confirmed command;
- hide failed or partial execution.

ACK/NAK RECORD SHAPE
--------------------
Example companion output state:

```text
id = reveal-next.20260703T000000Z
state = complete
ack = accepted
nak_reason = ""
query = bge.engine
packet_source = recur-reveal next bge.engine --json
executed_field = verify
command = julia usability\bge\julia-tests\runtests.jl
exit_code = 0
summary = Suite complete; TestFailed: 0
```

Rejected example:

```text
id = reveal-next.20260703T000100Z
state = stopped
ack = rejected
nak_reason = "pull.first requires operator confirmation"
query = main.improvement.29
executed_field = pull.first
command = cargo test --quiet
```

IMPLEMENTATION SLICES
---------------------

### Slice 1: Contract docs and fixtures

- Add this proposal.
- Add docs-side eventness bridge under `docs/main.improvement.29.**`.
- Add fixture packets that show default, explicit domain, ambiguous, and no-focus
  behavior.

### Slice 2: Packet generator

- Add `recur-reveal next` while preserving existing `recur reveal <lane>`
  behavior in core recur.
- Support text output first.
- Add JSON output using the same field names.
- Do not execute commands.

### Slice 3: Trust and warnings

- Warn on multiple active indexes.
- Warn when paused lanes are being pulled without explicit query.
- Warn on missing verification receipts for fixed/verified claims.
- Warn on unknown trace-id verbs when a trace-id convention is declared.

### Slice 4: Capability card update

- Extend `.recur-reveal` to explain `recur-reveal next` and the core reveal
  purity boundary.
- `recur capability explain reveal` should surface the distinction.

### Slice 5: Optional execution mode

- Add `recur-reveal next --dry-run`.
- Add confirmation-gated execution for declared `pull.first` / `verify` fields.
- Write ACK/NAK records under `.recur/reveal/`.

RELATION TO EXISTING IMPROVEMENTS
---------------------------------

- Improvement 22 defines reveal doctrine and lane-local rehydration capsules.
- Improvement 27 defines the pure query / companion automation boundary for
  warp.
- Improvement 28 defines root `.recur-*` capability cards and `recur capability
  explain reveal`.
- The purity decision defines the general split: core `recur` inspects, companion
  actors perform operations and leave eventness.
- `recur-version` is the current concrete precedent for a pure query surface
  plus companion writer.

SUCCESS CRITERIA
----------------

- A human can run `recur-reveal next` and know what to read first.
- An LLM agent can consume `recur-reveal next --json` without re-searching the
  whole repo.
- Paused lanes remain visible but excluded from planning by default.
- The packet shows evidence for its recommendations.
- The command is useful even when no companion exists.
- Any future companion writes ACK/NAK state that core `recur` can inspect.

TRACE-ID LINES
--------------

```text
defines: main.improvement.29 recur-reveal next orientation packet for current workspace or lane focus
defines: recur.reveal.core-boundary core recur reveal remains pure capsule listing and showing
defines: recur-reveal.next companion command for next orientation packet and confirmation-gated execution of declared fields
consumes: main.improvement.22 reveal doctrine and lane-local ignition capsules
consumes: main.improvement.27 command-boundary pure query surface plus companion automation split
consumes: main.improvement.28 capability-card query surface for root .recur-* files
consumes: main.recur.purity.decision core recur inspects and companion actors write ACK/NAK eventness
produces: recur-reveal.orientation-packet persona north-star active-index read-now exclusions first-pull verify receipts
triggers: main.improvement.29.contract future packet schema and fixtures before implementation
triggers: recur.reveal.capability-card-update explain reveal-next and recur-reveal companion boundary
```

DISCOVERY
---------

```powershell
recur files "README.CORE.IMPROVEMENT29" -d ./
recur files "main.improvement.29.**" -d docs/
recur files "main.command.reveal.**" -d docs/
recur capability explain reveal -d .
```
