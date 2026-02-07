# main.separator.history

Purpose:
- Track delimiter/separator policy and any behavior changes.
- Keep query behavior explicit across `src/`, `docs/`, and `julia-tests/`.

## Policy Baseline

- `docs/` and `julia-tests/`: dot hierarchy (`.`) by default.
- `src/`: underscore hierarchy (`_`) for Rust source naming, queried with `--sep _`.
- Preferred query style: pass one explicit `--sep` per command unless intentionally switching.

## Separator Change Log

### 2026-02-07
- change: repeated `--sep` is accepted by CLI.
- behavior: last provided value is used.
- example:
  - `recur tree main -d src --sep "." --sep "_"`
  - effective separator: `_`
- evidence: `src/main.rs` (`sep: Vec<String>`, last value selection).

### 2026-02-07
- change: standardized source dogfooding examples on `--sep _`.
- behavior: source branch queries are stable for `main_command_*` names.
- example:
  - `recur files "main_command_*_impl" -d src --sep _`

## Quick Checks

- Confirm source separator view:
  - `recur tree "main" -d src --sep _`
- Confirm docs separator view:
  - `recur tree "main" -d docs`
- Confirm active separator policy docs:
  - `recur files "main.separator.history" -d docs`
