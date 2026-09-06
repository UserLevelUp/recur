# Prompt discovery planning capsule
warp.id = main.command.prompt.discovery
warp.root = docs
observed.state = incomplete
readiness.slice = slice-1
goals.now = implement typed prompt registry and bounded source resolution when requested
pull.first = recur warp slices main.command.prompt.discovery -d docs
pull.then = read docs/main.command.prompt.discovery.readme.md
verify = julia julia-tests/main.command.prompt.discovery.test.jl
evidence = main.command.prompt.discovery.slice-0.complete.md
do.not.disturb = this bubble prepares local prompt data; LLM provider invocation is deferred
