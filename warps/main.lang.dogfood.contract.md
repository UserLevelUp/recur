# Lang dogfooding contract v1

Target: a.0.2.8. Paths below are repository-relative. Scope: the Julia evidence
lab and the existing Lang inspector, using shipped WIR1 query capabilities.

## Invariants

- A short function letter has an explicit scoped identity; declare bundles once
  and reference them by alias. Natural-language behavior remains interpreted.
- Lang checks establish only their reported static coverage. Behavior tests,
  recorded Eventness and Warp acceptance are separate facts.
- Core `recur lang` remains pure. Any new opinionated Lang CLI behavior belongs
  in `recur-lang`; this demo does not require a new CLI, grammar or coordinator.
- Preserve the existing ten demos, greeting API, server routes and inspector
  assertions. Do not silently tighten established behavior to fit a new model.
- Reuse `LangInspector.query_report`, `build_view` and `render_view` as practical;
  do not introduce another parser. WIR1 support does not imply CIR1 support.
- Human/LLM-derived models and fixtures must link back to a requirement. They
  are implementation choices, not mechanically generated proof from Lang.

## slice-0 — baseline and initial specification

Gate `baseline-and-specification`:

1. Observe existing `julia-tests/main.demo.lang-inspector.test.jl` and
   `demos/web-evidence-lab/main.server.test.jl`, recording runtime and binary.
2. Specify the greeting boundary in `demos/web-evidence-lab/main.greeting.recur`
   from observed source and existing assertions. Explicitly call this first
   example retrospective; the new inspector integration will be tests-first.
3. Check the new specification and inspect `greeting.g` with the real CLI.
   Record coverage, source fingerprint and any diagnostics without claiming
   that prose expectations were executed by the parser.

## slice-1 — behavior fixtures and model decisions

Gate `greeting-boundary-fixtures`:

Derive a table from the specification before changing greeting implementation:
missing/empty/whitespace names; trimmed Unicode; 1, 40 and 41 Julia characters;
English, Spanish and unknown locale fallback; exactly one message or error.
Check HTTP status and JSON shape, plus HEAD body suppression and unsupported
method rejection through the existing transport. Distinguish character count
from byte count and grapheme count. Preserve existing tests without weakening.
Record whether separate request/domain/view structs help; no mandatory model
hierarchy for a small function. Implementation changes require observed failures
or a declared new contract, not an arbitrary refactor for the demonstration.

## slice-2 — specify the new inspector boundary and write tests first

Gate `inspector-contract-and-red-tests`:

Before implementing new routes, author a compact Lang specification and freeze
the API request/response and status behavior in this contract or a linked file.
Use `GET /api/lang` and a small `/lang.html` page as the intended surface.
Choose a capability from a server-owned catalog: initial entries are the
greeting specification and existing inspector specification. Requests select
catalog IDs and documented scopes; they never choose binary paths, arbitrary
filesystem roots, command arguments or bindings. Define unknown IDs/scopes,
query failure, malformed packets and empty selection explicitly.

Create hand-authored adapter fixtures before the route implementation. Cover
exact argument boundaries, same-letter scoped identities, missing coverage,
unsupported CIR1, canonical aliases, source/hash preservation, boundaries and
findings. A missing implementation failure only proves absence; record the
behavior assertions exercised when the implementation becomes available.
Keep intentionally red work outside the normal regression runner until green.

## slice-3 — implement and display the scoped capability

Gate `server-inspector-integration`:

Implement the frozen adapter using an injectable query boundary and existing
inspector models. A real query runs only the configured Recur executable with
an argument vector; declared bindings and capsule instructions are never run.
Preserve the complete query packet in the response, including original source
fingerprint, findings and coverage. Do not replace CLI diagnostics with success.

The page allows catalog/scope selection and presents header/body/footer with
compact aliases and expandable detail. Display source-provided text as text.
Show recorded state, static findings and observed test evidence separately;
do not invent a green acceptance badge from a `.complete` name or a static check.
Include explicit loading, empty and error states. The page can be plain HTML/JS
served by the existing Julia server; no new frontend framework is necessary.

Pass the slice-2 fixture tests, then exercise real HTTP queries on an ephemeral
loopback port. Confirm unknown selection handling and unchanged source/config/
receipt files. Move green integration tests into the appropriate normal runner.

## slice-final — integration and useful residue

Gate `walkthrough-and-regression`:

Provide a short runnable walkthrough: inspect greeting.g, exercise a boundary
fixture, inspect a multi-stage inspector scope, and compare the contract with
the rendered behavior/evidence. Explain what was inferred and what was tested.
Rerun greeting/server, inspector and new integration suites plus the normal
Julia runner. Rust checks are required if Rust changes are actually introduced.
Record inputs, commands, output and limitations before completing slices.

Gate `lang-gap-assessment`:

Record friction encountered using Lang for this capability. Separate missing
query functionality from demo presentation needs and authoring preferences.
Only propose grammar or companion changes with a concrete example and desired
input/output behavior. No automatic grammar expansion is a completion criterion.
SGR1's existing graph coverage remains distinct from WIR1 and runtime behavior;
do not claim this demo proves absence of all architectural circular dependencies.
