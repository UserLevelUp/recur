---
name: recur-trace-id
description: Design and inspect hierarchical Recur trace IDs across contracts, code, tests and evidence using configured declaration roles. Use for Recur lineage; occurrences do not prove execution.
---

# Recur Trace ID

Resolve the project root and its conventions. Start from the subject's capsule,
map or existing IDs. Use stable semantic hierarchies such as
`demo.blackjack.rivals.table.settlement` for a demo behavior, and
`main.recur.skill.warp` for reusable guidance. Keep runtime attempt IDs,
artifact types, filename hierarchy and trace lineage distinct. A trace ID is
not a place to mutate host intelligence policy or scheduler state.

Inspect `recur trait -d ROOT get trace_id.producer_keywords` and the corresponding
`consumer_keywords`/`trigger_keywords` keys
before choosing annotation vocabulary. Classification is contextual keyword
matching, not parsing or semantic proof. In the Recur repository, read
`docs/main.command.trace-id.readme.md`, checking older design statements against
live help and `docs/main.command.lang.query.readme.md` when Lang is involved.

Current local keywords recognize `defines`, `publish`, `consumer`, and `trigger`:

```text
defines: demo.blackjack.rivals.table.settlement independent seat settlement
publish: demo.blackjack.rivals.table.settlement accepted settlement output
consumer: demo.blackjack.rivals.table.settlement HTTP projection
trigger: demo.blackjack.rivals.table.settlement behavioral verification
```

Verify the reported roles after edits; spelling a role near an ID does not
guarantee its classification under a project's configured keywords. UUID
metadata and incidental mentions are not automatically producers or consumers.

```powershell
recur trace-id 'demo.blackjack.rivals.**' --scope 'main.blackjack.rivals.**' -d demos/blackjack-web --json
```

Adapt both ID and file scope to the actual subject. Bound root, hierarchy and
extensions before widening the scan. Report omitted context and remember that
occurrence counts are not unique-ID counts. Missing relationships may be outside
the chosen scope. Saved query runs and freshness commands should be checked
against current help; fresh inputs do not prove runtime correctness.

Use lineage to locate contracts, publishers, consumers, tests and observations,
then read those artifacts. Core trace queries cannot decide Lang compatibility,
deadlock freedom, test sufficiency or gate acceptance. Typed Reveal capsules
use `artifact.type = skill` independently of their trace namespace.
