---
name: recur-demo-tests
description: Select, run and maintain optional Recur demo test suites and scoped coordinator checks. Use for demo validation and test routing, keeping unrelated demos out of ordinary regressions.
---

# Recur Demo Tests

Resolve the Recur repository root and the demo selected by the user or current
Warp slice. Read its contract, tests and local instructions. In another project,
inspect its actual runner before assuming these command names or conventions.

In this repository `julia-tests/runtests.selection.jl` is the demo registry;
`README.julia-tests.md` describes routing. Default `runtests.jl` runs core tests
without demo suites. An exact `--demo NAME` runs that demo alone. Repeat the flag
for explicitly selected demos; `--with-core` adds core tests. Invalid names/options
fail before setup. Direct demo tests are also explicit selections.

```powershell
julia julia-tests/runtests.jl --list-demos
julia julia-tests/runtests.jl --demo blackjack-web --dry-run
julia julia-tests/runtests.jl --demo blackjack-web
julia julia-tests/runtests.jl --demo blackjack-web --with-core
```

Use the project's dependency environment and a matching Recur binary/profile.
Installed binaries may lag local changes; do not replace them simply to run a
demo. Keep runtime/provider assumptions in demo configuration or scoped launchers,
not universal skill instructions. Check native executable paths and platform
policy when selecting a launcher; retain the failure rather than weakening
system security settings to make a script run.

Keep semantic unit checks, HTTP/session checks and browser behavior distinct.
Run additional language/runtime suites only when relevant to the selected demo.
When browser behavior changes, verify the actual served site and important
interactions. Preserve unrelated running demo servers. CLI test fixtures may
reference small demo artifacts without executing those demo suites.

When adding a demo, register its wrapper under its exact name and test selection,
deduplication and invalid-name handling in `main.test-selection.test.jl`. Keep
new demo-specific includes out of the unconditional core runner. Demo dependencies
may load shared runtime helpers; this does not select the other demo's test suite.
For coordinator checks, use the selected demo workspace, exact test argv and
the complete relevant source/config input fingerprints. Do not attach every
demo to each Warp slice or dispatch tests solely because a demo exists.

Observe assertion totals, expected-broken cases, exits and logs without treating
old counts as requirements. Test success is distinct from reviewed Warp gate
acceptance. Preserve historical observations; publish current results with their
scope and material limits rather than editing old evidence to claim a new run.
