# Final observed verification and re-entry

2026-09-12, Windows x64; Rust stable 1.97.1; CARGO_INCREMENTAL=0.
All commands below actually completed with exit 0:

| Command | Observed result | Artifact under checked-transition/ |
| --- | --- | --- |
| `cargo test --locked --offline --test lang_checked_reentry` | 4 passed, no failures/skips | final-reentry-tests.log |
| `cargo test --locked` | 240 passed, 7 pre-existing ignored documentation examples | final-cargo-tests.log |
| `cargo test --locked --offline --lib --bins --tests` | 240 passed, no failures/skips | final-regression-tests.log |
| `cargo clippy --locked --offline --all-targets --message-format=json` | exit 0; repository warnings retained | final-clippy.jsonl |

The no-skips checked regression gate binds the complete unit/binary/integration
run, while the full default run is preserved separately with its seven ignored
documentation examples. No required Lang test was ignored. Clippy emitted 148
warning messages across targets (including duplicates), with no diagnostics
located in the new assessor, actor or final re-entry test file. This is not a
warning-free repository or a `-D warnings` result. Static lint findings, observed
test results, evidence freshness and acceptance remain distinct.

The final tests use actual new recur/recur-lang processes. Ten independent
changes to source, implementation, test, config, runner, behavior, policy,
result, evidence manifest and attempt lose current acceptance without rewriting
the historical accepted file. Eight hand-authored durable interruption states
recover in fresh processes; slice-2 private callbacks separately establish that
those states are reachable at the actual mutation boundary. Dry recovery changes
no bytes, normal retries require recovery, exact replay preserves accepted bytes,
and original Eventness bytes survive the move. Case fixtures make producer claims;
their synthetic external counts are not claimed as production runner executions.
The real observed tests for the new behavior are the preserved Cargo red and
green logs from slices 1/2 and these final integration runs.

The first final test run, final-reentry-first.log, had 3 passes and one incorrect
test expectation: it demanded identical compact/expanded headers, despite the
frozen contract requiring expanded fields. The correction explicitly asserts
the expected input/output fields and exact remaining identities, contracts,
body, footer and coverage. No production behavior or accepted input was changed.
This was test-design feedback, not additional implementation red evidence.

## Recovered context

The re-entry artifacts preserve the actual funnel:

1. reentry-tree.txt and reentry-files.txt select Warp documents.
2. reentry-lineage.txt follows declared identifiers.
3. reentry-source-trace.txt locates run_hook but reports zero calls: this scanner
   result is not proof of no dependencies. Direct actor/source reads resolve
   assessment, read-set, publication and recovery boundaries.
4. reentry-help.txt exposes no recall command; reentry-reveal.txt selects the
   capsule. reentry-recovery.json is explicitly truncated and cannot project
   transitive checked evidence. reentry-recovery-focused.json narrows to the
   recorded handoff without truncation; `recur warp show ... -d .` separately
   provides live full-root acceptance. Recorded packets remain historical.
5. reentry-lang.json preserves a mistaken source-name LANG001. Live `lang list`
   resolved actual main.greeting.recur / greeting.g; reentry-lang-greeting.json
   and reentry-lang-greeting-expanded.json preserve the corrected scoped views.
   They retain unsupported coverage and do not claim runtime checks.

Usage is in docs/main.command.lang.checked-transition.readme.md. Original
query-v1 and legacy companion semantics remain intact. Expert skill, playbook
and reveal guidance now point to the additive mode, limits and current maps.
Local target/debug binaries expose the delivered commands; no installed tool
was replaced. Header contracts and behavioral requirements, body aliases and
boundaries, intended Eventness, recorded status, static findings, external
observations, current fingerprints and final ACK are separate report facts.

Earlier slice-1 and slice-2 manifests were rechecked against actual source and
remain checked. No historical receipt or accepted Rust input was revised.
The independent Julia/runtime-evidence Warp remains open; no Julia/HTTP/browser
success, new grammar, live coordination, installation or publication is claimed.
Compiler access violations/heap corruption remain recorded unresolved incidents;
successful retries do not establish their cause.

defines: main.lang.checked-transition.final.observed regression recovery and freshness
consumes: main.lang.checked-transition.slice-2.observed checked actor evidence
