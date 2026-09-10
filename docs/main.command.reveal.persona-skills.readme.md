# Warp: discover agents, personas and skills

Scope revision v2, 2026-09-09: agents, personas and skills are independently
discoverable artifacts. Persona skill configuration is one association mechanism,
not the goal of the bubble. Keep the existing Warp ID and trace identity for
continuity; all six slice contracts advance to v2 and remain unaccepted.

Status: planned for later implementation. No companion or configuration behavior
is implemented by this document. Independent from main.command.warp.identity-policy;
reuse its init safety patterns when available, without changing that bubble's scope.

## Current state and command boundary

The existing command is recur init, not a recur-init executable. It already writes
[reveal] policy/order/merge/rank defaults. Extend that generator rather than
inventing a second incompatible initialization path. Cargo.toml currently has no
recur-reveal binary. Existing recur reveal prints capsule fields and optional Warp
reconciliation; skill.path is presently only a printed pointer.

Under the target contract, core recur reveal remains read-only listing/showing of agents, personas and skills,
their source identities and explicitly declared associations. The proposed
recur-reveal companion manages opinionated profiles/config and prepares bounded
context for a selected agent or persona, in line with Improvement 29. Discovery
must also work for standalone skills and artifacts with no associations. This
Warp covers local discovery, associations and packet preparation, not an agent
runtime or the entire Improvement 29 vision.

Ownership: reuse the Reveal command and trait infrastructure for discovery and
inspection. `recur init` supplies project defaults; the proposed
`recur-reveal init` owns opinionated retrofit configuration. No separate agent
management system is introduced. `recur reveal init` is not part of this scope;
any future convenience route to a writer needs its own explicit contract.

## Discovery and association requirements

- Reuse existing `recur reveal --type agent`, `--type persona` and `--type skill`
  discovery. Explicit `artifact.type` and the shared configurable classifier
  establish type; an incidental field, familiar name or prose does not.
- Preserve separate identities for an agent, a persona and a skill even when
  their names coincide. Do not require every agent to have a persona, every
  persona to have skills, or every skill to belong to a profile.
- Resolve explicit agent-to-persona, agent-to-skill and persona-to-skill
  references. Show their declaring source and resolution status. No association
  is inferred from filename proximity, matching names or similar descriptions.
- Missing targets, ambiguous identities, wrong target types and conflicting
  declarations stay visible. They must not silently select a substitute.
- Listing/showing exposes metadata and pointers; it does not read every body,
  activate a persona, load a skill, execute an agent or establish permission.
- Packet preparation is a separate explicit operation. Bound its reads to the
  project root and selected declarations, report fingerprints and omissions,
  and never recursively expand arbitrary instructions. The v2 CLI and packet
  schema must be frozen in slice-0 before implementing the broadened resolver.

## Proposed editable defaults

Integration note (2026-09-07): artifact classification is shared through
`recur::reveal_artifact` and `[reveal.types]`, with core `recur reveal --type`.
Reuse that classifier in this bubble's query integration; profile association
must not infer type from incidental persona/agent/skill.path fields or introduce
another classifier. Hierarchical capsules do not require a SKILL.md name for
classification. This bubble's separate body-resolution and packet contract still
requires its own assessment before broadening beyond its proposed SKILL.md registry.
The v2 assessment must include agent/persona/skill capsules and explicit body
pointers. Retain SKILL.md interoperability without making that filename the only
way to discover a skill. Historical v1 observations do not accept this new scope.

```toml
# Proposed v2 association configuration; not implemented by this document.
[reveal.agents.workshop]
persona = "skippy"
skills = ["recur-expert"]

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
Names, order, skill bindings and level are editable per persona/use case. The
agent's identity is distinct from its persona. Freeze how these configuration
records bind to typed capsules, including precedence/conflict handling, in
slice-0; this example does not define a second artifact classifier.

New recur init configurations include commented examples and default bindings.
Existing recur init/--analyze/--force behavior stays compatible. Proposed
recur-reveal init supplies a non-destructive retrofit: --dry-run previews, explicit
init writes only missing defaults and repeats idempotently. Preserve user comments,
custom profiles, explicit empty skill lists and unrelated config. Never modify
user-global agent configuration or install skills as a side effect.

The earlier proposal `recur-reveal next skippy --json` assembles a deterministic, read-only
recur-reveal-packet-v1 packet with persona, ordered skill IDs, source paths,
resolution status and diagnostics. Missing required skills yield state=blocked,
structured JSON on stdout and nonzero exit. Resolved paths/fingerprints identify
the exact local guidance offered; available does not mean loaded or executed.
Do not recursively expand arbitrary references or shell-evaluate capsule fields.
For v2, freeze unambiguous agent/persona selection, whether this proposed CLI
changes, and how shared referenced skills are deduplicated while preserving
association provenance. Do not treat the earlier packet example as the complete
agent/persona/skill discovery contract.

## Slice acceptance matrix

- Slice 0: inspect init/reveal, shared artifact classification and Improvement 29;
  freeze v2 identity, association, selection, CLI/schema and read-bound contracts.
  Include agents, personas, standalone skills, duplicate names across types,
  missing/ambiguous/wrong-type references and empty associations. Establish new
  red coverage and passing legacy reveal/init/artifact-type baselines.
- Slice 1: typed agent/persona/skill config and shared editable defaults; test fresh init, nearest
  project, custom Skippy, another persona, explicit opt-out, malformed types and
  preservation/idempotency. No automatic personality activation.
- Slice 2: bounded local artifact and explicit association resolution; test stable
  order, duplicate IDs across and within types, agent-to-persona/skill and
  persona-to-skill links, missing/ambiguous/wrong-type targets, conflicting sources,
  standalone artifacts, SKILL.md and capsule body pointers, malformed bodies,
  unknown agents/personas, path traversal and symlink escapes.
  No implicit global lookup, downloads or execution. Paths resolve from config root.
- Slice 3: companion setup and explicit agent/persona packet preparation under
  the frozen v2 CLI; test dry-run/no-write packets, partial-write recovery,
  user-file preservation, clear missing prerequisites and deterministic JSON.
- Slice 4: core reveal lists/shows all three artifact types and reports explicit
  associations, sources and unresolved references without activating them;
  preserve legacy packet fields/capsule selection and tree/files/trace-id discovery.
  Existing reveal skill-pointer test must still prove no recursive loading.
- Final: integrate newly green tests into the normal runner; Cargo and full Julia
  suites pass with known-broken cases unchanged; help/docs separate implemented
  behavior from future agent-host activation. Accept slices only with evidence.

The initial standalone test is julia-tests/main.command.reveal.persona-skills.test.jl.
It is intentionally red and covers the older persona-only proposal, not v2
acceptance. Extend it in slice-0 and extend coverage before each
slice. No tests are marked broken to disguise absent functionality.

Deferred: host-specific skill activation, remote registries/installers, implicit
execution of verify/pull commands, persona inheritance graphs, self-modifying
persona feedback, full reveal-next scheduling and broad orchestration. No commit,
push, cleanup or actual profile change is authorized by revealing a persona.

defines: recur.reveal.persona-skills local agent persona skill discovery explicit associations and bounded context packets
consumes: main.improvement.29 existing reveal-next proposal
