# Command: trace-id Test Lane

Status: `todo.current` (active)
Date: 2026-03-03

## Goal

Create a phased, crunchable test suite for trace-id so implementation can proceed with clear checkpoints.

## Test Phases

### Phase 1: CLI Contract (High confidence)

1. `trace-id --help` exposes command and required flags.
2. Missing required args fail with actionable errors.
3. Invalid `--format` / guardrail args fail with actionable errors.

### Phase 2: Core Heuristic Roles (High confidence)

1. Detect `define` sites for identifier constants.
2. Detect `produce` publish/send/emit sites.
3. Detect `consume` subscribe/bind/routing sites.
4. Detect `trigger` pattern registration sites.

### Phase 3: Output Contracts (Medium confidence)

1. JSON output schema is stable for downstream tooling.
2. Terminal output includes grouped role sections.
3. Empty-result behavior is deterministic.

### Phase 4: Scope/stdin/Depth Guardrails (Medium confidence)

1. Scope and extension filtering constraints hold.
2. stdin-limited file set behavior works.
3. Depth guardrail and `--force` semantics are consistent.

## Initial Test Artifacts

- `julia-tests/runtests.trace-id.jl`
- `julia-tests/main.command.trace-id.test.jl`

## Notes

- Until command surface exists, contracts should be represented as `@test_broken` roadmap debt.
- Keep this lane separate from main full-suite inclusion until minimum CLI contract passes.
