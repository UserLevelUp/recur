# main.command.trace-stats.readme

Command overview for `trace-stats`.

## Status

Phase 3 core metrics are active:

- function discovery in scoped files
- direct/transitive/circular/depth/risk stats
- sort/filter/top-N
- table/json/csv outputs
- traversal budget policy via trait + `.recur/config.toml`
  - preferred: `[traits.traversal_budget]`
  - backward compatible fallback: `[traversal]`

Hardening/accuracy follow-up remains in:

- `docs/main.command.trace-stats.metrics.todo.current.md`

## Examples

```bash
recur trace-stats --scope "**"
recur trace-stats --scope "**" --sort-by transitive --top 10
recur trace-stats --scope "**" --filter circular-only
recur trace-stats --scope "**" --format csv
recur trace-stats --scope "**" --json
recur trace-stats --scope "**" --depth 8 --depth-guard clamp
recur trace-stats --scope "**" --depth 8 --force
```
