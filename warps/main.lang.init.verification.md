# Lang implementation verification — 2026-09-30

defines: recur.lang.init.verification implemented configuration and observed acceptance
defines: recur.lang.verification.repairs fourteen demonstrated faults repaired
consumes: recur.lang.init.v1 frozen initialization contract
consumes: recur.lang.verification.v1 bounded verification repairs

This pass implements `recur-lang init` and repairs the 14 failing demonstrations.
The base revision is `b4ade84`; the implementation is the commit containing this
report. Exact scoped source fingerprints and installed executable hashes are
in [the evidence directory](lang-verification/implementation-20260930/).
Observations were collected from a dirty implementation checkout before commit.

## Behavior

- Init previews or adds missing `[recur-lang]` preferences to the nearest project
  config. It preserves explicit choices, unknown data, comments and repeat bytes.
  Invalid fields, escaping paths, detected concurrent edits and publication
  failures preserve user data. The policy table cannot become a file lane.
- Queries recognize configured suffixes containing `.md` and return an empty
  recorded-Eventness list for CIR. Named-block discovery ignores comments and
  quoted strings without changing original source spans.
- Checked status binds the source identity, before/after paths and complete
  assessed-input set. Changed bytes make current evidence stale while preserving
  historical acceptance. Both durable records are checked when both exist;
  accepted-only recovery remains supported.
- Path tests import one shared helper. Valid adjacent and reversed routes pass;
  disconnected, malformed, missing, empty and singleton routes fail.

## Observed checks

| Check | Result | Original output |
| --- | --- | --- |
| Cargo complete suite | 257 passed; 7 existing doctests ignored; exit 0 | [Cargo](lang-verification/implementation-20260930/cargo.passed.log) |
| Julia complete suite | 5,458 passed; 73 existing expected-broken; exit 0 | [Julia](lang-verification/implementation-20260930/julia.full.log) |
| Installed verification demos | 58 cases passed; no failed/error/broken/empty cases; exit 0 | [Observation](lang-verification/implementation-20260930/installed.observation.json) |
| Installed init contract | 211 assertions passed; exit 0 | [Init](lang-verification/implementation-20260930/installed.init.log) |
| Cargo installation | All seven binaries replaced; exit 0 | [Install](lang-verification/implementation-20260930/install.log) |

Cargo includes four deterministic init publication tests: concurrent edit,
concurrent creation of an empty config, fresh-publication failure cleanup and
Windows sharing-lock rollback. Legacy core/Warp/Reveal init, queries, checked
transitions and recovery remain covered by the complete regressions.

The first Cargo attempt caught a regression that erased historical acceptance
when current inputs drifted. Its [failed log](lang-verification/implementation-20260930/cargo.attempt-0.failed.log)
is preserved. The subsequent fix retains history and invalidates current
evidence; the complete Cargo and Julia runs above then passed.

Use Rust 1.85.0, `release-safe`, one job, `CARGO_INCREMENTAL=0`,
`RUST_MIN_STACK=67108864`, and
`--config 'profile.release-safe.package.recur.codegen-units=1'` on this host.
The candidate directory is `target/release-validation-1.85/release-safe`.
Julia was Juliaup 1.12.7 with
`--startup-file=no -O0 -C generic --project=demos/web-evidence-lab`.
The default Chocolatey Julia and optimized Julia runs previously faulted;
this result does not certify those configurations. The path-test source had
line endings restored to its existing LF convention after the full run; the
installed focused run covers the shared helper after that restoration.

## Gate mapping and limits

`main.lang.init` retains its declared-evidence policy. Slice 0 reviews the
[frozen contract](main.lang.init.contract.md) and [original red baseline](main.lang.init.baseline.md).
Slice 1 uses the installed init suite's defaults, discovery, preservation and
repeatability checks. Slice 2 uses its invalid-input/path/lane checks plus the
four deterministic Cargo publication tests. Final uses the full regressions,
[CLI documentation](../docs/main.command.lang.init.readme.md), updated expert
guidance and installed smoke. Declared gate acceptance does not turn the 73
expected-broken assertions or seven ignored doctests into executed successes.

The verification Warp can accept its reviewed baseline, shared path helper and
exact status-identity slices using their scoped evidence. Query, parser and
record-consistency fixes are present, but their broader matrices remain open.
The V01–V32 todo, evidence integration, native CI/package coverage, exhaustive
poker tests and advanced contract-first Warps remain separate work. No Chocolatey
package is published, no planner is implemented, and no old stale acceptance
is silently refreshed by these observations.

Query live full-root Warp status to distinguish recorded completion from fresh
evidence. A `.todo.checked.md` filename is a human work marker, not an evidence
validator.
