# Identity-policy verification — 2026-09-06

Implementation gates for Slices 1–4 passed against the source fingerprints in
`main.command.warp.identity-policy.verification.source-hashes.json`.
These are observed working-tree results, not clean-commit Git receipts.

Result hash: `sha256:9333efa7a3c28f9b9978a350e1c6e2a02d112cdbff765a771e9b986f5919fb80`.

## Observed passing checks

- `cargo build --locked --profile release-safe --bins`: passed.
- `cargo test --locked`: 180 passed, 0 failed; 7 existing ignored doc tests.
  Includes the Windows config-lock test proving rollback after publication failure.
- `julia julia-tests/main.command.warp.identity-policy.test.jl`: 144 passed.
- `julia julia-tests/main.command.warp.list-format.test.jl`: 58 passed.
- All 14 Warp-related Julia files passed individually: companion, bubble,
  companion-policy, discovery, docs-reconciliation, evidence-integrity,
  identity-policy, list-format, query-compatibility, ring-topology, structure,
  usability, warp status and the Improvement 27 fixture contract.
- `julia julia-tests/runtests.reveal.jl`: 31 passed in a direct invocation.
- `git diff --check`: passed (repository line-ending warnings only).

Raw local logs: `target/identity-policy-cargo-test.log` and
`target/identity-policy-warp-tests.log`. The sole new-code regression found by the
broader run was loss of the human `error:` diagnostic prefix. It was restored
inside the escaped `error` value; all existing discovery assertions now pass.
The new identity and presentation tests are included in `julia-tests/runtests.jl`.

## Initial full-run blocker (resolved after reboot)

`julia julia-tests/runtests.jl` was attempted twice. The first attempt stalled;
the retry demonstrated hanging PowerShell subprocesses in the existing real-pipe
test and the psyche file-timestamp fixture. Separate PowerShell 5 and 7 no-profile,
noninteractive hello probes also timed out. Clearing only task-owned hung
processes did not restore the host. No test assertions were disabled or weakened.

The full runs were terminated and are not passing regression evidence. Their
partial logs are `target/identity-policy-julia-test.log` and
`target/identity-policy-julia-test-retry.log`; the retry also predates the now-fixed
diagnostic-prefix change. Those interrupted runs do not establish acceptance.

## Final regression acceptance after reboot

Both PowerShell versions now execute a no-profile, noninteractive hello command
and exit normally. The real pipeline suite passed 44 tests and the psyche suite
passed 27 tests, including the formerly hanging timestamp fixture.

`cargo test --locked` passed again: 180 passed, 0 failed, 7 ignored doc tests.
Log: `target/identity-policy-cargo-test-after-reboot.log`.

The full, unchanged Julia runner passed with:

```powershell
julia --startup-file=no -C generic -O0 julia-tests/runtests.jl
```

Observed exit code: 0. Summary: **3205 passed, 73 known-broken, 3278 total**,
no failures or test errors. Runtime: 1m08.9s. No assertions were removed or marked
broken to obtain this result. The source fingerprints above still match.

Log: `target/identity-policy-julia-test-generic.log`.
Log SHA256: `a28e4d588890feb7e6ec850d3aee8243149253c6b53f698719bbc06854fa94aa`.

Qualification: default Julia 1.12.0 invocations suffered access violations after
reboot; minimal compilation also crashed in the main.lang tests. Conservative
CPU/optimization settings allowed the complete suite to finish. The passing run
still emitted a nonfatal internal type-inference BoundsError diagnostic in a
JSON3 filter; all assertions completed successfully. This acceptance covers the
Recur implementation and the explicitly recorded test configuration, not a claim
that the default Julia runtime is healthy. Crash logs remain under
`target/identity-policy-julia-test-after-reboot*.log` and
`target/identity-policy-julia-test-final.log`.

Slice-final contract v2 satisfies `regression-closeout` on this evidence. The
original Slice 0 receipt is unchanged.

produces: recur.warp.identity.policy implementation and focused verification evidence
