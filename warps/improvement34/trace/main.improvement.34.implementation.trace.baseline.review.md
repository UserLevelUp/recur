# Improvement 34 trace baseline review

artifact.type = lane
defines: main.improvement.34.implementation.trace.baseline.review
consumer: main.improvement.34.implementation.trace.contract
warp.id = main.improvement.34.implementation.trace
status = scope and expected-red reviewed

The planned first bubble covers path discovery, exact Markdown declarations and
explicit revision pins. Contextual current resolution and graph traversal are
separate contracts. The planning base was `48aca7d` on `recur-lang`.

The [expected-red log](observations/trace-v2.expected-red.log.txt) records 9
passing and 9 failing assertions against the pre-implementation CLI. Scoped
legacy details still worked; the old CLI required `--scope` and did not expose
`--declared`. The new Julia cases exercised the absence of the planned surfaces,
not a failed build or missing CLI. The original release-safe binary was not
retained for this run, so its exact bytes cannot be independently rechecked.

The retained v2 build reran the [existing baseline](observations/trace-v2.baseline.log.txt)
with 13 passing assertions and no skips. False heuristic roles remain diagnostic
observations and are not made a regression requirement. The separate
[new-behavior evidence](observations/trace-v2.evidence.json) binds the actual
build, inputs, command and results. This baseline review is a declared gate;
the expected-red output is not passing test evidence.
