# Split tests-first baseline

defines: demo.blackjack.web.split.baseline pre-implementation observation
consumer: demo.blackjack.web.split.test.plan acceptance matrix
publish: demo.blackjack.web.split.baseline.result missing collection demonstrated

Before changing engine/server/assets, authored main.blackjack.web.split.runtime.test.jl
from the existing 22-case matrix. It failed its collection capability assertion:
0 passed, 1 failed, 0 errors. Dependent multi-hand tests were guarded until the
required state existed. Raw log: observations/runtime-baseline.log.

Earlier proposal evidence independently had 12 passing model checks, 2 intended
runtime failures (missing split action and hands projection), and 8702 existing
website assertions. Those observations/logs remain historical. Pre-implementation
root sources are retained under observations/v1-source as .snapshot files.

This is acceptance of the demonstrated baseline, not a passing implementation.
