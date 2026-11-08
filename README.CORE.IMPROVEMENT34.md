# Recur Improvement 34: Trace Relationships, Revision Checks and Workflow Clarity

Date: 2026-11-07
Updated: 2026-11-08 (corrections, evidence scope, review and skill guidance)
Status: Proposed / implementation Warps planned; no implementation acceptance

defines: main.improvement.34
consumes: main.command.trace-id main.command.warp

## Intent

Improve how Recur connects semantic identities, artifact revisions, Eventness and
acceptance evidence. The immediate examples come from Robo Alpha: trace queries
misclassified incidental producer mentions, an interface register described an
older circuit, and recorded completion survived later findings of engineering
defects. The workflow also required avoidable repeated commands and displayed a
stale current-slice warning after completion.

This README records requirements and proposed interfaces. The
[initial Warp plan](warps/improvement34/main.improvement.34.implementation.initial.current.md)
routes them into six implementation bubbles and a coordinator. Their maps and
gates are declared; they do not establish accepted work or freeze command/schema
choices. Improvement 33's rules engine remains a separate proposal.

## main.improvement.34.observed_value

defines: main.improvement.34.observed_value

The Robo work demonstrated useful, distinct responsibilities:

| Surface | Contribution | Limit |
|---|---|---|
| Filename hierarchy | Locates bounded areas of work | Does not describe all semantic relationships |
| Eventness | Preserves attention, unresolved findings and restart notes | A current/complete/next declaration can become outdated |
| Trace IDs | Connect requirements, source, circuits, tests and evidence across files | Keyword matches do not establish execution or ownership |
| Warp gates | Record criteria, dependencies and acceptance | Declared acceptance does not establish engineering correctness |
| Project sequence guard | Checks predecessor completion, handoff binding and artifact hashes | Fresh files can still contain incorrect designs or inappropriate revisions |

Robo's guard checked 47 artifact hashes. The delivered Julia suite passed 111
assertions and the circuit passed ERC, yet subsequent review reproduced a startup
sequencing defect in the model and found an invalid logic-level assumption.
Hardware behavior was not measured. These observations
justify clearer evidence boundaries and stronger dependency checks; they do not
imply Recur should become an electrical simulator.

Source case: `C:/src/kicad_ra/docs/robo.interfaces.dock.predecessor.review.md` and
`C:/src/kicad_ra/evidence/robo.interfaces.dock.r06.review.json`. These are external
project references. Implementation tests must use small self-contained fixtures
in Recur and must not require access to the private Robo repository.

## main.improvement.34.declarations

defines: main.improvement.34.declarations

Priority 1: distinguish explicit declarations from incidental matches.

**Confirmed in the current CLI.** A local `trace-id` query for
`capability.baseline` against this five-line Markdown fixture reported lines
2, 3 and 5 as producers and line 4 as a consumer. The current `define` list
also included lines 2 and 5. These are useful search hits, but unsafe as an
authority list. In source, `build_record` populates `define` from search hits;
producer matching uses substring checks and consumer token boundaries do not
treat `.` as part of the full hierarchical identity.

```text
defines: capability.baseline
Notes discuss how to publish capability.baseline later.
publish: capability.baseline_generator
consumer: capability.baseline.extra
publish: capability.baseline
```

- Retain current heuristic discovery for arbitrary source and prose.
- Start the opt-in declaration mode with one explicitly delimited metadata block
  in Markdown. Specify allowed role keys, identifier grammar and case behavior,
  comment/code-block handling, and malformed-record diagnostics. Treat other
  formats as separate adapters; a JSON field counts only under a recognized
  schema, and source code that prints the field remains a reference.
- Report declaration, reference and heuristic match provenance separately, with
  exact identity, role, path and source span. Explain how a role was obtained.
  Only structured declarations should contribute to authority checks or the
  declared graph. Keep the legacy query and its JSON shape compatible.
- Keep exact identity matching separate from intentional prefix/glob queries.
  `capability.baseline_generator` must not silently become a declaration of
  `capability.baseline`. Test dotted descendants too. Reject unsupported ID
  characters with a span and reason rather than silently shortening the ID;
  the [Watch lineage check](warps/watch-evidence/observations/lineage-v1/failed.json)
  already recorded a hyphenated segment being shortened.
- A generator's source literal used to emit a JSON `produces` key is not by
  itself a second authority for the generated artifact. Model the generator,
  artifact and their relationship explicitly without obscuring ordinary code.
- Define duplicate-authority checks per identity, revision and project policy.
  Multiple implementations or occurrences of a capability are not universally
  prohibited, and a define authority is distinct from a runtime producer.

Acceptance examples: an explanatory sentence, an emitted dictionary key and a
longer identifier remain discoverable without becoming false declarations;
two genuinely competing declarations are reported with both source locations.
In a strict fixture with the recognized metadata delimiters, only the exact
`defines` and `publish` entries attach to `capability.baseline`; the longer IDs
remain distinct. An explicit prefix query may intentionally include them.
Existing heuristic queries retain documented behavior unless migration is explicit.

## main.improvement.34.discovery

defines: main.improvement.34.discovery

Make `--scope` optional and use it to switch from finding locations to inspecting
relationships. The trace pattern still selects what to look for; `-d ROOT` still
bounds where to look and defaults to the working directory.

| Proposed query | Result |
| --- | --- |
| `recur trace-id capability.baseline -d ROOT` | Find matching files under the root and list their folders and paths once, in stable order. Do not print matching lines, infer roles, or expand relationships. |
| `recur trace-id capability.baseline --scope 'lab.**' -d ROOT` | Limit the file search to that hierarchy and return detailed trace results. Structured declarations are a separate opt-in. |
| `recur trace-id capability.baseline --scope '**' -d ROOT` | Explicitly request detailed trace results across the root, subject to normal limits. |

The current CLI requires a scope. Keep existing scoped invocations and output
compatible. Give the new path-only JSON result a distinct shape so callers
cannot mistake it for a relationship report. Reuse normal root, exclusion,
extension and identifier-pattern handling, and bound discovery itself. If the
search is cut short, report incomplete coverage rather than treating an empty
list as proof that the ID is absent. Decide how detail-only flags behave without
scope before implementation; they must not silently upgrade the query to a
full relationship scan.

## main.improvement.34.revision_dependencies

defines: main.improvement.34.revision_dependencies
consumes: main.improvement.34.declarations

Priority 2: detect inappropriate revision dependencies from explicit information.

Specify artifact families, revision identities, supersession relationships and
consumer expectations. Possible dependency intents include current implementation,
pinned revision and historical comparison; these are proposed semantics, not
existing schema fields. Reuse the artifact-version facilities where appropriate
rather than introducing a competing history store.

Start with an explicit pin: an artifact family, actual revision, expected
revision and source reference. `recur version` can supply existing subject,
lifecycle and snapshot identities; its current manifest entries record versions
and summaries, not consumer constraints or a supersession graph. Keep the byte
digest separate from the revision label. If two different contents claim the same
family/revision in the same resolution context, report a conflict.

A `current` dependency needs an explicit resolution context, such as a project,
variant and selected release lane. Add it after the pin case works. A split in
lineage is not a conflict by itself; it becomes ambiguous when a consumer asks
for one applicable revision and the selected context has multiple candidates.
Do not use lexicographic order, modification time or filename suffix to break
that tie. Include the resolved scope and selection basis in the report.

A newer filename alone does not obsolete every older reference. Preserve
intentional historical tests, source ancestry and reproducible pinned inputs.
Report unknown or ambiguous relationships when the selected scope is incomplete;
do not silently choose a revision or rewrite a consumer.

Keep three questions separate:

1. Do the recorded bytes/fingerprints match?
2. Does the consumed revision satisfy the declared dependency expectation?
3. Do its verification and review evidence satisfy the relevant contract?

Acceptance examples: a register requiring circuit R06 but bound to R04 is flagged
even when every hash matches; a test explicitly comparing historical R04 and R06
remains valid; missing input or metadata yields `unknown`, not a passing check.
Detect conflicting applicable successors and supersession cycles. Diagnostics
identify the consumer, actual input, expected relationship and the evidence for
the finding. A passing revision check never substitutes for verification coverage.

## main.improvement.34.relationship_graph

defines: main.improvement.34.relationship_graph
consumes: main.improvement.34.declarations main.improvement.34.revision_dependencies

Priority 3: expose a bounded semantic relationship graph independently of Warps.

- Show artifacts, semantic IDs and revisions as distinguishable entities, with
  labelled declaration, consumption, production, generation and supersession
  relationships where supported. Do not invent edges from co-occurrence alone.
- Support a readable CLI projection and stable machine-readable graph output.
  Exact command names and rendering formats remain undecided.
- Show source provenance, unresolved targets, scope boundaries, cycles and
  truncation. A graph is not necessarily a tree; repeated/shared nodes must not
  be presented as independent copies or mistaken for cycles.
- Allow traversal toward inputs or dependents, with explicit depth and size limits.
  Set separate discovery and traversal budgets; depth alone does not bound the
  scan or a wide graph. Report omissions and incomplete coverage. Emit each node
  once and distinguish a shared dependency from a cycle in the traversed path.
- Keep filename scope independent from semantic namespaces. A `robo.*` file can
  consume an independently rooted capability; matching prefixes are not required.

Acceptance examples: one capability links a Markdown requirement, generator,
artifact and Julia verification across different file prefixes; a consumer is
discoverable without a Warp map; cycles terminate with clear diagnostics; JSON
and CLI views describe the same declared relationships.

This supports the user's category-oriented organization of capabilities and
relationships. It does not claim to implement a formal category-theory calculus.

## main.improvement.34.workflow

defines: main.improvement.34.workflow

Priority 4: reduce ceremony using existing capabilities before adding commands.

Current source/help already establish:

- `recur trace-id` searches scoped repository files outside Warp bubbles.
  `recur trace` is the separate call-graph surface. A required search scope bounds
  discovery; it does not confine trace IDs to Warps.
- `recur-warp complete` does not require a preceding lifecycle `receipt` call.
  It checks supplied gates and writes an acceptance layer. A separate lifecycle
  declaration may be useful under local policy, but is not automatically needed.
- `recur-git checkpoint` supports `--snapshot` and `--append-parallel` together.
  It prints a snapshot and appends a parallel entry from the same captured state.
  The log is append-only by convention: repeated checkpoint IDs are not deduplicated.
  Test flags run after that state is captured, so this is not a fresh test receipt.

Update examples and focused skill guidance to show the shortest supported workflow
while retaining evidence review and the applicable preview/confirmation boundary.
If a combined receipt/completion operation is still justified, first specify its
distinct value, idempotency, failure recovery and preservation of both record types.
Do not add an alias that merely hides duplicate work or weakens acceptance checks.

Acceptance examples: a standard slice can be accepted without a redundant lifecycle
declaration where policy permits; one checkpoint invocation displays and records
the same captured state. Keep retry guarantees separate: `complete` recognizes an
existing identical attempt and rejects incompatible content at that identity;
checkpoint append does not offer that guarantee. Define any new recovery behavior
before promising exactly-once writes.

## main.improvement.34.attention

defines: main.improvement.34.attention

Priority 5: make current attention and evidence limitations clearer.

Core currently warns when a recorded `current_slice` is no longer ready and does
not infer a replacement. Preserve that read-only boundary. Show recorded attention
and computed ready choices separately; offer a companion-owned update after
completion. Do not silently choose among multiple ready slices or suppress real
conflicts. A completed Warp should have an understandable settled display.

Keep the current JSON field's meaning (a ready recorded selection or null).
Propose an additive recorded value and reason such as `settled`, `completed`,
`unknown_slice` or `not_ready`. Show a settled note for a known completed slice
when the Warp is complete, while keeping diagnostics for unknown IDs, blockers
and conflicts. A later companion update should be explicit and check that the
map and readiness still match the preview before changing the selection.

Distinguish recorded gate completion, evidence freshness, verification coverage and
open review findings. Decide how explicit later findings can challenge reliance
on an accepted result while preserving the original receipt. Eventness prose alone
must not automatically revoke acceptance or resolve a finding.

Start by displaying explicit findings next to historical acceptance. A finding
should name the challenged layer or receipt, artifact revision, affected criterion,
counterevidence and resolution status. Show its scope so a finding against R06
does not automatically carry over to a distinct successor. A reproducible failure
can challenge current reliance even if the accepted inputs still match their
hashes. Leave the original receipt intact and require a separate reviewed record
to resolve the finding. A future opt-in gate policy may act on unresolved findings;
define that policy before changing downstream readiness.

Acceptance examples: completing the selected slice gives actionable ready choices;
parallel ready slices remain explicit; a later review finding remains visible
alongside historical acceptance; a stale `.next` note cannot override live progress.

## main.improvement.34.eventness.expansion

defines: main.improvement.34.eventness.expansion
consumes: main.improvement.34.revision_dependencies main.improvement.34.attention

Keep progressive expansion: detail the next bounded task when its predecessor's
actual outputs are available. A deferred outline still needs a small grounded
baseline: existing artifacts/components, their revisions and trace identities,
what is already implemented, known constraints, and decisions deliberately left
open. Deferred detail must not imply a blank starting point. In the Robo example,
an outline should identify the existing dock sheet before scheduling dock design.

On expansion, compare that baseline with the accepted predecessor handoff and
record what changed, what can be reused and which assumptions need reassessment.
The finishing worker can draft this update while context is available. Recording
that draft, reviewing its sufficiency and accepting its new criteria are separate
steps; predecessor completion alone does not establish the successor plan's quality.

Propose an opt-in independent-review gate for material successor expansions,
especially when changes affect protection behavior or invalidate inherited
constraints. Project/companion policy selects when it applies. Record the reviewer,
exact input revisions/hashes, checks performed, findings and their disposition.
Self-review must be labelled as such; changing a model name is not proof of an
independent check. Review should inspect source evidence and test assumptions,
rather than merely endorse the finishing worker's summary. No model is dispatched
or human approval requested automatically by a core query.

Acceptance examples: an existing subsystem survives a sparse outline; a changed
rail-supervision interface is reflected when the outline expands; a reproducible
counterexample prevents acceptance under the opted-in review policy despite a
complete predecessor and matching hashes. Low-impact legacy workflows retain
their existing policy unless the project explicitly adopts new criteria.

## main.improvement.34.eventness.collapse

defines: main.improvement.34.eventness.collapse
consumes: main.improvement.34.revision_dependencies main.improvement.34.attention

Keep collapse as selective reduction of active attention. A concise handoff is a
useful entry point into evidence, not a replacement for that evidence or a claim
that every outstanding issue is resolved. Preserve semantic trace IDs, selected
artifact revisions/hashes, decisions and rationale, corrections/supersession links,
unresolved findings with owners and effects, and the next useful action.

Distinguish summarizing a settled subject from the existing companion's archival
operation. Writing a handoff does not itself run `recur-warp collapse`, move files
or certify gates. Keep existing conservative archive checks and confirmation
behavior. Do not hide current blockers behind a completed filename.

The collapsed record should link to exact recoverable source and evidence versions.
References into private `.recur/` storage must state their availability; a handoff
for another checkout needs portable evidence or an explicit retrieval route.
On a later relevant change or finding, expand only the affected subject and its
known dependents, retaining the previous handoff and receipt as historical facts.
Do not rewrite history or automatically reopen unrelated work.

Acceptance examples: a fresh checkout follows a compact handoff to its exact
inputs, all corrections and an unresolved finding; missing archived evidence is
reported explicitly; re-expansion preserves stable identities and historical
receipts. The test concerns durable decisions and evidence, not reconstruction
of every discarded conversation or working note.

## main.improvement.34.contract_revision

defines: main.improvement.34.contract_revision
consumes: main.improvement.34.eventness.expansion

Specify a written decision rule for contract identities before adding automation:

| Change | Contract treatment |
|---|---|
| Editorial clarification that changes no obligation, scope, assumption or pass/fail interpretation | Keep the contract identity; record the clarification and handle changed file fingerprints explicitly. |
| New implementation or evidence under unchanged acceptance criteria | Keep the contract identity and publish a new attempt/evidence record. Use existing refresh only where its same-contract freshness rules apply. A failed result cannot be repaired by refreshing its label. |
| Changed required behavior, operating assumptions/limits, dependencies, deliverables or acceptance criteria | Issue a new contract revision and reassess affected slices/consumers. Prior receipts remain historical and do not automatically satisfy the new criteria. |
| A materially different responsibility or split/merged slice | Review identity as well as revision; preserve explicit lineage rather than silently reusing a slice identity. |

A changed dependency matters when it changes what the slice must rely on or prove.
Do not let cosmetic renaming create needless revisions or let a material semantic
change hide behind a reused version. Record the affected criteria and rationale;
where the distinction is uncertain, require a documented review decision under
the project's policy. Keep unchanged slice UUIDs stable where identity continues.

## main.improvement.34.correction_records

defines: main.improvement.34.correction_records
consumes: main.improvement.34.attention main.improvement.34.contract_revision main.improvement.34.revision_dependencies

Standardize the correction links that Robo had to author repeatedly. A proposed
correction record should identify the exact original artifact/layer/receipt and
digest, the affected claim or criterion, the error and counterevidence, the
replacement artifact/revision if available, and the scope of the correction.
Record author/reviewer provenance and distinguish an open finding, a proposed
correction and a correction supported by reviewed replacement evidence.

Keep the original record immutable. Show its correction links in queries and
handoffs, and follow explicit supersession edges without choosing the newest
filename as a winner. Preserve multiple corrections to separate claims; conflicting
replacements, missing targets and correction cycles require visible diagnostics.
A correction to one electrical value must not silently invalidate or validate
every other claim in the original slice.

Current behavior is narrower: `recur-warp complete` rejects a different accepted
result hash for the same Warp, slice and contract, even with a new attempt ID.
Same-contract evidence refresh addresses eligible freshness renewal; it cannot
make a failed result pass. Evolution handles its supported conflict/stale-contract
cases. These mechanisms do not constitute a general correction-record lifecycle.

Start with opt-in sidecar records and a read-only correction view, preserving the
existing layer format and conflict checks. Specify separate assessment and
publication rules before a correction can change effective acceptance. Merely
recording a correction or creating a new attempt must not bypass existing gates.
Apply the contract-revision rule when the obligations themselves change.

Acceptance examples: one immutable accepted layer links to a later correction of
one claim and its supporting evidence; an unresolved correction remains visible;
a different result hash still conflicts through the legacy completion path;
repeated publication is idempotent only under a specified identity/content rule.
No correction command is claimed to exist by this proposal.

## main.improvement.34.evidence_scope

defines: main.improvement.34.evidence_scope
consumes: main.improvement.34.attention main.improvement.34.correction_records

Qualify completion without redefining the existing `complete` state. It means
the selected contract's gates are recorded as satisfied under its evidence mode.
It does not establish every possible claim about the resulting product.

Propose an optional, domain-configurable evidence summary that keeps these axes
separate: contract acceptance; declared versus checked evidence; integrity and
freshness; methods and verification coverage; independent review; and physical
validation where applicable. Describe the tested revision, configuration and
operating envelope. Report unknown, unperformed, failed, stale and not-applicable
states distinctly. Missing metadata is unknown, not validated or not applicable.

For Robo, a useful summary would state that the design contract was accepted,
which Julia/ ERC observations exist, which review findings remain open, and that
the named hardware tests are unperformed. Software-only projects must not acquire
a compulsory hardware-validation field or gate. Even successful hardware tests
support their measured conditions, not an unrestricted claim that a product is
"fully proven". No universal confidence score should merge these dimensions.

Reuse existing evidence manifests and results where they supply these facts. A
`checked` manifest does not mean Recur ran the producer or assessed test sufficiency.
Expose the summary through opt-in/versioned output while preserving legacy state
and JSON contracts. Manufacturing or other high-consequence release policy can
explicitly require particular dimensions; core must not invent that policy.

Acceptance examples: a design-only contract can remain complete while bench
validation is shown as unperformed; an existing physical-test requirement cannot
be waived by labelling the work design-only; failed, stale and unknown evidence
remain visible beside historical acceptance.

## main.improvement.34.gate_review

defines: main.improvement.34.gate_review
consumes: main.improvement.34.eventness.expansion main.improvement.34.evidence_scope

Extend the proposed independent-expansion review to selected high-consequence
acceptance gates through explicit project policy. The author may prepare evidence
and a review request, while a separately evidenced review examines source inputs,
assumptions, counterexamples and verification coverage before new acceptance.
Record who authored and reviewed, what each checked, and any unresolved findings.

This is not a global prohibition on self-acceptance or an automatic model/human
approval flow. Existing projects retain their policy. New contracts can require
independence where justified; an author's test run alone does not satisfy that
requirement, and a second model's agreement alone does not prove correctness.
Keep slice ordering tied to actual dependencies. Serial gates help organize work
but cannot prevent acceptance of a mistaken architecture; independent slices can
remain parallel under their existing dependency rules.

Acceptance examples: an opted-in review-required gate rejects an unreviewed
submission while preserving its evidence; review finding a reproducible defect
keeps that new gate unsatisfied; a legacy gate without the policy behaves as before.

## main.improvement.34.skill_guidance

defines: main.improvement.34.skill_guidance
consumes: main.improvement.34.workflow main.improvement.34.correction_records main.improvement.34.evidence_scope

Improve the mental model before adding more ceremony. Explain these distinctions
near the beginning of the relevant skills, with one small running example:

| Concept | Useful explanation |
|---|---|
| Filename hierarchy | A discoverable address and attention convention; it is not the only identity of an artifact. Stable UUID and semantic identities can survive a filename change. |
| Eventness capsule | Bounded current context, decisions, evidence pointers and next action. It may retain relevant history; it is not restricted to a history-free status note. |
| Trace ID | A semantic identity for a capability, requirement, artifact or event. Declared relationships between identities/artifacts form edges; the ID itself is not an edge or folder. |
| Warp bubble | A bounded objective with a contract, dependency-ordered slices and evidence gates. Eventness helps expose its lifecycle; a current capsule alone does not define or accept it. |
| Acceptance | A record against a particular contract and evidence basis, separate from broader validation and later review findings. File existence alone is insufficient for stronger evidence modes. |

Offer expert -> Eventness -> Warp -> trace-ID as one introductory learning path,
not a mandatory loading sequence for every task. Focused users may enter at the
relevant skill. Discovery presents pointers; it does not load a skill's body or
grant permission. Preserve the existing focused skills and keep their openings
short, linking to deeper references rather than copying the whole manual.

Provide one runnable, self-contained example: grounded outline, predecessor
evidence, reviewed expansion where opted in, implementation checks, completion,
compact handoff and a later correction. Use current supported commands for the
executable portion. Label future correction/review records as proposed fixtures
until implemented. A separate lifecycle receipt is optional where policy permits;
do not teach receipt-plus-completion as an unconditional requirement.

Add a concise common-rejections reference derived from current executable help,
source and tests. Capture exact messages with executable/version context; keep
stable reason codes separate from wording if new diagnostics introduce them.

| Current issue or message | Guidance to teach |
|---|---|
| `... must be a canonical UUIDv7: ...` | `bubble_uuid` and `slice_uuid`, when supplied, must satisfy the map's UUID rules. Readable `warp_id`/`slice_id` are different fields; `slice-0` is valid. Legacy maps without UUID fields remain readable. |
| `accepted Slice '...' already has conflicting result hash '...' in '...'` | Preserve the accepted record. Inspect whether this is a replay, correction or changed contract; changing the attempt ID does not authorize replacement. |
| `evidence gates do not match the Slice contract; missing [...], unexpected [...]` | Inspect the live contract and provide the required evidence keys; do not rename criteria merely to make publication pass. |
| `Declared current_slice '...' is no longer ready; selection not inferred.` | Read computed readiness and recorded attention separately. Do not restart a completed slice; use the supported companion/policy to update attention. |
| Receipt or completion previews without a persisted record | Inspect the preview, then use `--confirm` within existing authorization when publication is intended. Do not infer acceptance from the preview. |
| A scoped command rejects missing `--scope` | Check that command's help. Distinguish `trace` call paths from `trace-id` lineage; the proposed unscoped discovery mode is not yet available. |

Update guidance from reproduced friction and verified changes, not periodic
rewrites or every speculative complaint. Capture the example, root cause, safe
resolution and applicable regression coverage, then update the canonical skill
and linked reference. Preserve project-local customizations and existing skill
sync policy; do not blindly overwrite installed skills. Avoid unbounded growth
by retaining a short common guide and linking detailed incident records.

## main.improvement.34.compatibility

defines: main.improvement.34.compatibility
consumes: main.improvement.34.declarations main.improvement.34.eventness.expansion main.improvement.34.eventness.collapse main.improvement.34.contract_revision

Backward compatibility is a required design target, not a verified implementation
claim. Introduce stricter declarations, revision expectations, correction views,
validation summaries and review policies through explicit opt-in. Preserve existing scoped queries and their
documented output contracts, filenames, Eventness conventions, trace namespaces,
slice identities, receipts and archive behavior. New result shapes need distinct
schema identities or an explicitly compatible extension; do not silently reinterpret
legacy JSON fields or turn heuristic hits into authoritative edges.

Old artifacts without new metadata remain readable. New checks report missing
information as unknown; whether that prevents new acceptance is governed by the
selected policy and contract, not by retroactively rewriting an old receipt.
Adopting a new review requirement may intentionally tighten future acceptance,
while leaving historical acceptance visible under its original criteria.

Verify compatibility with archived legacy fixtures and current consumers: the
same existing commands must still work without migration, opt-in failures must
be scoped and explained, and read-only inspection must not modify inputs.
Do not advertise compatibility as achieved until these regression checks pass.

## main.improvement.34.boundaries

defines: main.improvement.34.boundaries

Core queries parse, inspect and report deterministically. Companion tools own
record publication, attention updates and confirmed lifecycle changes. No query
edits artifacts, dispatches a model, calls an external service or selects a new
engineering design. Missing context and unknown semantics remain visible.

These changes do not replace Julia, domain-specific verification, manufacturer
data, physical testing or independent review. Hash checks are integrity checks,
not proof of truth. Trace graph edges are declared relationships, not proof that
code executed. Recur Lang remains optional for formal coordination; this proposal
does not make every repository adopt its grammar.

## main.improvement.34.next

defines: main.improvement.34.next

Start with the [trace bubble](warps/improvement34/trace/main.improvement.34.implementation.trace.contract.md):
**prove one declared relationship and one revision expectation end to end**.
Use a small local fixture with a requirement,
two revisions, and two consumers: a current delivery pinned to R06 but bound to
R04, and an intentional R04 comparison. With all artifact hashes matching, the
first must show a revision mismatch and the second must satisfy its pin. Add
incidental prose, a generator literal, a duplicate authority and an out-of-scope
target. Require exact identities, actionable spans, explicit unknowns and unchanged
legacy queries. Start with a relationship report; defer general traversal until
its inputs and diagnostics are reliable.

Workflow guidance and the settled-attention display can ship independently of
that Warp. Reuse existing commands for the guide. Take review findings as a
separate lifecycle design after display-only evidence can be inspected. No rules
engine, Lang program, Watch subscription or domain tool is needed for the first
fixture.

Use the grounded-outline and compact-handoff guidance immediately as documented
practice. Specify independent expansion review and contract revision rules before
implementing their optional companion policies; they do not require replacing the
existing expand/collapse model.

Correction records, evidence-scope display and selected gate review are follow-on
work with their own lifecycle contracts. Clarify the skill mental models and
common rejections using verified current behavior first; documentation must not
present those future mechanisms as already implemented.

## Julia verification

The [standalone baseline](julia-tests/main.improvement.34.baseline.test.jl) runs
against today's binaries. It verifies scoped legacy discovery and the combined
snapshot/append workflow, and prints the incidental trace roles as observations
rather than making them required behavior. Run it from the repository root:

```powershell
julia --project=demos/web-evidence-lab julia-tests/main.improvement.34.baseline.test.jl
```

The project argument supplies JSON3; no demo suite runs. Reuse the existing
[completion suite](julia-tests/main.command.recur-warp.test.jl) for preview,
confirmation and retry checks, and the
[Warp usability suite](julia-tests/main.command.warp.usability.test.jl) for
ready-slice and completion projections. All use temporary or self-contained
fixtures. No Robo checkout is required.

Add these Julia CLI acceptance cases as each proposed contract is selected.
Exercise the real command with `mktempdir` and parse JSON with JSON3. Assert
exact identities, machine-readable reasons and source spans where applicable;
also check that queries leave inputs unchanged. Keep tests for unimplemented
behavior standalone and explicitly red until the corresponding slice works.

| Proposed case | Fixture and expected result |
| --- | --- |
| Discover paths, then inspect | Put the same ID in two folders and unrelated text in a third. An unscoped query lists only matching paths and their folders, deduplicated, with no role arrays or source lines. A scoped query reports detail only for the selected hierarchy; explicit `--scope '**'` reports detail for both. No mode searches beyond `-d`. |
| Exact declarations | In the trace fixture above, put declarations in the recognized metadata block. Query `capability.baseline` exactly: exclude prose, a generator literal, the underscore variant and the dotted descendant. An explicit family query includes descendants by design. |
| Malformed and competing declarations | Use an unsupported ID character and two authoritative records for the same identity/revision. Report the full malformed token and both competing spans; never truncate the ID into a valid declaration. |
| Matching bytes, wrong revision | Both R04 and R06 pass their digest checks. A consumer requiring R06 but bound to R04 fails its revision check; the intentional R04 pin passes. Changing R04 bytes then fails integrity independently. |
| Incomplete or ambiguous resolution | Remove the target or cut it out of the selected scope: report `unknown`. Select a context with two applicable revisions: report ambiguity. A narrower explicit context can resolve one. |
| Shared graph and cycle | Use a diamond and a separate directed cycle. The shared node appears once without a cycle finding; the real cycle reports its path. Exhaust a traversal or discovery budget and require an explicit omission. |
| Settled attention | Show a completed Warp with a recorded completed selection: ready choices are empty and the UI explains that it is settled. An unknown selection still warns; parallel ready slices remain choices. Querying never edits the map. |
| Later finding | Add a counterexample scoped to an accepted R06 layer. Show historical acceptance, current evidence freshness and the open finding together; leave the layer unchanged and do not apply the finding to R07. |
| Grounded expansion and review | Start with an outline naming an existing subsystem, then change one interface in its predecessor. Expansion preserves the existing subsystem, records the interface delta and, under opted-in review policy, requires a separately evidenced review before acceptance. |
| Collapse and re-expansion | Collapse a subject with a correction and an unresolved finding into a compact handoff. From a fresh fixture root, recover exact referenced inputs and both records. Re-expand the affected subject without changing prior receipts; report unavailable archives explicitly. |
| Contract revision decision | Compare an editorial change, a new result under identical criteria and a changed dependency/operating limit. Keep the first two contract identities where semantics are unchanged, publish new evidence as needed, and require a new revision for changed obligations. |
| Legacy compatibility | Run existing scoped queries and lifecycle fixtures without new metadata/policies. Preserve their output contracts and records; enable stricter policy separately and verify explicit, scoped unknowns or failures without retroactive mutation. |
| Scoped correction | Keep an accepted layer byte-for-byte intact, attach a correction to one claim with counterevidence and a replacement reference, and show both. Missing targets, conflicting replacements and cycles are explicit; a new attempt cannot bypass the old result-hash conflict. |
| Acceptance versus validation | A design-only fixture reports complete with physical validation unperformed. Another fixture requiring physical evidence cannot pass by changing a display label. A software fixture without validation metadata reports unknown or explicit not-applicable according to its contract, never inferred proof. |
| Opted-in gate review | An author supplies passing automated tests but no separately evidenced review to a review-required gate. It stays unsatisfied; a legacy gate behaves unchanged. A reviewer counterexample remains visible and cannot be resolved by mere agreement. |
| Skill walkthrough and rejections | Run the supported portion of the small walkthrough in a fresh fixture. Distinguish readable IDs from UUID fields, preview from persistence, heuristic matches from declarations and correction proposals from existing commands. Confirm documented rejection examples against the selected binary. |

Relevant current implementation references:

- `docs/main.command.trace-id.readme.md`, `src/main_command_trace_id_impl.rs`
  and `src/trait/trace_id.rs`.
- `docs/main.command.version.readme.md`, `src/main_command_version_impl.rs`.
- `src/recur_warp_main.rs`, `src/main_command_warp_impl.rs`.
- `src/recur_git_main.rs`.
- `recur-trace-id/SKILL.md`, `recur-eventness/SKILL.md`, `recur-warp/SKILL.md`.

No implementation, Warp acceptance, benchmark result or release is claimed by
this README. Existing unrelated changes on `recur-rule` remain separate work.
