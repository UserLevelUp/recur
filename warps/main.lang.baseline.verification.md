# Recur Lang baseline verification

Candidate implementation: `7d042c4` on `a.0.2.8`, 2026-09-08.
Portable regression correction: `cc7adee` (no Rust or Cargo source changes).
All six slices have been reviewed against the evidence below. Final integration,
archive and nupkg checks passed; live layers record declared acceptance.

## Starting evidence

Baseline commit: `c97e32666314243c07e36fd4308541b8d9fe5ced`.
Installed binary: `C:/Users/marcn/.cargo/bin/recur.exe`, 0.2.8.
Previously built binary: `C:/src/recur/target/release-safe/recur.exe`, 0.2.8.
Both lacked the `recur lang` command. `recur-lang warp` was already implemented;
its source remains unchanged in this candidate. `recur warp` and `recur warp list`
both discover remaining declarations; installed help is not the source of truth
for later builds of the same package version.

The existing Rust WIR1/CIR1 suite passed 15 tests using Rust 1.85.0 with
`CARGO_TARGET_DIR=target/lang-1.85 CARGO_INCREMENTAL=0 cargo +1.85.0 test --locked
--lib recur_lang -j 2`. Focused Julia fixtures passed with their existing known
broken allowances under Julia 1.12.7, `--startup-file=no
--project=demos/web-evidence-lab`. See the `rust-baseline.txt` and
`julia-baseline.txt` evidence files.

`julia-tests/main.command.lang.baseline.test.jl` against the pre-implementation
CLI produced the expected missing-command red baseline: 1 pass, 4 failures.
`graph-baseline-red.txt` reconstructs a graph API acceptance probe against an
isolated archive of `c97e326`: compilation fails with E0432 for the missing
`recur_lang_graph` module. This is a retrospective baseline reproduction, not
an assertion that the probe preceded implementation. The live implementation's
graph tests exercise the same five-lane/three-wait result plus negative graphs.

The exact CLI, exit codes, schemas, supported fragments, config lookup and read
boundary are frozen in `docs/main.command.lang.query.readme.md` under contract v1.

## Observed local regression

Rust 1.85.0 completed full regressions on Windows and Linux during development.
The preserved Linux log records the test cases. Companion tests include confirmed
success, stale or rejected receipts, unknown IR schema and outside-root rejection.
The final candidate is additionally tested by the native CI run below.

The Windows full Julia runner passed **3,722 assertions, 73 known-broken**, using:

```powershell
$env:RECUR_BIN='C:/src/recur/target/lang-1.85/debug/recur.exe'
$env:RECUR_PROFILE='lang-1.85/debug'
& C:/Users/marcn/AppData/Local/Microsoft/WindowsApps/julia.exe --startup-file=no --project=demos/web-evidence-lab julia-tests/runtests.jl
```

This run preceded the final compact-footer presentation and enum-layout changes;
those require final candidate coverage. No existing broken allowance was removed.
One obsolete Julia assertion was updated to preserve the user's revised companion
ownership statement. New tests exercise actual query behavior and are integrated
in the normal runner.

Local Windows and WSL compilers intermittently failed inside rustc/LLVM, including
unchanged dependencies. These crashes are not passing evidence. The Windows
candidate uses the existing `release-safe` profile; Linux uses `release`.
Neither requires changing the user's default Rust toolchain.

## Native candidate acceptance

Manual run: https://github.com/UserLevelUp/recur/actions/runs/34216797900

The first CI job status was misleading: its `tee` pipeline swallowed the Julia
failure exit. Actual logs show 3,679 pass, 16 fail, 3 errors and 73 broken. Failures
came from an ignored synthetic Warp config and Sudoku flow fixture, a checkpoint
test reading local private lanes, Windows-only path/mtime assumptions and a hardcoded
watcher executable. `cc7adee` checks in the intended synthetic fixtures, uses a
temporary lane project, uses native paths/mtime and the selected watcher binary,
and explicitly enables Bash pipefail. No legacy assertion was downgraded to broken.
The focused corrected Windows tests passed (`portable-fixtures-windows.txt`).
Corrected run: https://github.com/UserLevelUp/recur/actions/runs/34217859504

The corrected run passed on `cc7adee`: **200 Rust tests**, **3,727 Julia assertions**,
**73 existing known-broken Julia cases**, and **7 existing ignored Rust doc tests**.
Rust 1.85.0 and Julia 1.10.12 ran in a fresh native Linux checkout. Both archive
jobs passed; release and Chocolatey publication jobs were skipped. The actual
logs contain no test failures/errors. No Lang module or new integration-test
Clippy warning appears; existing unrelated lint warnings remain.
See `rust-ci-final.txt`, `clippy-ci-final.txt`, `julia-ci-final.txt` and
`ci-final.json`. This establishes final integration acceptance independently of
the first run's misleading job status.

The manual release workflow builds and smoke-tests both native archives and runs
full Cargo/Julia regression. Release creation, Debian packaging and Chocolatey
publication are restricted to tag-push runs; this invocation performs none of them.
The packaging script checks all six declared binaries after extracting the actual
archive, verifies 0.2.8 versions/help, runs a pure alias query and concurrent check,
and exercises `recur-lang warp` in dry-run mode while checking source inventory.

Both native archive jobs passed and their downloaded SHA256 values match their
smoke receipts. Windows uses Rust 1.85.0 `release-safe`; Linux uses Rust 1.85.0
`release`. All six declared executable names and README are present in each archive.

| Artifact | SHA256 |
| --- | --- |
| Windows zip | `4064e91df2c8c3ab792db9b5d966270cffd9e4196a1b3a1f3da690cf6334d034` |
| Linux tar.gz | `40ae9aafa232786da25406c69ef957ca4ff2fc71c77da3498bf28b1cc8937dd6` |
| recur.0.2.8.nupkg | `7456d0049062bdc0241058f64fcbd76fdb84e9d69901c3f339ff8ee76806a2f5` |

`scripts/pack_choco_release.ps1 -Version a.0.2.8 -ZipPath <downloaded-windows-zip>
-OutputDirectory target/lang-final/choco` passed with Chocolatey 2.6.0.
`scripts/test_lang_chocolatey.ps1` exercised the actual packed install/uninstall
scripts using mocked Chocolatey helpers, checking version, URL, archive checksum
and all five companion shims. No software was installed/uninstalled or published.
The nupkg is a download package bound to the exact Windows zip above; submitting
it requires that archive at its declared release URL. Rebuilding an archive
requires repacking the checksum-bound nupkg.

The extracted Windows candidate passed all 15 new CLI assertions and the focused
Julia language fixtures under Julia 1.12.7. `julia-packaged.txt` records the results.
The package reports preserve per-file hashes, command arguments, outputs and source
fixture hashes. The table identifies the corrected run's `cc7adee` candidate,
whose compiled Rust/Cargo sources match `7d042c4` exactly. Local artifacts are
under `target/lang-final/windows`, `linux`
and `choco`. Durable receipts are in `warps/main.lang.baseline.evidence/`.

## Gate review

SGR1's authority remains the exact acceptance document initially named
`docs/main.improvement.30.static-graph.todo.current.md`. Its analyzer consumes
CIR1, never source text. Nodes/messages/waits preserve authored order; cycles and
set comparisons use deterministic order. Source/schema mismatch, cycles,
unreachable lanes and unsatisfied joins are exercised by focused Rust tests.

Baseline tests prove exact alias identity, distinct local-letter scopes, fan-in
members/expansion, deterministic JSON, Unicode paths/descriptions, root isolation,
unchanged source/config/receipt inventory and default/custom recorded Eventness.
Scoped projection retains the entire selected CIR1 flow's analysis and boundary
references. A filtered graph cannot become sound by hiding an outside cycle.

Static review of `src/recur_lang_query.rs` and `src/recur_lang_graph.rs` confirms
there is no subprocess, worker, file writer, scheduler or repair recommendation.
The main dispatch only prints a result and exits. Whole-document grammar, worker
execution, coordinator input signatures and CIR1 lifecycle remain explicit limits.

The SGR1 implementation commit must precede its manual Eventness transition.
Parent baseline acceptance will separately cite the child projection and exact
SGR1 contract; merely completing the child cannot accept baseline slice-1.

The child now has 4/4 covered slices, `declared-gates-satisfied`, with no conflicts
or pending gates. `sgr1-accepted.json` captures that projection. Parent review
checked the original SGR1 contract SHA256
`3e71f9d25d999d5d87bd81520ab121918c314943b374e97dda1380c898df790a`, the CIR1-only
analyzer implementation, exact fixture topology and negative/determinism tests.
This establishes baseline slice-1's explicit prerequisite review independently.

| Baseline gate | Reviewed evidence |
| --- | --- |
| baseline-and-cli-contract | Pre-implementation Rust/Julia and CLI red logs; query contract v1 |
| static-graph-prerequisite-reviewed | Exact original SGR1 contract, separate SGR1 verification and 4/4 child projection |
| source-bound-model-projection | Rust model/hash/schema refusal and deterministic WIR1/CIR1 projections; same SGR1 report |
| pure-list-show | Native CLI tests: supported/unsupported/empty inventory, aliases, Unicode, ambiguity, root isolation and unchanged file inventory |
| lossless-header-body-footer | Exact aliases and fan-in tests, identical compact/expanded graph identities, text relationships and capability notice |
| scope-eventness-and-diagnostics | Default/custom recorded-state tests, crossing-scope cycle test, missing-join exit 1 and malformed-input exit 2 |
| regression-and-companion-boundary | Corrected native CI: 200 Rust tests, 3,727 Julia assertions, 73 preserved broken; companion's seven tests passed; no new Lang Clippy diagnostics |
| packaged-lang-smoke | Both actual archives, extracted help/queries/dry-run and checksum-bound nupkg/mocked shim checks |

Writer commands use explicit `-d docs` and `-d warps` roots to avoid the isolated
baseline source archive under `target/`; child and parent are reviewed separately.
Root-level read-only projections still discover both accepted bubbles.

defines: recur.lang.baseline.verification observed evidence and explicit applicability
consumes: recur.lang.query.v1 frozen pure query contract
consumes: recur.lang.static.graph.report.v1 bounded shared graph contract
