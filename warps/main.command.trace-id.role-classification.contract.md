<!-- recur-trace-v1
defines: main.command.trace_id.role_classification.contract
consumer: main.command.trace_id.role_classification.verification
-->
# Repair scoped recur trace-id role classification while preserving legacy contracts

artifact.type = contract
warp.id = main.command.trace-id.role-classification
status = prediction; acceptance requires observed evidence

## Initial Eventness

At `f0c9745`, drilling down on a single trace ID in `C:/src/kicad_ra` via `recur trace-id "robo.mod.traction" --scope "**" -d C:/src/kicad_ra --json` exposed five compounding issues in `src/main_command_trace_id_impl.rs`:

1. **Keyword-argument symbol explosion (1,505 false positives)**: `extract_symbol_name_from_definition` split `consumes=["robo.mod.traction"],` on `=` and extracted `"consumes"` as a transitive code symbol name, causing `line_has_symbol_reference` to match every line in the repository containing the English word `consumes`.
2. **Binary `.pyc` leakage**: `collect_scoped_lines` read every file via `fs::read` + `String::from_utf8_lossy` without honoring `policy.skip_binary_files` or `policy.max_file_bytes`, injecting `scripts/__pycache__/*.cpython-312.pyc` binary garbage into JSON `line` strings.
3. **Substring trigger false positives**: `contains_any_keyword` used raw substring matching (`lower.contains(&kw)`), so default trigger keyword `"register"` matched JSON property `"board_register": "docs/robo..."`.
4. **Unconditional `define` duplication & duplicate `--format full` lines**: `classify_line` set `roles.define = true` on every line containing the identifier literal—even pure `consume`, `produce`, or `trigger` lines—causing every site in `robo.mod.traction` to appear twice (or three times) in `--format full` with no `[define]`/`[consume]` role label.
5. **Confusing `"legacy --scope"` error wording**: Running `recur trace-id "robo.mod.traction" --depth 1` without `--scope` failed with `Invalid arguments: --depth, --format, --stdin, and saved-run flags require a legacy --scope query`, misleading callers into thinking `--scope` is deprecated even though `--list-ids`, `--context`, and `--declared` also require `--scope`.

## Strict backwards-compatibility contract

Every existing caller and regression suite (`julia-tests/runtests.trace-id.jl`, `julia-tests/main.improvement.34.trace.test.jl`, `julia-tests/runtests.lane.coordination.jl`, `julia-tests/runtests.demo.skippy.jl`) MUST remain 100% green without breaking schema or CLI changes:

1. **Scoped JSON schema is frozen**:
   - Single-identifier scoped JSON output remains `{ "request": {...}, "identifier": "...", "define": [...], "produce": [...], "consume": [...], "trigger": [...] }` with **NO** top-level `"schema"` key (`main.improvement.34.trace.test.jl` line 57 explicitly asserts `!haskey(scoped, :schema)`).
   - Glob scoped JSON output remains a top-level JSON `Array` of those objects.
   - Each role site retains `path`, `line_number`, `line`, and `edge_type`, and piping `--json` into `recur merge --stdin` (`runtests.trace-id.jl` lines 567–597) continues to work identically.
2. **Valid symbol extraction preserved**:
   - `public const string OwnershipCreate = "ulu.topic.dot.ownership.create";` MUST still extract `"OwnershipCreate"` and resolve transitive C# usages (`DotLevelService.cs`, `DotWatcherHostedService.cs` in `runtests.trace-id.jl` lines 110–156).
   - Guard rules for `extract_symbol_name_from_definition`:
     - Reject candidate tokens that match any configured `producer_keywords`, `consumer_keywords`, `trigger_keywords`, or reserved trace-role names (`define`, `defines`, `produce`, `produces`, `consume`, `consumes`, `trigger`, `triggers`, `id`, `trace_id`).
     - Reject call-site keyword-argument syntax where `<ident>=` is preceded inside an open `( ... )` parameter/argument list or followed immediately by `[` / `(` without whitespace or a declaration/assignment left-hand side.
3. **Binary & size filtering in `collect_scoped_lines`**:
   - Honor `policy.skip_binary_files` (skip known binary extensions such as `.pyc` and NUL-byte binary samples via `has_binary_extension` / `is_binary_sample`) and `policy.max_file_bytes` before lossy UTF-8 conversion.
4. **Identifier-boundary matching for word-like role keywords**:
   - When a configured keyword consists purely of ASCII identifier characters (`[A-Za-z0-9_]`, e.g. `register`, `publish`, `send`, `emit`, `bind`, `subscribe`, `advance`), match on ASCII identifier boundaries (`!prev.is_ascii_alphanumeric() && prev != '_'` and `!next.is_ascii_alphanumeric() && next != '_'`). Note that `.` is NOT an identifier character, so `registry.Register(...)`, `_bus.Publish(...)`, `register(...)`, and `trigger advance` ALL still match, whereas `board_register` and `interface_register` do NOT match `register`!
   - Keywords containing punctuation (e.g. `*.` or `register(`) continue to match via case-insensitive substring so wildcard routing (`"*.level.create"` in `DotWatcherHostedService.cs`) remains intact.
5. **Accurate `define` vs `produce`/`consume`/`trigger` classification**:
   - Explicit definition lines (`=` assignment/const definition, `define` / `defines` keyword, JSON `"defines"` / `"trace_id"` property, or standalone/comment/prose lines containing the identifier with **no** `produce`/`consume`/`trigger` match) classify as `define = true`.
   - Lines that match `produce`, `consume`, or `trigger` and have **no** explicit definition marker (`define`/`defines` keyword, const/variable assignment, or `"defines"`/`"trace_id"` field) do NOT spuriously set `define = true` (matching the comment intent in `runtests.lane.coordination.jl` line 240 while keeping `define lane.a.api.contract = ...`, `transition.audit.order.42 = todo.current`, and `skippy.relationship.playful.precise.current = active` in `define`).
6. **`--scope` diagnostic & `--format full` readability**:
   - Missing `--scope` error message keeps the literal flag `--scope` in stderr (`main.improvement.34.trace.test.jl` line 52 asserts `occursin("--scope", errors)`), replacing `"require a legacy --scope query"` with `"require --scope <SCOPE> (for example: --scope \"**\")"`.
   - `--format full` groups or annotates roles per `(path, line_number)` (e.g. `[define]`, `[consume]` or combined `[define, trigger]`) so identical source lines are not printed multiple times without context, while `--list-ids --format full` remains rejected (`main.improvement.34.trace-context.test.jl` line 108).

## Primary ownership and boundaries

- **Primary files**: `src/main_command_trace_id_impl.rs`, `src/trait/trace_id.rs`, `docs/main.command.trace-id.readme.md`.
- **Out of scope**: `src/trace_relationships.rs` and `src/trace_inspection.rs` budget starvation / `--exclude-dir` (owned by `main.command.trace-id.scan-coverage`).

## Slices and checked gates

1. `slice-0` (`declared`, gate `role-classification-red-baseline`): Record baseline reproduction of `consumes=["..."]` symbol explosion, `.pyc` binary line inclusion, `board_register` false trigger, and pure-consume `define` duplication.
2. `symbol-and-binary-guards` (`checked`, gate `symbol-and-binary-guards-acceptance`): Harden `extract_symbol_name_from_definition` against keyword-argument syntax and role-keyword names, and enforce `skip_binary_files` + `max_file_bytes` in `collect_scoped_lines`, with Rust unit tests and fixture integration tests.
3. `role-boundaries-and-dedup` (`checked`, gate `role-boundaries-and-dedup-acceptance`): Implement identifier-boundary matching for word-like role keywords while preserving punctuation keywords (`*.`, `register(`), and prevent pure `produce`/`consume`/`trigger` lines from duplicating into `define`.
4. `scope-diagnostics-and-format-full` (`checked`, gate `scope-diagnostics-and-format-full-acceptance`): Clarify missing-`--scope` diagnostics (without `"legacy"`) and deduplicate/label role output in `--format full`.
5. `slice-final` (`checked`, gates `trace-id-legacy-and-merge-compatibility-regressions`, `docs-trace-id-readme-sync`): Pass `cargo test --locked`, `julia julia-tests/runtests.trace-id.jl`, `julia julia-tests/main.improvement.34.trace.test.jl`, `julia julia-tests/main.improvement.34.trace-context.test.jl`, `julia julia-tests/runtests.lane.coordination.jl`, and `julia julia-tests/runtests.demo.skippy.jl`, and update `docs/main.command.trace-id.readme.md` with zero skipped checks.

## Predicted final Eventness

Scoped `recur trace-id` role queries return clean, high-signal `define`, `produce`, `consume`, and `trigger` classifications on Python, Rust, C#, JSON, and Markdown codebases without 1,500-hit keyword explosions or `.pyc` binary noise, while every legacy JSON consumer and `recur merge --stdin` pipeline continues working unchanged.
