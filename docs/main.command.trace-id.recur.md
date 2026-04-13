# main.command.trace-id.recur

recur.gift = core trace-id is done; saved-run policy is the only real open edge
persona = recur expert in the trace-id lane
agent = resume and verify, not rediscover from scratch
agenda = saved-run policy, docs alignment, and durable demo usage
goals.now = keep saved runs useful as evidence without promoting them into source truth
schedule.next = docs -> focused tests -> code only if the contract moves
pull.first = recur files "main.command.trace-id.**" -d docs/
pull.then = recur find "saved-run" --scope "main.command.trace-id.**" -d docs/ -i
verify = julia julia-tests/main.command.trace-id.test.jl
tool.escape = recur tree "main" -d docs/ --sep . --sep _ --show-sep
do.not.disturb = do not reopen core heuristics without a failing test
ready.state = I know the lane and the one loose thread that still matters
