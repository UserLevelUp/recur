# Slice 2 observed verification

2026-09-12, Windows x64, Rust stable 1.97.1, CARGO_INCREMENTAL=0.
Command: `cargo test --locked --offline --test lang_checked_transition --bin recur-lang`.
Actual exit 0; 20 passed, 0 failed, 0 skipped, including 7 unchanged legacy tests.
Log: `checked-transition/slice-2-accepted-tests.log`.

Historical behavioral red: five process failures in slice-2-red.log against
the archived callable slice-2-red-seam.rs.txt; two missing mutation/fault hooks
in slice-2-fault-red.log; missing publication interruption in
slice-2-publication-red.log; canonical Windows E0 alias in
slice-2-path-alias-red-retry.log. Original process-test SHA256 before additions:
EA85C169323887D039AF73C430AB8A9A2926EDA3363F14159B95B4B7F2287A41.
Initial stub SHA256: F0E6374C1E73EE2850D5EC4571D1D65BD169AE4645EBFE34345ED6B4F1347278.

slice-2-path-alias-red.log instead records rustc STATUS_HEAP_CORRUPTION;
it is a compiler failure, not behavioral red. The unchanged retry reached the
expected failing assertion. Incremental compilation was disabled for both,
so it is not an established fix or root cause. Earlier runtime crashes remain
unresolved; no Julia or browser success is claimed.

New behavior: bounded assessed read-set revalidation, exact E0/Ef paths,
no-clobber prepared/accepted publication, explicit recovery, exact-byte replay,
current drift distinct from historical acceptance. Tests include real processes,
Windows case aliases and an actual junction escape. Private fault callbacks
cover staged/published intent, E0/Ef link, E0 unlink and staged/published status.
No commands or bindings execute. Recovery blocks changed artifacts/evidence,
torn staging, unexpected files and conflicting requests without overwriting.

The filesystem contract assumes exclusive operator ownership during the short
write. Files are synced; this is process-interruption recovery, not a claim of
host-power-loss durability across every filesystem. Unsupported hard links fail
without removing E0. Staged bytes are preserved on failed publication.

Acceptance uses actual Rust implementation/test/config inputs in the checked
manifest; archived stubs are historical observations only. These actor and test
inputs freeze on acceptance. Final integration tests belong in a new file.

defines: main.lang.checked-transition.slice-2.observed checked actor evidence
