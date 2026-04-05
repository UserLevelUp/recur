# Command: trace-id Test Lane - Complete

Status: `complete` (Phases 1-4 closed here, Phase 5 closed via improvement 9)
Date completed: 2026-04-05
Original date: 2026-03-03

## What Was Done

Phased test suite for trace-id, implemented in `julia-tests/runtests.trace-id.jl`.

### Phase 1: CLI Contract - DONE
- `trace-id --help` exposes command and required flags
- missing required args fail with actionable errors
- invalid `--format` / guardrail args fail with actionable errors

### Phase 2: Core Heuristic Roles - DONE
- detect `define` sites for identifier constants
- detect `produce` publish/send/emit sites
- detect `consume` subscribe/bind/routing sites
- detect `trigger` pattern registration sites

### Phase 3: Output Contracts - DONE
- JSON output schema is stable for downstream tooling
- terminal output includes grouped role sections
- empty-result behavior is deterministic

### Phase 4: Scope/stdin/Depth Guardrails - DONE
- scope and extension filtering constraints hold
- stdin-limited file set behavior works
- depth guardrail and `--force` semantics are consistent

### Phase 4b: Saved Runs - DONE
- `--save-run`, `--check-run`, `--reuse-if-fresh`, and `--run-name` are covered
- freshness transitions from `fresh` to `stale` are exercised in the suite

### Phase 5: Cross-Command JSON Pipeline - DONE
- `trace-id -> merge` now passes and retains `edge_type`
- the three non-trace-id edge-metadata placeholders remain permanent `@test_skip`
- close-out record: `docs/main.improvement.9.trace-id.complete.md`

## Test Counts

Current suite snapshot: `63 pass, 3 broken, 66 total`

## Artifacts

- `julia-tests/runtests.trace-id.jl`
- `julia-tests/main.command.trace-id.test.jl`
- `docs/main.improvement.9.trace-id.complete.md`
