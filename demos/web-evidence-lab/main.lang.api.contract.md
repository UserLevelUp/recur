# Catalog inspector API v1

Derived from dogfood slice-2/3 and `main.lang.api.recur`, before implementation.
This file freezes the new boundary; greeting and existing static routes retain
their established behavior.

`GET /api/lang` without parameters returns 200 JSON with `catalog` and
`selection: null`. No process runs. `catalog` is an ordered array of objects
with `id`, `label`, and `scopes`. Entries:

| id | server-owned source/root | allowed scopes |
| --- | --- | --- |
| greeting | main.greeting.recur / web-evidence-lab | greeting, greeting.g |
| inspector | main.lang.inspector.recur / lang-inspector | query, query.q, model, model.m, view, view.v |

Both `id` and `scope` are required to select. Empty/missing either, duplicate
parameters, or any parameter other than `id`/`scope`: 400 `invalid-selection`.
Unknown ID: 404 `unknown-capability`. Unknown scope for a known ID: 400
`unknown-scope`. Values match exactly, including case; no local-letter shorthand.
No request supplies binary, root, source, arguments, eventness or bindings.

Selected success: 200 JSON with `catalog`, `selection: {id, scope}`, `packet`,
`cards`, `text`, and `observed_evidence: []`. `packet` is the entire original
query object, without rewriting source/hash, findings, coverage or extra fields.
Cards expose the existing FunctionCard fields. Text comes from render_view.
An empty packet header is valid and displays "No functions selected".
Static findings are successful query data, never a runtime acceptance claim.

The adapter delegates to LangInspector.query_report using exactly
`[binary, "lang", "report", source, "--scope", scope, "-d", root, "--json"]`.
Binary is configured at server startup from RECUR_BIN (default local release-safe
binary); roots/sources come from the fixed catalog. An injectable argument-vector
adapter supplies fixtures. No shell, binding or capsule instruction is executed.

Query exceptions/nonzero exit, invalid JSON, malformed required display data,
missing coverage or unsupported IR (including CIR1): 502 `query-failed`.
All errors have exactly `{error: {code, message}}`; query failure message retains
the available CLI diagnostics. Responses use JSON MIME and no-store. HEAD applies
the same status/headers and suppresses its body; unsupported methods return 405.

`GET /lang.html` serves an accessible plain HTML/JS page. It selects only catalog
IDs/scopes; shows loading, empty, error and populated states; displays header
aliases, body composition and footer events/state with expandable packet detail.
All source-provided strings use text nodes. Recorded state, static findings and
observed test evidence remain separate. Empty observed_evidence means no runtime
evidence supplied by this API, irrespective of any complete filename.

Tests use hand-authored packets, including two scoped `q` letters, canonical
aliases, a boundary edge, a finding, recorded complete state and markup text.
Real loopback tests must compare untouched spec/config/receipt bytes before/after.
