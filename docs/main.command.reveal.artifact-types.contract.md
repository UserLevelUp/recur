# Reveal artifact types: frozen contract

Contract version: v1, 2026-09-07. Acceptance is recorded separately.

## Classification

Capsule `artifact.type` values are case-sensitive identifiers matching
`[A-Za-z][A-Za-z0-9_.-]*`. Quoted values are accepted by the existing capsule
parser. Custom identifiers are preserved. Empty or malformed declarations yield
`invalid`; distinct explicit declarations yield `conflict`; repeated equal
declarations resolve with a diagnostic. Explicit metadata takes precedence over
prefix hints, with disagreement reported. Invalid/conflicting metadata never
falls back to a prefix. Existing persona/agent/skill.path fields and prose do not
classify an artifact.

Optional configuration (no built-in prefix mappings):

```toml
[reveal.types]
prefix_hints = true

[reveal.types.prefixes]
skill = "skill"
persona = "persona"
agent = "agent"
"skill.team" = "team-skill"
```

`prefix_hints` defaults to true; false disables configured hints but preserves
explicit metadata. Prefixes match complete leading hierarchy segments. The
longest matching prefix wins. Equally specific contradictory hints are unresolved.
Malformed types, blank prefix segments, unsupported keys within `reveal.types`,
and incorrectly typed configuration are errors, including when hints are disabled.
No aliases or executable handlers are introduced.

Repeatable `--sep <CHAR>` overrides directory-specific configured separators;
otherwise the containing file's configured lane separator is used, then `.`.
The existing `reveal.entry_suffix` remains the literal capsule suffix; no required
body filename or fixed hierarchy depth is introduced. Type hints do not rename
lanes or remove Eventness segments from their identity.

## Output and selection

Every JSON list entry and show result adds an `artifact` object:

```json
{"type":"skill","source":"metadata","status":"resolved","diagnostics":[]}
```

`type` is null for unresolved/untyped artifacts. `source` is `metadata`, `prefix`,
or `none`; `status` is `resolved`, `untyped`, `invalid`, or `conflict`.
Diagnostics are deterministic strings. Human output reports the same information.
Existing fields, lane/path identities and lane-then-path ordering are retained.

`recur reveal --type <TYPE>` lists resolved artifacts of that type. Empty listings
remain successful and contain `entries: []`. Untyped/conflicting/invalid artifacts
are visible in unfiltered discovery and cannot match a type filter.

Selection uses the existing exact, suffix, then substring tiers, choosing the
first nonempty tier BEFORE filtering. Thus an exact lane with the wrong type
cannot fall through to another artifact. Duplicate matches can be resolved by
the type filter or a more precise hierarchy/path. Path queries with the configured
entry suffix must select the exact file. Missing, ambiguous and type-mismatch
queries retain legacy successful exit status and list-shaped JSON, with a new
`status` field (`missing`, `ambiguous`, `type-mismatch`). Normal list/show status
is `listed`/`found`. Invalid CLI filters/configuration and I/O errors exit nonzero.
No reserved `list` positional alias is introduced; no-argument reveal lists.

## Discovery bounds and purity

Explicit `-d` is the discovery boundary, including an explicit hidden directory.
Nearest config lookup supplies policy without broadening discovery. With no `-d`,
retain nearest-project-root discovery. Nested roots use their nearest config;
siblings outside explicit scope cannot enter selection. Symlink entries and
directories are not followed; canonical file paths must remain within the root.
The discovery root itself must exist and be a directory. Existing generated/cache
directory exclusions remain in force.

Queries do not write, load referenced bodies, execute capsule commands, install,
activate, or schedule anything. Warp evidence paths remain bounded by the query
root; configuration-root-relative bindings do not permit sibling evidence reads.
Companion profile setup/packets remain separately owned; their future integration
must reuse this classifier. Companion-managed defaults may author prefix rules;
core neither seeds those mappings nor migrates existing files.
