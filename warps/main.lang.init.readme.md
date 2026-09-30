# Initialize recur-lang project policy

Warp: `main.lang.init`. Status: planned, implementation not started.

Add `recur-lang init` using the existing companion initialization conventions.
It installs a small `[recur-lang]` section into the nearest `.recur/config.toml`,
with preview, preserved preferences/comments and harmless repeated invocation.
Target language starts as `unspecified`; project authors can choose Julia, Rust
or another target. Planning flags are inert until a future planner consumes them.

The exact proposed CLI, defaults, output schema and acceptance gates are in
[the contract](main.lang.init.contract.md). The map retains companion-generated
UUIDs. No acceptance receipt is created merely by writing this plan or observing
the expected unsupported-command failure.

| Slice | Result |
| --- | --- |
| 0 | Frozen contract and standalone failing acceptance suite |
| 1 | Fresh install, preview, inherited config discovery and preservation |
| 2 | Invalid-input/path/publication safety and core-policy isolation |
| final | Green regressions, documented discovery and installed CLI smoke |

```powershell
recur warp show main.lang.init -d .
recur warp slices main.lang.init -d .
recur tree main.lang.init -d warps
recur trace-id recur.lang.init.v1 --scope '**' -d . --format full

$env:RECUR_BIN = (Get-Command recur).Source
$env:RECUR_LANG_BIN = (Get-Command recur-lang).Source
julia --startup-file=no -C generic --project=demos/web-evidence-lab julia-tests/main.command.recur-lang.init.test.jl
```

The last command is intentionally standalone and expected to fail until the
feature is implemented. It writes only temporary test fixtures. Existing project
configuration is not initialized by creating this Warp. Review the baseline
record for the actual run and its limitations.

Useful implementation references:

- `src/recur_lang_main.rs`: current companion exposes only `warp`.
- `src/recur_warp_init.rs`: missing-key insertion, preview and staged publication.
- `src/reveal_profiles.rs`: ancestor lookup, TOML preservation and concurrent-byte check.
- `src/project_config.rs`: shared config discovery and policy/lane classification.
- `julia-tests/main.command.warp.identity-policy.test.jl`: established init tests.

No parser extension, implementation generator, runner, secondary TOML file,
global setting or automatic upgrade of another companion's config is in scope.

defines: main.lang.init.plan first companion-policy initialization increment
consumes: recur.lang.init.v1 frozen initialization contract
