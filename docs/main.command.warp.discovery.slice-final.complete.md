# Slice final: Integrated verification

Status: complete

CLI fixtures and Cargo/Julia regression pass. Document scope and error semantics; verify real remaining Warp 10 and close this bubble.

Gate: verification. Retain commands and observed results before acceptance.

defines: recur.warp.discovery.slice.final Integrated verification


## Verified slice outcome

Completed 2026-09-05 against source baseline 4c71c9d plus this uncommitted change.

Commands and observed results:
- cargo test --quiet: 179 passed; seven ignored documentation tests.
- cargo build --profile release-safe: exit 0.
- julia julia-tests/main.command.warp.discovery.test.jl: 36 passed.
- julia julia-tests/runtests.jl: 2,436 passed, 73 expected broken; exit 0.
- cargo clippy --all-targets --quiet: exit 0, existing repository warnings.
- git diff --check and rustfmt --check --config skip_children=true on changed Rust: passed.

The focused suite checks default/explicit equivalence, empty roots, deterministic output,
read-only bytes, nested discovery, private-directory exclusion, marker-only exclusion,
completed filtering, invalid and duplicate maps, stale contracts, conflicting receipts,
failed checked evidence, invalid roots, and ring/coordinator projection parity.

Tested file SHA256:
```text
src/main.rs 9ca2c661d3e9a630dde957efdfa6ccc7eacdb34d962740d4e5b1ed884b480910
src/main_command_warp_impl.rs f5aa23938da7252b3b9fe8badf8b89b1023c177a9ddc466ef0c5f183927220bf
julia-tests/main.command.warp.discovery.test.jl dc53bd0661bb138887cffe1359262957289ed551cd1da90362408e8a39693d8a
julia-tests/runtests.jl 4e62a998fa50a032efda48898f4175eaea92e10a3588b7c5a7a3d4e68de57b92
```

Closeout queries use target/release-safe/recur.exe:
warp -d docs; warp list --all -d docs --json; warp merge main.command.warp.discovery -d docs --json;
reveal main.command.warp.discovery --json.
Default inventory must retain Warp 10 and hide this completed bubble after final acceptance.
The installed PATH binary has not been replaced. This map uses declared references to these
observed local tests, not machine-checked external evidence. See main.command.warp.readme.md
for scope and error handling; no unrelated bubble or private capsule was changed.
