# Prompt discovery verification

Date: 2026-09-06. Implementation acceptance for pending v2 contracts; the accepted
slice-0 v1 baseline and its historical red results remain unchanged.

Observed commands:

- `cargo build --locked --profile release-safe --bin recur --bin recur-warp`: exit 0.
- `cargo test --locked`: 185 passed, 0 failed; 7 ignored doc tests; exit 0.
- `julia --startup-file=no -C generic -O0 julia-tests/main.command.prompt.discovery.test.jl`:
  379 passed, 0 failed, 0 errors, 0 broken; exit 0.
- `julia --startup-file=no -C generic -O0 julia-tests/runtests.jl`:
  3584 passed, 73 known-broken, 0 failed, 0 errors; exit 0. The prompt suite is
  included in this runner. Conservative Julia settings match the established
  working configuration for this Windows machine.

Logs: `target/prompt-build.log`, `target/prompt-cargo-regression.log`,
`target/prompt-discovery-implementation.log`, `target/prompt-julia-regression.log`.
Final binaries were rebuilt before the final focused and full Julia runs.
Changed Rust files pass rustfmt checking; `git diff --check` passes.
Built executables are in `target/release-safe`; no global installation was performed.

Registry unit tests exercised Windows junction containment including a missing
target source, invalid source encoding/size, exact SHA-256 bytes, provider collisions
and portable unsafe paths. The CLI suite covers all three default prompts, typed
overrides, nearest config, aliases, references and decision-evidence scenarios.
The default test fingerprint assertion was corrected to the existing documented
`sha256:<hex>` contract; no assertion was weakened or converted to broken.

Prompt bodies are bundled in `src/prompts/`; no initialization or provider API is
required. Source and context operations are read-only. The app catalog interface
supports additional compiled catalogs; project registration supports custom traits.
No model response was generated or evaluated. Scenario rubrics are future model
evaluation criteria, not proof of LLM quality.

Bounded packets disclose omitted data. Checked/transitive gate assessments require
the normal Warp query with its evidence root and are not silently read beyond the
packet budget. The packet exposes raw selected maps/receipts and diagnostics when
such a projection is unavailable.

## Verified working-file fingerprints

SHA-256 of the working bytes used for verification (before Git line-ending normalization).

| File | SHA-256 |
| --- | --- |
| src/prompt.rs | 1750c30c184721e4d27f9c205aed4e7b640594473706d18818bab71584ccb06e |
| src/prompt_context.rs | 8e417c7aec98653c3d37bd58b733fc2848ff3ed73226d8640a56e9dae4f21343 |
| src/prompts/warp.naming.md | e6598aa448291ef2faa3c9c6eaeee057dedf5a39e723515b00ba8f9f76d1068d |
| src/prompts/warp.slicing.md | 75fca5a6fef82f1192b8e6b400b7d451860d85d717fd201829ecac5cf8ce2382 |
| src/prompts/warp.recovery.md | bdc84f9ad9ed8337861a408b2fcd4320d7a8615a441c5e57656bdfdd3b01b1c0 |
| julia-tests/main.command.prompt.discovery.test.jl | fccb2e687f04e20f22fa83412943af0bdad88df07c3947c8aa4bbd49e1e07a46 |
| julia-tests/main.command.prompt.defaults.cases.jl | faff486c742e882ab002509f34ece672dacbdb61cb88f3281234b58f4af09f6c |
| julia-tests/main.command.prompt.edges.cases.jl | e67467f827b313b22bb6eaac1a408344b7754e079ae11a8ebfd696ac1e7f089b |
