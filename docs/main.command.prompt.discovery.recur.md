# Prompt discovery capsule
warp.id = main.command.prompt.discovery
warp.root = docs
observed.state = complete
readiness.slice = none
goals.now = discover app prompts and prepare bounded evidence; model invocation remains a separate follow-on
prompt.ids = warp.naming, warp.slicing, warp.recovery
pull.first = recur warp slices main.command.prompt.discovery -d docs
pull.then = read docs/main.command.prompt.discovery.readme.md
verify = julia --startup-file=no -C generic -O0 julia-tests/main.command.prompt.discovery.test.jl
evidence = main.command.prompt.discovery.verification.md
do.not.disturb = this bubble prepares local prompt data; LLM provider invocation is deferred
