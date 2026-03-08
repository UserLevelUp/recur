# recur Discover Family

Status: `readme` (permanent)
Date: 2026-03-08

Commands that explore file sets and hierarchy structure.

```bash
recur children "main.command.discover" -d docs/ --sep .
recur related "main.command.discover.files" -d docs/ --sep .
```

## Commands

main.command.discover.files — list files matching a hierarchical pattern
main.command.discover.find — content search: find pattern occurrences in scoped files
main.command.discover.tree — show identifier tree across file names
main.command.discover.children — list direct children of a hierarchy node
main.command.discover.related — list siblings of a hierarchy node
main.command.discover.stats — hierarchy metrics: depth, width, node count per level
main.command.discover.id — show all unique identifiers in scope

## Common Flags

All discover commands support `--scope` and `--sep`.
`files`, `find`, `id` support `--stdin` for pipe-based filtering.

## Pipeline

`files` and `id` publish recur.pipe.json.
`files` and `id` trigger recur.pipe.stdin (output piped to other commands as stdin).

## References

- `docs/main.command.files.readme.md`
- `docs/main.command.find.readme.md`
- `docs/main.command.tree.readme.md`
- `docs/main.command.children.readme.md`
- `docs/main.command.related.readme.md`
- `docs/main.command.stats.readme.md`
- `docs/main.command.id.readme.md`
- `docs/main.command.map.readme.md`
