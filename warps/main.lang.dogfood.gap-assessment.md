# Lang dogfood gap assessment — 2026-09-10

Gate: `lang-gap-assessment`. Scope: greeting and catalog inspector WIR1 demo.

No new CLI query or grammar was needed. Existing reports already provided exact
aliases, source/hash, flows, boundary edges, findings, recorded state and excluded
coverage. The adapter could retain the complete packet and reuse the inspector's
models and text renderer. WIR1 does not validate the prose about character limits,
HTTP statuses or HTML text safety; independent examples tested those claims.

Presentation needs differed from query needs. Serializing declared events with
source spans made the footer long; compact `consume`/`produce`/`state` and
`E0 → dE → Ef` lines solved it, with original spans retained in expandable JSON.
The local/canonical identities were already sufficient to preserve aliases.
The new browser has explicit empty/loading/error states; none requires grammar.

Authoring decisions: the API protocol lives in a linked Markdown contract while
Lang stays small. Header bundles are declared once, body lines compose function
symbols, footer lines describe events/state. One-function greeting remains a
small function; adding request/domain/view structs would add indirection without
a demonstrated invariant. Tests are hand-authored interpretations with explicit
requirements, including character-versus-byte-versus-grapheme behavior.

Evidence gap: no runtime-receipt loader was requested for the API. It returns an
empty observed-evidence list and states that limitation. A future evidence viewer
would need a separate contract binding source fingerprints and structured test
results to selected capabilities. A `.complete` path or successful static check
cannot fill that gap. This is a potential demo feature, not missing Lang syntax.

The server catalog intentionally allows only specified WIR1 scopes. CIR1 support
would require a separate presentation contract for coordinator/lane/message and
SGR1 graph data; this demo rejects CIR1 instead of pretending its WIR1 cards apply.
The tests retain synthetic findings to verify display, not to demonstrate an
actual WIR1 cycle analysis. Existing SGR1 coverage is separate and does not prove
absence of every architectural circular dependency.

Runtime friction was in Windows Julia compilation/precompilation, not Lang:
several configurations crashed; the focused integration passed with generic CPU,
compiled modules enabled and compile=min. The first complete runner reached all
tests but failed an existing empty-stderr pipeline assertion on precompile output.
See the final verification for the warmed-cache rerun result.

No grammar or companion change is proposed without a demonstrated missing
input/output behavior. The useful next work, if desired, is runtime-evidence
presentation under its own contract, rather than expanding this completed scope.

produces: main.lang.dogfood.gap-assessment scoped authoring and presentation findings
