# Runtime-evidence baseline freshness blocker

Observed 2026-09-12 after completing the independent checked-transition Warp.
Live `recur warp show main.lang.runtime-evidence -d . --json` reports blocked:
0 covered, 2 blocked, 3 pending. The historical slice-0/1 layers still exist.
This is current gate assessment, not deletion of historical observations.

The accepted baseline external manifest fingerprints these now-changed files:

- src/recur_lang_query.rs
- src/warp_evidence.rs
- target/debug/recur.exe

The shared Rust changes implement the authorized checked-transition capability;
the executable was rebuilt during verification. The old baseline therefore
cannot serve as current-source evidence. Slice-1 is dependency-blocked by slice-0;
the map's recorded slice-2 cursor is no longer ready. The separately recorded
Julia loader/runtime failures also remain unresolved.

Full captured query: checked-transition/runtime-evidence-preserved.progress.json.
The independent checked-transition Warp remains accepted complete with fresh
gates; it does not waive these runtime-evidence gates.

Next bounded action: inspect current `recur-warp evolve` help/source and freeze
a supported explicit baseline contract/evidence revision, preserving every old
receipt. Account for the rebuilt executable's freshness coupling. Do not simply
add another passing layer: an older stale checked gate can still block projection.
Do not rewrite the old manifest, substitute archived source, fabricate an
explosion or restore obsolete shared code to force acceptance. Only after the
supported revision and observed baseline checks restore dependency readiness
should the reduced Julia diagnostic and full loader suite resume. If existing
evolution cannot represent this revision, record that exact limitation and agree
on a separate bounded change before touching accepted contracts.

produces: main.lang.runtime-evidence.baseline-refresh-blocker stale live baseline
