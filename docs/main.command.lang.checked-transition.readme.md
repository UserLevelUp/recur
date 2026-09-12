# Checked Lang evidence and confirmed transitions

The additive checked mode connects one qualified WIR1 function to one explicit
external test attempt. It reuses the existing parser and external evidence checker.
It does not execute the declared function, run a producer or scan receipt history.

Use a binary whose help exposes `lang evidence` and `warp --checked-contract`.
This repository's compiled binaries are `target/debug/recur.exe` and
`target/debug/recur-lang.exe`; installed binaries can lag despite the same version.
No installation is required to use the local build.

```powershell
.\target\debug\recur.exe lang evidence spec.recur --scope build.f `
  --contract policy.json --receipt attempt.json -d PROJECT --json

.\target\debug\recur-lang.exe warp spec.recur build.f `
  --checked-contract policy.json --receipt attempt.json `
  --eventness build.current.md -d PROJECT --json
```

The second command is a dry run. Add `--confirm` to authorize the exact E0-to-Ef
transition after a checked assessment. The Eventness stem must equal declared
E0; Ef retains its extension and directory. The producer's case IDs must cover
the policy requirements and agree with the checked aggregate counts. A native
ACK or arbitrary test-reference string remains declared evidence.

The full JSON schemas, exact exit codes, limits and association rules are in the
[frozen wire contract](../warps/main.lang.checked-transition.wire.contract.md).
The hand-authored fixture in `tests/support/checked_fixture.rs` shows complete
policy, attempt, result and external manifest shapes; its synthetic results are
test inputs, not observations of a production worker.

The policy binds actual specification, implementation, tests, configuration,
runner and behavior inputs. The external manifest also includes the policy file.
Use fresh distinct attempt IDs and preserve old observations. Paths must be
root-relative, forward-slash, contained files. Source/JSON/config/status files
are limited to 1 MiB, other explicit inputs to 8 MiB, and total reads to 16 MiB
and 64 unique files. No snapshots substitute for actual source fingerprints.

`recur-lang-evidence-report-v1` separates:

- `packet`: exact header/contracts, body aliases and dependency boundaries,
  intended footer events, static findings and fragment coverage.
- `observation`: the selected producer's phase and named case claims.
- `assessment` and `checked_inputs`: relevance and freshness as of bounded reads.
- `transition_status`: historical record and whether it remains currently accepted.
- `execution: not-run` and `recorded_inventory: not-scanned`: explicit limits.

The inspector can consume this envelope without changing its query-v1 renderer.
Read `packet.coverage`; a supported fragment never validates unknown syntax or
runtime behavior. Case labels and producer identities are claims. Fingerprints
detect changes; they do not prove authenticity or complete dependency coverage.
No HTTP/browser adapter is delivered by this capability.

## Recovery and history

The actor rereads its assessed input set before mutation. A prepared intent is
published under `.recur/lang/checked/ATTEMPT/prepared.json`, then E0 is linked to
Ef without replacing an existing file, E0 is removed, and accepted status is
published as `accepted.json`. Staging uses create-new files and sync. Only final
acceptance emits `action.ack: true`. Legacy statuses are separate and unchanged.

After interruption, repeat the exact command with `--recover --confirm`.
Recovery requires matching request fingerprints, unchanged evidence and exact
artifact bytes. Torn/conflicting staging or unexpected files are bounded blockers:
preserve them and inspect the named path before choosing a new bounded attempt.
Do not overwrite history or delete a receipt to force acceptance. Identical
accepted replay succeeds only with current evidence and unchanged Ef.

Query a recorded acceptance explicitly:

```powershell
.\target\debug\recur.exe lang evidence spec.recur --scope build.f `
  --contract policy.json --receipt attempt.json `
  --status .recur/lang/checked/attempt-1/accepted.json -d PROJECT --json
```

Historical files remain immutable when current inputs drift. Query status is
qualified by the selected request; a conflicting current policy/attempt cannot
borrow historical acceptance. Normal rejected evidence preserves E0. Unsafe
paths, collisions and incomplete transactions exit 2; evidence rejection exits 1
(malformed evidence exits 2). Dry queries allow absent/declared evidence at exit 0.

Exclusive operator ownership of chosen Eventness/transaction paths is required
during the short write. This is process-interruption recovery, not a guarantee
against hostile concurrent filesystem mutation or host power loss on every
filesystem. Unsupported hard links fail before removing E0.

Run `cargo test --locked --test lang_checked --test lang_checked_transition
--test lang_checked_reentry --bin recur-lang` for executable boundary examples.
Recover development acceptance with `recur warp show main.lang.checked-transition
-d . --json`; documentation capsules live under `warps`, while the live map and
contained evidence root are the repository root. Julia/browser gates remain in
the independent runtime-evidence Warp.

defines: recur.lang.checked.transition.v1 bounded current evidence to confirmed state
consumes: main.lang.checked-transition.wire.v1 frozen wire contract
