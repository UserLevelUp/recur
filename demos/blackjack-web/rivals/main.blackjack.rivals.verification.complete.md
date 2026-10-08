# Optional rivals: accepted bounded increment

defines: demo.blackjack.rivals.acceptance observed completed increment
consumes: demo.blackjack.rivals.verification reviewed observed integration
consumes: demo.blackjack.rivals.browser.acceptance production UI observations
consumes: demo.blackjack.rivals.review current source-bound author assessment

Accepted 2026-10-06. Optional zero/one/two computer rivals are available through
New table. Rivals share Ada and one deck, retain separate wallets/results, and
play a labeled public-information Stand on 17 benchmark. Human Split and Double
remain available. Solo retains v2; rival mode uses guarded v3. This introduces
a usable benchmark and observed comparisons, not a player skill rating.

Actual CLI dogfood: two concurrent Codex workers produced engine and browser
outputs. The root coordinator reviewed them, then published their completion
gates with `recur-warp complete`. `recur-watch dispatch` coordinated their attempt
records and later a dependency-ready Node verification job. It polls and returns
at quiescence; acceptance still requires evidence review. Model/configuration and
script-policy host failures remain recorded and did not count as test failures.

Final observations:

- Integrated Julia website: 16,080 passing assertions, including 1,193 rival
  engine assertions, 43 new HTTP assertions, original solo/Split regressions and
  76 current source/reference checks. Node: eight passing browser tests.
- Full repository Julia run: 35,417 passed, 73 expected-broken, exit0, with matching
  explicit local Recur binaries and installed Julia1.12.7. Raw output retained at
  `C:/Users/marcn/Documents/Codex/2026-10-06/le/work/recur-rivals-regression.log`.
- Full Cargo suite: passed after coordinator workers exited. Twelve focused Warp
  query unit tests and75 discovery assertions include parent/child separation.
- Trace comments were normalized to the configured `publish`/`consumer` spellings
  after the broad run. The final integrated website/Node launcher and bounded
  trace query then passed again. `observations/website-final.log` records this
  current-source rerun. The review registry reports zero hash mismatches.
- Production browser: one and two rivals, automatic actions/settlement, privacy,
  reload preservation, no console errors/warnings and no horizontal overflow at
  the tested narrow viewport. See `observations/browser-acceptance.md`.

The live server runs on `http://127.0.0.1:8794`; original8791 was not interrupted.
Desktop and narrow screenshots were inspected. Source/behavior author review
and hashes are in `main.blackjack.rivals.review.md`/`.json`. Initial worker and
integration evidence remains separately queryable in observations and `.recur`.
Original Split receipts were not rewritten or retroactively refreshed.

Known Windows Julia inference diagnostics occurred during testing; the malformed
Boolean-deck fixture construction was adjusted without removing its assertion.
The successful runs above reached their complete expected assertion totals.

Gate evidence is declared, source-bound author-reviewed observations rather than
machine-checked dependency closure or independent proof. Lang remains optional;
existing fragments do not model or prove the whole rival implementation. Follow-on
policies, normalized skill metrics and persistence are future Warp candidates in
`main.blackjack.rivals.improvements.todo.md`. No install, commit or push performed.
