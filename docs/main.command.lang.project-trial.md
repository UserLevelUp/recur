# Recur Lang trial on the Recur repository

## Scope and method

Project: `C:/src/recur`. Evaluated the existing local debug executable, without
changing product implementation, project configuration, language sources or
historical receipts. KiCad was not accessed. The executable SHA256, selected
input hashes, UTC timestamp, commands, exit codes and complete query outputs are
in [observations.json](../warps/lang-project-trial/observations.json).

The [trial script](../warps/lang-project-trial/run.py) refuses to overwrite an
existing observation file. It checks existing programs and creates adversarial
copies only in a temporary directory. Benchmark samples use one warm-up and seven
subsequent separate process invocations per command. Times include process startup,
filesystem discovery and output serialization. This is one machine and a small
corpus, not an optimized release benchmark or a scalability claim.

## Observed behavior

43 recorded query invocations plus 24 timing/warm-up invocations produced 47
explicit expectation assessments: 45 met, 2 unmet. The unmet expectations remain
recorded as failures, not relaxed or converted into acceptance.

| Area | Observation |
| --- | --- |
| Discovery | 10 `.recur` sources, all under `demos/`; 6 supported and 4 rejected as unsupported 0.3 coordination inputs |
| Direct checks | All 6 supported programs exit 0 within fragment coverage; all 4 unsupported programs exit 2 with LANG005 |
| Exact selection | All 19 advertised function identities can be selected, each yielding exactly its requested header |
| Aliases and boundaries | `merge.f` retains its `bubble.o(b)` input alias and external dependency; expansion preserves body relationships |
| Missing join | A modified CIR1 await exits 1 with SGR004, including from a different selected lane |
| Dependency cycle | A modified CIR1 dependency exits 1 with SGR001; the global cycle remains visible from `test_bird` |
| Invalid selection | Ambiguous `f` yields LANG004; unknown qualified scope yields LANG003 |
| Root containment | An existing source outside the explicit root yields LANG002 |
| Query purity | Temporary adversarial fixture inventories remain byte-for-byte unchanged by queries; selected original repo inputs and executable hashes remain unchanged during the query phase |
| Worker semantics | Changing the Euclid output to the wrong constant still exits 0, with whole-source validation explicitly false: worker behavior is outside the static fragment |
| Eventness configuration | **Unmet:** the repo's `.complete.md` convention fails to classify/select a matching recorded completion |
| List JSON documentation | **Unmet:** README promises an empty CIR1 `recorded_eventness` array; output contains five scope entries with empty `records` arrays |

The negative fixtures are tests of query behavior. They are not actual agent runs,
executed worker algorithms, or accepted project runtime receipts.

The existing focused suites were also rerun with `CARGO_INCREMENTAL=0` and
`RUST_MIN_STACK=33554432`:

```text
cargo test --locked --offline --test lang_query --test lang_checked -j 1 -- --nocapture
```

Exit 0: 12 checked-evidence tests and 5 query tests passed, none failed or skipped.
Their [actual log](../warps/lang-project-trial/focused-tests.log) records the
individual expected/actual evidence verdicts. Passing those existing suites does
not cancel the two newly observed trial failures.

The later Cargo run rebuilt the executable, whose bytes differ from the binary
used for the query/timing phase. Both hashes are retained in the
[focused test observation](../warps/lang-project-trial/focused-tests.observation.json).
All other selected query input hashes remained unchanged. Timings and query
outputs are observations of the explicitly fingerprinted trial binary, not a
freshness assertion about a subsequently rebuilt executable.

## Timing and presentation

| Command scope | Median | Observed range |
| --- | --- | --- |
| `lang list -d . --json` | 236.7 ms | 232.7–242.1 ms |
| `lang show ... --scope gcd.f -d . --json` | 28.9 ms | 28.7–32.7 ms |
| Same function with `-d demos/main.lang` | 20.3 ms | 19.9–24.5 ms |

All timed commands exited 0. Narrowing the root also changes applicable root
configuration and recorded-file scope, so it is not a universally interchangeable
performance optimization. The normal overview was 32 lines / 1,454 bytes; the
readable merge view was 31 lines / 1,097 bytes. Full JSON for the scoped negative
CIR1 cases was about 20 KB, because global graph evidence remains included.

## Findings and bounded improvements

1. **Fix Eventness suffix interoperability first.** In an isolated copy of the
   actual repo configuration, `demo.algorithm.gcd.complete.md` is discovered as
   recorded evidence but its `state` is null. Filtering by `complete.md` returns
   an empty header with exit 0. Without that config, the default `complete`
   filter selects `gcd.f`. `state_vocabulary` removes a leading dot but retains
   `.md`, while `recorded` compares against `file_stem`, which has removed the
   file extension. Freeze how full filename suffixes map to semantic states,
   then add failing tests for both supported conventions before fixing it.
2. **Resolve the CIR1 JSON/documentation disagreement.** Empty per-lane records
   are not an implemented lifecycle. Choose and test one documented shape; retain
   the visible `unavailable (CIR1)` distinction.
3. **Make coverage easier to understand.** Four of the ten repository examples
   require unsupported syntax, and successful checks do not validate worker
   implementations. Label runnable/static examples and proposed design examples
   clearly. Improving this distinction does not require adding new grammar.
4. **Make navigation easier.** The compact overview omits function names, so a
   user must request JSON before knowing what to pass to `--scope`. A bounded
   symbol summary or copyable next command would reduce that extra step. This is
   a usability assessment, not a measured user-study result.
5. **Add one production-oriented dogfood specification.** All current programs
   live in demos; these results do not establish Lang coverage of Recur's Rust
   implementation. A bounded specification for an existing core workflow, such
   as typed Reveal selection, would test contract usefulness against actual
   source and observed tests. Keep that work within the existing grammar and
   preserve the query/companion execution boundary.

## Try the useful views

From the Recur repository root:

```powershell
.\target\debug\recur.exe lang
.\target\debug\recur.exe lang show demos/main.lang/main.lang.algorithm-lab.recur --scope merge.f
.\target\debug\recur.exe lang show demos/main.lang/main.lang.skippy-watch-coordination.recur --scope review_bird.f
.\target\debug\recur.exe lang check demos/web-evidence-lab/main.lang.evidence.recur
```

The last command checks the formal fragment, not the unfinished runtime evidence
loader. Existing Warp evidence staleness and the separate Julia loader blocker
remain separate from this exploratory trial. No Warp acceptance was recorded.
