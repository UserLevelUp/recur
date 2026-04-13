# Improvement 21: Directory Projection / Namespace Mapping

Status: `todo.future-plan` (proposal / future direction)

## Objective

Keep the directory-projection idea visible in eventness so later work can
rediscover it through the normal improvement tree.

## Current Posture

- proposal only
- no implementation lane should open from this note alone
- use the root proposal as the longform design reference

## What This Improvement Is About

A folder is a stronger lane boundary than a naming prefix alone.

The core idea:
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

## Related

- `README.CORE.IMPROVEMENT21.md`
- `README.CORE.IMPROVEMENT22.md`
- `docs/main.improvement.22.todo.future-plan.md`
- `README.CORE.IMPROVEMENT18.md`
- `docs/main.demo.sudoku.trace-id.todo.current.md`
