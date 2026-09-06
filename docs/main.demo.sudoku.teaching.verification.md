# Sudoku teaching verification — 2026-09-05

Plan commit: `4b9d8af805a3e35040bf3a1a2fdfc211cbdc0ac3`, pushed to
`recur-lang` and fast-forwarded `a.0.2.8` before implementation.
Checkpoint: `ck-sudoku-teaching-plan-20260905`, `unix:1788675756`.
The Recur Git snapshot is state metadata, not a copy of file contents; private
lane/persona data from that snapshot is not published here.

Implementation was tested as an uncommitted working tree. Exact changed source
bytes are bound by `main.demo.sudoku.teaching.sources.json`; these are local-byte
hashes, so Git newline conversion can change them. No core Rust source changed.

## Executed checks

- `cargo test --locked`: passed, 179 tests; 7 ignored doctests. Existing unused
  `Element` variant warning remains.
- `julia julia-tests/runtests.jl`: 2,723 passed, 73 expected broken, zero failed;
  2m45.3s. Existing expectations were not weakened. This includes 139 new
  playable-generation assertions and the isolated watcher check.
- `julia julia-tests/runtests.demo.sudoku.teaching.jl`: 139 passed. Capped solution
  counting, invalid/ambiguous boards, bounded failure, three presets, technique
  replay, legacy preservation, failed publication and complete replacement.
- Browser harness: `python demos/sudoku/html5/tests/browser_test.py --package
  <temporary-generated-package>/sudoku.playable.json`: 47 deduction assertions,
  8 malformed-package rejection cases, and real keyboard/click interaction checks.
  Zero page exceptions and zero unexpected console errors. Two expected 404s
  (legacy package/favicon) and one deliberately injected generation 500 were
  counted separately, not silently represented as a clean network log.
- `python demos/sudoku/html5/tests/api_test.py`: real Julia HTTP generation,
  all 25/35/45-gap presets loaded by the browser, independently unique according
  to a separate Python solver, 81 cascades with nonempty produce/consume roles,
  zero page exceptions. Uses its own server, port, browser profile and output.
- `julia demos/sudoku/html5/generate.jl <temporary-output-directory>`: actual
  81-call Recur pipeline succeeded. Example package ID
  `e5b4bf3a89552de969e99b251e079c02b4e06ce489ebe3cf7f28565f71de580f`.
  All three examples happened to be naked-single-solvable; no hard-technique
  claim was inferred from their gap counts.
- `git diff --check`: passed.

Runtime: Windows, Julia 1.12.0, Python 3.14, disposable Playwright 1.62.0 venv,
installed Edge 152.0.4191.62. Recur binary SHA256:
`b86cce7237278f98adfdd51e076b2513dc9c886f713ded91a027e6bac1517a4e`.
Watcher binary SHA256:
`0bfc6c1f41861c4886734a9f2b7cb7df5178e774da66c43fc230d7ac0f55d87c`.

## Browser evidence and recovery

The synthetic board in `html5/tests/teaching.test.js` is transcribed from the
user's screenshot, not read from their live puzzle. Independent search verifies
one solution. It has 19 blanks and 12 naked singles. Browser tests follow:
select unresolved cell → ask for help → suggest easier cell without selecting it →
explicit jump → four progressive steps → accept value → refreshed pencil state →
manual note preservation → tentative entry → blocked hint → Backspace recovery →
finish → change preset → failed generation → successful reset.

Selection alone reveals no candidate or answer. Hints never place digits or
silently erase a tentative entry. Auto-fill gives 19 annotated cells; filling
r2c9 removes its row and reports 18. Final state reports zero remaining/annotated.
Focus remains on the progressive-step button through the conclusion.

Local, regenerable screenshots: `TEMP/sudoku-teaching-baseline.png` (original
candidate spoiler) and `TEMP/sudoku-teaching-current.png` (new UI; visually reviewed).
These are local artifacts, not checked-in or permanent evidence. The harness is
the durable reproduction method; see `demos/sudoku/html5/TEACHING.md`.

## Limitations and acceptance meaning

Only naked singles are certified by the new teaching flow. Legacy advanced solver
exports remain, but are not represented as verified teaching records. Presets now
label gaps; grading reports actual naked-single replay or ungraded. Legacy data
stays usable but is explicitly unverified. The browser checks package structure
and trusts the local Julia generator's uniqueness certification. Pre-generated
Eventness relationship data is not a live mathematical proof; temporary trace
paths do not imply durable source snapshots. Live deduction persistence is out of scope.

Warp layers bind reviewed evidence references. They are **declared evidence**, not
an independent execution of these commands by Recur. The separate docs-reconciliation
Warp remains pending. User's live puzzle files, running server and table were not
regenerated, replaced or restarted by these checks.
