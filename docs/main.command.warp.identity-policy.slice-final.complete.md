# slice-final: regression-closeout
Status: complete

Contract v2 depends on slice-4. Cargo: 180 passed, 0 failed, 7 ignored doc tests.
Full Julia runner: 3205 passed, 73 known-broken, no failures, exit 0.

Command: `julia --startup-file=no -C generic -O0 julia-tests/runtests.jl`.
The conservative Julia settings were required after default-runtime access
violations. One nonfatal compiler diagnostic remains recorded in the passing log.
PowerShell hello, real pipeline and timestamp-fixture checks now pass after reboot.

Acceptance gate: regression-closeout.
Evidence: main.command.warp.identity-policy.verification.current.md.
