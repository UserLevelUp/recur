# Dispatch verification — 2026-10-06

defines: main.command.warp.dispatch.verification observed local v1 checks
consumes: main.command.warp.dispatch dependency-aware asynchronous host assignments

`cargo test --locked` passed, including all seven Windows dispatch integration
tests. `cargo build --locked` passed. Existing flatten dead-code warning remains.
Tests use deterministic local PowerShell hosts; they do not establish provider
authentication or live model quality.

Covered behavior: preview without execution; two disjoint parallel assignments;
duplicate wakeups and simultaneous schedulers; conservative workspace overlap;
missing hosts; test-failure reasoning increase and fast-success decrease; host
timeouts without test-failure escalation; bounded coordinator passes; config and
verification-input drift; exact-attempt recovery preserving observations; optional
Lang design packets and separate implementation readiness. Produced attempts do
not accept gates or unblock the parent integration slice.

Init unit tests passed for additive defaults, retained custom values, inline
tables and repeat-run byte stability. Actual project init added disabled dispatch
and host/polling defaults; a subsequent dry-run proposed no writes. Actual
dispatch and coordinator previews launched no processes. Core inspection reported
zero live attempts for this feature Warp.

Full Julia regression uses matching explicit binaries:

```powershell
$env:RECUR_PROFILE = 'debug'
$env:RECUR_BIN = 'C:/src/recur/target/debug/recur.exe'
$env:RECUR_LANG_BIN = 'C:/src/recur/target/debug/recur-lang.exe'
$env:RECUR_WARP_BIN = 'C:/src/recur/target/debug/recur-warp.exe'
julia --startup-file=no julia-tests/runtests.jl
```

The earlier run without explicit Lang/core paths mixed binary generations and
is not acceptance evidence. The matching-binary run has one observed real-root
discovery failure at `julia-tests/main.command.warp.discovery.test.jl:209`.
`main.demo.blackjack-web` captures the existing `.split` child's layer filename
prefix and rejects the different Warp identity. The layer-selection code at
that boundary was not modified by this feature; those Blackjack artifacts are
preserved. The run finished with 34,172 passed, one failed, zero errors and
73 expected-broken cases, exit 1. Blackjack engine/Split, HTTP, assets and Lang
reference suites passed. Raw output is retained at
`C:/Users/marcn/Documents/Codex/2026-10-06/le/work/recur-regression.log`.

The four declared feature gates remain pending. No immutable acceptance layers
have been published, no missing historical red-first receipt has been invented,
and the broad regression gate is not claimed green.
