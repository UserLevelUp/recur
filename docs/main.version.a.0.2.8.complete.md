# Version a.0.2.8 — Release Record

Status: `complete` (permanent)
Date: 2026-03-08
Branch: `a.0.2.8`

## What Landed in This Version

### Commands

main.version.a.0.2.8.command.trait
  - `recur trait` promoted to first-class Commands enum variant
  - `recur trait list` / `get` / `set` for managing `.recur/config.toml`
  - Previously ran as a side-channel; now fully integrated with clap dispatch

main.version.a.0.2.8.command.trace_id
  - `recur trace-id` MVP: define/produce/consume/trigger role classification
  - `--scope`, `--ext`, `--stdin`, `--format`, `--json`, `--depth`, `--depth-guard`, `--force`
  - Trait config: `recur trait set trace_id.producer_keywords "publish,emit"`
  - `edge_type` field added to every JSON site object ("define"/"produce"/"consume"/"trigger")
  - Works on any text file — source code, eventness files, docs, config

main.version.a.0.2.8.command.trace_stats
  - Phase 3 metrics: direct, transitive, circular, depth, risk
  - Circular pattern counting: distinct back-edges via visit_key (function:path:line)
  - Stdin integration, medium/high risk fixtures, performance fixture all activated
  - Sorting, filtering, top-N, table/json/csv output
  - Traversal budget policy via `[traits.traversal_budget]` in config

main.version.a.0.2.8.command.version
  - `recur version` pure query surface: status, manifest, policy, schema, query, explain
  - `recur-version` companion binary: next, save
  - Save writes preserved snapshots, updates manifests, and records ACK/NAK status under `.recur/version/`
  - Query history answers state-transition questions from config-defined CSV identity/state fields

### Fixes

main.version.a.0.2.8.fix.release_safe_stack_overflow
  - `release-safe` Cargo profile opt-level 0→1
  - opt-level=0 caused stack overflow on recursive traversal (unoptimized stack frames)
  - opt-level=1 restores unwind panic + no LTO semantics while preventing overflow

### Docs + Eventness

main.version.a.0.2.8.docs.command_map
  - `docs/main.command.map.readme.md` — hierarchical command surface index
  - `docs/main.command.traverse.readme.md` — traverse family (trace, trace-id, trace-stats, callers, callees)
  - `docs/main.command.discover.readme.md` — discover family (files, find, tree, children, related, stats, id)
  - `docs/main.command.compose.readme.md` — compose family (merge, flatten)
  - `docs/main.command.config.readme.md` — config family (trait, init)
  - recur now navigates its own command surface: `recur tree "main.command" -d docs/`
  - Pipeline relationships encoded: `recur trace-id "recur.pipe.json" --scope "main.command.**" -d docs/`

main.version.a.0.2.8.docs.trace_id_readme
  - `docs/main.command.trace-id.readme.md` — full command reference
  - Includes: role table, all flags, JSON shape, trait config, pipeline, Sudoku context

main.version.a.0.2.8.docs.version_readme
  - `docs/main.command.version.readme.md` — pure/companion version-eventness reference
  - Captures current artifact, manifest, snapshot, and config shapes

main.version.a.0.2.8.docs.sudoku_demo
  - `docs/main.demo.sudoku.trace-id.todo.current.md` — full Sudoku demo planning doc
  - `docs/main.demo.sudoku.trace-id.todo.md` — persistent tracking
  - Architecture: recur as discoverability engine, Julia as bridge, HTML5 as consumer
  - Three-layer model: game state / engine code / tool docs
  - Pattern A game management: `-d` flag as game scope selector

### Tests

main.version.a.0.2.8.tests.trace_stats_phase3
  - 5 of 6 @test_skip placeholders activated (stdin, medium risk, high risk, no-false-positive, performance)
  - DistinctCycleService fixture: two distinct back-edges, circular=2 confirmed passing

main.version.a.0.2.8.tests.trace_id
  - Phase 3b: edge_type field assertions (8 new passing tests)
  - Phase 3b was @test_broken, now active @test after Rust implementation landed

main.version.a.0.2.8.tests.version
  - `julia-tests/main.command.version.test.jl` covers pure status/policy/schema/query and `recur-version` ACK/NAK save behavior

## Test Baseline at Release

```
trace-stats: 94 passed, 0 broken, 0 failed
trace-id:    42 passed, 4 broken (Phase 5 pipeline = Improvement 9 scope)
Rust unit:   120 passed, 0 failed
```

## Deferred to Next Version

main.version.a.0.2.8.deferred.merge_edge_type
  - `merge --edge-type` semantic lane stitching (Improvement 9)
  - trace-id Phase 5 pipeline tests remain @test_broken pending this

main.version.a.0.2.8.deferred.sudoku_demo
  - Sudoku demo implementation (all 7 phases)
  - Deferred until trace-id tests stabilize — now unblocked for next version

## Navigation

```bash
# What landed in this version?
recur find "main.version.a.0.2.8" --scope "**" -d docs/

# What was deferred?
recur find "deferred" --scope "main.version.a.0.2.8**" -d docs/

# What docs exist for commands added this version?
recur children "main.command.trace" -d docs/ --sep .
```

## References

- `docs/main.improvement.7.phase3.todo.current.md` — trace-stats phase 3 context
- `docs/main.improvement.8.trace-id.todo.current.md` — trace-id improvement context
- `docs/main.command.trace-id.edge-type.todo.current.md` — edge_type lane (Layer 1 complete)
- `docs/main.command.trace-stats.circular-distinct.todo.current.md` — circular distinct (complete)
- `docs/main.command.version.readme.md` — version-eventness command reference
