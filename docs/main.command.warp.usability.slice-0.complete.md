# Slice 0: red-first-contract
Status: complete

Existing Cargo/Julia coverage was retained. The initial new CLI test run failed as expected: 14 passed, 1 failed because create was unavailable. Negative-command assertions alone were not treated as proof of safety. The expanded implementation suite now has 85 passing assertions.

Acceptance gate: red-first-contract. Evidence: julia-tests/main.command.warp.usability.test.jl,
observed passing standalone run on 2026-09-06. This is reviewed declared evidence.
