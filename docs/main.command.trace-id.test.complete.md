# Command: trace-id Test Lane — Complete

Status: `complete` (Phases 1-4 done, Phase 5 absorbed into improvement 9)
Date completed: 2026-03-24
Original date: 2026-03-03

## What Was Done

Phased test suite for trace-id, implemented in `julia-tests/runtests.trace-id.jl`.

### Phase 1: CLI Contract — DONE
- `trace-id --help` exposes command and required flags
- Missing required args fail with actionable errors
- Invalid `--format` / guardrail args fail with actionable errors

### Phase 2: Core Heuristic Roles — DONE
- Detect `define` sites for identifier constants
- Detect `produce` publish/send/emit sites
- Detect `consume` subscribe/bind/routing sites
- Detect `trigger` pattern registration sites

### Phase 3: Output Contracts — DONE
- JSON output schema is stable for downstream tooling
- Terminal output includes grouped role sections
- Empty-result behavior is deterministic

### Phase 4: Scope/stdin/Depth Guardrails — DONE
- Scope and extension filtering constraints hold
- stdin-limited file set behavior works
- Depth guardrail and `--force` semantics are consistent

### Phase 5: Cross-Command JSON Pipeline — ABSORBED
Moved to `docs/main.improvement.9.trace-id.todo.current.md`.
The `@test_broken` placeholders in `runtests.trace-id.jl` flip when merge edge-type lands.

## Test Counts (at close)

42 pass, 4 broken (Phase 5 pipeline placeholders)

## Artifacts

- `julia-tests/runtests.trace-id.jl`
- `julia-tests/main.command.trace-id.test.jl`
