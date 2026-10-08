# Start the evidence repair here

artifact.type = lane
defines: main.command.watch.eventness.evidence.initial
consumer: main.command.watch.eventness.evidence.contract
consumer: main.command.watch.eventness.evidence.binary.investigation.ready
warp.id = main.command.watch.eventness.evidence
status = planned; no workers launched; no gates accepted

The first-use problem is simple: passing test evidence must identify the exact
executables tested. Cargo can replace those files during another build. The repair
will keep tested binaries separately and detect mutation, then renew evidence
without discarding its original source coverage or historical receipts.

Read `main.command.watch.eventness.evidence.final.predicted.md` in this directory
for the final Eventness predicted for this single Warp. Revise that prediction as
findings reveal what this Warp can attain, until it matches attainable reality.
Each later Warp has its own perspective and predicted final Eventness. The map's
`predicted_final_eventness` field is an optional authoring pointer, not a runtime
acceptance rule.

Treat this Warp as a predicted transition from initial Eventness through its
slices into the final product. Failure is acceptable when its explanation is
useful and supported. Capture what failed, why, and what remains uncertain;
use those clues to predict a later Warp's own initial and final Eventness.

Read these artifacts in order:

1. `warps/main.command.watch.eventness.hash-investigation.current.md`
2. `warps/watch-evidence/main.command.watch.eventness.evidence.contract.md`
3. `warps/main.command.watch.eventness.evidence.warp-map.json`
4. The verification runner and acceptance suite in `warps/watch-eventness/`.
5. `src/warp_refresh.rs`, `tests/warp_refresh_bounds.rs` and the evidence-refresh
   guide when working on the integration slice.

Use the local binaries for live discovery; installed CLI builds may lag:

```powershell
.\target\release-safe\recur.exe warp show main.command.watch.eventness.evidence -d . --json
.\target\release-safe\recur.exe warp slices main.command.watch.eventness.evidence -d . --json
```

Load `recur-warp`, `recur-eventness` and relevant Watch/trace-ID skills when useful.
Ordinary code and test work needs no Lang translation or watcher registration.
Do not load unrelated demo context. Test commands for the repair's new regressions
will be recorded once the tests slice creates them; none are claimed to exist now.

Existing regression commands, when the corresponding inputs are ready:

```powershell
cargo test --profile release-safe --test warp_refresh --test warp_refresh_bounds
node --test warps/watch-eventness/main.command.watch.eventness.integration.test.cjs
```

The original acceptance script presently depends on a fresh evidence bundle that
has not been produced. Do not treat its current stale/missing-bundle failures as a
Watch behavior failure or hide them by rewriting old run metadata.

Coordination is optional. Before launching any worker, configure an exact slice,
source scope, disjoint writable workspace, test argv and supported host. Share the
persisted artifact path and hierarchical trace IDs; a notification is not acceptance.
No coordinator or subscription is started by this planning setup.
