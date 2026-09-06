# Warp: accessible progress and configured creation

Status: implemented and complete locally. All four Slice gates have reviewed
declared receipts. See main.command.warp.usability.verification.md for test results.

Scope: `recur warp show <name>`, `recur warp slices <name>`, and
`recur-warp create <name> --goal <text>`. Queries reuse discovery and qualified
Slice projections. Creation previews a single map, uses configured directory and
optional JSON template, refuses unsafe names/paths, invalid contracts and overwrite.
No task execution, inferred completion, arbitrary templates/scripts or restructuring.

Default location: warps/. Config: [warp.creation], directory and template fields
relative to the nearest configuration's project root (or -d with no config).
-d always bounds writes; inherited output outside it is refused, not silently widened.
Template is a JSON bubble map with {warp} and {goal} string placeholders. No keys,
paths or commands are evaluated. Every Slice needs a nonempty acceptance gate.
Default template has baseline and final slices. Generated declarations are scaffolds,
not accepted evidence. Multi-file scaffolding remains deferred.

Test first: discovery including .recur, unknown/duplicate identities, pending and
completed/blocked slices, stale current selection, JSON/text/no-write queries,
create dry run, configured/template creation, containment, malformed template,
duplicate target preservation, and legacy suites. Gate evidence is reviewed declared
evidence, not independent producer execution by Recur.

Remaining larger features: main.command.warp.roadmap.md.
