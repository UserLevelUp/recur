# slice-0: baseline-contract
Status: complete

Observed 2026-09-06. The command, registry, output-schema, scope/budget and alias
contracts are frozen in main.command.prompt.discovery.readme.md. The seven-slice
map was scaffolded through the local recur-warp create command with fresh UUIDv7
bubble/slice identities, then placed in docs alongside the repository's other maps.

## Red-first execution

Command:
`julia --startup-file=no -C generic -O0 julia-tests/main.command.prompt.discovery.test.jl`

Result: exit 1; **43 passed, 29 expected failures, 0 errors, 0 broken**.
Failures expose absent prompt commands and absent trait/reveal prompt metadata.
Successful no-write assertions and legacy fields do not imply the feature exists.
Log: target/prompt-discovery-red-baseline.log.

Test SHA256: `591a09a9338118787803231c8723e3a3e8a28d8af4055098f749ea5b088524e0`.
Readme SHA256: `72b47df0d3fc0836799f991913c2fe6d063d8570397a4f13fedd5e4cc791de20`.

## Passing compatibility baseline

All ran with `julia --startup-file=no -C generic -O0` against local release-safe binaries:

- julia-tests/main.command.trait.capabilities.test.jl: 65 passed.
- julia-tests/runtests.reveal.jl: 31 passed.
- julia-tests/main.command.warp.usability.test.jl: 85 passed.

Logs: target/prompt-discovery-trait-baseline.log,
target/prompt-discovery-reveal-baseline.log, target/prompt-discovery-warp-baseline.log.
Conservative Julia settings reuse the configuration that completed the preceding
identity-policy regression after default Julia runtime crashes.

The new test file is deliberately outside runtests.jl until implementation is
green. Production code and command availability were not changed by this planning
work. Later slices must extend the readme's remaining edge cases before acceptance.

Acceptance gate: baseline-contract.
produces: recur.prompt.discovery red-first contract and compatibility baseline
