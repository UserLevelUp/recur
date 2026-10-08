# Binary hash repair: current lineage and evidence

artifact.type = lane
eventness.role = current-reference
publish: main.command.watch.eventness.evidence.lineage current navigation record
consumer: main.command.watch.eventness.evidence.binary.identity historical investigation
consumer: main.command.watch.eventness.evidence.bundle retained artifact implementation
consumer: main.command.watch.eventness.evidence.final.ready observed completion signal
warp.id = main.command.watch.eventness.evidence
warp.uuid = 01a11d10-5e20-7023-a567-6d2d57373cce
status = repair implemented and published; re-query gates for current freshness
implementation.commit = 0e1acd4
integration.commit = a87e59d16bcdddb99827b174f1b409428c41eea6
verification.bundle = watch-native-v9

Start here to distinguish the original mismatch, reproduced causes, selected
artifact identity, and observed repair. Trace annotations are navigation claims;
occurrences and Watch notifications do not establish execution or acceptance.

## Why the hashes differed

publish: main.command.watch.eventness.evidence.binary.cause.metadata reproduced PE metadata difference

Two identical-source builds differed in 20 bytes: one COFF timestamp byte, three
debug timestamp bytes and a 16-byte PDB GUID. The inspected code/data sections
matched, but whole-file SHA-256 correctly identified different executables.

publish: main.command.watch.eventness.evidence.binary.cause.build.graph distinct Cargo dependency selection

Cargo build and test also selected different dependency artifacts, including
Serde's additional alloc feature in the test graph. Source identity, machine-code
similarity, and exact executable identity answer different questions.

Read `warps/main.command.watch.eventness.hash-investigation.current.md` as the
historical pre-repair investigation. Its "fixes pending" and "promotion held"
wording describes that stage, not the current branch state. The byte comparison
is `warps/watch-eventness/observations/hash-investigation-20261008/classification.json`;
commands and dependency observations are in that directory's `report.json`.
The original v6 binary was not retained, so the precise historical byte difference
remains unknown. The reproduced mechanism is evidence, not reconstruction of v6.

## What fixed it, and where the proof lives

| Trace ID | Artifact to read | Meaning |
| --- | --- | --- |
| `main.command.watch.eventness.evidence.binary.identity` | `warps/main.command.watch.eventness.hash-investigation.current.md` | Historical mismatch and bounded causal claims |
| `main.command.watch.eventness.evidence.bundle` | `warps/watch-evidence/main.command.watch.eventness.evidence.bundle.cjs` and `.bundle.test.cjs` | Exact copies, integrity guards and negative controls |
| `main.command.watch.eventness.evidence.binary.retained` | `warps/watch-eventness/observations/watch-native-v9.binaries.json` | Selected executable paths, exact SHA-256 and build provenance |
| `main.command.watch.eventness.evidence.binary.rebuild` | `warps/watch-evidence/observations/rebuild-after-retention.v9.json` | Mutable target changed; retained identity and evidence remained valid |
| `main.command.watch.eventness.evidence.final.ready` | `warps/watch-evidence/main.command.watch.eventness.evidence.final.observed.complete.md` | Observed outcome, failures, test counts and limits |
| `main.command.watch.eventness.evidence.lineage.model` | `warps/watch-evidence/main.command.watch.eventness.evidence.lineage.recur` | Optional static contract model of build, retention and verification |

publish: main.command.watch.eventness.evidence.binary.retained selected v9 manifest reference
publish: main.command.watch.eventness.evidence.binary.rebuild recorded replacement proof reference

The manifest binds the retained core to SHA-256
`d6c4b0d0a5cb467cb7f1ea3fa4d8194df9e6f19be0ffe6f8dd84547cc5fd4dc4`.
The later mutable target became
`d57058fb8e80e6d17c53d259269019ad6e668aed5d32e6f0e21900384c9e61d8`.
The retained core kept its identity. Do not substitute a new build, edit historical
hashes, or strip linker metadata to manufacture an artifact match.

The final Eventness records 546 passing selected checks and both Warps at 5/5
accepted slices. Query live gates to establish present freshness. The old accepted
layers and new immutable refresh receipts retain their separate histories.
Local binaries under `.recur/evidence-binaries/watch-native-v9/` are ignored by Git;
a clone without them cannot reproduce these local evidence assessments.

## Optional Lang and Watch boundary

Lang declares build -> retain -> verify contract aliases. Custom binding names
are descriptive links to the existing code; Lang does not execute those bindings,
prove SHA-256 policy, validate arbitrary field semantics, or accept either Warp.
The observed runtime proof remains in the artifacts above. No new demo is selected.

The existing Watch topic `main.command.watch.eventness.evidence.results` published
only `main.command.watch.eventness.evidence.final.ready`; the saved receipt is
`warps/watch-evidence/observations/watch-result.v9.json`. This reference does not
dispatch an agent or create another notification.

From the repository root, use the retained CLI (or a compatible installed CLI):

```powershell
$recurEvidence = '.\.recur\evidence-binaries\watch-native-v9\recur.exe'
& $recurEvidence trace-id 'main.command.watch.eventness.evidence.**' --scope 'main.command.watch.eventness.**' -d warps --json
& $recurEvidence lang check warps/watch-evidence/main.command.watch.eventness.evidence.lineage.recur -d . --json
& $recurEvidence lang report warps/watch-evidence/main.command.watch.eventness.evidence.lineage.recur --scope verify -d . --json
& $recurEvidence warp show main.command.watch.eventness.evidence -d . --json
& $recurEvidence warp show main.command.watch.eventness -d . --json
```

Saved static and live-query observations for this navigation change are in
`warps/watch-evidence/observations/lineage-v2/`. The first query attempt in
`lineage-v1/` found that the trace reader truncated a hyphenated ID segment;
the declaration now uses separate dotted `build.graph` segments. Its failed
validation is retained as a naming deficiency, not a passing observation.
These are query observations, not
a fresh run of the 546 runtime tests or new acceptance receipts.
