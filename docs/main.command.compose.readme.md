# recur Compose Family

Status: `readme` (permanent)
Date: 2026-03-08

Commands that combine and reshape hierarchy views.

```bash
recur children "main.command.compose" -d docs/ --sep .
recur related "main.command.compose.merge" -d docs/ --sep .
```

## Commands

main.command.compose.merge — multi-separator unification: combine dot/underscore/hyphen hierarchy views
main.command.compose.flatten — hierarchy collapse: squash nested structure to a flat list

## merge

`recur merge` is the composition hub. It subscribes to `recur.pipe.json` —
any command with `--json` output can pipe into merge:

```bash
recur callers "MyFunc" --json | recur merge --stdin --sep . --sep _
recur trace-id "my.id" --json | recur merge --stdin --sep . --sep _
recur files "src.**" --json | recur merge --stdin --sep .
```

merge supports `--sep` (multiple allowed), `--stdin`, `--base`, `--show-sep`.

## flatten

`recur flatten` collapses a deep hierarchy to a flat list of leaf identifiers.
Useful for generating grep inputs or downstream file lists.

## References

- `docs/main.command.merge.readme.md`
- `docs/main.command.flatten.readme.md`
- `docs/main.command.map.readme.md`
- `demos/ascii-drinks/demo.ps1` — merge pattern reference
