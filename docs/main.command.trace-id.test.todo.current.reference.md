# Reference: trace-id Test Patterns

## Similar Test Lanes

- `julia-tests/runtests.trace-stats.jl` - command contract + metrics progression
- `julia-tests/runtests.trace.jl` - trace contract and depth behavior
- `julia-tests/runtests.id.jl` - identifier search baseline
- `julia-tests/runtests.merge.jl` - stdin JSON composition and auto-JSON behavior
- `julia-tests/runtests.unflatten.jl` - frozen future-contract style with `@test_broken`

## Study Commands

```bash
cat julia-tests/runtests.trace-stats.jl
cat julia-tests/runtests.trace.jl
cat julia-tests/runtests.id.jl
cat julia-tests/runtests.unflatten.jl
```

## Recommended Test Strategy

1. Start with a frozen contract (`@test_broken`) to make scope explicit.
2. Activate tests in order:
   - CLI contract
   - heuristic roles
   - output shape
   - stdin/guardrail behavior
3. Keep fixtures minimal and deterministic for role matching.
