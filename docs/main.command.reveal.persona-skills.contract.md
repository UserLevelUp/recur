# Reveal associations and packets v2

Frozen for implementation, 2026-09-09. This supplements existing artifact-types
v1 selection and root rules; it does not replace the shared classifier.

Core `recur reveal [selector] [--type agent|persona|skill] -d ROOT --json`
retains its existing capsule selection and output. List entries and show results
add `associations`, with each edge identifying declaring config, owner type/ID,
relation, target type/ID and resolution status. Configured records are also
discoverable when their configuration is inside ROOT. A record may explicitly
bind `capsule` to an exact lane or root-relative capsule path; no name inference.
Ambiguous, missing, wrong-type or multiply bound capsules are diagnostics.

Configuration tables: `reveal.agents`, `reveal.personas`, `reveal.skills`.
Each record permits `capsule` and `path` (optional strings); agents additionally
permit `persona` (one persona ID) and `skills` (ordered skill IDs); personas
permit `skills` and `guidance_level`. Skills permit no outgoing associations.
Unknown record fields and malformed types fail visibly. Empty skill lists are
explicit opt-outs. References address exact config IDs in their target table;
an exact typed capsule lane is a fallback only if that config ID is absent.
Record IDs are separate from capsule lanes; two records cannot bind one capsule.
Repeated skill IDs collapse to one edge per owner with a diagnostic. Distinct
owners keep their provenance. No transitive persona inheritance is supported.

`recur-reveal init -d ROOT [--dry-run] [--json]` adds only missing top-level
association tables to the nearest project config (or creates ROOT/.recur/config.toml).
Existing tables, including empty tables, are preserved whole. Existing comments
and unrelated settings survive. Validate before writing, write one config using
an atomic temporary-file replacement, and make repeats byte-for-byte no-ops.
Fresh `recur init` uses the same defaults. Sample Skippy skill pointers can be
unresolved; defaults never create referenced files or install anything.

`recur-reveal next ID [--type agent|persona] -d ROOT [--max-files N]
[--max-bytes N] [--json]` selects exact record IDs or exact capsule lanes. Omitted
type searches agents and personas together and reports ambiguity. No fuzzy
packet selection. Defaults: 16 body files, 65536 total bytes. Zero bounds are
valid and report omitted bodies. All explicit sources are project-relative;
absolute paths, parent traversal, symlink escapes and reads outside ROOT fail.
No global/remote registry lookup or arbitrary recursive reference expansion.

Packet schema remains `recur-reveal-packet-v1`, extended additively with
`subject`, `associations`, `sources`, `context`, `diagnostics`, `execution` and
`mutation`. Preserve legacy persona/skills fields for persona requests. Sources
carry path, SHA256 and body; config and capsule fingerprints bind metadata.
Deduplicate skills by exact typed identity, retaining all incoming edges. Skill
bodies use explicit `path`, otherwise a capsule's explicit `skill.path`, otherwise
the typed capsule itself. Agent/persona bodies use `path` or their bound capsule.
SKILL.md must contain YAML frontmatter with nonempty name and description;
other explicit Markdown bodies need nonempty UTF-8 content. No body is executed.
Missing or invalid required references, source conflicts, invalid bodies and
budget omissions produce `state=blocked`, JSON stdout, exit 1. Input/config errors
use a structured error and exit 2. Ready packets exit 0; readiness means context
available, not task acceptance. Init errors exit 2. Queries never write.

All advertised discovery must remain useful without a companion or any bodies.
Core adds metadata and fingerprints only; it does not load referenced bodies.
