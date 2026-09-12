# Slice-2 blocked: no complete green runtime observation

2026-09-12. Contract: contract:main.lang.runtime-evidence.slice-2:v1.
Gate: scope-evidence-binding, checked tests, no skips. **Not accepted.**

Live progress after authorized slice-0/1 completion: 2 covered, 3 pending;
slice-2 dependency-ready. The new isolated Julia loader implements the frozen
association boundary and delegates external results to the existing Recur checker.
No server route or browser integration has been made. Existing query/API files
and historical receipts remain unchanged.

## Observed attempts

All attempts execute the unchanged `demos/web-evidence-lab/main.lang.evidence.test.jl`
with LANG_EVIDENCE_RED=0 and RECUR_BIN=C:/src/recur/target/release-safe/recur.exe.
Project: demos/web-evidence-lab; --startup-file=no. Logs are preserved under
`warps/runtime-evidence/`; no run is recorded as a passing external result.

| Attempt | Runtime/configuration | Actual observation |
| --- | --- | --- |
| slice-2-green-1 | Julia 1.12.7, -C generic --compile=min | exit 1; 156 passing assertions, 0 failed assertions, 1 error; limits.files raises ReadOnlyMemoryError in deepcopy before its assessment |
| slice-2-green-2 | Julia 1.12.7, -C generic, default compilation | exit 1; EXCEPTION_ACCESS_VIOLATION, compiler SSA/backtrace stack |
| slice-2-green-3 | Julia 1.12.7, -C generic --compile=min -O0 --compiled-modules=no --pkgimages=no | exit 1; EXCEPTION_ACCESS_VIOLATION in interpreter |
| slice-2-green-4 | Julia 1.12.7, -C generic --compile=min --gcthreads=1 --check-bounds=yes, JULIA_NUM_THREADS=1 | exit 1; Parsers/JSON3 precompilation failed, compiler BoundsError |
| slice-2-green-5 | existing Julia 1.12.0, -C generic --compile=min -O0 --compiled-modules=no | exit -1073741819 (Windows access violation), empty redirected log |

1.12.7 runtime directory:
C:/Users/marcn/.julia/juliaup/julia-1.12.7+0.x64.w64.mingw32/bin.
1.12.0 runtime directory:
C:/Users/marcn/AppData/Local/Programs/Julia-1.12.0/bin.

The first run reached correct verdicts for valid/unaccepted and explicitly
accepted evidence, identities, aliases, transitions, producer, red history,
changed inputs/results, counts, native/manual references, traversal/junction
escape, other limits and purity assertions. Partial success is not gate success.
The native file-symlink setup limitation from slice-1 remains separately recorded;
the canonical junction-escape test ran and passed in the first loader attempt.

## Next bounded action

Reproduce the Julia memory/compiler fault with a reduced harness around repeated
JSON3 parsing, bounded file reads and Dict deepcopy, retaining all acceptance
assertions. Establish a stable available runtime/configuration, then rerun the
full suite, checking exact counts and no errors/skips. A different installation
requires user authorization; none was installed or requested automatically.
Before acceptance, audit all frozen limits (including total-byte aggregation),
status precedence and full producer-not-rerun/acceptance reference coverage;
extend tests first for any uncovered behavior. No gate may be weakened to avoid
the runtime fault. Run regressions and bind actual observed green results only
after the complete suite executes. Then complete slice-2 via recur-warp and
re-query before starting slice-3.

Recovery commands:

```powershell
recur reveal main.lang.runtime-evidence -d warps
recur warp show main.lang.runtime-evidence -d . --json
recur warp slices main.lang.runtime-evidence -d .
recur lang show demos/web-evidence-lab/main.lang.evidence.recur --scope assess.a -d .
```

This filename records blocked attention; the live map still correctly reports
slice-2 as pending without accepted evidence. No passing layer or result has
been invented to make the projection say blocked or complete.

produces: main.lang.runtime-evidence.runtime-blocker observed incomplete green gate
