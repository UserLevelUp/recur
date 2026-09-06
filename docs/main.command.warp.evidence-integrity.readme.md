# Warp: Completion evidence integrity

Created: 2026-09-05
State: completed; all nine slices accepted; no current slice remains.
Warp identity: `main.command.warp.evidence-integrity`

## Objective

Address the seven requirements in `main.command.warp.evidence-integrity.reported.reference.md` through
bounded, tested changes to existing Recur capabilities. Slice 0 captures the
current state (E0); Slices 1-7 repair demonstrated gaps; Slice Final specifies
the completed version (Ef). All slices are now implemented and verified; retained
results are in `main.command.warp.evidence-integrity.verification.reference.md`.

The report's BasicGameEngine build and 50 passing Visual Studio tests are
user-reported external evidence. They are not a local Recur test run.

## Sequence and report coverage

| Slice | Purpose | Report issue |
| --- | --- | --- |
| 0 | Current state and reproducible baseline | All seven report items |
| 1 | Unsupported suffix diagnostics | Issue 3 (high) |
| 2 | Reveal exposes effective state policy | Issue 4 (medium) |
| 3 | Supported policy-aware receipt workflow | Issue 2 (high) |
| 4 | Recorded completion and evidence semantics | Issue 1 (high) |
| 5 | Structured external evidence and freshness | Issue 6 (medium) |
| 6 | Whole-map contract and gate validation | Issue 5 (high) |
| 7 | Reconcile reveal summaries with receipts | Issue 7 (medium) |
| final | Completed version and integrated acceptance | All seven issues integrated |

The map explicitly requires all nine slices. Repairs form a sequential chain;
Final additionally depends directly on every prerequisite. Keep only one
current marker in this Warp. Slice 0 becomes a retained baseline completion
record after its audit is verified; it does not certify that repairs are done.
Final's gates passed and its record is retained in a `.complete.md` artifact.

## Existing capabilities and bounded changes

At baseline commit b34179d, `recur-warp complete` already writes confirmed
acceptance layers, `recur-git test-receipt` invokes existing Cargo/Julia tests,
and bubble map/merge already check missing dependencies, cycles, contract
identity and conflicting results. At baseline, gate coverage checked reference
presence. The completed repairs add opt-in checked external outcome validation.
Do not build replacement runners or assume the reported CLI was this version.

The current `contract_hash` implementation compares opaque strings. This map
uses `contract:...:v1` identifiers deliberately, with no claim of cryptographic
hashing. Each identifier binds the corresponding Slice document's acceptance
criteria and named gates. Change its version when that contract changes; retain
prior receipts as historical evidence. Result/source fingerprints have their
own separately documented semantics.

Planning is scoped to these report items. A specialization service, general
scheduler, build system, unrelated editor problems, and automatic repair of
other projects are excluded. Do not silently weaken acceptance if an external
artifact is unavailable: record the missing evidence and use labeled fixtures
for local behavior tests.

## Commands from the repository root

```powershell
recur reveal main.command.warp.evidence-integrity
recur tree main.command.warp.evidence-integrity -d docs --sep .
recur files "main.command.warp.evidence-integrity.**.current" -d docs --sep .
recur warp config -d docs --json
recur warp map main.command.warp.evidence-integrity -d docs --json
recur warp merge main.command.warp.evidence-integrity -d docs --json
recur trace-id "recur.warp.evidence.integrity.**" --scope "main.command.warp.evidence-integrity.**" -d docs --ext .md --format full
```

Use `recur-warp complete --help` for the existing acceptance writer. Supply
this Warp ID, exact slice ID, unique attempt ID, actual result fingerprint,
and every map gate's retained evidence reference. Review the dry run before
adding `--confirm`. This repair map retains declared references to reviewed test
records. For machine-checked external gates, use the explicit checked mode described
in `main.command.warp.evidence.readme.md`; bare references do not prove outcomes.

## Verification and recovery discipline

For each repair: capture the failing behavior, implement within the listed
scope, run focused Rust/Julia checks, retain results, then update Eventness.
Where the baseline already meets a requirement, record the reproducible proof
and improve guidance only if needed. Final runs the full regression suites and
an integrated positive/negative external-evidence scenario.

After interruption, reveal points to the current artifact; that artifact holds
observations, exact next commands and unresolved decisions. Stable trace IDs
connect acceptance criteria to implementation/test roles added during repairs.
Record material reasons and rejected alternatives before collapsing attention.
Keep useful history and report inputs when closing current work.

## Trace identities

```text
defines: recur.warp.evidence.integrity.contract precise completion claims and external evidence requirements
consumes: recur.warp.closed.loop.complete existing Warp implementation baseline
```
