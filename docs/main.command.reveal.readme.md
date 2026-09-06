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
```

## Config

`recur reveal` reads the nearest `.recur/config.toml` when present.

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
