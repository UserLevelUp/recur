# Trigger Events: Improvement 8 trace-id MVP

## On Start

```bash
recur files "**.current" -d docs/
recur tree "main.improvement.8" -d docs/
cat docs/main.improvement.8.trace-id.todo.current.md
cat docs/main.improvement.8.trace-id.todo.current.reference.md
```

## During Work

```bash
# Command + search lanes
recur find "TraceStats {" --scope "**" -d src/
recur find "Id {" --scope "**" -d src/
recur files "main_command_*_impl" -d src/ --sep _

# Verify target command lanes as they are added
recur files "main_command_trace_id_*" -d src/ --sep _
recur files "main.command.trace-id.**" -d docs/
recur files "main.command.trace-id.**" -d julia-tests/

# Tight implementation loop
cargo test --bin recur
julia julia-tests/main.command.trace-id.test.jl
```

## On Validate

```bash
# Focused lane checks
julia julia-tests/main.command.id.test.jl
julia julia-tests/main.command.trace.test.jl
julia julia-tests/main.command.trace-stats.test.jl

# New lane checks (after files are added)
julia julia-tests/main.command.trace-id.test.jl
```

## On Complete

```bash
cargo test

# cleanup ephemerals
rm docs/main.improvement.8.trace-id.todo.current.md
rm docs/main.improvement.8.trace-id.todo.current.reference.md
rm docs/main.improvement.8.trace-id.todo.trigger.event.md

# completion marker to add:
# docs/main.improvement.8.trace-id.complete.md
```
