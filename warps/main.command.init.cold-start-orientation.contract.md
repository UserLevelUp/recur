<!-- recur-trace-v1
defines: main.command.init.cold_start_orientation.contract
consumer: main.command.init.cold_start_orientation.verification
-->
# Cold-start base-prefix discovery and Eventness orientation

artifact.type = contract
warp.id = main.command.init.cold-start-orientation
status = prediction; acceptance requires observed evidence

## Initial Eventness

At `f0c9745`, dogfooding `recur` cold-start orientation on `C:/src/kicad_ra` exposed three discovery gaps before an agent could run its first productive `recur tree` or `recur trace-id` command:

1. `recur init --analyze -d C:/src/kicad_ra --json` detected lane directories (`docs/`, `eventness/`, `scripts/`, `trace/`, `warps/`) and `.` separators, yet never reported that 120+ hierarchical files shared the repository-wide `robo.` base prefix. Because `recur tree` requires a concrete `<BASE>` prefix, a cold-start agent had to inspect raw directories or read `AGENTS.md` to discover `robo`.
2. `recur reveal -d C:/src/kicad_ra` returned `No reveal entries found` even though `eventness/` held live `*.complete.md` Eventness files and `warps/` held `.warp-map.json` files. `recur-expert/SKILL.md` and `recur-eventness/SKILL.md` did not clearly distinguish authored `*.recur.md` Reveal capsules (entry capsules) from live Eventness state artifacts (`*.todo.current.md`, `*.complete.md`, `*.warp-map.json`).
3. Root `recur --help` summarized `trace-id` solely as `Trace semantic identifier roles (define, produce, consume, trigger)`, hiding unscoped path search (`recur trace-id <PATTERN>`) and Improvement 34 bird's-eye inventory/context (`--list-ids`, `--context`, `--declared`).

## Strict backwards-compatibility contract

All changes in this bubble MUST be strictly additive and non-breaking:

- **`recur init --analyze` JSON compatibility (`julia-tests/runtests.init.jl`)**:
  - Existing JSON keys (`root`, `config_found`, `config_path`, `detected_lanes`, `additions`, `separator_updates`, `missing_directories`) MUST keep their exact names, value types, and semantics.
  - New fields (`detected_prefixes`: array of `{ "prefix": String, "file_count": usize, "sample_lanes": Vec<String> }`, sorted by descending `file_count` then lexicographically; `dominant_prefix`: `Option<String>` when a clear leading prefix exists) are **strictly additive**.
  - Human-readable `recur init --analyze` stdout appends a concise `Detected base prefixes:` section (with a ready-to-run `recur tree <prefix>` / `recur files "<prefix>.**"` hint) without removing or breaking existing lane output lines checked by `runtests.init.jl`.
- **`recur init` write mode**:
  - `.recur/config.toml` generation, lane naming, and collision suffixes (`[test-quick-2]`, etc.) remain 100% unchanged. `--analyze` remains strictly read-only.
- **Root `recur --help` invariants (`tests/reveal_profiles.rs`)**:
  - `tests/reveal_profiles.rs::root_help_routes_new_users_to_skill_discovery_and_explicit_loading` asserts the presence of `recur reveal --type skill`, `recur reveal recur-expert`, `does not load or execute`, `supported loader`, `file reader`, and `when available`, plus zero filesystem side effects. Every one of those phrases MUST remain intact while updating the `trace-id` subcommand summary and cold-start orientation hints.

## Primary ownership and boundaries

- **Primary files**: `src/project_config.rs`, `src/main_command_init_impl.rs`, `src/main.rs` (root `trace-id` about/help text only), `recur-expert/SKILL.md`, `recur-eventness/SKILL.md`, `recur-trace-id/SKILL.md`, `docs/main.command.init.readme.md`.
- **Out of scope**: `recur tree` flag changes (owned by `main.command.tree.hierarchy-depth`), `recur trace-id` role classifier logic (owned by `main.command.trace-id.role-classification`), and `trace_relationships.rs` scan coverage (owned by `main.command.trace-id.scan-coverage`).

## Slices and checked gates

1. `slice-0` (`declared`, gate `cold-start-orientation-red-baseline`): Record red-baseline evidence showing `recur init --analyze --json` lacks `detected_prefixes` on a multi-lane prefixed fixture and root `recur --help` omits bird's-eye `trace-id` discovery.
2. `init-prefix-discovery` (`checked`, gate `init-prefix-discovery-acceptance`): Implement multi-segment base-prefix detection across scanned lanes in `project_config.rs` / `main_command_init_impl.rs` (ignoring generic single-segment stems without hierarchical children, ranking prefixes by hierarchical file count across lanes), expose additive JSON + text output in `--analyze`, and verify with unit/integration tests.
3. `skill-and-help-orientation` (`checked`, gate `skill-and-help-orientation-acceptance`): Update `recur-expert/SKILL.md`, `recur-eventness/SKILL.md`, `recur-trace-id/SKILL.md`, and root `recur --help` to front-load:
   - Step 0 cold-start base-prefix discovery (`recur init --analyze` -> `recur tree <prefix>`),
   - Eventness state artifacts (`*.todo.current.md`, `*.complete.md`, `*.warp-map.json`) vs authored `*.recur.md` Reveal capsules,
   - The 4-step `trace-id` funnel (unscoped path hunt -> `--list-ids` inventory -> `--context` excerpt -> scoped role classification / `--declared`).
4. `slice-final` (`checked`, gates `init-and-reveal-compatibility-regressions`, `docs-and-skill-sync`): Pass `cargo test --locked --test reveal_profiles`, `julia julia-tests/runtests.init.jl`, new prefix-discovery tests, and verify `docs/main.command.init.readme.md` matches live behavior with zero skipped checks.

## Predicted final Eventness

An agent entering an unfamiliar repository runs `recur init --analyze -d .` and immediately sees both lane directories and dominant hierarchical base prefixes (such as `robo.`), understands why `recur reveal` only lists `*.recur.md` / `[reveal.*]` capsules while `recur files "**.current"` / `recur warp` surface live Eventness, and follows the `trace-id` bird's-eye-to-drill-down funnel without breaking any existing automation.
