# Trigger Events: trace-id Test Lane

## On Start

```bash
recur files "**.current" -d docs/
recur files "main.command.trace-id.test.**" -d docs/
cat docs/main.command.trace-id.test.todo.current.md
cat docs/main.command.trace-id.test.todo.current.reference.md
```

## During Work

```bash
# Check command and test lanes
recur find "Trace {" --scope "**" -d src/
recur find "TraceStats {" --scope "**" -d src/
recur files "main.command.trace-id.**" -d docs/
recur files "main.command.trace-id.**" -d julia-tests/

# Tight loop
cargo test --bin recur
julia julia-tests/main.command.trace-id.test.jl
```

## On Complete

```bash
# Validate full Rust lane before promotion
cargo test

# remove ephemeral test lane docs
rm docs/main.command.trace-id.test.todo.current.md
rm docs/main.command.trace-id.test.todo.current.reference.md
rm docs/main.command.trace-id.test.todo.trigger.event.md

# add completion marker
# docs/main.command.trace-id.test.complete.md
```
