# Reveal agent/persona/skill Warp verification

Observed 2026-09-10 on `recur-lang`, based on `4e99976` plus the local
implementation recorded in `main.command.reveal.persona-skills.evidence/inputs.json`.
No installation, publication or host activation was performed.

## Baseline and scope

The original standalone suite covered only persona defaults and an absent
companion. The relevance audit broadened it to independent agent/persona/skill
discovery: 34 assertions passed, with 1 passing init assertion and 4 expected
failures for absent agent/skill/persona defaults and the companion executable.
Legacy reveal (31), artifact types (123), and init (33) passed separately.
These are historical observations in the test audit, not a claim that every
final negative test was written before implementation. The detailed association
contract was frozen before the resolver implementation; focused Rust and CLI
acceptance coverage was extended during implementation and review.

The frozen v2 contract covers exact typed identities, explicit capsule bindings,
config-ID references, independent artifacts, source provenance, pure discovery,
bounded bodies and a separate writer/packet companion. Initial metadata scanning
is separate from packet body budgets. Neither ready packets nor filename states
establish acceptance of downstream work.

## Final observed checks

| Check | Result |
| --- | --- |
| `cargo +1.85.0 test --locked -j 2` on Windows | 211 passed; 7 pre-existing ignored doc tests |
| Linux `cargo +1.85.0 test --locked --test reveal_profiles -j 2` | 11 passed, including body/config symlink escapes |
| Focused Julia Reveal suite against final release-safe binaries | 73 assertions passed |
| Full Julia runner against final release-safe binaries | 3,833 passed, 73 existing known-broken; no failures/errors |
| `cargo +1.85.0 clippy --locked --all-targets -j 2` | Exit 0; existing warnings, none in new profile/companion modules |
| Repository recur-expert skill validator | Passed; validator-only PyYAML dependency isolated under target |
| Actual Windows release-safe archive | All seven executables, help/version, Lang regressions, Reveal discovery and inert next packet passed |
| Actual Linux release archive | Same extracted-archive checks passed |
| Packed Chocolatey install/uninstall scripts | Mocked helpers verified checksum/version and all six companion shims; no installation |

Julia used 1.12.7 with `--startup-file=no --compiled-modules=no --compile=min -O0`
and `--project=demos/web-evidence-lab`. `RECUR_BIN` selected the final
`target/release-safe/recur.exe` and `RECUR_PROFILE=release-safe`. Full-run duration
was 4m14.5s. The new suite is included in the normal runner; no old assertion was
removed or changed to broken. The earlier debug Julia run was interrupted during
slow trace-analysis cases after later fixes made it obsolete. Its log/fixture
were preserved locally; it does not establish acceptance. One concurrent debug
rebuild hit a Windows executable lock; subsequent sequenced builds passed.

## Gate review

| Gate | Evidence |
| --- | --- |
| baseline-contract | Frozen v2 contract, dated red/baseline audit, retained legacy tests |
| editable-artifact-association-defaults | Fresh init tables; strict record types; existing custom/empty/inline config preserved; preview and idempotency; partial setup recovery |
| bounded-artifact-association-resolution | Exact IDs and explicit capsule types; ambiguous/missing/wrong-type/conflicting bindings; shared-skill deduplication retaining incoming edges; invalid/empty body handling; path and Linux symlink checks |
| opinionated-reveal-companion | CLI init/next tests; deterministic JSON and fingerprints; zero/limited body budgets block visibly; unchanged project inventories and no instruction execution |
| query-integration | Shared scanner/classifier moved into library and reused; existing selection tests preserved; agent/persona/skill query fixtures and configured body pointers; legacy full suite |
| regression-closeout | Full Rust/Julia, focused Linux boundary tests, both native archives, nupkg script checks, updated guidance |

Static review: core association inspection reads configuration and capsule
metadata only. Body collection exists in explicit companion packet preparation.
The companion exposes only init and next; no agent activation, network resolver,
shell-evaluation path, scheduler or install operation is added.

## Artifact identities

Windows zip SHA256: `5b46df0f7bf60e8adef3854c91c90a142085775aa315c2c7473319e400c7d9bf`.
Linux tar.gz SHA256: `0f3a1695dcfd59c5bcb69e671bff3acba4495a4fb4cc6aacfd6298a5f838c96f`.
Chocolatey nupkg SHA256: `56bef2d08c3147ae92898f9d49330ec50853995851cc7ad35c9b1e65be76c0e9`.
Archives are under `target/reveal-packages` and `target/reveal-linux-packages`;
nupkg is under `target/reveal-choco`. The download package is bound to the exact
Windows archive. A rebuilt zip needs a newly bound nupkg. These are local
candidates; release URLs are not populated by this work.

Durable logs and extracted-package reports are in
`main.command.reveal.persona-skills.evidence/`. Warp layers cite this reviewed
evidence in declared mode; the Warp engine does not rerun the test producers.

defines: recur.reveal.persona-skills.verification observed v2 acceptance evidence
consumes: recur.reveal.persona-skills independent artifacts explicit associations bounded context
