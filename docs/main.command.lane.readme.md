# recur lane

Status: `implemented`
Date: 2026-07-15

`recur lane` creates a bounded lane sub-root or lists existing lane roots.
It is a coordination primitive: the command creates only the scope fence and
the ignition capsule; it does not assign an agent, execute work, or start a
watcher.

## Commands

```powershell
recur lane docs
recur lane impl
recur lane
recur lane --json
```

`recur init` writes the lane scaffold policy:

```toml
[lanes]
root = "lanes/"
entry_suffix = ".recur.md"
```

Creating `recur lane docs` creates `lanes/docs/`, initializes its scoped
`.recur/config.toml`, and preserves or writes
`lanes/docs/.recur/docs.recur.md`. Repeating the command never overwrites an
existing capsule.

## Boundary

`recur lane` establishes a physical scope. `recur reveal` rehydrates the
capsule within that scope. `recur psyche` audits whether the work, status, and
capsule records remain structurally coherent.
