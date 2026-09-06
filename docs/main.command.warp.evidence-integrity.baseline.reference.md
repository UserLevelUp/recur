# Evidence integrity baseline: 2026-09-05

Source revision: b34179dde5db61497f07b8a28fdbaa5610bd0510.
Working state: Warp planning documents were untracked; implementation matched HEAD.
Executables: recur 0.2.8, Cargo 1.97.1, Julia 1.12.0, Windows PowerShell.

- `cargo test`: exit 0, 176 passed; 7 ignored doc tests.
- `cargo build --profile release-safe`: exit 0; existing unused Element warning.
- `julia julia-tests/runtests.jl`: exit 0, 2292 passed, 73 expected broken.
- `RECUR_EI_BASELINE=1 julia julia-tests/main.command.warp.evidence-integrity.test.jl`:
  7/7 assertions; unsupported .verified receives only generic no-eventness error;
  a bare .complete with no structured evidence receives optimum.
- `recur warp config --json`: active=current, complete=complete,
  interesting=strange, blocked=blocked.

Source inspection: main_command_warp_impl validates bubble dependencies,
cycles, missing references, contract equality, conflicting results and nonempty
gate reference lists. It does not inspect referenced test outcomes. Existing
Julia bubble and companion regressions cover partial/stale/conflicting cases.
recur-warp complete already produces confirmed layers; recur-git test-receipt
already produces revision-bound results. Reveal exposes execution policy but
does not project the Warp suffix policy. No external Visual Studio run was
performed: the reported 50 passing tests remain external user observations.

This is a manually recorded audit of observed command results, not structured
external-runner evidence. The first repair gate is the suffix diagnostic probe.

defines: recur.warp.evidence.integrity.baseline observed implementation baseline
