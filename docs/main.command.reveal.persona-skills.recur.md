# Discover agents, personas and skills
warp.id = main.command.reveal.persona-skills
warp.root = docs
observed.state = complete
persona = Recur reveal and configuration implementation expert
goal = Discover agents personas and skills independently and expose explicit relationships without loading or execution.
contract.revision = v2; broadened scope 2026-09-09; prior persona-only observations are historical.
pull.first = read docs/main.command.reveal.persona-skills.readme.md; recur warp slices main.command.reveal.persona-skills
verify = julia julia-tests/main.command.reveal.persona-skills.test.jl
do.not.disturb = Context preparation does not install skills activate personas or execute instructions.

evidence = docs/main.command.reveal.persona-skills.verification.md
