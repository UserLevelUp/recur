<!-- recur-trace-v1
defines: main.command.warp.discovery_performance.contract
consumer: main.command.warp.discovery_performance.verification
-->
# Cross-functional Warp discovery performance and evidence memoization

artifact.type = contract
warp.id = main.command.warp.discovery-performance
status = prediction; acceptance requires observed evidence

## Initial Eventness

At `f0c9745`, `recur warp -d C:/src/recur` took ~7,956 ms (and every single `recur warp show <id> -d .`, `recur warp slices <id> -d .`, and `recur reveal <id> -d .` call paid the same ~8,000 ms penalty), compared to 114 ms on `C:/src/kicad_ra`. Source inspection isolated four compounding bottlenecks:

1. **6× full recursive walks of `target/` (72,905 files) and `.git/` during ring projection**:
   - `main.improvement.34.implementation.warp-ring.json` sits at repository root (`C:/src/recur`) with 6 accepted worker domains (`trace`, `workflow`, `graph`, `eventness`, `evidence`, `review`).
   - Although `merge_ring_inner` (`src/main_command_warp_impl.rs:900`) already loaded the coordinator's layers via `load_warp_layers_scoped(root, warp, local)`, `parent_acceptance_state` (`src/main_command_warp_impl.rs:1131`) ignored them and called `load_warp_layers(root, warp)` (`local = false`) for every worker domain.
   - `load_warp_layers_scoped(..., false)` filtered entries only with `keep_entry` (`entry.file_name().to_str() != Some(".recur")`), walking all 72,905 files in `target/` plus `.git/` six times (~450,000 NTFS `stat` calls) on every `recur warp` invocation.
2. **Divergent walker exclusions across Warp entrypoints**:
   - `keep_entry` (`src/main_command_warp_impl.rs:1996`) only excluded `.recur`, leaving `.git`, `target`, `target2`, `build`, `dist`, `node_modules`, and `__pycache__` unexcluded in `warp status`, `warp merge`, `warp map`, `warp explain`, `warp next`, and `warp collapse-plan`.
   - `DEFAULT_EXCLUDE_DIRS` (`src/warp_discovery.rs:9`) excluded `.git`, `target`, `build`, `dist`, `node_modules`, and `fixtures`, omitting `target2` and `__pycache__`.
3. **Full-inventory composition on single-Warp queries**:
   - `bubble_progress` (`src/main_command_warp_impl.rs:397`) called `list_warps(root, true, false)` to compose and fingerprint all 84 discovered bubbles/rings before filtering down to the single requested `warp_id`.
4. **Repeated uncached source fingerprinting across checked gates**:
   - `warp_evidence::check` (`src/warp_evidence.rs:215-225`) repeatedly called `canonicalize()`, `fs::read()`, and `fingerprint()` on shared files (`Cargo.toml`, `src/main.rs`, CAD/PCB exports, test suites) across multiple checked gates in a single invocation.

## Cross-functional and backwards-compatibility contract

Performance improvements MUST remain completely discipline-agnostic (supporting mechanical/Blender, electrical/KiCad, manufacturing, documentation, research, and polyglot software repositories alike) and 100% backwards compatible:

1. **Domain-agnostic manifest placement (cross-functional invariant)**:
   - NEVER restrict Warp discovery to `warps/` or code folders. `.warp-map.json`, `.warp-ring.json`, `.warp-layer.json`, and external `.evidence.json` artifacts must continue to resolve in any lane (`docs/`, `boards/`, `blender/`, `exports/`, `demos/`, `warps/`, root `.`, or `.recur/work/`).
   - `[warp.discovery]` `roots` and `exclude_dirs` in `.recur/config.toml` plus `--scan-all` retain full override authority so projects that intentionally store artifacts in `build/` or `fixtures/` can configure `[warp.discovery]` explicitly.
2. **Preserve `.recur` default discovery and `main.command.warp.discovery.test.jl`**:
   - `.recur` MUST NOT be added to `DEFAULT_EXCLUDE_DIRS`: `julia-tests/main.command.warp.discovery.test.jl` (lines 46, 61, 112, 175, 182) explicitly tests private Warp bubbles under `.recur/` and `.recur/work/`.
   - **Do NOT edit `julia-tests/main.command.warp.discovery.test.jl`**, which is content-fingerprinted by `warps/watch-eventness/observations/*.evidence.json`. Place new tests in dedicated Rust/Julia test files.
3. **Exact behavioral parity for `show`, `slices`, `reveal`, and `list`**:
   - Targeted candidate lookup in `bubble_progress` must apply the exact same `DiscoveryPolicy::load(root, false)` walk to detect duplicate/ambiguous `warp_id` manifests across scopes and ring-vs-bubble mismatches, composing only the matched candidate(s) so output JSON (`warp-progress-v1`) and error diagnostics remain identical.
4. **Stateless read-only memoization**:
   - File canonicalization and FNV-1a fingerprinting caches live strictly in memory for a single CLI command execution. No cache files are written to disk, preserving read-only purity and immediate detection of source/result edits on the next invocation.

## Primary ownership and boundaries

- **Primary files**: `src/main_command_warp_impl.rs`, `src/warp_discovery.rs`, `src/warp_evidence.rs`, `docs/main.command.warp.readme.md`.
- **Coordination note**: Disjoint from `init`, `tree`, and `trace-id` bubbles; if `src/project_config.rs` `[warp.discovery]` template comment/list is updated to include `target2` / `__pycache__`, coordinate cleanly with `main.command.init.cold-start-orientation` while keeping `julia-tests/main.command.warp.discovery.test.jl` lines 173–174 green.

## Slices and checked gates

1. `slice-0` (`declared`, gate `warp-discovery-performance-red-baseline`): Record baseline timing/walk-count evidence reproducing the 6× `parent_acceptance_state` unbounded walk and 84-manifest composition overhead during single-bubble `warp show`.
2. `ring-and-walker-hygiene` (`checked`, gate `ring-and-walker-hygiene-acceptance`): Pass already-loaded coordinator `&[LocatedWarpLayer]` into `parent_acceptance_state` inside `merge_ring_inner` (eliminating redundant recursive root walks), update `keep_entry` to skip `.git`, `target`, `target2`, `build`, `dist`, `node_modules`, and `__pycache__`, and add `target2` + `__pycache__` to `DEFAULT_EXCLUDE_DIRS` while keeping `.recur` discovered by default.
3. `targeted-single-warp-lookup` (`checked`, gate `targeted-single-warp-lookup-acceptance`): Refactor `bubble_progress` so `recur warp show`, `recur warp slices`, and `recur reveal <warp-id>` scan `DiscoveryPolicy` roots for matching `{warp}.warp-map.json` / `{warp}.warp-ring.json` filenames and compose only matching entries while preserving ambiguity, ring-kind, and error reporting.
4. `in-memory-evidence-memoization` (`checked`, gate `in-memory-evidence-memoization-acceptance`): Add per-invocation in-memory memoization for canonicalized evidence root/file paths and FNV-1a content fingerprints across `list_warps` / `warp_evidence`, verified with unit tests counting physical file reads across shared multi-slice inputs (CAD/PCB/JSON/code fixtures).
5. `slice-final` (`checked`, gates `warp-cross-functional-and-compatibility-regressions`, `docs-warp-readme-sync`): Pass `cargo test --locked`, `julia julia-tests/main.command.warp.discovery.test.jl` (untouched), `julia julia-tests/runtests.warp.jl`, and new cross-functional performance regression tests, and sync `docs/main.command.warp.readme.md` with zero skipped checks.

## Predicted final Eventness

`recur warp`, `recur warp show <id>`, `recur warp slices <id>`, and `recur reveal <id>` complete in interactive sub-second time on `C:/src/recur` and on cross-functional hardware/CAD/software repositories with huge ignored build or cache directories, with zero disk cache state and 100% compatibility with existing Warp manifests and tests.
