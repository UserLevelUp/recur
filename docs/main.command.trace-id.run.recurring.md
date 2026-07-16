# trace-id saved runs: recurring evidence policy

Status: `recurring`
Date: 2026-07-15

## Stable contract

- Saved runs live at `.recur/trace-id/runs/<name>/` as `manifest.toml` plus
  `latest.json`.
- Retention is **latest only**; no timestamped `history/` directory exists.
- Freshness is metadata based: request shape, nearest config content, and
  scoped path/size/modified-time fingerprints.
- Content hashing is deliberately deferred; it is not silently promised by the
  current flags.
- Saved output is derived audit evidence, never canonical eventness.

## Coordinator rule

`fresh` can satisfy a read-only evidence pull. `stale` and `missing` are
explicit residuals: recompute with a live trace or report the gap. A future
warp verdict must not score a stale cached trace as present evidence.

## Verification

```powershell
julia julia-tests/main.command.trace-id.test.jl
recur trace-id "transition.audit.order.42" --scope "transition.audit.**" --ext .txt --json --check-run --run-name transition.audit.order.42
```
