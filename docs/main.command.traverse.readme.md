# recur Traverse Family

Status: `readme` (permanent)
Date: 2026-03-08

Commands that follow identifier and call relationships through a codebase.

```bash
recur children "main.command.traverse" -d docs/ --sep .
recur related "main.command.traverse.trace_id" -d docs/ --sep .
```

## Commands

main.command.traverse.trace — single-hop caller/callee lookup
main.command.traverse.trace_id — identifier flow: define/produce/consume/trigger classification
main.command.traverse.trace_stats — bulk complexity metrics: direct, transitive, circular, depth, risk
main.command.traverse.callers — upstream: who calls this?
main.command.traverse.callees — downstream: what does this call?

## Common Flags

All traverse commands support `--scope`, `--ext`, `--json`.
`trace-id` and `trace-stats` additionally support `--depth`, `--depth-guard`, `--force`.

## Pipeline

All traverse commands publish recur.pipe.json (via --json flag).
Output pipes into `recur merge` or `recur flatten` for composition.

## References

- `docs/main.command.trace.readme.md`
- `docs/main.command.trace-stats.readme.md`
- `docs/main.command.callers.readme.md`
- `docs/main.command.callees.readme.md`
- `docs/main.command.map.readme.md`
