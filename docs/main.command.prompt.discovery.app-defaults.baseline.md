# App-provided prompt defaults: contract revision v2

The user clarified that opinionated apps are the home of preconfigured prompts.
Pending contracts now cover immutable bundled defaults, project override precedence,
source/provider metadata and an explicit app-default opt-out. An empty project
registry no longer implies an empty effective catalog. The shared registry and
all aliases resolve the same app/project definitions without launching providers.

Slice 0's original v1 contract and accepted receipt remain historical evidence.
Only pending Slice 1–5 and final contracts advance to v2. No implementation is
claimed and no baseline completion receipt is rewritten.

Observed red baseline:
`julia --startup-file=no -C generic -O0 julia-tests/main.command.prompt.discovery.test.jl`

Result: exit 1; **46 passed, 33 expected failures, 0 errors, 0 broken**.
Log: target/prompt-discovery-app-defaults-baseline.log.
Test SHA256: `814fdf0275ebc7e27cbd7c80753b8470284f2d26eb3e7169537bfb27b33c4044`.

The suite remains outside runtests.jl until implementation passes. Baseline passes
check existing read-only behavior; they do not establish prompt functionality.

produces: recur.prompt.discovery app-default precedence red-first contract revision
