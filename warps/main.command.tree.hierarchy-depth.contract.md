<!-- recur-trace-v1
defines: main.command.tree.hierarchy_depth.contract
consumer: main.command.tree.hierarchy_depth.verification
-->
# Opt-in hierarchy segment-depth bounding for recur tree

artifact.type = contract
warp.id = main.command.tree.hierarchy-depth
status = prediction; acceptance requires observed evidence

## Initial Eventness

At `f0c9745`, running `recur tree robo -d C:/src/kicad_ra --depth 1` dumped all 120+ leaf files down to 6+ dot-separated segments (`robo.prototype.refinement.sol.slice-2...`), because `--depth` on `recur tree` passes `max_depth` to `WalkDir` (filesystem directory depth) rather than bounding dot/separator hierarchy segments beneath `<BASE>`. Meanwhile:

- `recur tree --help` describes `-L, --depth <DEPTH>` only as `Maximum depth to display`, which agents read as hierarchy depth.
- `tests/tree_configured_lanes.rs::configured_wildcard_scope_preserves_depth_and_separator_markers` (line 232) **explicitly requires** `--depth 2` to limit filesystem directory traversal depth.
- `julia-tests/runtests.tree.jl` (lines 52–53) contains `@test_broken !contains(output, "Create")` on `recur tree "UserService" --depth 1`; changing `--depth` semantics would both break `tests/tree_configured_lanes.rs` and trigger an `Unexpected Pass` failure in Julia `Test`!
- Running `recur tree "**"` exits `2` with `No files found matching base pattern '**'` and no hint toward `recur init --analyze` or concrete `<BASE>` prefixes.

## Strict backwards-compatibility contract

1. **`--depth` (`-L`) is NEVER changed in behavior**:
   - `-L, --depth <DEPTH>` remains filesystem directory walk depth (`options.max_depth = depth`).
   - Only its `--help` text is clarified to state `Maximum filesystem directory depth to traverse (use --levels for hierarchy depth)`.
   - Both `tests/tree_configured_lanes.rs` and `julia-tests/runtests.tree.jl` (including its `@test_broken` line) MUST pass untouched.
2. **New opt-in hierarchy segment-depth flag**:
   - Add `--levels <N>` (with visible CLI alias `--tree-depth <N>`, `N >= 1`) to `recur tree`.
   - `--levels 1` restricts rendered hierarchy nodes to at most `1` segment below `<BASE>` (e.g. `recur tree robo --levels 1` shows `archive`, `concept`, `manufacturing`, `prototype`, etc., without expanding deeper sub-segments).
   - Works consistently across Unicode tree output, `--ascii`, `--count`, `--sep` / `--show-sep`, configured multi-lane trees, and `--json`.
   - When `--levels` / `--tree-depth` is omitted, `recur tree` output (text and JSON `name`, `children`, `path`) MUST remain byte-for-byte identical to `f0c9745`.
   - Passing `--levels 0` fails cleanly with exit code `2` (`Invalid arguments: --levels must be >= 1`).
3. **Wildcard compatibility (`julia-tests/main.command.tree.wildcard-current.test.jl`)**:
   - Non-bare wildcard queries such as `recur tree "main.**.current.**"` MUST keep their exact existing tree construction and output.
   - Only when a bare root wildcard (`"*"` or `"**"`) matches zero literal base prefixes, stderr appends an actionable hint (e.g. suggesting `recur init --analyze` to discover base prefixes or `recur files "**"` to list files) while preserving the non-zero exit code when no tree root matches.

## Primary ownership and boundaries

- **Primary files**: `src/tree.rs`, `src/main_command_tree_impl.rs`, `src/main.rs` (`Commands::Tree` args only), `docs/main.command.tree.readme.md`.
- **Out of scope**: `recur init` prefix discovery (`main.command.init.cold-start-orientation`) and `recur trace-id` modules.

## Slices and checked gates

1. `slice-0` (`declared`, gate `tree-hierarchy-depth-red-baseline`): Capture baseline evidence demonstrating `recur tree <BASE> --depth 1` expanding deep dot-hierarchy leaves in a flat directory and `recur tree --help` conflating display depth with directory walk depth.
2. `hierarchy-levels-flag` (`checked`, gate `hierarchy-levels-flag-acceptance`): Implement opt-in `--levels <N>` / `--tree-depth <N>` pruning in `src/tree.rs` and `src/main_command_tree_impl.rs` across text, `--ascii`, `--count`, configured-lane, and `--json` outputs, with Rust unit/integration tests covering `N = 1`, `N = 2`, `N = 0` error handling, and `--depth` + `--levels` composition.
3. `help-and-root-wildcard-guidance` (`checked`, gate `help-and-root-wildcard-guidance-acceptance`): Clarify `recur tree --help` descriptions for `--depth` vs `--levels` / `--tree-depth` and add actionable guidance when bare `"*"` / `"**"` is passed to `recur tree`.
4. `slice-final` (`checked`, gates `tree-compatibility-regressions`, `docs-tree-readme-sync`): Pass `cargo test --locked --test tree_configured_lanes`, `julia julia-tests/runtests.tree.jl`, `julia julia-tests/main.command.tree.wildcard-current.test.jl`, new `--levels` tests, and update `docs/main.command.tree.readme.md` with zero skipped checks.

## Predicted final Eventness

Agents and humans can inspect top-level namespaces in large repositories (`recur tree robo --levels 1 --count`) in ~15 lines instead of 150+ leaf paths, `--help` makes filesystem `--depth` vs hierarchy `--levels` unmistakable, and every existing Rust and Julia tree test continues to pass without modification.
