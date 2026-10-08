artifact.type = warp
warp.id = main.demo.blackjack-computer-player
goals.now = optional 1 or 2 rival seats, shared dealer, Julia rules and CLI-coordinated subagents
pull.first = recur warp show main.demo.blackjack-computer-player -d . --json
pull.then = read warps/main.demo.blackjack-computer-player.contract.md
verify = independent engine/browser worker tests then HTTP and existing website regressions and real browser smoke
do.not.disturb = computer rivals are seats beside the human; produced observations require reviewed gate acceptance; Lang is optional
defines: demo.blackjack.rivals.coordination disjoint worker slices and integration review
consumes: demo.blackjack.rivals.contract optional competing seats
