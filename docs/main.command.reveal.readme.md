# recur reveal

Status: `readme` (permanent)
Date: 2026-04-13

`recur reveal` surfaces lane-local `*.recur.md` ignition capsules so a human or
agent can rehydrate one lane without re-reading half the repo.

## What It Does

- discovers reveal files ending in the configured `entry_suffix`
- lists known reveal capsules when no lane is requested
- opens one reveal capsule when given a lane/query
- orders the visible fields using `[reveal.order]` from `.recur/config.toml`
- keeps reveal as a coordination layer, not a replacement for source, tests, or
  `*.current.md`

## Usage

```bash
recur reveal
recur reveal main.command.trace-id
recur reveal skippy
recur reveal main.improvement.22 --json
recur reveal --type skill
recur reveal persona.skippy -d .recur --json
```

## Config

Agent/persona/skill discovery also includes explicit configured records inside
the query root. `associations` exposes declaring sources and target resolution.
`recur init` supplies editable association defaults; `recur-reveal init` retrofits
missing tables and `recur-reveal next` explicitly prepares local context bodies.
See [the association and packet contract](main.command.reveal.persona-skills.contract.md).
Core discovery does not load those bodies or activate their instructions.

`recur reveal` reads the nearest `.recur/config.toml` when present.
An explicit `-d` bounds discovery to that directory even if config is found above
it. Without `-d`, discovery starts at the nearest project root (or working
directory without config). This replaces the older behavior that broadened an
explicit subdirectory to the config root. Directory links/junctions and file
symlinks are not followed.

Current defaults from `recur init`:

```toml
[reveal]
mode = "single-thread"
entry_suffix = ".recur.md"
trust = "config-first"
max_threads = 1
skip_persona_if_known = true
```

Ordered fields come from:

```toml
[reveal.order]
steps = [
  "persona",
  "agent",
  "agenda",
  "goals.now",
  "schedule.next",
  "pull.first",
  "pull.then",
  "verify",
  "tool.escape",
  "do.not.disturb",
  "ready.state",
]
```

`recur.gift` is shown first when present, even though it is not part of the
ordered field list.

## Reveal File Shape

```text
# main.command.trace-id.recur

recur.gift = saved-run policy is the only real open edge
persona = recur expert in the trace-id lane
agent = resume and verify, not rediscover from scratch
pull.first = recur files "main.command.trace-id.**" -d docs/
verify = julia julia-tests/main.command.trace-id.test.jl
ready.state = I know the lane and what to pull next
```

## Notes

- `recur reveal` works even when the nearest config has no `[reveal]` section;
  it falls back to `.recur.md` plus the built-in default order.
- reveal files are lane-local gifts, not canonical implementation truth
- if multiple reveal capsules match a loose query, `recur reveal` lists the
  candidates instead of guessing

## Artifact types

Capsules can declare `artifact.type = skill`, `persona`, `agent`, or a custom
identifier. Type describes the artifact; Eventness names its changing attention.
Ordinary hierarchical filenames work, such as `skill.recur.eventness.recur.md`,
`persona.skippy.recur.md`, and `agent.backend.recur.md`. `tree` and `files`
continue to discover them. No `SKILL.md` filename is required for classification.

Explicit metadata is authoritative. Optional project prefix hints are editable
data; core has no built-in skill/persona/agent prefix mappings:

```toml
[reveal.types]
prefix_hints = true

[reveal.types.prefixes]
skill = "skill"
persona = "persona"
agent = "agent"
```

The longest matching hierarchy prefix wins. Repeatable `--sep` overrides the
containing directory's configured lane separator, with `.` as fallback.
Set `prefix_hints = false` to disable hints while retaining explicit metadata.
Malformed configuration is an error. Conflicting explicit types remain unresolved;
metadata/prefix disagreement is reported. Prose and incidental persona/agent or
skill.path fields alone leave legacy capsules untyped.

Human and JSON list/show output includes type, source, status and diagnostics.
JSON adds `artifact: {type, source, status, diagnostics}`; unresolved type is null.
`--type TYPE` selects only resolved types and preserves full lane/path identity.
An exact-name type mismatch does not select a different artifact. Duplicate names
can be resolved with a path or filter. Empty listings and missing/ambiguous queries
retain successful exit status; JSON distinguishes `listed`, `found`, `missing`,
`ambiguous`, and `type-mismatch` through `status`. Invalid filters/configuration or
I/O failures exit nonzero. No-argument reveal lists; `list` is not a reserved alias.

See the [frozen contract](main.command.reveal.artifact-types.contract.md) and
[example project](../demos/reveal-artifact-types/README.md).

## Skills handoff

`recur reveal recur-expert` exposes the expert capsule and a pointer to
`recur-expert/SKILL.md`. The canonical playbook remains
`julia-expert/references/recur-playbook.md`; root/docs expert prompt files point there.

`skill.name`, `skill.path` and `skill.loading` are ordinary capsule fields, not
special loader directives. Reveal prints them; it does not discover/install Codex
skills, parse their YAML frontmatter, follow references or execute instructions.
An agent can separately load the indicated skill through its own supported loader
or explicitly read the file. Keep `.recur.md` capsule discovery unchanged rather
than switching entry_suffix to SKILL.md (the formats serve different purposes).

## Related

Planned: [persona-skill configuration Warp](main.command.reveal.persona-skills.readme.md)
for recur init defaults and a future recur-reveal companion. Not implemented yet.

- `README.CORE.IMPROVEMENT22.md`
- `docs/main.improvement.22.todo.future-plan.md`
- `docs/main.recur.expert.recurring.md`
