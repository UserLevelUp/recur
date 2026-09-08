# Recur Lang baseline acceptance contract

Acceptance contract: v1. Target release: a.0.2.8.
The slice-0 command/schema boundary is frozen in
`docs/main.command.lang.query.readme.md`. Acceptance remains evidence-bound;
inspect the live Warp projection and verification record.
Paths are repository-relative unless a path is explicitly described otherwise.

## Product and ownership invariants

Recur Lang supplies exact formalism inside a selected scope without imposing
that schema on the whole repository. Its purpose is to reduce cognitive load:
declare full parameter/return bundles once, then compose functions using short
local letters and explicit input/output roles. Expand detail when needed.

- `recur lang` only discovers, filters, projects, validates and describes
  authored facts. It neither recommends a workflow nor makes repair, priority,
  policy or execution choices.
- All opinionated language behavior belongs to `recur-lang`, even when a
  proposal writes nothing. Preserve its existing confirmed Warp transition and
  ACK/NAK contract; the baseline does not add arbitrary worker execution.
- Header explains exact contracts, local symbols and familiar function meaning.
  Body explains functional connections. Footer exposes checks, Eventness and
  evidence with an explicit distinction between recorded and checked facts.
- A compact symbol such as `f(a)` remains bound to its scoped identity. Local
  letters may repeat across scopes; the view must disambiguate them without
  rewriting source identifiers. Input/output bundles retain canonical identity,
  aliases and every producer-to-sub-input relationship on expansion.
- Filtering reduces visible detail while retaining or referencing boundary
  dependencies and the paths needed to explain relevant cycles or missing joins.
- Use existing parsed models and SGR1. Do not add a third competing parser or
  pretend WIR1 and CIR1 already form one unified coordination AST.
- Query execution leaves source, Eventness, configuration and receipts unchanged.
  It does not invoke Julia, Cargo, Git, watchers, an LLM or a worker.

## Initial query surface

Freeze the exact flags, errors, exit codes, schemas and supported grammar in
slice-0 around these useful operations:

```text
recur lang list -d <root> --json
recur lang show <source> --scope <scope-or-symbol> [--expand] [--json]
recur lang report <source> [--scope <scope-or-symbol>] [--eventness <state>] [--json]
recur lang check <source> [--scope <scope-or-symbol>] [--json]
```

The scoped source is explicit, so an identical letter in a different file never
silently resolves. Keep command meanings consistent with existing Recur root
and separator conventions. Define how `-d` applies to source-specific queries.
`--eventness` selects recorded, configured states; it does not rank work by an
invented urgency policy. Selection/expansion must not silently broaden the root.

Support only the existing WIR1/CIR1 subsets needed by the algorithm and Skippy
fixtures. Report consumed schema, source identity/hash and scope. Unsupported
grammar/features must be visibly rejected or excluded with explicit capability
diagnostics; do not imply a whole document is validated because one supported
fragment parsed. Reserve grid rendering, feedback execution and subsystem/import
syntax for later contracts.

## Slice gates

### slice-0: baseline and command contract

Gate `baseline-and-cli-contract`:

- Inspect current Cargo binary declarations and installed/local help; capture
  which commands actually exist. Record executable paths as well as versions.
- Observe existing Rust tests for `recur_lang_ir`, `recur_lang_concurrent_ir`
  and the `recur-lang` companion, plus the focused Julia language fixtures.
  Preserve known-broken prototype cases and distinguish them from new failures.
- Freeze the initial CLI argument grammar, result schema versions, exit codes,
  capability limits and read scope. Specify stable errors for missing source,
  unknown/ambiguous scope, unsupported source version and malformed input.
- Define focused red CLI cases for the proposed commands; a failing baseline
  records missing behavior and must not be mislabeled implementation acceptance.

### slice-1: shared graph prerequisite and query projection

Gate `static-graph-prerequisite-reviewed`:

- Inspect `recur warp show main.improvement.30.static-graph -d docs --json` and
  its applicable contracts, layers and test records. Record the exact accepted
  SGR1 contract and source evidence; an old complete filename is insufficient.
- Do analyzer implementation in that existing Warp. This gate accepts its use
  in the baseline; child completion alone does not establish parent acceptance.

Gate `source-bound-model-projection`:

- Project supported WIR1/CIR1 models into query data without reparsing their
  content through a separate grammar. Preserve their distinct schema identities.
- Consume SGR1's exact nodes, edges, awaits, findings, spans and source hash for
  the concurrent fixture. Do not infer runtime lane execution from static facts.
- Prove deterministic repeated projection and refusal of mismatched source or
  unsupported model/schema assumptions.

### slice-2: discover and inspect symbols

Gate `pure-list-show`:

- `list` finds the supported sources in a fixture root with deterministic
  identities; empty roots have a defined empty result. Explicit scope excludes
  an unrelated sibling project, and invalid/unsupported inputs stay visible.
- `show` on an algorithm scope resolves its familiar function, exact inputs,
  outputs, aliases, source span and implementation binding. Inspecting a binding
  never executes it.
- Same-letter symbols in two scopes remain distinct. Unknown or ambiguous
  symbols have deterministic errors; Unicode paths and descriptions survive.
- Compare source/config/receipt inventories before and after query execution
  and prove there are no writes or background processes.

### slice-3: compact and expanded explanations

Gate `lossless-header-body-footer`:

- A scoped report explains header, body and footer from the same source-bound
  model. Text and JSON express the same relationships and findings.
- The compact view uses local function letters and input/output references;
  full field lists are not repeated at every edge. The selected header or
  expanded view resolves them to the exact canonical contracts.
- A fan-in fixture represents several producers with a compact bundle while
  expansion preserves the producer and contract for every sub-input. A similarly
  shaped but separately identified contract must not become an implicit alias.
- Compact/expanded views preserve the same node and edge identities and do not
  invent new persisted aliases. Unrelated functions' detail stays out of scope.
- Footer descriptions distinguish requested checks, recorded Eventness,
  supplied receipt references and actual validation findings. Missing evidence
  remains visible; prose descriptions do not certify runtime success.

### slice-4: focused Eventness and dependency checks

Gate `scope-eventness-and-diagnostics`:

- Test default and project-custom Eventness vocabulary. Filtering by source,
  language scope and state composes predictably without mutating lifecycle data.
- A cycle crossing a selected scope remains explainable through boundary
  references and a deterministic path. Hiding an outside node cannot turn a
  failing graph into a sound one; limited analysis declares its coverage.
- Surface SGR1's dependency/wait cycles, unreachable nodes and unsatisfied joins
  using exact symbols and available source spans. Each negative fixture produces
  the expected stable finding; valid fixtures produce no blocking finding.
- `check` has distinct documented success, validation-failure and input-error
  outcomes. No query recommends repairs, silently fills missing contracts or
  starts a transition.

### slice-final: baseline integration and packaging

Gate `regression-and-companion-boundary`:

- Pass focused query/graph tests and appropriate full Cargo/Julia regressions.
  Record actual commands, runtime versions, source inputs and known-broken
  allowances. Do not count proposed tests or historical runs as fresh evidence.
- Existing confirmed `recur-lang warp` success, stale/rejected receipt handling
  and bounded writes still work. Core queries remain read-only.
- Help, README, reveal/skill guidance and examples identify implemented versus
  deferred features, with the pure/opinionated ownership boundary intact.

Gate `packaged-lang-smoke`:

- Build the actual intended release binaries and inspect Windows/Linux archive
  contents. Include `recur-lang` and the declared companion set; a Cargo binary
  declaration alone does not prove archive inclusion.
- Add and verify Chocolatey install/uninstall handling for `recur-lang` and
  align nuspec commands with packaged help. Exercise staged executable help,
  a representative pure query and a companion dry run without installing or
  publishing as an implicit consequence of this gate.
- Confirm version alignment at 0.2.8. Bind final artifact checksums and tests
  to the candidate that will be submitted; publication is a separate action.

## Evidence and completion

All slices begin pending. Gate references use the repository's declared evidence
mode; that mode does not independently verify a test result or its freshness.
Inspect the cited logs, contracts and input applicability before acceptance.
When supplying external evidence, use the existing Warp evidence format and
explicit input scope; do not invent a second receipt subsystem for this plan.

`depends_on` orders this bubble's slices only. The named SGR1 prerequisite in
metadata is advisory; slice-1's explicit review is required. Reassess acceptance
after material contract changes and version the affected contract identities.
Capture successful implementation and regression evidence before recording any
accepted layer. Creating this plan authorizes no implementation success claim.

defines: recur.lang.baseline.contract.v1 exact bounded query experience and acceptance gates
consumes: main.improvement.30.static-graph.warp existing analyzer implementation boundary
consumes: recur.lang.static.graph.report.v1 exact graph report contract
produces: recur.lang.baseline.acceptance required evidence for a useful a.0.2.8 baseline
