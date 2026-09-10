# Warp: discover agents, personas and skills

Implementation target: a.0.2.8. Contract revision v2 broadens the original
persona-only proposal to independently discoverable agents, personas and skills.
The existing Warp ID and trace identity are retained. Acceptance is recorded in
the live map/layers and the separate verification document, not implied here.

## Command ownership

- `recur reveal` and the existing Reveal trait infrastructure discover and inspect
  typed capsules, in-root configured records, and explicit associations.
- `recur init` includes shared editable association defaults for a fresh project.
- `recur-reveal init` adds missing association tables to an existing project;
  `--dry-run` previews, and existing tables (including empty opt-outs) are preserved.
- `recur-reveal next ID --type agent|persona -d ROOT --json` prepares bounded local
  context. It does not activate a persona, install skills or execute instructions.

There is no `recur reveal init` writer alias or separate agent-management runtime.
Improvement 29's wider scheduling/execution proposal remains deferred.

## Explicit associations

```toml
[reveal.agents.workshop]
capsule = "agent.workshop"
persona = "skippy"
skills = ["recur-expert"]

[reveal.personas.skippy]
skills = ["recur-expert"]
guidance_level = "advanced"

[reveal.skills.recur-expert]
path = "recur-expert/SKILL.md"
```

The `capsule` binding is optional and exact. If present, its type must match the
record's registry. Without it, a config record is independently discoverable.
Shared artifact classification remains in `reveal_artifact`; matching names or
incidental persona/agent fields never establish capsule type or relationships.
Default Skippy bindings also include `recur-warp/SKILL.md`, an unresolved example
until the project supplies it. Available context does not mean accepted work.

```powershell
recur reveal --type agent -d . --json
recur reveal --type persona -d . --json
recur reveal --type skill -d . --json
recur-reveal init --dry-run -d . --json
recur-reveal next skippy --type persona -d . --max-files 16 --max-bytes 65536 --json
```

Core discovery exposes pointers and association diagnostics without reading
referenced bodies. Packet preparation resolves the subject, its direct persona,
and their declared skills. Shared skills appear once while every incoming
association retains provenance. Missing, ambiguous, wrong-type and conflicting
references remain visible. Bodies and metadata carry source fingerprints.

Body budgets constrain collected body content, not the initial capsule/config
metadata scan. Explicit `-d` prevents collecting parent profiles. Body paths
resolve from the configuration's project root but must stay inside the requested
read root, including after symlink resolution. No global or remote fallback.

See [the frozen v2 contract](main.command.reveal.persona-skills.contract.md)
for schemas, exact selection, exit codes, validation and configuration preservation.

## Slice acceptance matrix

| Slice | Gate | Evidence required |
| --- | --- | --- |
| 0 | baseline-contract | Frozen v2 contract, original red observations and legacy discovery/init baseline |
| 1 | editable-artifact-association-defaults | Typed config, fresh init, custom/empty/inline tables, preservation and repeatability |
| 2 | bounded-artifact-association-resolution | Explicit links, conflicts, missing/ambiguous/wrong types, ordered deduplication, body validation and root bounds |
| 3 | opinionated-reveal-companion | Init preview/write, partial-setup recovery, inert deterministic packets, limits and structured errors |
| 4 | query-integration | Pure typed discovery, source pointers and associations, legacy classifier/selection/hierarchy behavior |
| final | regression-closeout | Integrated Cargo/Julia coverage, package wiring, expert guidance and observed evidence |

Tests: `tests/reveal_profiles.rs` and
`julia-tests/main.command.reveal.persona-skills.test.jl`. The Julia suite is now
included in the normal runner. The earlier
[test relevance audit](main.command.reveal.persona-skills.tests.md) records the
pre-implementation gaps and baseline; consult final verification for closure.

Deferred: host-specific activation, remote registries/installers, executing
verify/pull instructions, persona inheritance, self-modifying feedback and
full reveal-next scheduling. Loading or revealing an artifact grants no authority.

defines: recur.reveal.persona-skills local agent persona skill discovery explicit associations and bounded context packets
consumes: main.improvement.29 existing reveal-next proposal
