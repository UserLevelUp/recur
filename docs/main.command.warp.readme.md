# recur warp status

Status: `implemented` (read-only v1)
Date: 2026-07-15

`recur warp status <lane>` reads a bounded lane membrane and returns an
evidence-backed `optimum`, `sub_optimum`, or `blocked` verdict. It does not
rename files, collapse eventness, write receipts, start watchers, or approve
anything.

```powershell
recur warp status demo.project.good -d julia-tests/fixtures/warp-status-v1/optimum
recur warp status demo.project.needs -d julia-tests/fixtures/warp-status-v1/sub-optimum --json
recur warp explain demo.project.needs -d julia-tests/fixtures/warp-status-v1/sub-optimum
recur warp next demo.project.needs -d julia-tests/fixtures/warp-status-v1/sub-optimum --json
```

The `warp-status-v1` response contains concrete files, suffix and state-group
counts, trace-id role counts, signals, residual pressures, and suggested next
actions. `objective` is the sum of residual weights; signals explain evidence
but do not make residual pressure negative.

Default suffix groups are `current` → active, `complete` → complete, `strange`
→ interesting, and `blocked` → blocked. A project may override the non-active
groups with `[warp.suffixes]` in `.recur/config.toml`.

The command treats a `blocker` or `operator approval` marker as an external
blocker and reports it rather than attempting to resolve it.

`recur warp explain` renders the same status evidence with signals and residual
paths. `recur warp next` emits only the suggested actions. Both are read-only;
their suggestions are not commands to execute and do not start `recur-warp`.
