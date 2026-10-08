# Retain the binaries verified by a Watch Warp

The source repository's opt-in verification runner creates a new evidence bundle
instead of assuming Cargo output will keep its identity after another build.
Windows linker timestamps and debug IDs can change an executable's exact hash
even when its source and machine code remain unchanged. Exact hashes still
identify distinct artifacts; this workflow keeps that distinction.

From the repository root, with Cargo, Node and Julia available:

```powershell
$env:RECUR_WATCH_EVIDENCE_PREFIX = 'watch-native-v7'
node warps/watch-eventness/main.command.watch.eventness.verify.cjs rust
node warps/watch-eventness/main.command.watch.eventness.verify.cjs native
node warps/watch-eventness/main.command.watch.eventness.verify.cjs integration
node warps/watch-eventness/main.command.watch.eventness.verify.cjs julia
node warps/watch-eventness/main.command.watch.eventness.verify.cjs bundle
node warps/watch-eventness/main.command.watch.eventness.verify.cjs final
```

Use a fresh prefix for each new attempt. Existing outputs, reservations and partial
binary directories are refused; inspect failed attempts and choose a new prefix
rather than overwrite observations. Rust runs the isolated fault-hook build first,
copies that artifact to its test location, then runs Cargo tests with normal features.
It retains the resulting core, Watch, Warp and fault-hook executables beneath
`.recur/evidence-binaries/PREFIX/`. Subsequent native and Julia checks execute those
copies through explicit binary overrides. The sibling Warp companion remains
available to the Watch coordinator.

Each public observation has a raw log, structured result, run metadata and a
source-bound manifest. The public binary manifest records exact SHA-256 identities
and build provenance. Verification checks bytes before and after execution and
preserves child observations plus a failed result when postcheck integrity fails.
Evidence also fingerprints the retained binaries and run/log metadata, so live
source assessment detects later mutation or removal. Executable permission bits
are preserved when the platform supports them.

Retained binaries are local ignored artifacts. Keep them for as long as their
evidence is needed; cleanup makes that evidence missing/stale. Git publication
alone does not distribute them to another machine. Export, remote storage and
automatic retention policy remain future work. The original v6 binary was not
retained and cannot be reconstructed from its hash. Independent builds need not
produce identical hashes, and this repair does not claim they do.

The full predecessor source scope remains mandatory for same-contract refresh.
A map may select `evidence_refresh_max_files` from 1 through 1024; omitted policy
keeps the 128-file default. The Watch and repair maps explicitly select 512 for
their full scope. All byte, containment and receipt limits remain effective.
Read the evidence-refresh guide before publishing immutable replacement receipts.

This runner is selected verification tooling, not a requirement for ordinary Recur
coding. Demo suites remain unselected. Integration tests start bounded Watch and
coordinator fixture processes and stop them at test cleanup. A produced result
needs review and explicit Warp gate acceptance.

defines: main.command.watch.eventness.evidence.workflow
