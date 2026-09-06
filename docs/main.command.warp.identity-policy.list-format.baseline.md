# List presentation baseline — 2026-09-06

Pre-implementation contract run:
`julia julia-tests/main.command.warp.list-format.test.jl`

Observed: 16 passing assertions, 19 expected red failures, 0 errors.
Failures expose the missing trait-style sections/fields and retained inline counts.
Existing JSON, empty-inventory summary and query byte-preservation checks pass.
This suite stays standalone, outside runtests.jl, until implemented. No tests are
marked test_broken to disguise the missing formatter.

Slice 4 was added; the still-pending final Slice contract was advanced to v2.
The accepted Slice 0 record remains historical and unchanged. No formatter code
was changed in this update. Additional ring and escaping fixtures remain required
before Slice 4 acceptance.
