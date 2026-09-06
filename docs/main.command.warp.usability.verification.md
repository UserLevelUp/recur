# Usability Warp verification — 2026-09-06

Local implementation on base commit 17626c8b94f326a949b46c22b515d0863c1af7ac,
including the previously uncommitted companion-policy foundation. Not a published
revision or a claim about the globally installed recur binary.

Observed commands and results:

- Initial new CLI suite: 14 pass, 1 expected red failure (create absent).
- `cargo build --profile release-safe --locked --bins`: successful, including final formatting.
- `cargo test --locked`: 179 passed, 0 failed; 7 documentation tests ignored.
- `julia julia-tests/main.command.warp.usability.test.jl`: 85 passed, 0 failed.
- `julia julia-tests/runtests.jl`: final run 2941 passed, 73 known broken,
  0 unexpected failures, 1m17.1s. Earlier run before additional comparison cases:
  2922 passed, 73 known broken.
- Final documentation reconciliation standalone after adding roadmap links:
  54 passed, 0 failed.
- `git diff --check`: passed (Git emitted normal Windows line-ending warnings).

Tests use temporary projects for writes. Queries are checked for byte preservation;
complete/partial/exploded projections agree with existing merge fixtures. Invalid
templates, gates, cycles, current IDs, portable names, path escape, duplicate
creation and ambiguous discovery are refused. Existing Rust, Julia and Sudoku
coverage remains enabled. Known broken assertions were not disabled or converted
into passing assertions.

Boundaries: single JSON map only; hard-link-capable filesystem required for atomic
no-overwrite publication. No adversarial concurrent filesystem/security guarantee,
multi-file transaction, semantic repartition or broader lifecycle setter is claimed.
Current slice is optional map metadata, not inferred from filename eventness.
Completion receipts for this Warp are reviewed declared evidence, not automatically
executed producer evidence. No global install, commit, push or branch movement.
