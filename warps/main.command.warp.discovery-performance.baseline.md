<!-- recur-trace-v1
defines: main.command.warp.discovery_performance.baseline
-->
# Warp discovery performance baseline

Observed on Windows at `de76aa0` using the existing `target/release-safe/recur.exe`
(`recur 0.2.8`). One invocation per command, PowerShell stopwatch wall time,
JSON output captured in memory. All commands used `-d C:/src/recur` and exited 0.

| Command | Wall time |
| --- | ---: |
| `warp list --all` | 9,204 ms |
| `warp show main.command.tree.hierarchy-depth` | 9,012 ms |
| `warp slices main.command.tree.hierarchy-depth` | 9,110 ms |
| `reveal main.command.tree.hierarchy-depth` | 607 ms |

Source inspection found that `bubble_progress` composed the whole inventory
before recomputing the selected bubble. Ring parent acceptance recursively loaded
coordinator layers again for every child whose acceptance slice was covered.
The observed Improvement 34 ring was blocked. The number of recursive walks
therefore depends on which coordinator slices have current coverage.

The Reveal capsule returns a `pull.then` pointer to `warp show`. Its own timing
measures capsule discovery separately from the subsequent Warp query.
