# Warp command roadmap — 2026-09-06

Current: read-only list/show/slices/map/merge/status/explain/next/config/evidence/fingerprint/
collapse-plan; companion create/receipt/complete/evolve/collapse. Shared suffix policy
foundation is implemented locally. Existing behavior/tests remain the compatibility
baseline. `merge` projects layers; it is not arbitrary project union.

Completed locally: **main.command.warp.usability** — simple show/slices queries
and configuration-driven create. Creation initially templates a single canonical
JSON map, with goal and optional invariant/Slice-description metadata; multi-file document
scaffolds are deferred. This avoids inventing a competing task graph or silently
accepting placeholder work. Canonical map/layer filenames remain stable.

Remaining foundational work, not implemented by that bounded Warp:

Next declared Warp: [identity and policy initialization](main.command.warp.identity-policy.readme.md).
Its standalone tests are intentionally red. It covers init, automatic UUIDv7 bubble/
slice identities, optional preservation configuration and trait-style human list
presentation, not deletion enforcement. List JSON and query semantics stay stable.

- Semantic repartition: many source bubbles to many destinations, stable identity,
  hierarchical tags, explicit contract/dependency revisions and lineage. Freeze
  small pool/core separation fixtures before 1000-to-500 scale cases. Account for
  every retained/moved/superseded/retired task, no duplicated execution, no inherited
  invalid acceptance. Preview, conflict refusal, idempotency and interruption recovery.
- Lifecycle selection: explicit current/active Slice and paused/abandoned/superseded
  bubble declarations, kept separate from evidence-derived verified completion.
- Broader failure evolution: not only conflicting/stale-contract explosions;
  preserve failed predecessor and rejected assumptions, validate carried evidence.
- Preservation and cleanup: configurable requirements for an exact committed
  snapshot, durable reference and optional pushed reference before separately
  authorized deletion. Allow explicit Git-policy opt-out; never confuse configured
  policy with implemented enforcement. Hiding from active work must not require deletion.
- Opinionated transition policy: configured scopes, protected artifacts, evidence,
  escalation and retries without configuration granting itself authority.
- Multi-file templates and recoverable batch transactions, after the initial
  single-map creation contract proves useful.

Reuse Improvement 26 artifact preservation and recur-git checkpoints/receipts;
inspect Improvement 30 `recur-lang warp` E0/dE/Ef and identity/graph contracts before
adding task representation. Core `recur lang` and Improvement 29 `recur-reveal next`
are not available merely because their proposal text contains commands.
Flatten/merge/unflatten are data transforms, not semantic acceptance validators.

Bubble lifecycle may return to resting while artifacts have changed. Every
transition must separate intended changes, invariants, starting state and arrival
evidence. Preserve original goals only when still valid; explicit redesign must
be possible without retaining the original mistake.

defines: recur.warp.roadmap semantic restructuring and accessible commands with evidence and authority boundaries
