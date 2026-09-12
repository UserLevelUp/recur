# Checked transition wire contract v1

Frozen during slice-0. Existing query-v1 and TOML legacy receipts keep their
semantics. New files are UTF-8 JSON with deny-unknown-fields structures.

## CLI and ownership

`recur lang evidence SOURCE --scope FUNCTION --contract POLICY [--receipt ATTEMPT]
[--status STATUS] [--expand] -d ROOT --json` is a new pure query. FUNCTION must
be qualified (`build.f`); unqualified selection is ambiguous and rejected.
Output is `recur-lang-evidence-report-v1`, with unchanged query-v1 shape under
`packet`, `assessment`, `observation`, `checked_inputs`, and `transition_status`.
The packet reuses the WIR1 projector with an explicitly empty recorded-file
inventory, qualified by `recorded_inventory: not-scanned`. No recursive discovery
is performed. Known config is a bounded optional `.recur/config.toml` read.
All exact header contracts, aliases, boundary edges and coverage remain visible.
Text and expanded views describe the same identities and verdicts.

`recur-lang warp SOURCE FUNCTION --checked-contract POLICY --receipt ATTEMPT
[--eventness E0_FILE] [--confirm] [--recover] -d ROOT --json` consumes the same
assessment. Checked mode is explicit; it cannot be combined with legacy --id.
Without --confirm it does not write, including when --recover is present.
Checked confirmation requires --eventness. Without --checked-contract, existing
companion behavior is unchanged; --recover alone is invalid. No binding runs.

Pure query/dry-run exit 0 for checked, absent or declared; 1 for failed, stale,
mismatched or ambiguous; 2 for malformed input or CLI errors. Checked confirm
returns 0 only for completed/idempotent acceptance; 1 for evidence rejection;
2 for unsafe paths, collisions, IO errors or incomplete/recovery-required state.
Stable evidence codes CE001 malformed, CE002 ambiguous, CE003 mismatched,
CE004 failed, CE005 stale, CE006 absent, CE007 declared. Reasons retain checker
diagnostics. Gate/freshness precedence: malformed > ambiguous > mismatched >
failed > stale > declared > absent > checked. Structural failures may stop
unsafe traversal; report examined reasons without claiming exhaustive checks.

## Policy and attempt schemas

Policy `recur-lang-checked-contract-v1`:

```
schema, contract_id, source, source_hash, scope,
aliases: {local_port: canonical_port},
transition: {current, slice, desired},
inputs: {specification: [path], implementation: [path], tests: [path],
         configuration: [path], runner: [path], behavior: [path]},
requirements: [{id, cases: [case_id]}]
```

All identities are nonempty, matching is case-sensitive. Requirement IDs and
case IDs within a requirement are unique. Reuse of a case across requirements
is allowed. Each role has at least one explicit actual input; source belongs
to specification. Policy path itself is automatically required in external
evidence source.files, binding changes to requirements/configuration. Policies
do not declare arbitrary command lines or infer dependency closure.

Attempt `recur-lang-checked-receipt-v1`:

```
schema, attempt_id, contract_hash, source_hash, scope, phase: red|green,
producer, runtime, evidence: reference, cases: [{id, outcome: passed|failed}]
```

contract_hash is FNV of exact policy bytes. Attempt IDs use ASCII letters,
digits, dot, underscore and hyphen, max 80 bytes; no dot/dot-dot path names.
Case IDs are unique; all required cases must be listed. The actual external
manifest must have the same producer and test kind. Its structured result
counts must agree exactly with listed case pass/fail totals, with no skipped
cases; existing checker validates outcome/count/fingerprint rules. Case labels
are producer claims, not proof that a test body ran; an authorized runner must
publish truthful case observations. This trust boundary is shown in reports.
A green phase cannot turn failed or empty results into checked evidence.

An `evidence:relative.json` reference uses existing warp-external-evidence-v1
and warp-external-result-v1. Other references, including a legacy native ACK,
are declared only and cannot authorize checked confirmation. No optional
artifact/test_receipt string is promoted to proof. Missing receipt is absent;
missing referenced manifest/input is malformed. Content drift is stale. Wrong
source/scope/alias/transition/producer or missing required input/case is mismatched.
Duplicate requirement/case IDs are ambiguous. Unsupported schemas are malformed.

One invocation selects exactly one immutable attempt file; no timestamp, filename
suffix or implicit history selection. `observation` preserves its phase/cases
independently of assessment. Older attempts remain separately queryable as
historical observations, even when their live-source assessment is stale.

## Read limits and freshness

Paths are nonempty root-relative forward-slash paths. Reject absolute, drive,
UNC, backslash, NUL, empty, dot and dot-dot components; canonical paths must be
contained files. Optional status/receipt omission is distinct from missing
named files. Root-local config cannot escape through a symlink. Unicode path
components are supported; Lang symbol grammar is unchanged.

Maximum source or JSON/config/status file: 1 MiB; other explicit input: 8 MiB;
aggregate unique canonical file bytes: 16 MiB; unique files: 64; requirements:
64; cases: 128 total unique; no history enumeration. At-limit input is permitted.
Bound readers before allocating or reading beyond limit+1, reject file growth.
Read shared checker inputs through a bounded reader callback; reuse validation
logic rather than unbounded rereads or another implementation of count rules.
No subprocess or whole-root acceptance scan is needed. The report's read-set
records actual paths/fingerprints from this invocation, not archived copies.

Current assessment is as of those reads, not an atomic snapshot of a mutable
filesystem. The confirmed actor rereads and compares the full read-set before
mutation, rejects drift, and requires exclusive operator ownership of the chosen
E0/transaction paths during the short write. It does not lock arbitrary external
tools or claim safety against concurrent hostile filesystem mutation.

## Durable transition and bounded recovery

New checked status schema `recur-lang-checked-status-v1` records request source,
qualified scope, policy/attempt fingerprints, complete assessed input set,
before/after paths, original E0 content fingerprint, attempt ID, and historical
acceptance. Transaction files are beneath `.recur/lang/checked/ATTEMPT/`;
existing symlink ancestors, conflicting prepared/accepted requests, unsafe IDs,
or an already occupied unrelated destination are rejected. Legacy statuses are
never read as checked status and never overwritten.

Publish a prepared intent before moving E0. Publish files with create-new
temporary staging, sync, and no-clobber final publication. No overwrite of an
accepted status, existing Ef, or unrelated file is allowed. Implementation may
use a no-clobber hard link E0->Ef followed by unlink E0 (same directory/filesystem)
to avoid platform rename replacement; unsupported filesystems fail before E0
removal. Prepared intent distinguishes interrupted work from accepted work.

The durable state sequence is no intent -> prepared with E0 -> prepared with
E0 and linked Ef -> prepared with Ef only -> accepted with Ef. Only the last
state emits ACK. An explicit --recover --confirm revalidates matching request,
current evidence, E0/Ef content and recorded intent before finishing an interrupted
sequence. A partial unpublished staging file is not an accepted/prepared record;
recovery must either safely republish the same request or report a specific
bounded blocker without moving E0. Unexpected files/bytes or stale evidence
block recovery. Do not roll back by overwriting an existing path.

Identical accepted replay verifies request fingerprints and Ef content; it may
return idempotent acceptance only while current evidence is checked. Conflicting
attempt reuse fails. Pure --status reports historical acceptance separately from
current assessment; drift does not erase the accepted historical receipt.
Fault injection is test-local, not a production command/environment backdoor.

## Development evidence and earlier gates

Root layout is the repository-root map plus warps/checked-transition immutable
attempts. Slice-0 uses declared baseline observation references, so later intended
source changes do not reinterpret a historical baseline as a current checked gate.
Exercise the baseline external checker separately against actual source files.

For checked slices, freeze assessed Rust module/query files and their tests at
acceptance. Later companion work lives in separate files and does not change
those frozen inputs. Record runtime executable identity as provenance; gate
fingerprints bind relevant actual source/test/config/runner files, not a rebuilt
shared target executable that changes for unrelated binaries. Do not claim this
explicit set is full dependency closure. Final integration adds all relevant
actual inputs and rechecks earlier manifests. If a correction requires changing
an accepted input, pause before the edit and plan a supported explicit contract
revision/successor with preserved history; do not accumulate stale/contradictory
layers or manufacture an explosion to force evolution. No such revision is
needed or authorized merely by freezing this policy.

defines: main.lang.checked-transition.wire.v1 exact checked-mode boundary
