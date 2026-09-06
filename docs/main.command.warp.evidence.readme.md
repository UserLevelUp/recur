# Warp completion and external evidence

`warp-status-v1` retains its legacy verdict and adds `recorded_state`,
`evidence_status`, `contract_status`, `gate_evidence`, and `verdict_scope`.
`optimum` describes the queried Eventness/coverage; inspect the evidence and
contract fields to determine whether tests were checked. A bare completion
marker has absent evidence. Trace-role counts do not prove test success.

Evidence states are `absent`, `declared`, `checked`, `stale`, and `failed`.
Here `checked` means Recur validated the supplied structured result and scoped
content fingerprints. It does not mean Recur independently ran the producer,
authenticated its claims, or verified the completeness of the author's scope.
Every assessment names its validation method. Failed checks and missing result
artifacts are explicit; source or result changes produce stale evidence.

## Policy and receipt workflow

```powershell
recur warp config -d <project-root> --json
recur reveal <lane> -d <project-root> --json
recur-warp receipt <warp> <slice> --attempt-id attempt1 -d <project-root> --json
recur-warp receipt <warp> <slice> --attempt-id attempt1 --evidence tests=evidence:evidence/test.json -d <project-root> --confirm
```

Reveal displays effective Eventness policy, configuration path, and per-field
default/configuration provenance. Policy uses the nearest `.recur/config.toml`.
`[warp.suffixes]` accepts active/complete/interesting/blocked arrays, including
multi-part suffixes; matching uses the longest ending. Unknown endings receive
non-mutating guidance. Conflicting group assignments are rejected.

`receipt` previews a template by default. Confirm writes a lifecycle Markdown
declaration beside the map using the first configured completion suffix.
It records slice, attempt, contract, dependencies, gate policy and references.
An existing attempt under any suffix prevents overwrite. Templates may have
unfilled gates; status reports those gates unresolved. Receipt creation alone
does not produce a `warp-slice-layer-v1` acceptance layer.

After reviewing actual evidence, use the existing `recur-warp complete`
dry-run/`--confirm` workflow to publish an accepted layer. The companion and
pure map queries share structural validation. A layer for Final cannot cover
missing prerequisites; changing the contract ID invalidates prior acceptance.

## Checked gates

Legacy maps default to `evidence_mode: "declared"` and accept manual reference
strings as declared coverage. They report `declared-gates-satisfied` rather
than verified tests. Opt in per required slice:

```json
{
  "slice_id": "tests",
  "contract_hash": "contract:tests:v2",
  "depends_on": ["implementation"],
  "evidence_gates": ["all-tests"],
  "evidence_mode": "checked",
  "gate_rules": {"all-tests": {"kind": "test", "allow_skipped": false}}
}
```

Bind the gate to `evidence:<manifest-path>` through `--evidence` or a layer's
evidence map. Paths are relative to the explicit query/writer root, including
when the Warp map is discovered in a subdirectory. Use the same root for
writing and later queries. Parent traversal and symlink escapes are refused.
Bare references stay declared. Malformed, failed or stale structured evidence
does not cover the gate, including in legacy maps. Multiple supplied references
must all satisfy policy; a passing reference cannot hide a failing reference.

The v1 `contract_hash` is an opaque, exact-match contract identity. A `sha256:`
prefix does not cause cryptographic verification. Change the contract ID when
acceptance semantics change. Content fingerprints below are separate values.

## External artifacts

An external runner or explicit adapter produces normalized result JSON:

```json
{
  "schema": "warp-external-result-v1",
  "kind": "test",
  "outcome": "passed",
  "exit_code": 0,
  "tests": {"discovered": 50, "executed": 50, "passed": 50, "failed": 0, "skipped": 0}
}
```

`kind` is test, build, or scan. Build requires passed and exit code zero.
Test additionally requires nonzero execution, consistent totals, no failures,
and no skipped tests unless the gate explicitly allows them. Scan additionally
requires `matches: 0` (for checks such as no remaining Assert::Fail calls).
Original TRX/build logs can be retained alongside the normalized output; this
v1 does not parse arbitrary TRX, console text, or existing Markdown receipts.

The manifest binds the result and declared input scope:

```json
{
  "schema": "warp-external-evidence-v1",
  "kind": "test",
  "producer": "external test adapter",
  "project": "BasicGameEngine",
  "configuration": "Debug",
  "platform": "x64",
  "executed_at_unix": 1788566400,
  "result_artifact": "evidence/results.json",
  "result_fingerprint": "fnv1a64:<computed hexadecimal value>",
  "source": {
    "revision": "<recorded Git revision or null>",
    "dirty": true,
    "files": {"src/tests.cpp": "fnv1a64:<computed hexadecimal value>"}
  }
}
```

Compute fingerprints from actual bytes, not the example placeholders:

```powershell
recur warp fingerprint src/tests.cpp evidence/results.json -d <project-root> --json
recur warp evidence evidence/test.json -d <project-root> --json
recur warp merge <warp> -d <project-root> --json
```

FNV-1a 64 is a change-detection checksum, not a cryptographic signature.
The manifest's execution timestamp must be nonzero and not in the future.
`source.files` must be nonempty; freshness covers exactly those named files.
Include all relevant implementation/build/configuration inputs. New or changed
files outside that declared scope are not detected. `revision` and `dirty`
record producer provenance; Recur does not independently attest Git cleanliness.
Queries do not rerun tools. They return assessment JSON, so inspect `status`
even when the command exits successfully.

Cargo/Julia and `recur-git test-receipt` remain the existing test execution
mechanisms. Their Markdown receipts can be declared references; checked mode
requires an explicit normalized artifact/manifest, not an automatic conversion
of a historical passing summary into fresh evidence.

## Reveal reconciliation

Opt in with explicit capsule fields:

```text
warp.id = demo.release
warp.root = planning
goals.now = Desired product behavior
observed.state = pending
readiness.slice = slice-1
```

`warp.root` is relative to the reveal project root. Reconciliation uses the
same bubble projection as merge and reports missing/invalid maps as warnings.
`observed.state` supports pending, complete (recorded), or verified (checked
gates). `readiness.slice = none` denotes no selected next slice. Desired prose
is preserved and unassessed. Stale readiness or contradicted completion claims
produce warnings and suggested structured fields; no narrative is overwritten.

defines: recur.warp.evidence.integrity.external versioned evidence inspection contract
consumes: recur.warp.evidence.integrity.verification behavioral regression evidence
