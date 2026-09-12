# Evidence-refresh verification

Observed 2026-09-12 on Windows x64, Rust stable 1.97.1, CARGO_INCREMENTAL=0.

- Tests preceded behavior: callable CLI stub produced 4 expected failures,
  extended process stub 5, bounded-reader stub 2 and publication stub 1.
  Exact logs and stub sources remain under evidence-refresh/. Compilation and
  missing-command failures were not counted as behavioral red.
- `cargo test --locked --offline --test warp_refresh --test warp_refresh_bounds
  --bin recur-warp`: 12 passed, 0 failures/skips; slice-1-final-tests.log.
- `cargo test --locked`: 251 passed, 0 failed; 7 pre-existing ignored doc examples;
  final-cargo.log. These ignored examples are not acceptance evidence.
- `cargo test --locked --offline --lib --bins --tests`: 251 passed, no failures
  or skips; final-regression.log is the checked integration observation.
- `cargo clippy --locked --offline --all-targets --message-format=json`: exit 0,
  existing repository warnings, no diagnostics in refresh implementation/tests;
  final-clippy.jsonl. This is not a warning-free or -D warnings claim.

New tests exercise preview purity, source drift, confirmed refresh and process
restart, idempotence, second-generation refresh, malformed/failed/skipped/zero
results, altered historical results, reduced input scope, wrong project/contract,
rejected layer, other-reference blockers, forks/orphans/cycles, changed gate
policy and conflicting accepted result hashes. Actual Windows junction escapes
are rejected. Reader limits are tested at/beyond JSON/input/aggregate/file-count
boundaries; directory count and prepublication byte revalidation are tested.
Private publication hooks interrupt staging and publication; exact retry recovers
without overwriting. Torn staging remains a bounded blocker.

The real dogfood operation published four same-contract refreshes for the
checked-transition Warp and one for the runtime-evidence baseline. Each uses
actual current test observations and retains every predecessor source path.
The shared integration manifest checks 104 actual inputs; the runtime baseline
checks 91, including its historically required local executable. The latter
will correctly become stale if that executable changes or is absent elsewhere.
The new resolver does not authenticate external producers or prove scope closure.

Captured restored-lang.json reports 4 covered, 0 pending/blocked/conflicting.
restored-runtime.json reports 2 covered, 3 pending, slice-2 ready and no blocked
gates. Julia loader errors and browser/integration gates remain unaccepted.
history-before.json was compared against all 20 named historical maps, accepted
layers, manifests and result files after refresh: no byte changes.

The old standalone evidence references remain stale; current acceptance comes
from explicit refresh lineage, never rewritten history or snapshot substitutes.
New behavior preserves exact gate contracts and dependencies. Generic Warp
refresh belongs to recur-warp; Lang header/body/footer, state and runtime tests
retain their separate meanings.

Git publication is a separate observation after test acceptance. The user
authorized commit, push and fast-forward to a.0.2.8. Scoped .gitattributes preserves
the actual fingerprinted checkout bytes across that operation. The current
handoff and final response record the actual Git outcome; no publication is
inferred from these passing tests.

Byte preservation leaves existing CRLF and mixed-ending files intact. The plain
Git whitespace check interprets preserved CR characters as trailing whitespace;
`git -c core.whitespace=cr-at-eol diff --check` exits 0 without normalizing or
rewriting evidence. The staged checkout-filter comparison checks actual bytes.
It found two pre-existing mixed-ending watcher files whose cached index entries
still normalized bytes; explicitly staging those exact bytes repaired the mismatch.
All 126 tracked evidence inputs/historical artifacts now reproduce the working
bytes through Git checkout filters. The baseline's local executable is deliberately
untracked. The scoped staged source/docs whitespace check exits 0; the all-files
check reports preserved whitespace in original command-output logs, which were
not rewritten to hide it.

defines: recur.warp.evidence.refresh.observed current tests and preserved history
