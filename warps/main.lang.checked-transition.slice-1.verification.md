# Slice-1 checked assessment and query

Observed 2026-09-12. New shared Rust assessor, callback-based reuse of the existing
external checker, and additive `recur lang evidence` query implemented. Legacy
query-v1 commands retain their behavior. New reports use an explicitly unscanned
recorded inventory, original WIR1 projector, requirements, immutable observation,
current verdict and separately qualified historical/current transition status.

Red: eight executable Rust test groups compiled and failed at the callable
assessor seam; log slice-1-red.log. Initial stub is retained as historical text
slice-1-red-seam.rs.txt, not a live input substitute. Original test SHA256:
fe3780415ec4b5b759903032c9a281bb05c4991097d0868609f61bd810bc0e38.
Additional coverage found two actual omissions before fixes: changed Ef still
looked currently accepted (slice-1-status-red.log), and an unknown qualified
function was malformed instead of mismatched (slice-1-unknown-red.log).
All original assertions remain, with added limit/process/status cases.

Final command: `CARGO_INCREMENTAL=0 cargo test --locked --offline --test
lang_checked --test lang_query`, exit 0. Log slice-1-accepted-tests.log:
12 new test groups plus 3 legacy query tests passed, zero failed/ignored.
Groups execute the authored identity/count/drift/path/limit cases and prove
actual CLI purity, Unicode paths, repeated scoped letters, canonical alias and
boundary preservation, Windows junction rejection, exact byte/file/requirement/
case thresholds, and historical-versus-current status. Existing checker unit
test also passed separately after reader refactoring.

`cargo clippy --locked --offline --lib --test lang_checked --message-format=json`
exited 0 (slice-1-clippy-final.jsonl). Strict -D warnings earlier reported existing
repository lints plus a new MSRV warning; the new MSRV issue was fixed. Existing
unrelated warnings remain; no blanket lint suppression was added.

One initial Rust compilation exited STATUS_ACCESS_VIOLATION. Disabling
incremental compilation allowed subsequent runs to complete. This is an observed
workaround, not a proven root cause of Rust or earlier Julia failures.

Structured slice-1.result.json/evidence.json were generated from the completed
green log by record_gate.py. Actual command exit was independently observed as
0; the recorder parses counts and does not execute tests. `recur warp evidence`
returned checked with 12 explicit actual input fingerprints. Runtime identity is
recorded provenance; this scope does not claim whole-repository dependency closure.

Next: freeze these accepted inputs; implement companion checked mode in separate
binary-owned files after transition tests are observed red. No production checked
transition has been applied by this slice. Existing runtime-evidence gates remain
separate and historical source-based receipts may now assess stale after changes.

produces: main.lang.checked-transition.assessment observed checked query behavior
