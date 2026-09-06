# Reveal persona skills
warp.id = main.command.reveal.persona-skills
warp.root = docs
observed.state = incomplete
readiness.slice = slice-0
persona = Recur reveal and configuration implementation expert
pull.first = read docs/main.command.reveal.persona-skills.readme.md; recur warp slices main.command.reveal.persona-skills
verify = julia julia-tests/main.command.reveal.persona-skills.test.jl
do.not.disturb = Planning Warp only; no automatic skill installation activation or command execution.
