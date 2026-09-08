# Reveal artifact types: acceptance plan

Coverage contract for `julia-tests/main.command.reveal.artifact-types.test.jl`.
The suite was observed red in slice 0, then expanded and integrated into the normal
runner after passing. See the [baseline](main.command.reveal.artifact-types.baseline.md)
and [verification](main.command.reveal.artifact-types.verification.md) for actual results.

| Area | Required cases |
| --- | --- |
| Metadata | Explicit skill, persona, agent and custom type; empty/invalid value; duplicate equal and conflicting declarations |
| Prefix policy | Configured rules, opt-out, custom prefix/type, overlapping rules, metadata disagreement, no prose inference |
| Legacy | Existing capsules with persona/agent/skill.path fields stay untyped unless explicitly classified; existing fields preserved |
| Listing/showing | Human and JSON type/source/diagnostics agree; stable ordering; full hierarchy identity retained |
| Filtering | Matching and absent type; custom type; exact lane mismatch; duplicate short name within/across types; unresolved conflicts |
| Hierarchy | tree/files discovery of actual hierarchical skill/persona/agent files; custom separator and entry suffix; no SKILL.md requirement |
| Scope | Explicit hidden root, nested project and sibling exclusion; nearest configuration independent of discovery bounds; link escapes/cycles |
| Purity | Files unchanged before/after queries; command-like metadata inert; no recursive references, downloads or runtime activation |
| Regression | Existing reveal/init/prompt tests unchanged; green new suite integrated into full runner; Rust and Julia acceptance recorded |

The suite uses temporary fixture projects so behavior is independent of private
.recur content. Rust unit tests additionally exercise directory links/cycles
(Windows junctions on this host, symlinks on Unix). Exact CLI/JSON expectations are
in the frozen contract. This matrix alone is not acceptance evidence; companion
profile tests remain independently owned by their bubble.
