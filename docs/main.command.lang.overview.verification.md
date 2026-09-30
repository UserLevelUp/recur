# Bare Lang hierarchy verification

## Contract and inputs

Bare `recur lang` selects list mode. Its normal output groups `.recur` sources
by directory and authored filename prefix; JSON retains list-v1 fields and adds
per-scope `recorded_eventness`. All discovered programs remain visible, including
unsupported inputs and programs without recorded state. This is discovery, not
runtime activity or an active-only lifecycle filter. `--help` retains the menu.

Implementation: `src/main.rs`, `src/recur_lang_query.rs`.
Behavior tests: `tests/lang_query.rs`, using the existing
`demos/main.lang/main.lang.algorithm-lab.recur` and
`demos/main.lang/main.lang.skippy-watch-coordination.recur` fixtures, an explicit
unsupported version, a recorded `demo.algorithm.gcd.todo.current.md`, an excluded
`target` fixture, and temporary empty/missing roots. Tests compare file inventories
before/after queries, retain diagnostics, and compare bare/list JSON results.

## Observations

Commands below used `CARGO_INCREMENTAL=0`, existing installed toolchains and
`--locked --offline`. No tool installation or binding execution was performed.
Logs are under `warps/lang-overview/`.

1. Before implementation, `cargo test --locked --offline --test lang_query
   bare_lang` exited 101: both new tests failed because bare Lang required a
   subcommand (CLI exit 2). Log: `lang-overview-red.log`.
2. After implementation, the default build, single-job default build, and
   single-job `--profile release-safe` build each exited 101 because `rustc`
   terminated with `STATUS_ACCESS_VIOLATION` (0xc0000005) while compiling the
   library. No tests ran. Logs: `lang-overview-green.log`,
   `lang-overview-green-2.log`, `lang-overview-release-safe.log`.
3. With `RUST_MIN_STACK=33554432`, `cargo test --locked --offline --test
   lang_query -j 1` exited 0: 5 passed, 0 failed, 0 ignored. Log:
   `lang-overview-stack.log`. This success does not establish the cause of the
   earlier compiler crashes.
4. With the same environment, `cargo test --locked --offline -j 1` exited 0:
   253 passed, 0 failed, 7 existing ignored documentation examples. Log:
   `lang-overview-regression.log`.
5. Local `target/debug/recur.exe lang` displayed 10 discovered sources. Explicit
   roots `demos/main.lang` and `demos/web-evidence-lab` displayed 2 and 3 sources.
   `lang --help` retained the command menu. The executable on PATH remains the
   installed version; run the local executable to exercise this change.

## Evidence freshness and handoff

No historical receipts were rewritten and no Warp acceptance was recorded for
this UI change. The modified source/doc/test inputs make earlier checked evidence
stale. Live local queries after the change report:

- `main.lang.runtime-evidence`: 0 covered, 2 blocked, 3 pending.
- `main.lang.checked-transition`: 1 covered, 3 blocked.
- `main.command.warp.evidence-refresh`: 2 covered, 1 blocked.

The next bounded evidence action is to assess unchanged gate contracts and bind
the actual new regression observation through explicit immutable refresh receipts,
retaining every predecessor input. These test results alone do not refresh those
gates or close the separate Julia loader blocker. No commit, push or installation
was performed for this change.
