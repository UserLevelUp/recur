# Pipeline compatibility verification

Date: 2026-09-05
Source context: HEAD 88ef6a6 plus retained uncommitted discovery, trait and documentation work.
This step changes tests/docs only, not production behavior or existing test expectations.

Observed results:

- julia julia-tests/main.command.pipeline.compatibility.test.jl: 44 passed.
- cargo test --quiet: 179 passed; seven ignored documentation tests.
- julia julia-tests/runtests.jl: 2,584 passed; 73 expected broken; zero failed, exit 0.
- git diff --check: passed.

Tests used target/release-safe/recur.exe and actual process pipes, including PowerShell.
The shell Unicode equality test initially failed with café becoming caf?. Explicit
session UTF-8 settings fixed transport; the exact equality assertion was retained.
Test helper command quoting/pipeline construction were also corrected before acceptance.

The existing successful text response for empty merge stdin is preserved and documented.
Producer exit 7 plus consumer exit 0 is explicitly tested; consumer success alone must
not authorize a writer. Invalid JSON and unsupported shapes report nonzero receiver
status with diagnostics on stderr. No automatic execution/writer policy was added.

Tested SHA256:

```text
julia-tests/main.command.pipeline.compatibility.test.jl ac426ede7f48071195da221df27ed0d27d0fc3864c371e24e1da1f7d1f61cb3b
julia-tests/runtests.jl e81b8a7735d8bc6116c6807f3d980e0f3792d7d498941676e559d92f2b67970f
```

See main.command.pipeline.compatibility.readme.md for examples and caveats.
Changes remain uncommitted. These are observed local results, not external producer receipts.

produces: recur.pipeline.compatibility.verification existing behavior preserved through real pipe regression checks
