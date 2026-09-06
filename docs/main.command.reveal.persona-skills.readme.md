# Warp: opinionated reveal personas and skill profiles

Status: planned for later implementation. No companion or configuration behavior
is implemented by this document. Independent from main.command.warp.identity-policy;
reuse its init safety patterns when available, without changing that bubble's scope.

## Current state and command boundary

The existing command is recur init, not a recur-init executable. It already writes
[reveal] policy/order/merge/rank defaults. Extend that generator rather than
inventing a second incompatible initialization path. Cargo.toml currently has no
recur-reveal binary. Existing recur reveal prints capsule fields and optional Warp
reconciliation; skill.path is presently only a printed pointer.

Core recur reveal remains read-only listing/showing. The proposed recur-reveal
companion manages opinionated profiles/config and assembles a selected persona's
next context packet, in line with Improvement 29. This Warp implements only bounded
local profile setup and packet preparation, not the entire Improvement 29 vision.

## Proposed editable defaults

```toml
[reveal.personas.skippy]
skills = ["recur-expert", "recur-warp"]
guidance_level = "advanced"

[reveal.skills.recur-expert]
path = "recur-expert/SKILL.md"

[reveal.skills.recur-warp]
path = "recur-warp/SKILL.md"
```

These are desired project-relative bindings, not assertions that new projects
contain these files. Only recur-expert/SKILL.md currently exists here; a standalone
recur-warp skill is future work. Missing references remain visibly unresolved.
Advanced is a guidance preference, not a measured competence or permission level.
Names, order, skill bindings and level are editable per persona/use case.

New recur init configurations include commented examples and default bindings.
Existing recur init/--analyze/--force behavior stays compatible. Proposed
recur-reveal init supplies a non-destructive retrofit: --dry-run previews, explicit
init writes only missing defaults and repeats idempotently. Preserve user comments,
custom profiles, explicit empty skill lists and unrelated config. Never modify
user-global agent configuration or install skills as a side effect.

Proposed recur-reveal next skippy --json assembles a deterministic, read-only
recur-reveal-packet-v1 packet with persona, ordered skill IDs, source paths,
resolution status and diagnostics. Missing required skills yield state=blocked,
structured JSON on stdout and nonzero exit. Resolved paths/fingerprints identify
the exact local guidance offered; available does not mean loaded or executed.
Do not recursively expand arbitrary references or shell-evaluate capsule fields.

## Slice acceptance matrix

- Slice 0: inspect init/reveal and Improvement 29, freeze CLI/schema contracts,
  observe red tests and passing legacy reveal/init baseline. Record exact commands.
- Slice 1: typed config and shared editable defaults; test fresh init, nearest
  project, custom Skippy, another persona, explicit opt-out, malformed types and
  preservation/idempotency. No automatic personality activation.
- Slice 2: bounded local skill registry resolution; test stable order, duplicate
  IDs, missing/invalid SKILL.md, unknown personas, path traversal and symlink escapes.
  No implicit global lookup, downloads or execution. Paths resolve from config root.
- Slice 3: companion init/next; test dry-run/no-write packets, partial-write recovery,
  user-file preservation, clear missing prerequisites and deterministic JSON.
- Slice 4: core reveal can report configured associations without activating them;
  preserve legacy packet fields/capsule selection and tree/files/trace-id discovery.
  Existing reveal skill-pointer test must still prove no recursive loading.
- Final: integrate newly green tests into the normal runner; Cargo and full Julia
  suites pass with known-broken cases unchanged; help/docs separate implemented
  behavior from future agent-host activation. Accept slices only with evidence.

The initial standalone test is julia-tests/main.command.reveal.persona-skills.test.jl.
It is intentionally red and only starts this matrix; extend coverage before each
slice. No tests are marked broken to disguise absent functionality.

Deferred: host-specific skill activation, remote registries/installers, implicit
execution of verify/pull commands, persona inheritance graphs, self-modifying
persona feedback, full reveal-next scheduling and broad orchestration. No commit,
push, cleanup or actual profile change is authorized by revealing a persona.

defines: recur.reveal.persona-skills proposed editable persona skill associations and bounded context packets
consumes: main.improvement.29 existing reveal-next proposal
