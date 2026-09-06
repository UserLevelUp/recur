# Slice 1: read-only-progress
Status: complete

Implemented discovery-scoped show/slices using the existing qualified bubble projection. Tests compare complete, partial and exploded fixture projections, check hidden-folder discovery and ambiguity refusal, and preserve query input bytes. Current selection is explicit map metadata, never inferred from ready work. Rings retain their merge domain view.

Acceptance gate: read-only-progress. Evidence: julia-tests/main.command.warp.usability.test.jl,
observed passing standalone run on 2026-09-06. This is reviewed declared evidence.
