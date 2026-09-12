# WIR1 runtime evidence association/API v1

Frozen for slice-0, 2026-09-12. Behavioral requirements below are interpreted by
hand-authored tests; WIR1 parsing does not prove them.

## Roots and compatibility

Development map move: `warps/main.lang.runtime-evidence.warp-map.json` to
`main.lang.runtime-evidence.warp-map.json`, preserving all UUIDs. One live map;
documentation stays under `warps/`. Resume/complete with `-d .`. Result files
and immutable attempts live under `warps/runtime-evidence/`. Every checked input
names the actual repository file. No copied source substitutes for live inputs.
No global defaults or containment rules change.

The original `/api/lang` contract and catalog remain unchanged. A separate
`/api/lang/evidence` endpoint accepts exactly `id` and `scope`, with the same
400/404 selection rules, 502 query-failed response, HEAD/405 and no-store rules.
Its catalog contains only `runtime-evidence`, source
`demos/web-evidence-lab/main.lang.evidence.recur`, root the repository directory,
scopes `associate`, `associate.a`, `assess`, `assess.a`. No request accepts paths,
roots, commands, binaries or arbitrary scope letters. Both scopes deliberately
reuse local function `a`; aliases retain their qualified identities.

Response extends the existing presentation envelope with `evidence`, schema
`lang-evidence-report-v1`; `packet` remains the original query-v1 object including
unknown fields, coverage, findings and execution:not-run. Empty selection returns
catalog/selection:null without processes. Evidence errors are successful reports
with a nonchecked verdict, never silently empty evidence or a static query error.

## Explicit association

Server-owned registry: `demos/web-evidence-lab/main.lang.evidence.associations.json`.
Schema `lang-evidence-associations-v1`, `associations` array. Each association has:

- `id`, `source`, `source_hash`, exact function `scope`;
- `aliases`: local-to-canonical map for the selected header's input/output;
- `warp_id`, `slice_id`, `contract_hash`, `map`: explicit map path;
- `requirements`: nonempty map of requirement IDs to nonempty test case ID arrays;
- `required_inputs`: paths for spec, linked behavior contract, implementation,
  tests, configuration and runner; each must be present in checked input scope;
- `gates`: nonempty gate ID array; `transition`: {E0, dE, Ef};
- `current_attempt`: explicit attempt ID or null; `attempts`: ordered observations.

Each attempt has `id`, `phase` (red/green), `producer`, `runtime`,
`gate_refs` (gate to reference), `cases` (case ID to passed/failed), and
`source_hash`. External references use `evidence:relative/path.json` and the
existing warp-external-evidence-v1/result-v1 formats. Manual/native ACK paths
are declared observations only; missing/invalid test references cannot pass.
Requirement IDs and case results are provenance supplied by a runner, not
inferred from aggregate counts. Every required case must pass in the selected
green attempt, whose producer matches each external evidence producer.

Selection matches exact source and qualified scope from the original packet.
Scope requests normalize through that packet's header, never string similarity.
More than one association for a selected function is ambiguous. A registry
entry with wrong source is mismatched when it claims that scope; wrong scope is
mismatched when no exact match exists. No associations is absent.

Map Warp/slice/contract/gates must exactly agree with the association. Source
hash and aliases must agree with the original selected header; aliases never
derive from equal field shapes. Declared transition must match packet events.

## Assessment and history

Report fields: `schema`, `status`, `reasons`, `associations`, `execution`:
`not-run`, `qualification`: producer not rerun; fingerprints are change detectors,
not authenticity or dependency-closure proof. Per association retain identity,
requirements, transition, explicit inputs, immutable attempt observations and
separate current assessment and Warp acceptance. No timestamp chooses a winner.
Missing current_attempt is declared; unknown/duplicate attempt IDs are malformed
or ambiguous. Historical red observations stay red even after implementation.
History is recorded provenance, not independently authenticated execution.

| Outcome | Rule |
| --- | --- |
| absent | registry missing or empty |
| declared | manual/native receipt or no selected green attempt |
| checked | identity, all required cases/gates and existing external checker pass |
| stale | source/hash or checker explicitly named input/result changed |
| failed | observed failing case/result, skipped/zero/inconsistent counts, checker failure |
| malformed | invalid schema/type, unsafe/missing required file, invalid JSON or limits |
| mismatched | wrong source/scope/alias/Warp/slice/contract/gate/transition/producer/input scope |
| ambiguous | duplicate association or attempt identity |

Precedence for multiple findings: malformed, ambiguous, mismatched, failed,
stale, declared, absent, checked. Preserve all reasons and checker detail.
Static findings and complete markers never alter evidence status. Warp
acceptance is a separate live `recur warp show` projection, checked only for
the exact associated slice/contract and checked gates; an unaccepted checked
test remains checked evidence with acceptance false. Evidence queries do not
invoke writers, bindings, test runners or capsule commands.

## Read boundary and limits

All file references are nonempty forward-slash relative paths; reject absolute,
drive/UNC, backslash, NUL, dot/dot-dot components, directories and canonical
symlink escapes. Missing registry alone means absent; other missing files fail
visibly. No recursive discovery. Before invoking the checker, bound every
referenced manifest, result and named input. JSON files <=1 MiB, each other input
<=64 MiB, total unique reads <=128 MiB, <=128 unique files, <=16 associations,
<=16 attempts per association and <=128 cases per attempt. Exceeding a limit is
malformed; never return a truncated report that could be checked.

Checker invocation is the argument vector `[binary, "warp", "evidence", path,
"-d", fixed_root, "--json"]`; acceptance uses `[binary, "warp", "show",
warp_id, "-d", fixed_root, "--json"]`. Capture/check output, no shell. Reuse
the existing fingerprint/result verdicts; association adds relevance checks.
Result history may use snapshots only as labeled historical data. Current
required_inputs must name live files; assessment never redirects them to copies.

## Presentation and acceptance cases

Header shows exact contracts and requirement/case links; body retains scoped
letters, aliases and boundary edges; footer separately labels intended E0/dE/Ef,
recorded state, static findings, observed red/green attempts, current assessment
and Warp acceptance. Compact/expanded reports share one data object. All strings
use text nodes. Empty/loading/error/nonchecked states remain visible; desktop
and mobile must be readable without horizontal page overflow.

Stable requirement groups for slice-1: identity, association, history, freshness,
counts, containment, limits, purity, native-ack, compatibility, presentation,
reentry. The slice-1 matrix assigns exact cases before loader implementation.

defines: main.lang.runtime-evidence.association.v1 bounded association and API
consumes: recur.warp.evidence.integrity.external existing checker semantics
