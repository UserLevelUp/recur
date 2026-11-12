<!-- recur-trace-v1
defines: main.command.trace_id.role_classification.baseline
consumer: main.command.trace_id.role_classification.contract
-->
# Scoped trace-id role classification red baseline

Observed on Windows at `2ad068c` (`recur 0.2.8`, `target/release-safe/recur.exe`) against `C:/src/kicad_ra` before modifying any `trace-id` source files.

## Observed baseline defects

1. **Keyword-argument symbol explosion (`consumes=[...]`)**:
   - Command: `recur trace-id "robo.electronics.carriers.handoff" --scope "**" -d C:/src/kicad_ra --json`
   - Observed counts: `define = 16`, `produce = 27`, `consume = 1,940`, `trigger = 33`.
   - Cause: `extract_symbol_name_from_definition` split `consumes=['robo.electronics.carriers.handoff', ...]` in `scripts/robo.electronics.carriers.correction.handoff.py:41` on `=` and extracted `"consumes"` as a transitive code symbol, matching 1,900+ unrelated lines containing `"consumes"`.
2. **Binary `.pyc` leakage in `collect_scoped_lines`**:
   - On `robo.electronics.carriers.handoff`, **27 role sites** came from `scripts/__pycache__/*.pyc`; on `robo.electronics.carriers.board_register`, **4 role sites** came from `.pyc` files because `collect_scoped_lines` bypassed `should_skip_content_file` (`skip_binary_files` and `max_file_bytes`).
3. **Compound `snake_case` false trigger matches**:
   - Command: `recur trace-id "robo.electronics.carriers.board_register" --scope "**" -d C:/src/kicad_ra --json`
   - Observed counts: `define = 32`, `produce = 8`, `consume = 10`, `trigger = 35`.
   - **All 35 `trigger` sites** matched solely because `"board_register"` contains substring `"register"`.
4. **Unconditional `define` duplication & unlabeled duplicate `--format full` output**:
   - On `robo.electronics.carriers.board_register`, `8/10` `consume` sites, `7/8` `produce` sites, and `32/35` `trigger` sites were simultaneously duplicated in `define` because `build_record` mapped every `IdentifierSearcher` hit into `define` regardless of role keywords, and `print_terminal` (`TraceIdFormat::Full`) concatenated `define`, `produce`, `consume`, and `trigger` with no role labels or `(path, line_number)` grouping.
5. **Misleading missing-`--scope` diagnostic**:
   - Command: `recur trace-id "robo.base.mobile" --depth 1 -d C:/src/kicad_ra`
   - Exited `1` with `Error: --stdin, formatting, depth and saved-run options require a legacy --scope query`.
