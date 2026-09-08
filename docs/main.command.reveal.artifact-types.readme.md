# Warp: Reveal artifact types

Status: complete, 2026-09-07; all five slices accepted with recorded evidence.
Shared type recognition, diagnostics and filtering are available in the local
release-safe build. See the [verification record](main.command.reveal.artifact-types.verification.md).

## Purpose

Use the shared reveal surface to discover and distinguish skills, personas,
agents and project-defined artifact types. Improvements 31 and 32 are closed as
standalone proposals. Core Recur remains an unopinionated, read-only hierarchy
query engine; companion apps own editable defaults and operational behavior.

Canonical artifacts can be ordinary hierarchical files, for example:

```text
.recur/skill.recur.eventness.recur.md
.recur/persona.skippy.recur.md
.recur/agent.backend.recur.md
```

No required SKILL.md filename, agent-host adapter or MCP layer. Generic tree/files
queries must continue to discover these files using the selected root/separators.

## Implemented contract

Exact CLI/config/JSON behavior is defined in the
[frozen contract](main.command.reveal.artifact-types.contract.md).

- Explicit `artifact.type = skill` capsule metadata is authoritative. Preserve
  custom type names; do not close the vocabulary around three built-in labels.
- Optional configured hierarchy-prefix rules supply type hints when metadata is
  absent. Companion defaults may map skill/persona/agent prefixes. Core does not
  guess a type from prose, directory names or incidental persona/agent fields.
- Report type, its source and diagnostics in list/show JSON and human output.
  Legacy untyped capsules remain discoverable with an explicit untyped state.
  Metadata/prefix disagreement is visible; conflicting explicit declarations are
  unresolved, never silently chosen. Preserve the existing lane and path identity.
- `recur reveal --type skill` filters resolved types; an exact lane plus
  a filter must not silently select a different artifact. Duplicate short names
  across types remain ambiguous until hierarchy or an explicit filter resolves
  them. Define empty-result/error behavior consistently with existing commands.
- Respect configured entry suffixes and separators. Type rules and any aliases
  must be validated data, not executable handlers or hardcoded filename depth.
- Freeze explicit `-d` discovery bounds, nearest-config lookup and hidden-root
  behavior separately: configuration lookup must not silently broaden an explicit
  query into sibling project artifacts. Test and document any compatibility change.
- Type recognition neither loads skill bodies/references nor launches agents,
  activates personas, evaluates commands, installs files or grants authority.

## Slices

0. Baseline: inspect existing reveal/config/tests, freeze CLI/JSON/config details,
   add standalone red acceptance tests and record passing legacy reveal baseline.
1. Classification: shared metadata and configurable prefix resolution, custom
   types, untyped handling, conflict diagnostics and validated configuration.
2. Query integration: list/show/filter output, stable ordering and identity,
   ambiguity handling, configured separators/suffixes and bounded discovery.
3. Guidance: hierarchical example fixtures, companion-default ownership and
   expert/reveal documentation; no migration or writes during core queries.
Final. Acceptance: integrate green tests, run Rust and full Julia regressions,
   record source-bound evidence and only then accept the implementation slices.

Detailed coverage: [acceptance plan](main.command.reveal.artifact-types.tests.md).
Baseline evidence: [slice 0](main.command.reveal.artifact-types.baseline.md).
Runnable examples: [typed artifact fixture](../demos/reveal-artifact-types/README.md).

## Related work and boundaries

[Persona/skill profiles](main.command.reveal.persona-skills.readme.md) remains a
separate existing bubble for companion configuration and bounded profile packets.
Its contracts/receipts are unchanged. Its query integration must reuse the shared
`recur::reveal_artifact` classifier and `[reveal.types]` policy; the profile readme
records that integration boundary.
Full-body delivery, agent scheduling, delegation and execution remain outside this
classification bubble. No separate core skill or agent command is planned here.

Closed decisions: [skills](main.improvement.31.closed.md),
[agents](main.improvement.32.closed.md).

defines: recur.reveal.artifact-types shared artifact classification and discovery
consumes: recur.reveal.persona-skills profile integration boundary
