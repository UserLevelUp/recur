# Lang dogfooding: from Hello World to an inspectable capability

This Warp grows the existing Julia Hello World evidence lab with compact Lang
specifications, behavior fixtures, and a read-only view of selected contracts.
The useful result is a developer opening a demo, choosing a capability, and
seeing its input/output boundaries and evidence limits alongside its behavior.

Lang is a high-level specification that a human or LLM interprets. Use it where
precision helps: greeting behavior, selection boundaries and evidence display.
The surrounding application and architecture remain flexible. This is a worked
example, not an evaluation of model intelligence or automatic test generation.

## Progression

| Slice | Deliverable | Acceptance focus |
| --- | --- | --- |
| slice-0 | Existing baseline and initial greeting specification | Current tests observed; supported Lang coverage understood |
| slice-1 | Greeting fixture table and behavioral tests | Unicode, limits, locale fallback and transport preserved |
| slice-2 | Inspector request contract and fixtures before implementation | Bounded selection, failure mapping, original evidence retained |
| slice-3 | Julia server adapter and small inspector page | Real queries and fixtures agree; earlier demos still work |
| slice-final | Reproducible walkthrough and integration evidence | Source-bound results, honest limits, useful Lang gap assessment |

Read `main.lang.dogfood.contract.md` for the gates. Paths there are relative to
the repository root. The existing ten examples are context, not ten mandatory
rewrites. The existing `demos/lang-inspector` implementation is reused.

Baseline slice-0 is accepted with declared evidence: 33 inspector assertions,
33 server assertions, and the greeting specification's bounded static check.
See `main.lang.dogfood.slice-0.verification.md`. Slice-1 is current; the new
inspector endpoint and page are not implemented yet.

```powershell
& target/release-safe/recur.exe warp show main.lang.dogfood -d warps --json
& target/release-safe/recur.exe warp slices main.lang.dogfood -d warps --json
& target/release-safe/recur.exe lang show main.greeting.recur --scope greeting.g -d demos/web-evidence-lab
```

Release target: a.0.2.8. Creating this Warp does not publish the release.

defines: main.lang.dogfood specification-first demo progression
consumes: demo.lang.inspector reusable query and presentation boundary
consumes: main.greeting.api.response existing greeting behavior
produces: main.lang.dogfood.acceptance observed demo integration evidence
