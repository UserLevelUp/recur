# Inspector contract and red tests — 2026-09-10

Contract: `contract:main.lang.dogfood.slice-2:v1`.
Gate: `inspector-contract-and-red-tests`.

Before writing routes, authored `demos/web-evidence-lab/main.lang.api.contract.md`,
`main.lang.api.recur`, and `main.lang.api.test.jl`. The linked API contract freezes
catalog selection, exact argument vector, status/error shape, complete packet
retention, WIR1-only display, empty selection and separate evidence categories.
The Lang file declares bundles once in the header, composes symbols and shares
in the body, and declares events/transitions in the footer.

Observed with Julia 1.12.7, `--startup-file=no -C generic
--project=demos/web-evidence-lab demos/web-evidence-lab/main.lang.api.test.jl`:
exit 1, one failed `implemented` assertion. No route or MainLangAPI module exists
at this point. This establishes absence only: the guarded behavioral assertions
have NOT run yet. They must be exercised after implementation in slice-3.
This intentionally red suite remains outside the normal Julia runner.

Hand-authored fixtures assert independent expectations for exact catalog roots/
argv, same-letter scoped identities, canonical aliases, coverage missing, CIR1
rejection, malformed display fields/JSON, query diagnostics, boundaries/findings,
original source/hash/extra fields, empty header and transport. Real loopback
assertions cover all documented scopes and byte snapshots of both demo trees and
repository configuration, including existing receipt files.

Observed local `target/release-safe/recur.exe lang check main.lang.api.recur
-d demos/web-evidence-lab --json`: exit 0, source hash
`fnv1a64:dba65c1b438cb118`, `sound-within-coverage`, no findings,
`whole_source_validated: false`, `execution: not-run`, LANG101 coverage notice.
This is static fragment evidence only; prose and API behavior were not executed.

produces: main.lang.dogfood.inspector-contract frozen tests-first boundary

Preimplementation input fingerprints:

demos/web-evidence-lab/main.lang.api.contract.md: sha256:bfcee6ce6d146c365a4f008b14b66bf67f04110277683530d5e7eaf8d16067c5

demos/web-evidence-lab/main.lang.api.recur: sha256:d997b2d36969a56361f4454d4f439e98192b4ea58ccce70ed644fb32dc30eb5d

demos/web-evidence-lab/main.lang.api.test.jl: sha256:52ecd076072b4f1bdd24becb0f16ddd1b91e59964bc9fc6892defb4e3c4f649d
