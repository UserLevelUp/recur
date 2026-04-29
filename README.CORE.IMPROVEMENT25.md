RECUR IMPROVEMENT 25
Reveal Lanes — Role Topology Over the Vault
============================================
Date: April 25, 2026
Status: Proposal / future direction
Author: Captured live during a 2026-04-25 design-flash
  (Joe Bishop / Skippy coordinator — pattern observed mid-conversation)

INTENT
------
Improvements 23 and 24 gave the vault two things it did not previously have:
- a subscription primitive over filename eventness (`recur watch`)
- a sealed-cycle dispatcher that loads, runs, and judges (`recur spin`)

Both speak fluently in two of recur's existing axes:
- Hierarchy — where work belongs (`prefix.baseline.suffix`)
- Eventness — what state work is in (`.current`, `.complete`, `.spike`, ...)

Two more axes are visible in real coordination but are not yet first-class
in the recur surface:
- Lane — who or what should act on the work
- Topology — whether the lane is expanding, collapsing, merging, or stable

Today these axes live in human heads, in `.recur/skippy/skippy.work.current.md`
prose, and in informal conventions across `.recur/docs-monkey/`,
`.recur/test-monkey/`, `.recur/git-monkey/`, and friends.
That is enough to operate the small-N happy path.
It is not enough to ask the vault "what should the tester be doing right now?"
without a human translating.

Improvement 25 proposes the missing surface: `recur reveal`.
The goal is not to invent a new vocabulary.
The goal is to expose what the naming doctrine already encodes — role and
phase — through a single discovery command that composes cleanly with
`recur watch` and `recur spin`.

SUMMARY
-------
Improvement 25 proposes one deliverable:

`recur reveal` — a role-and-topology-aware view over the vault.

`recur reveal` answers three questions the existing commands answer only
indirectly:

1. **What lanes exist?**
   `recur reveal` (no argument) lists the active role-oriented lanes the
   project is currently expressing.
2. **What work belongs to a lane?**
   `recur reveal <lane>` enumerates the artifacts in that lane and the phase
   each artifact is in.
3. **How is each lane shaped right now?**
   For each lane, `recur reveal` reports the topology state: expanding,
   collapsing, merging, stable, or blocked.

The governing slogan is structural:

- Hierarchy tells **where** work belongs.
- Eventness tells **what state** it is in.
- Lane tells **who** should act on it.
- Topology tells **whether** the lane is expanding, merging, collapsing,
  or stable.

`recur reveal` is the discovery surface that crosses the last two axes.
It does not invent them — it surfaces what is already implicit in lane
prefixes (`expert.`, `tester.`, `monkey.`, `coordinator.`) and lane lifecycle
files (`*.current`, `*.spike`, `*.complete`, `*.merged`, `*.resolved`,
`*.frozen`).

THE PROBLEM
-----------
Three pressures make the missing axis visible.

First, lanes already exist but are not addressable.
The vault holds at least:
- `.recur/skippy/`           (coordinator)
- `.recur/docs-monkey/`      (executor — documentation)
- `.recur/test-monkey/`      (executor — testing)
- `.recur/git-monkey/`       (executor — version control)
- `.recur/julia-expert/`     (expert — domain reasoning)
Each of these is a role.
The directory structure encodes it.
But there is no command that says "show me everything currently expressing
the `monkey` role across the vault."
A human reads directory names by hand and infers.
That inference layer is the next thing to delete.

Second, lane state is not just a single eventness suffix — it is a topology.
Improvement 23 made eventness pub/sub-shaped on a per-file basis.
But a lane as a whole has a shape that single-file eventness cannot express:
- a lane is **expanding** when new sub-lanes or work artifacts are appearing
- a lane is **collapsing** when many artifacts are converging into one
  stable record
- a lane is **merging** when its artifacts are being absorbed into a sibling
  lane
- a lane is **stable** when its current artifacts are resolved or frozen
- a lane is **blocked** when its forward motion depends on an external lane

These are not file states.
They are derived from the population of files in a lane and how that
population is changing over time.
Right now this derivation lives in human pattern-matching ("the tester lane
keeps producing new probes — it's expanding") rather than in a recur view.

Third, coordinators currently route work by reading a coordinator's
`work.current.md` ledger and translating role intent into directory walks.
That translation works at low N.
At higher lane counts, the coordinator burns context re-discovering what
each lane is doing.
A `recur reveal` surface lets the coordinator (human or agent) ask the
vault directly:

- `recur reveal` — what's live across all roles?
- `recur reveal tester` — what's the tester lane carrying?
- `recur reveal feature.serialization` — what's converging here?

The coordinator stops chronicling and starts querying.

THE CORE IDEA
-------------
Coordination has three phases inside a lane lifecycle, already named in the
recur whitepaper (Section 9):

1. **Expand** — new sub-lanes and work appear; eventness markers like
   `.todo`, `.priority`, `.probe`, `.drift`, `.spike` accumulate.
2. **Discover** — `recur tree`, `recur find`, `recur files`, `recur scope`
   surface what expansion made visible.
3. **Collapse** — eventness markers shift toward `.resolved`, `.merged`,
   `.deprecated`, `.promoted`, `.frozen`; the lane consolidates into stable
   artifacts.

`recur reveal` is the surface that names where in this cycle each lane
currently sits.
The middle phase — **discover** — is where existing recur commands already
shine.
Improvement 25 adds the **bookends**:
- a way to see, at a glance, which lanes are expanding (need attention) and
  which are collapsing (can be left to converge)
- a way to address each lane by role, not by directory path

The mental model is intentionally small:

```text
recur reveal                 → list of all live lanes + topology state
recur reveal <lane>          → enumerate that lane's work + per-item phase
recur reveal <lane>.<sub>    → drill into a sub-lane
```

In the middle, the operator (human or coordinator agent) uses the existing
discovery commands to walk the structure:

```text
recur tree   <lane>          → shape
recur files  <lane>.**       → enumeration
recur find   <pattern>       → text search inside the lane scope
recur related <lane>         → adjacent lanes
recur children <lane>        → contained sub-lanes
```

`recur reveal` is the entry point.
The existing commands are the inner loop.
Lane lifecycle files (`*.current`, `*.complete`, `*.merged`, ...) are the
collapse signal.

DELIVERABLES
------------
Improvement 25 proposes one deliverable, with three sub-surfaces.

### Deliverable: `recur reveal`

Proposed surface:

```text
recur reveal [<lane>] [--dir <path>] [--format <text|json>] [--topology <state>]
```

#### Surface 1 — `recur reveal` (no argument)

Lists all currently expressed lanes across the vault, one per line, with a
topology badge.

Default text output:

```text
expanding   coordinator    .recur/skippy/
expanding   monkey.docs    .recur/docs-monkey/
collapsing  monkey.test    .recur/test-monkey/
stable      monkey.git     .recur/git-monkey/
blocked     expert.julia   .recur/julia-expert/  (waits on monkey.test)
```

Detection rules:
- A lane is any directory containing at least one `*.current` or
  `*.recurring` file under `.recur/` (or under any directory mapped by
  `.recur/config.toml` to a lane scope).
- A lane's role prefix comes from the directory name's stem, normalized
  through the configured separators.

#### Surface 2 — `recur reveal <lane>`

Enumerates the artifacts in a single lane with their eventness phase.

```text
$ recur reveal tester
tester.serialization.coverage     spike
tester.serialization.smoke        current
tester.parser.regression          resolved
tester.parser.fuzz                blocked  (waits on expert.parser.review)
```

Phase mapping is derived from the eventness suffix on each artifact's
filename — Improvement 25 does not define new eventness markers, it reads
the ones already in `.recurring.md`, `.current.md`, `.spike.md`,
`.resolved.md`, `.merged.md`, `.frozen.md`, etc.

#### Surface 3 — `recur reveal <lane>.<sub>`

Drills further. The same logic applies recursively:

```text
$ recur reveal feature.serialization
expert.serialization.review        current
tester.serialization.coverage      spike
documenter.serialization.schema    current
monkey.serialization.rename        complete
```

This view is the "what's converging here?" question.
A coordinator looks at the cluster and decides whether the spread of phases
is consistent with a near-term collapse to:

```text
feature.serialization.ready
```

That collapse is performed by the existing collapse vocabulary — no new
machinery for it is proposed in this improvement.

### Topology detection

The topology badge is derived, not declared.
A lane is classified by the recent population delta of its artifacts:

- **expanding**
  more artifacts entered the lane in the last window than left it; new
  sub-lanes appeared; eventness markers tilt toward `.todo`, `.spike`,
  `.probe`, `.priority`.
- **collapsing**
  artifacts are shifting suffix from `.current` / `.spike` toward
  `.resolved`, `.merged`, `.promoted`, `.deprecated`, `.frozen`; the
  population is converging.
- **merging**
  artifacts are being absorbed into a sibling lane (detected by `.merged`
  files referencing a sibling prefix).
- **stable**
  no artifact has changed phase in the lookback window and there are no
  expanding markers.
- **blocked**
  one or more `*.blocked` or `*.waits-on` files reference a lane outside
  the current one. The blocking lane is reported in parentheses.

The lookback window defaults to recent vault mtime activity but is not
load-bearing — `recur reveal` is allowed to be approximately right.
The topology badge is a hint to the coordinator, not a contract.

LANE TAXONOMY
-------------
Improvement 25 standardizes a small set of canonical lane prefixes the
project already uses informally.

| Lane prefix       | Role                                                |
|-------------------|-----------------------------------------------------|
| `coordinator`     | dispatches work, owns alignment judgment            |
| `agent`           | generic AI executor                                 |
| `expert`          | domain reasoner (julia, parser, etc.)               |
| `monkey`          | scoped specialist executor (docs, test, git, ...)   |
| `tester`          | runs and authors test artifacts                     |
| `documenter`      | edits docs, READMEs, whitepapers                    |
| `feature`         | converged cross-lane outcome (post-collapse)        |

These are conventions, not enforcement.
Projects may add lanes; recur does not require declaring them in
`.recur/config.toml` — the directory structure is sufficient.
But standardization helps `recur reveal` produce predictable output across
projects.

The `feature` lane is special: it is where collapse lands.
When `expert.X.review`, `tester.X.coverage`, and `documenter.X.schema` all
reach `.resolved` or `.merged`, they collapse into a single
`feature.X.ready` artifact.
That collapse is not automated by Improvement 25 — it is performed by
existing vocabulary (`recur merge`, manual file moves, or `recur spin` from
Improvement 24 with a collapse manifest).
`recur reveal` simply makes the convergence visible.

EXAMPLE LIFECYCLE
-----------------
A canonical multi-lane feature lifecycle, viewed through `recur reveal`:

Phase 1 — expansion:

```text
$ recur reveal
expanding   coordinator             .recur/skippy/
expanding   expert.serialization    .recur/expert/
stable      monkey.docs             .recur/docs-monkey/
stable      monkey.git              .recur/git-monkey/
```

The coordinator opens lanes by writing briefs:

```text
$ recur reveal expert.serialization
expert.serialization.review        current
expert.serialization.scope         current
```

Phase 2 — discovery (existing commands handle the inner loop):

```text
$ recur tree expert.serialization
$ recur find "format constraint" --scope "expert.serialization.**"
$ recur related expert.serialization
```

Phase 3 — sibling lanes activate:

```text
$ recur reveal
expanding   expert.serialization    .recur/expert/
expanding   tester.serialization    .recur/test-monkey/
expanding   documenter.serialization .recur/docs-monkey/
expanding   monkey.serialization    .recur/docs-monkey/
```

Phase 4 — collapse:

```text
$ recur reveal feature.serialization
expert.serialization.review        resolved
tester.serialization.coverage      resolved
documenter.serialization.schema    merged
monkey.serialization.rename        complete
```

A `recur merge` (or a `recur spin` collapse manifest) writes:

```text
feature.serialization.ready
```

Phase 5 — stable:

```text
$ recur reveal
stable      feature.serialization   .recur/feature/
```

The lane has cycled through expand → discover → collapse → stable.
`recur reveal` made each phase legible without the coordinator having to
chronicle it.

NON-GOALS
---------
Improvement 25 is explicit about what it does not propose:

- NOT introducing new eventness markers.
  The existing markers (`.current`, `.spike`, `.resolved`, `.merged`,
  `.frozen`, ...) are sufficient. `recur reveal` reads them; it does not
  add new ones.
- NOT auto-collapsing lanes.
  Collapse is performed by existing vocabulary (`recur merge`, manual
  moves, or a `recur spin` manifest). `recur reveal` reports state; it
  does not transition state.
- NOT enforcing lane prefixes.
  The taxonomy is convention. Projects may add lanes; recur infers them
  from directory structure.
- NOT requiring declaration in `.recur/config.toml`.
  Lanes are discovered, not registered. Optional config-level role hints
  may be added later; they are not load-bearing.
- NOT replacing `recur tree`, `recur files`, `recur find`, or `recur
  related`.
  Those remain the inner-loop discovery commands. `recur reveal` is the
  entry point that names which lane to inspect.
- NOT building a UI or dashboard.
  Output is text or JSON. A coordinator agent or human reads it.
- NOT becoming a daemon or watcher.
  `recur reveal` is a one-shot query. For continuous observation, compose
  with `recur watch --filter "<lane>.**"` from Improvement 23.

These non-goals matter because each of them, if added, re-inflates the
proposal into infrastructure that the existing naming doctrine already
makes unnecessary.

FAILURE-MODE TAXONOMY
---------------------
Expected failures in `recur reveal`:

- A directory under `.recur/` contains no recognized eventness files.
  Resolution: report the lane as `unknown` and skip phase classification.
- Eventness markers conflict (a file is both `.current` and `.resolved`).
  Resolution: prefer the more recent mtime; surface the conflict in
  `--format json` output.
- A lane references a blocking lane that does not exist.
  Resolution: report `blocked (missing: <lane>)`. This is exactly the
  signal `recur psyche` (Improvement 23) would also flag.
- Topology classification is ambiguous (mixed expand/collapse markers).
  Resolution: prefer the dominant signal in the lookback window. The badge
  is a hint, not a contract; ambiguity does not block the report.

Unexpected failures:

- Vault directory structure is unreadable.
  Resolution: emit an error and exit non-zero. `recur reveal` does not
  fall back to partial output for filesystem errors — that would mask
  vault corruption.
- Two lanes claim the same prefix.
  Resolution: report both. The coordinator decides whether to consolidate
  via `recur merge`.

None of these failure modes require `recur reveal` to mutate state.
It is a read-only surface.
Repair (if needed) is performed by the coordinator using existing
vocabulary.

COMPOSITION WITH IMPROVEMENT 23
-------------------------------
Improvement 23 established `recur watch` as the subscription primitive over
vault writes.
Improvement 25 composes with it cleanly:

- `recur reveal` is the **point-in-time** view of lane topology.
- `recur watch --filter "<lane>.**"` is the **continuous** view of lane
  events.

A coordinator typically uses both:

```text
recur reveal                       # snapshot: what's live now?
recur watch --filter "monkey.**"   # stream: what's happening to monkeys?
```

`recur reveal` answers "where are we?"; `recur watch` answers "what just
changed?"
Together they replace the human-as-message-passer pattern Improvement 23
named.

`recur psyche` (also from Improvement 23) reports vault inconsistencies.
`recur reveal` reports lane topology.
Both are read-only observation tools; they do not overlap.
A vault inconsistency surfaced by `recur psyche` (e.g., a `status.current`
without a `work.current`) may show up in `recur reveal` as an `unknown` or
`blocked` topology — but the diagnostic detail comes from `recur psyche`.

COMPOSITION WITH IMPROVEMENT 24
-------------------------------
Improvement 24 established `recur spin` as a sealed-cycle dispatcher.
Improvement 25 composes with it in two directions:

**Pre-spin (load moment).**
A coordinator runs `recur reveal` to decide which lane needs a spin manifest
authored:

```text
$ recur reveal
expanding   tester.serialization    .recur/test-monkey/  (no coverage yet)
```

That topology badge tells the coordinator the tester lane is open and
needs work. The coordinator authors a spin manifest and dispatches.

**Post-spin (judge moment).**
After `recur spin` releases its evidence pack, the coordinator runs
`recur reveal` to confirm the lane has shifted phase:

```text
$ recur reveal tester.serialization
tester.serialization.coverage      resolved
```

The collapse is visible in the reveal output without the coordinator
re-reading every round's prose.
This is exactly the "binary alignment judgment" Improvement 24 advocated:
the coordinator reads the topology, not the chronicle.

A future extension — explicitly NOT proposed in Improvement 25 — would let
`recur spin` accept a "collapse manifest" that consumes a lane's resolved
artifacts and emits a `feature.X.ready` consolidation.
Improvement 25 makes the collapse visible; Improvement 24's machinery
(eventually) executes it.
The two improvements are designed to fit, not to overlap.

PRODUCT POSITIONING
-------------------
`recur reveal` is the surface that lets the project pitch be stated
plainly:

> recur turns project structure into dynamic work lanes for humans, agents,
> experts, monkeys, and coordinators.

Without `recur reveal`, that sentence is aspirational — the lanes are real
but a human has to know where to look.
With `recur reveal`, the sentence becomes operational — the vault answers
"what lanes exist right now and what shape are they in?" directly.

This is the same shift `recur watch` performed for events:
the events were always in the vault; recur just had to admit it.
Improvement 25 performs the same admission for lanes and topology.

CLOSING
-------
Improvement 25 says something narrow and consequential:

`recur reveal` should be the role-and-topology entry point over the vault.

The four axes are:

- Hierarchy   — where work belongs
- Eventness   — what state work is in
- Lane        — who should act on it
- Topology    — whether the lane is expanding, collapsing, merging, or
                stable

Hierarchy and eventness already have first-class commands.
Lane and topology have not — until now.

The design rule is consistent with Improvements 23 and 24:

- prefer one entry point over many sibling commands
- prefer reading existing markers over inventing new ones
- prefer composition with existing primitives (`watch`, `spin`, `merge`)
  over standalone machinery
- prefer read-only observation over state mutation
- prefer convention over enforcement

The vault already encodes role and topology.
`recur reveal` makes the vault admit it out loud.
