# Current acceptance evidence, October 7, 2026

Root actually ran these checks; this is not inference from static source.

- Git HEAD 2ecae66df03c6e7a4dd783ba3e7bd389e53d245a was extracted with git archive
  into a separate directory. Seven prepared dispatch integration tests all failed
  there (exit 101, missing commands). Current source initially passed those seven.
  This is reconstructed differential evidence, not original chronological TDD.
  Root proposes explicitly changing the pending tests contract to v2 and gate to
  baseline-differential; no historical red-first receipt will be invented.
- Root appended the independent expansion tests before changing expand: four
  failed and five passed (exit 101). After integrating the proposed single-pass
  replacement, all nine passed (exit 0). This fix has actual new red/green evidence.
- Root then resolved R5, R6, R9, R15 and R16: original_outcome survives repeated
  recovery for feedback; startup, confirmed recovery and terminal publication
  share an exclusive per-attempt transition fence; startup reads claimed after
  taking it; live-worker recovery stays forbidden; timeout retains nullable exit,
  termination errors and log fingerprints; bound context is checked again at
  verification endpoints; completed test results are persisted incrementally;
  later invocation errors become verification_error and preserve earlier results.
- New deterministic tests exercise the shared fence, inability to start a recovered
  claim, repeated recovery feedback, context-only mutation, first verification
  success followed by missing executable, timeout observation fields and positive
  prerequisite acceptance/final dispatch.
- Full current Cargo run: 287 passed, 0 failed, 7 ignored doctests, exit 0. Includes
  9 dispatch integration tests, 9 expansion tests and the transition fence test.
  The workspace snapshots reflect this tested implementation.
- Default Julia core with matching release binaries: 4368 passed, 73 expected-broken,
  4441 total, exit 0. Root is repeating it against the final rebuilt companions
  and will require success before publication. Demo suites remain unselected.
  Blackjack-web dry-run selected only that demo and skipped core. Gameplay unchanged.
- Actual Copilot 1.0.93 review at medium: 151 seconds agent, 152 claim-to-finish.
  Actual Codex 0.160.1 / gpt-6-astra high independent review: 279 seconds agent.
  Independent review recommended revision of earlier source; root resolved defects
  above. Gemini 0.62.0 reached OAuth provider but received UNSUPPORTED_CLIENT;
  no Gemini review exists. Reviewed availability disposition is separate.
- Copilot initializer --effort corrected to --reasoning-effort; custom settings
  preserved. Actual Copilot job used a tested bounded file handoff adapter.

Remaining limits: trusted local records and host permissions; working directories
are not sandboxes; Windows-targeted runtime tests; polling/files rather than a
bidirectional live bus; endpoint hashes miss change-then-restore; manual stale-lock
owner inspection (including transition-lock owner crashes); host-owned descendants
after normal exit; per-command timeout; large argv requires supported stdin/file
transport; attempt budgets span contract edits; empty reasoning-level adapters need
a compatible default interface. Bounded terminal retries are not test failures.
Broader robustness and portability coverage belong to later Warps.

Raw logs under .recur/dispatch-acceptance; handoffs and reviews under
warps/dispatch-review/observations. The observer uses recorded claim/finish Unix
seconds and worker duration plus a 500ms polling observation. Restarting it does
not inflate elapsed time. Lane/Eventness handoffs have hierarchical trace IDs;
capture does not accept gates. Recur-git checkpoint --snapshot reports state and
appends a checkpoint; it does not create a recoverable Git commit.
