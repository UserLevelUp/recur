# Lang init contract baseline — 2026-09-29

This records the planned command's intentionally red test baseline. It does not
accept a slice or claim an implemented `recur-lang init`.

Observed against the installed `C:/Users/marcn/.cargo/bin/recur-lang.exe` (0.2.8):
`--help` exposes only `warp`; `init` is an unrecognized subcommand. The matching
core executable is `C:/Users/marcn/.cargo/bin/recur.exe`.

```powershell
$env:RECUR_BIN = 'C:/Users/marcn/.cargo/bin/recur.exe'
$env:RECUR_LANG_BIN = 'C:/Users/marcn/.cargo/bin/recur-lang.exe'
julia --startup-file=no -C generic --project=demos/web-evidence-lab julia-tests/main.command.recur-lang.init.test.jl
```

Julia 1.12.7 completed this run without a compiler/runtime fault. Process exit 1:
**57 passing, 44 failing, 0 errors, 0 broken/skipped assertions**, 101 observed
assertions total. The full raw output is retained in
`lang-init/main.lang.init.attempt-0.red.log`. SHA-256 input and output fingerprints
are recorded in `main.lang.init.baseline.json`.

| Test group | Pass | Expected fail |
| --- | ---: | ---: |
| CLI discovery | 1 | 3 |
| Fresh preview/install/defaults/repeat | 2 | 5 |
| Ancestor config/comments/false/unrelated values | 2 | 4 |
| Inline and empty tables | 0 | 4 |
| Invalid nearest config/no mutation | 44 | 22 |
| Invalid root/config directory | 5 | 3 |
| Policy table isolated from lane discovery | 0 | 2 |
| Symlink escape | 3 | 1 |

The passing assertions largely observe exit codes and unchanged fixtures when
the old executable refuses the command. They do **not** establish config
validation, safe publication, or initialization support. Assertions requiring a
successful response are guarded to avoid cascading KeyErrors; additional checks
will execute once the command exists. The future green count will therefore be
larger than this baseline count. Symlink fixture creation succeeded on this host.

The suite covers preview determinism and absence of created directories, exact
defaults, whole-config preview, repeat byte identity, inherited-root selection,
Unicode/comments, custom fields, explicit false values, inline/empty tables,
invalid schema/types, malformed nearest config, missing/file roots, config path
as a directory, reserved policy/lane identity, and an escaping directory symlink.

Deterministic concurrent-edit and publication-failure tests remain explicit
implementation gates in the contract. They need Cargo fault-injection tests,
including the established Windows file-sharing lock case. No claim is made that
ordinary CLI fixtures already cover those races.

The map is queryable with four pending slices and `slice-0` ready. It has no
acceptance layers, conflicts or warnings. The red suite is intentionally absent
from `julia-tests/runtests.jl`. Existing Hold'em work and project config were
preserved. No production Rust implementation, build, install, commit or push was
performed for this Warp-creation task.

consumes: recur.lang.init.v1 baseline for the proposed CLI and configuration contract
produces: main.lang.init.red-baseline observed unsupported feature, not acceptance
