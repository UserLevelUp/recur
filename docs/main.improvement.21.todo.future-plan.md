# Improvement 21: Directory as Separator + Lane Root

> !! TITLE SUPERSEDED BY ADDENDUM 2026-04-18 — see end of file.
> Doctrine-aligned title: **Directory as Prefix Extension + Lane Root**.

Status: `todo.future-plan` (proposal / future direction; phase 1 `recur lane` in active build)
Reframed: 2026-04-18 (superseded by addendum same-day)

## Objective

Keep the directory-as-separator-and-lane-root idea visible in eventness so
later work can rediscover it through the normal improvement tree.

## Current Posture

- proposal only
- no implementation lane should open from this note alone
- use the root proposal as the longform design reference

## What This Improvement Is About

> !! THE THREE-LAYER FRAMING BELOW IS SUPERSEDED BY THE ADDENDUM AT THE END OF THIS FILE.
> Kept intact so the correction arc stays visible. The doctrine-aligned
> framing uses prefix/baseline/suffix (whitepaper Section 5.3), not three
> invented layers.

A folder is a stronger lane boundary than a naming prefix alone — AND a folder
is also just another separator.

The reframe (2026-04-18) collapses the original projection idea into three
layers that reuse recur's existing separator/scope engine:

1. **Directory is a separator.** Folder traversal projects into the hierarchy
   namespace the same way `.`, `-`, `_`, and `:` do today. `/` is just another
   separator token, composable with the others via `--sep`. Projecting
   `lanes/docs/main.x` to `lanes.docs.main.x` is the existing separator engine
   given `/` as an additional separator — no new projection engine required.

2. **Directory is a lane fence.** A folder that owns a `.recur/config.toml` is
   a scoped sub-root. Reveal, traits, and separator policy are local to that
   lane. This is what `recur lane` scaffolds (phase 1, actively in build).

3. **Separator per lane is configurable.** The lane's own config declares
   which separator(s) apply within it (`.`, `_`, `-`, `/`, or multi). A lane
   can override the root's separator policy. `--sep /` treats folder crossings
   as explicit separator tokens; without it, projection stays implicit.

The core scaffolding idea (unchanged from the original proposal):
- each agent or workstream gets a dedicated directory as its lane root
- that directory gets its own `recur init` — its own `.recur/config.toml`
- that config scopes the agent's separator policy, reveal doctrine, and trait overrides
- the agent's consciousness capsule lives in that folder's `.recur/`

The pure command is `recur lane`, not `recur init --agent`:

```bash
recur lane docs        # scaffolds a lane called docs
recur lane impl        # scaffolds a lane called impl
recur lane tests       # scaffolds a lane called tests
```

`recur` does not know what an agent is. That is the user's concern.
If the lane is used by an AI agent, the user writes `role = "agent"` in
the lane's `.recur/config.toml`. recur just manages the hierarchy.

The root `.recur/config.toml` declares the lane doctrine:

```toml
[lanes]
root = "lanes/"
entry_suffix = ".recur.md"
```

`recur lane <name>` reads that doctrine and scaffolds accordingly:
- creates `<root>/<name>/`
- drops a nested `.recur/config.toml` scoped to that lane root
- scaffolds `<name>.recur.md` in `.recur/` as the reveal capsule

Each lane then works entirely within its root:
- `recur reveal` from that root shows only that agent's capsule
- config inheritance flows from the nearest `.recur/config.toml`
- agents cannot accidentally read or overwrite each other's consciousness

The directory projection / namespace mapping layer is what makes this clean:
- the physical folder maps to a logical lane prefix
- recur commands scoped to that root see only that agent's hierarchy
- merge, build, test, and commit are coordinated through declared handoff conditions

## Phase 1 vs Phase 2 under the Reframe

> !! LAYER-NUMBER REFERENCES SUPERSEDED BY ADDENDUM AT END OF FILE.
> Phase 1 / Phase 2 split is correct; the layer-1 / layer-2 / layer-3
> vocabulary maps to the superseded three-layer framing above.

- **Phase 1 (active, tests in `julia-tests/runtests.lane.jl`)**: layer 2 only —
  `recur lane <name>` scaffolds a named sub-root, `recur init` writes the
  `[lanes]` scaffolding block, `recur reveal` works from within the lane.
  Goal: 21 failing lane tests go green. No scope expansion.
- **Phase 2 (design-only for now)**: layers 1 and 3 — directory-as-separator
  projection (`--sep /`, implicit folder traversal) and per-lane separator
  overrides. Tests for this land as a separate `@testset` block, deferred
  until phase 1 is green.

## Config Collision Note

Two `lanes`-adjacent TOML blocks already exist or are being added:
- `[lanes.<dir>]` (existing) — separator inference per directory
- `[lanes]` with `root` / `entry_suffix` (phase 1) — lane scaffolding doctrine

Under the reframe, these are two faces of one lane doctrine. The schema
distinction stays (plural-section vs single-section), but they sit under one
conceptual umbrella. Any phase 2 projection config should extend one of these
two blocks, not introduce a third.

## Connection to Improvement 22

Improvement 21 (folder-as-lane) and Improvement 22 (reveal doctrine) are the
joint foundation for multi-agent coordination:

- Improvement 21 gives each agent a physically scoped lane
- Improvement 22 gives each lane a consciousness capsule
- Together: agents work independently, know their own context, and coordinate
  on merge, build, test, iterate, commit, and push without stepping on each other

## Discovery

```powershell
recur files "main.improvement.21.**" -d docs/
recur files "README.CORE.IMPROVEMENT21" -d ./
recur find "projection" --scope "main.improvement.21.**" -d docs/ -i
```

## Addendum: Doctrine-Aligned Reframe (2026-04-18)

This addendum supersedes the three-layer framing earlier in this file. Kept
as an addendum (not an overwrite) so the correction arc stays tractable —
you can see where the framing was and where it is now, consistent with the
"independent re-derivation" culture noted in the eventness whitepaper's
Section 10.

### What was wrong in the three-layer framing

1. "Directory is a separator" — wrong. Directories extend the **prefix**
   toward the interesting regime level. They are not tokens in the separator
   engine sense. Prefix/baseline/suffix is the canonical ontology per
   whitepaper Section 5.3, not separator-ism.
2. "Three layers" doctrine — wrong. The whitepaper has a three-*phase*
   doctrine (Expand → Discover → Collapse, Section 9). Inventing a parallel
   three-layer scheme competed with canon instead of slotting under it.
3. "`--sep /` as first-class token" — over-promoted. This is mechanics
   sitting under the real doctrine, not a new doctrine of its own.

### Canonical doctrine (from docs/eventness_explained_whitepaper.docx)

Section 5.3 — naming encodes an operator ontology:

- **Prefix** = context / regime / scope (selects which operators are active)
- **Baseline** = reference state (instance / version)
- **Suffix** = operator / rule / dynamics (the behavior attached)

Section 3.1 — Sun / Earth / Moon is the canonical intuition:

- `sun[.eventness][.ext]` — rare interesting events
- `sun.earth[.eventness][.ext]` — more interesting
- `sun.earth.moon[.eventness][.ext]` — **the level recur is for**

> "Most of the time, most systems are boring." (Section 1.1)

Section 9 — the real three phases:

- **Expand** — `prefix.base.suffix[.expanding.eventness][.ext]`
  markers: `.todo`, `.priority`, `.probe`, `.drift`, `.spike`, persona
- **Discover** — `recur tree` / `recur find` / `recur scope` surface what expansion made visible
- **Collapse** — `prefix.base.suffix[.collapsing.eventness][.ext]`
  markers: `.resolved`, `.merged`, `.deprecated`, `.promoted`, `.frozen`

Eventness markers attach to **files**, not directories. The file system is
the event log. Directories are path extension toward the leaf.

### Doctrine-aligned improvement 21

Improvement 21 is **Directory as Prefix Extension + Lane Root**:

- A directory extends the prefix of any file it contains. It carries you
  up to the regime level where eventness becomes trackable.
- A lane is a prefix-fence marked by `.recur/config.toml`. The fence says
  "recur's eventness tracking is scoped from here; shallower is just prefix."
- Eventness lives on files inside the lane, never on the directory itself.

Phase 1 (`recur lane` command, active build) scaffolds the prefix-fence.
21 failing tests pending. No scope expansion.

Phase 2 (future) is explicit directory-to-prefix projection mechanics
(e.g., `--sep /` as a composable separator token). Mechanics under the
doctrine, not a competing doctrine.

### Mapping old layers to the new framing

| Three-layer (superseded)      | Doctrine-aligned (current)                                |
|-------------------------------|-----------------------------------------------------------|
| Layer 1 — directory as separator | Mechanics of prefix projection; not a standalone doctrine |
| Layer 2 — directory as lane fence | Lane = prefix-fence at the regime level (retained)        |
| Layer 3 — per-lane separator config | Mechanics of phase 2 projection; not a new doctrine       |

The Phase 1 / Phase 2 split is correct. The layer-numbered vocabulary
within those phases is the superseded part.


## Related

- `README.CORE.IMPROVEMENT21.md`
- `README.CORE.IMPROVEMENT22.md`
- `docs/main.improvement.22.todo.future-plan.md`
- `README.CORE.IMPROVEMENT18.md`
- `docs/main.demo.sudoku.trace-id.todo.current.md`
- `docs/eventness_explained_whitepaper.docx` — canonical doctrine source
- `docs/recur.white.paper.docx` — original recur framing
