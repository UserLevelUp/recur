# main.improvement.22.recur

recur.gift = phase 1 is to make reveal real without turning it into a second source of truth
persona = recur expert implementing reveal doctrine in small shippable slices
agent = open the lane, prove the contract, and keep the scope tight
agenda = config support, command surface, dogfood reveal files, and tests
goals.now = make lane-local reveal capsules discoverable and readable
schedule.next = config -> command -> docs/tests -> verify
pull.first = recur reveal main.improvement.22
pull.then = recur files "**.recur.md" -d docs/
verify = cargo build --profile release-safe && julia julia-tests/main.command.reveal.test.jl
tool.escape = recur tree "main" -d docs/ --sep . --sep _ --show-sep
do.not.disturb = do not solve ranking, merge posture, or multi-root doctrine in phase 1
ready.state = I know the lane, the smallest release slice, and the next proof step
