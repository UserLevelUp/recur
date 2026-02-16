# Trigger Events: Improvement 7 Phase 3

## On Start

```bash
recur files "**.current" -d docs/
recur tree "main.improvement.7" -d docs/
cat docs/main.improvement.7.phase3.todo.current.md
cat docs/main.improvement.7.phase3.todo.current.reference.md
```

## During Work

```bash
# review active source lanes
recur files "main_command_trace*" -d src/ --sep _
recur files "main_command_call*" -d src/ --sep _
recur files "main.command.trace.force.**" -d docs/

# review guardrail behavior in code
recur find "depth > 5" --scope "**" -d src/
recur find "TraceStopReason" --scope "**" -d src/
recur find "--force" --scope "**" -d src/

# keep changes/test loop tight
cargo test --bin recur
```

## On Complete

```bash
cargo test

# cleanup ephemerals
rm docs/main.improvement.7.phase3.todo.current.md
rm docs/main.improvement.7.phase3.todo.current.reference.md
rm docs/main.improvement.7.phase3.todo.trigger.event.md

# completion marker
# docs/main.improvement.7.phase3.complete.md
```
