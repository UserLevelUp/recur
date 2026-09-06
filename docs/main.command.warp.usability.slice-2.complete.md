# Slice 2: configured-creation
Status: complete

Implemented built-in and JSON-template single-map creation with nearest configuration, dry-run default, bounded paths, validated gates/dependencies and no-overwrite publication. Tests cover default/custom layouts, quoted goals, inherited scope refusal, invalid identities/templates and duplicate preservation. Hard-link publication is required; no multi-file transaction is claimed.

Acceptance gate: configured-creation. Evidence: julia-tests/main.command.warp.usability.test.jl,
observed passing standalone run on 2026-09-06. This is reviewed declared evidence.
