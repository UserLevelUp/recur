# Observed red and completed plan

2026-09-12. Contract completed before implementation; map at repository root.
Actual callable process stub: slice-0-red.log, exit 101, 1 passed / 4 failed.
Expanded process cases: slice-0-bounds-red.log, exit 101, 1 passed / 5 failed;
Cargo stopped at that target, so the reader suite was run separately.
slice-0-reader-red.log: 2 failing boundary tests. slice-0-publication-red.log:
1 failing interruption/retry test. These compile and reach behavior assertions.
Stub source artifacts are preserved under evidence-refresh as historical inputs.

Cases cover immutable history, actual source drift, checked replacements,
producer/result validity, unchanged contract/scope, multi-reference blockers,
chains/forks, traversal/junctions, collection limits and no-clobber publication.
Expected red is not checked acceptance. This slice accepts the completed plan
and observed red only. Implementation and final gates require actual green tests.

The authorized source changes will stale some prior checked-transition evidence;
after all current tests pass the new explicit renewal path must restore those
gates without editing their historical files. Git was already on a.0.2.8, so
feature branch evidence-refresh was created for a reviewable fast-forward.
