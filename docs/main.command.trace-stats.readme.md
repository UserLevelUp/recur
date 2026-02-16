# main.command.trace-stats.readme

Command overview for `trace-stats`.

## Status

Phase 3 bootstrap is active. Command surface and validation are implemented.
Metrics pipeline is pending in:

- `docs/main.command.trace-stats.metrics.todo.current.md`

## Examples

```bash
recur trace-stats --scope "**"
recur trace-stats --scope "**" --sort-by transitive --top 10
recur trace-stats --scope "**" --filter circular-only
recur trace-stats --scope "**" --format csv
recur trace-stats --scope "**" --json
```
