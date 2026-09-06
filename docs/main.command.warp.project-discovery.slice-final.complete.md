# Root acceptance

Status: complete

Run full Cargo/Julia suites. Bare repository-root inventory contains Warp 10 without fixture errors; retain completion and recovery records.

Gate: verification

defines: recur.warp.project.discovery.slice.final Root acceptance


## Verified slice outcome

Completed 2026-09-05 on source baseline 88ef6a6 plus this change.

Observed commands:
- cargo test --quiet: 179 passed; seven ignored documentation tests.
- cargo build --profile release-safe: exit 0.
- julia julia-tests/main.command.warp.discovery.test.jl: 75 passed.
- julia julia-tests/runtests.jl: 2,475 passed, 73 expected broken, zero failed; exit 0.
- cargo clippy --all-targets --quiet: exit 0 with existing warnings.
- git diff --check and rustfmt --check --config skip_children=true for changed Rust files: passed.

Regression coverage includes default/explicit CLI equivalence, hidden/direct .recur maps,
default and configured exclusions, scoped same-ID bubbles and rings, local checked evidence
and source drift, malformed roots/config, nested config precedence for scoped invocations,
overlapping roots, no writes, invalid maps, and a genuinely bare invocation from repository cwd.
The real-root regression uses tracked artifacts, not ignored local private files.

Final acceptance commands (use target/release-safe/recur.exe):
warp; warp list --all --json; warp merge main.command.warp.project-discovery -d docs --json;
reveal main.command.warp.project-discovery --json.
Default root inventory must retain Warp 10, omit completed bubbles, and report no errors
after this final layer is accepted. Diagnostic --scan-all intentionally exposes fixture problems.

Tested file SHA256:
```text
src\lib.rs 1028983335b286a83de65945327919a66ca4359000ef6acba0b5d9b96eef14a6
src\main.rs 4d6a0b124776d18d42dd419c5901d6d79ba883c8bea456a3a5848f8cf3a50e47
src\main_command_warp_impl.rs 710f2d9cb143d880a31105dc74c79b48a9e2d8c28db3f8c0bc62f404193f5964
src\project_config.rs 67f3e97299a7e111f83924a20cb2723b3fbf02353461ebb6077931dbc69adae8
src\warp_discovery.rs 4da7c39e27a6779a07a0e900f2bee37346b7b4fe8d2a14fbc3d8c72c4a23db7f
julia-tests\main.command.warp.discovery.test.jl 2838f1b0ea2a7b00b89805c9f0a2bbc3a1c3055ffa5919a61f057c5c4e7c06cb
```

This map uses declared references to observed local verification, not machine-checked
external receipts. Existing explicit query traversal is unchanged. Legacy dispersed layer
layouts need explicit queries or migration to co-located maps/layers; no such migration
was silently performed. Private capsules/config were not changed. Installed PATH binary
is not replaced; tests used the newly built release-safe binary.
