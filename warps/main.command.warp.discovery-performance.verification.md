<!-- recur-trace-v1
defines: main.command.warp.discovery_performance.verification
consumer: main.command.warp.discovery_performance.baseline
-->
# Warp discovery performance implementation observations

Windows, `C:/src/recur`, release-safe CLI. Each timing is the wall time of one
invocation with JSON output captured and exit code 0. See the separate baseline
for the initial observations.

| Command | Baseline | Implementation |
| --- | ---: | ---: |
| `warp list --all` | 9,204 ms | 787, 802, 791 ms |
| `warp show main.command.tree.hierarchy-depth` | 9,012 ms | 49 ms |
| `warp slices main.command.tree.hierarchy-depth` | 9,110 ms | 39 ms |
| `reveal main.command.tree.hierarchy-depth` | 607 ms | 119 ms |

A list run overlapping a Cargo test build took 1,827 ms. The sub-second
inventory result describes the three sequential runs above.

Ring acceptance uses the same coordinator layers as its projection. Single
bubble queries filter inventory candidates by identity before composition.
Source digests are shared in memory within one query. Refresh assessments still
charge shared inputs to their own file and aggregate byte budgets, validate
their own manifests and results, and read fresh inputs on the next invocation.

`cargo test --locked` passed 340 tests and reported seven pre-existing ignored
doctests separately. The checked result covers four new Rust tests and the
459 executed assertions in these Julia suites, all with zero failures or skips.

| Julia suite under `julia-tests/` | Passed |
| --- | ---: |
| `main.command.warp.discovery.test.jl` (unchanged) | 75 |
| `main.command.warp.test.jl` | 92 |
| `main.command.warp.bubble.test.jl` | 32 |
| `main.command.warp.ring-topology.test.jl` | 44 |
| `main.command.warp.evidence-integrity.test.jl` | 108 |
| `main.command.warp.query-compatibility.test.jl` | 54 |
| `main.command.warp.docs-reconciliation.test.jl` | 54 |
