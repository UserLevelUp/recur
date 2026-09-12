# Current handoff

Historical slice-0/1 acceptance receipts are preserved. The latest local-binary
projection is 2 covered, 3 pending, slice-2 ready, no blocked gates. The explicit
same-contract refresh in main.lang.runtime-evidence.evidence-refresh/ binds the
baseline to a current observed 251-test Rust regression, preserving every original
input including target/debug/recur.exe. The older baseline manifest remains stale
as a historical observation; its [recorded blocker](main.lang.runtime-evidence.baseline-refresh.blocked.md)
was resolved through refresh, without editing it or the accepted layer.
Development map is at repository root: use `-d .` for Warp
queries/writes, `-d warps` for capsule/docs discovery. The association/API v1
contract is frozen. Red observations: 105 pass, 51 expected verdict failures,
zero errors. Loader implemented in isolation, not accepted or integrated.
Five green attempts failed through Julia runtime/compiler errors; first reached
156 passing assertions and one harness error. See
[slice-2 blocker](main.lang.runtime-evidence.slice-2.blocked.md) for exact logs,
runtime flags and next bounded diagnostic. No checked green evidence exists.
Preserve baseline, red observations and old receipts. Resume the bounded Julia
diagnostic and slice-2 tests only after re-querying with the current local binary;
rebuilding/removing the fingerprinted executable can stale the baseline again.
