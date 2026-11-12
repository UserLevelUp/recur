<!-- recur-trace-v1
defines: main.command.trace_id.scan_coverage.contract
consumer: main.command.trace_id.scan_coverage.verification
-->
# Additive budget-starvation diagnostics, opt-in directory exclusions, and trace-v1 guidance

artifact.type = contract
warp.id = main.command.trace-id.scan-coverage
status = prediction; acceptance requires observed evidence

## Initial Eventness

At `f0c9745`, evaluating bird's-eye `recur trace-id` discovery on `C:/src/kicad_ra` surfaced three scan-coverage and expectation traps in `src/trace_relationships.rs` and `src/trace_inspection.rs`:

1. **Silent alphabetical directory starvation under `byte_budget` / `entry_budget`**:
   - `trace_relationships::scan` walks `WalkDir` in deterministic lexicographical path order (`sort_by_file_name`). In `C:/src/kicad_ra`, `exports/` (alphabetically before `scripts/`, `trace/`, and `warps/`) contained large SVG/HTML/JSON artifacts that exhausted `byte_budget` early.
   - Coverage reported `"complete": false` and `"omissions": { "byte_budget": 1 }`, and stderr printed `byte_budget: stopped after reading ... bytes`, but **neither JSON nor stderr revealed where traversal stopped or that `scripts/`, `trace/`, and `warps/` were 0% visited**.
2. **No opt-in directory exclusion for generated export/data trees**:
   - `visible()` in `src/trace_relationships.rs` hardcodes `["target", "target2", "node_modules", "build", "dist"]`. Repositories with generated `exports/`, `renders/`, or `vendor/` folders had no CLI flag (`--exclude-dir <NAME>`) to skip heavy artifact folders without inflating `--max-bytes` or running per-folder `-d` loops.
3. **`--declared` `<!-- recur-trace-v1` opacity & `--list-ids` filename-hyphen confusion**:
   - `recur trace-id --help` described `--declared` as `Show explicit structured trace declarations and pins` without mentioning that it exclusively parses Markdown files starting on line 1 with `<!-- recur-trace-v1` and dot-separated `[A-Za-z0-9_]` identifiers (`valid_id`). Consequently, agents expected `.warp-map.json` `"defines"` arrays to appear under `--declared`.
   - `--list-ids` extracts content tokens allowing at most one internal hyphen per segment (`slice-0`), so prose references to filenames like `robo.prototype.refinement.sol.slice-0-baseline.complete.md` surfaced truncated token `robo.prototype.refinement.sol.slice-0` without docs explaining how `recur files "<BASE>.**"` complements `--list-ids`.

## Strict backwards-compatibility contract

1. **All existing JSON schemas and `coverage` fields are preserved**:
   - `recur-trace-id-paths-v1`, `recur-trace-id-declarations-v1`, `recur-trace-id-inventory-v1`, and `recur-trace-id-context-v1` retain every existing key and type (`complete`, `entries`, `files_read`, `bytes_read`, `max_entries`, `max_bytes`, `omissions`).
   - New starvation fields on `coverage` (`stopped_at`: `Option<String>` root-relative path where `entry_budget` or `byte_budget` halted, and `unreached_top_dirs`: `Vec<String>` sorted root-level directory names under `-d` that had zero entries visited before termination, serialized with `#[serde(skip_serializing_if = "...")]` when `None` / empty) are **strictly additive**: when `complete == true`, serialized JSON is byte-for-byte unchanged!
   - On incomplete scans (`complete == false`), stderr `Trace-ID discovery incomplete:` keeps the existing `reason: stopped after ...` lines (preserving all assertions in `julia-tests/main.improvement.34.trace.test.jl` and `main.improvement.34.trace-context.test.jl`) and appends concise `stopped_at` / `unreached_top_dirs` + recovery hint lines (`--exclude-dir`, `--max-bytes`, or `-d <subdir>`).
2. **Opt-in `--exclude-dir <NAME>` (repeatable)**:
   - Default directory exclusions (`target`, `target2`, `node_modules`, `build`, `dist`, and hidden `.` entries) and default caps (`DEFAULT_BYTES = 33_554_432`, `DEFAULT_ENTRIES = 50_000`) remain unchanged when `--exclude-dir` is omitted.
   - Repeatable `--exclude-dir <NAME>` matches directory segment names (case-insensitively on ASCII, rejecting path separators or `..` with exit code `2`) across unscoped path search, `--declared`, `--list-ids`, `--context`, and scoped `trace-id` traversal.
3. **`--declared` & `--list-ids` grammar compatibility**:
   - `Records::parse` and `valid_id` in `src/trace_relationships.rs` keep their exact `<!-- recur-trace-v1` grammar and diagnostic codes (`invalid_identifier`, `invalid_artifact`, `invalid_dependency`, `unknown_record`, `unterminated_metadata`, `duplicate_authority`, `conflicting_revision`) so `julia-tests/main.improvement.34.trace.test.jl` lines 64–119 pass untouched.
   - `token_ids` in `src/trace_relationships.rs` keeps its exact identifier boundary rules so `julia-tests/main.improvement.34.trace-context.test.jl` passes untouched.

## Primary ownership and boundaries

- **Primary files**: `src/trace_relationships.rs`, `src/trace_inspection.rs`, `src/main.rs` (`Commands::TraceId` `--exclude-dir` arg and `--declared` / `--list-ids` help strings), `src/recur_trace_id_main.rs` (if CLI args mirror `trace-id`), `docs/main.command.trace-id.eventness.discovery.report.md`.
- **Coordination note**: If implemented alongside `main.command.trace-id.role-classification`, wire `SearchOptions` / `--exclude-dir` cleanly so both `trace_relationships::scan` and scoped `collect_scoped_lines` honor `--exclude-dir`.

## Slices and checked gates

1. `slice-0` (`declared`, gate `scan-coverage-red-baseline`): Record red-baseline fixture where an alphabetically early `exports/` directory exhausts `--max-bytes` before `scripts/` and `warps/`, showing that `coverage` and stderr omit `stopped_at` and `unreached_top_dirs`.
2. `budget-starvation-diagnostics` (`checked`, gate `budget-starvation-diagnostics-acceptance`): Compute `stopped_at` and `unreached_top_dirs` in `trace_relationships::scan` when `byte_budget` or `entry_budget` halts traversal, serialize them additively when `complete == false`, and include them in stderr incomplete-scan diagnostics; verify with unit and CLI JSON/stderr tests.
3. `exclude-dir-filter` (`checked`, gate `exclude-dir-filter-acceptance`): Add repeatable `--exclude-dir <NAME>` validation and filtering to `SearchOptions` / `trace_relationships::scan` (and standalone `recur-trace-id`), proving `--exclude-dir exports` reaches `scripts/`, `trace/`, and `warps/` within a tight `--max-bytes` budget.
4. `declared-and-filename-guidance` (`checked`, gate `declared-and-filename-guidance-acceptance`): Clarify `trace-id --help` and `docs/main.command.trace-id.eventness.discovery.report.md` regarding `<!-- recur-trace-v1` header syntax (`defines:`/`publish:`/`consumer:`/`trigger:` with `[A-Za-z0-9_]` dot-atoms) vs `.warp-map.json` metadata and `--list-ids` content tokens vs `recur files` filename discovery.
5. `slice-final` (`checked`, gates `trace-coverage-compatibility-regressions`, `docs-discovery-report-sync`): Pass `cargo test --locked`, `julia julia-tests/main.improvement.34.trace.test.jl`, `julia julia-tests/main.improvement.34.trace-context.test.jl`, and new starvation/`--exclude-dir` tests with zero skipped checks.

## Predicted final Eventness

When a large export or data folder exhausts scan budgets, `recur trace-id` immediately names where scanning stopped and which top-level folders (`scripts`, `trace`, `warps`) were left unvisited, lets the caller rerun cleanly with `--exclude-dir exports`, and makes `--declared` (`<!-- recur-trace-v1`) vs `--list-ids` vs `recur files` self-explanatory from `--help` alone.
