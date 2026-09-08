# Artifact-types implementation verification

Date: 2026-09-07. Working-tree implementation based on
`10d20153c79a631adceb7e6dabff9ef9d2977f70`; no commit or global install performed.

## Observed implementation checks

- `cargo build --locked --profile release-safe --bins`: exit 0. Local executables
  are under `target/release-safe`; the installed PATH binary is unchanged.
- `cargo test --locked`: 188 passed, 0 failed, 7 existing ignored doc tests; exit 0.
- `julia --startup-file=no -C generic -O0 --compile=min julia-tests/main.command.reveal.artifact-types.test.jl`:
  123 passed, 0 failed/errors/broken; exit 0 with Julia 1.12.0.
- The same focused suite with `--inline=no` in place of `--compile=min`:
  123 passed, 0 failed/errors/broken; exit 0.
- Changed Rust files pass rustfmt checking; `git diff --check` passes.
- Local binary demo listing reports `agent.backend`, `persona.skippy`, and
  `skill.recur.eventness` with explicit types. Filtering for skill returns only
  `skill.recur.eventness`. Queries left the example files unchanged.

The [baseline](main.command.reveal.artifact-types.baseline.md) records the initial
red suite and passing legacy reveal/init tests. The green new suite is now included
in `julia-tests/runtests.jl`; the separate persona-skills suite remains standalone.

Coverage includes custom metadata types, duplicate/invalid/conflicting declarations,
legacy untyped capsules, configured prefix specificity and opt-out, mixed separators,
human/JSON provenance, exact-path selection, filtered ambiguity, exact type mismatch,
custom suffixes, explicit hidden roots, nearest nested config, sibling exclusion,
generic tree/files discovery, and inert command/reference fields.

Rust tests additionally exercise Windows junction escapes/cycles and metadata
conflicts that cannot fall back to prefix hints. Windows symlink creation is not
permitted on this machine; junctions exercise the reparse-point traversal boundary.
The Unix branch of that test creates symlinks and was not executed on Windows.

Shared classification lives in `src/reveal_artifact.rs`; companion profiles can
reuse it. No activation, loader, runner, migration or default prefix seeding was
added. Explicit `-d` now limits discovery even when nearest config is above it;
this intentional compatibility change is covered and documented.

## Full regression status

Final regression acceptance passed with the already-installed Julia 1.12.4:

```powershell
& 'C:/Users/marcn/AppData/Local/Microsoft/WindowsApps/julia.exe' --startup-file=no -C generic julia-tests/runtests.jl
```

Observed: **3707 passed, 73 existing known-broken, 0 failed, 0 errors; exit 0**.
This uses normal optimization and the complete runner, including the 123 new
artifact-types checks. The run completed in 1m53.3s. Evidence log:
`target/artifact-types/full-julia124-default.log`.

Earlier Julia 1.12.0 runs failed internally across several configurations: an access violation
in compiler inlining, an interpreter error during Sudoku under `--compile=min`,
and access violations with inlining or module caches disabled. Isolated Sudoku
teaching tests passed 139 checks with caches disabled. The installed Julia 1.12.4
also failed with conservative optimization in package precompilation and one
cache-disabled run. Those failures remain recorded and do not count as passing
evidence. Normal optimization on Julia 1.12.4 supplied the successful full run;
the `julia` executable on PATH still resolves to 1.12.0. No runtime was installed
or upgraded and no global PATH changes were made.

Logs are retained locally in `target/artifact-types/`: `build.log`,
`cargo-final.log`, `red.log`, `green.log`, `noinline.log`, and `full-*.log`.
No failing test was removed, weakened, or marked broken.

produces: recur.reveal.artifact-types.classification metadata and prefix acceptance
produces: recur.reveal.artifact-types.query-integration bounded selection acceptance
produces: recur.reveal.artifact-types.guidance-and-purity documented inert examples
produces: recur.reveal.artifact-types.regression-closeout full Rust and Julia acceptance

## Verified working-file fingerprints

SHA-256 of working bytes before Git line-ending normalization.

| File | SHA-256 |
| --- | --- |
| src/reveal_artifact.rs | e045aeb283dc7ab35a2d3321bb9f718942260c45729ba5ce78c0fa8f46710740 |
| src/main_command_reveal_impl.rs | 844aacf209714eb8f93a849b189c4eb364ffe505e7cfbb7ea86851203d0951bf |
| src/project_config.rs | d01a5a297fc68a241ec6f248e730c8aadd11c0980ada3d97a787beb9183a7a63 |
| src/main.rs | 9e51360e817267946d404988150423347d6bf93a19d396fa9ad4ae9c7bcae021 |
| src/lib.rs | 559103f236616a84d84debccda0e7efef26f2e323fca40064a2b39cce7ee8811 |
| julia-tests/main.command.reveal.artifact-types.test.jl | d3a085ab1367edd820253015decb44c5f63c14e454410fff019372ffe7aba8fa |
| julia-tests/runtests.jl | 4acba0641055b505f8dcd520bba8851b6dd2f748cf48b8ac2221d19f470f06d9 |
| docs/main.command.reveal.artifact-types.contract.md | 0739b03fd7803396f7daac64bcc5532d7e47048cbca15aae6aa85ff469906caf |
