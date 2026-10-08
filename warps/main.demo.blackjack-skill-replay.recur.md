artifact.type = warp
warp.id = main.demo.blackjack-skill-replay
warp.root = warps
goals.now = Stronger public-information rivals, deterministic replay and meaningful statistics
observed.state = planned
pull.first = recur warp show main.demo.blackjack-skill-replay -d . --json
pull.then = read warps/main.demo.blackjack-skill-replay.contract.md and freeze interfaces/strategy action parity before tests
verify = julia julia-tests/runtests.jl --demo blackjack-web; select blackjack-lab separately if rules change
do.not.disturb = preserve accepted rivals baseline; no enabled dispatch until real worker inputs and failing tests exist; no small-sample skill rating
consumer: demo.blackjack.rivals.contract accepted zero/one/two rivals baseline
defines: demo.blackjack.skill.contract next planned implementation
