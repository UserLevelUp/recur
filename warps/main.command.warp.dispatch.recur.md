artifact.type = warp

warp.id = main.command.warp.dispatch
warp.root = warps
goals.now = CLI coordinator, asynchronous host assignments, optional Lang planning, observed verification and bounded feedback
pull.first = recur warp show main.command.warp.dispatch -d . --json
pull.then = read docs/main.command.warp.dispatch.readme.md and warps/main.command.warp.dispatch.contract.md
verify = cargo test --locked; julia --startup-file=no -C generic julia-tests/runtests.jl using matching local binaries
do.not.disturb = no automatic acceptance from agent exit or produced status; optional Lang remains separate from runtime verification
consumes: main.command.warp.dispatch.config companion-owned scheduling policy
consumer: main.command.warp.dispatch.review.coordination actual multi-provider reviews and retained handoffs
observed.state = complete
evidence.now = warps/dispatch-review/observations/acceptance/main.command.warp.dispatch.progress.json
evidence.limits = declared live gates; repository-root dispatch evidence checked separately; reconstructed baseline does not establish original TDD chronology
