# Initial app prompt scenarios

`cases.jl` supplies isolated projects without prompt overrides for the bundled
`warp.naming`, `warp.slicing`, and `warp.recovery` defaults. Each case records an
intent, scope, evidence and an expected-decision rubric. The discovery test suite
materializes these projects and checks the actual CLI packets, source integrity,
determinism, freshness, narrowing and byte limits.

The rubrics describe what a useful model response should do. They are not assertions
that an LLM has been evaluated: no model is called by this suite. A future model
evaluation should score the returned decision against these rubrics and require
citations to supplied evidence, without demanding one exact response string.
