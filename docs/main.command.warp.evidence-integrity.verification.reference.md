# Completion evidence integrity: final verification

Date: 2026-09-05
Source baseline: b34179dde5db61497f07b8a28fdbaa5610bd0510 plus the uncommitted implementation fingerprinted below.

## Observed regression results

Tests were run before production edits: Cargo 176 passed; Julia 2,292 passed,
73 expected broken. See main.command.warp.evidence-integrity.baseline.reference.md.

Final commands and outcomes:

- `cargo test --quiet`: 179 passed, zero failed; seven ignored documentation tests.
- `cargo build --profile release-safe`: exit 0.
- `julia julia-tests/main.command.warp.evidence-integrity.test.jl`: 108 passed,
  zero failed (16 map, 54 external-evidence, 12 receipt, 11 reveal-policy, 15 suffix/state assertions).
- `julia julia-tests/runtests.jl`: 2,400 passed, 73 expected broken, zero failed; exit 0.
- `cargo clippy --all-targets`: exit 0 with existing warnings.
- `git diff --check` and rustfmt checks for changed Rust files: passed.

The existing dead-code warning for EntryKind::Element remains. No new unexplained
regression was observed. These are retained human/agent-observed execution summaries,
not normalized machine-checked external evidence receipts.

## Integrated acceptance and negative cases

The focused suite exercises policy discovery, unsupported .verified diagnostics,
preview/confirmed policy-compatible lifecycle receipts, structured build/test/scan
evidence, checked whole-map gates and explicit reveal reconciliation. Negative cases
cover bare completion markers, zero tests, failed outcomes/nonzero exit, skips,
missing evidence/result files, byte drift, path escape, missing prerequisites,
stale contracts, conflicting results, empty maps/gates and stale reveal summaries.
Source comments and stable trace IDs connect these checks to each slice.

The external 50-test scenario is a synthetic normalized fixture, NOT a rerun of
BasicGameEngine or Visual Studio. Actual external runners remain responsible for
producing results. Checked evidence validates the declared files and artifact bytes;
it does not authenticate producers or detect unlisted files. Runtime FNV fingerprints
are noncryptographic. The SHA256 fingerprints below bind this recorded local run.

## Recovery and closeout

Use the release-safe binaries from this source when installed binaries are older:

```powershell
target/release-safe/recur.exe warp merge main.command.warp.evidence-integrity -d docs --json
target/release-safe/recur.exe reveal main.command.warp.evidence-integrity --json
target/release-safe/recur.exe files "main.command.warp.evidence-integrity.**.current" -d docs --sep .
target/release-safe/recur.exe trace-id "recur.warp.evidence.integrity.**" --scope "main.command.warp.evidence-integrity.**" -d docs --ext .md --format full
```

Final closeout observed nine accepted slices, no pending/blocked/conflicting/stale
slice, no current marker, and no reveal reconciliation warning. Trace recovery also
succeeded using the command above. This repair map uses
declared references to reviewed slice records; it does not claim checked-gate mode.
See main.command.warp.evidence.readme.md for opting other maps into checked evidence.

## Tested file SHA256

```text
src\lib.rs 4a3a45f774c79920219f0dd2900c2ad8531a26213d91a62c53d8292e7bf19d06
src\warp_policy.rs ee94c378e7b3695eea0ee44e15f7a325e5ea6858f823431bb8b552a481aa8af7
src\warp_evidence.rs d371a72c9f5c96656778bdc986941fe076715315eea5567ae63ec62f02907d7f
src\warp_bubble.rs 665d294901cbb177543799b49c9a95d58723df77861a7f2edd4586fc0d49a61f
src\main_command_warp_impl.rs 510f800a7f8e6039bf51ffc2f9fdc39480d712e97d6c5a90cff2d82039dd762b
src\main_command_reveal_impl.rs 6515350a134b45d0ba53cf4a53a4bb7869c2b7b5015ac47256aaa2de243588b4
src\recur_warp_main.rs a9ba5cf0b7cb7ac0367a9623ae59d8b5d8d1f6e6ffb1d55d377b3d2d94f83185
julia-tests\main.command.warp.evidence-integrity.test.jl b75632614344be80cc1684585839cd4cb75ba1eeb2b1c9c8cb5a69445d3b068d
julia-tests\runtests.jl 100c1d73e01da07686809ff39d590251364cb805b02565c4c9827a9eea3fec2e
```

defines: recur.warp.evidence.integrity.verification retained final source-bound regression evidence
