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

If supported by the selected binary, use `recur trace-id ID -d ROOT` to locate
matching folders and files, then apply `--scope` for detail. `--scope '**'`
intentionally requests details across the root. Inspect coverage before treating
an empty result as absence. `--declared` with scope reads exact opt-in Markdown
records and revision pins; a passing digest does not establish the expected
revision, runtime behavior or gate acceptance.

For an explicit ID set, use a scoped `--list-ids` view. Select exact IDs or
families with repeatable `--select`; selectors form a union within the primary
pattern. Use scoped `--context N` to read matching lines and neighboring
explanations with source spans; overlapping passages merge. These are observed
tokens and extracted text, not inferred authority or execution. Inspect both
scan and output coverage before concluding absence. Bound returned items with
`--max-results` and `--max-output-bytes`; excerpts can omit relevant context,
so read the original section or file when the decision requires more. Neither
view requires Watch, Lang, a Warp map, or project configuration.

For repeatable project defaults, use `recur-trace-id init -d ROOT --dry-run --json`
to preview `[recur-trace-id]` settings. Apply with `init` when requested.
`recur-trace-id inventory` and `context` use those settings to choose scope,
formats, and output limits for the core query. Shared role vocabulary and
content read policy remain under `[traits.*]`. Check the chosen binary’s help.

For legacy scoped queries, inspect `recur trait -d ROOT get trace_id.producer_keywords`
and the corresponding
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
