artifact.type = warp
recur.gift = Play and inspect the second blackjack demo, from symbolic contracts to an interactive Julia website.
warp.id = main.demo.blackjack-web
warp.map = warps/main.demo.blackjack-web.warp-map.json
pull.first = read demos/blackjack-web/main.blackjack.web.readme.md and main.blackjack.web.requirements.md
pull.then = inspect main.blackjack.web.review.md, source hashes, trace lineage and verification observations
verify = julia --startup-file=no -O0 -C generic --project=demos/web-evidence-lab demos/blackjack-web/main.blackjack.web.test.jl
ready.state = Query the live Warp and actual server; recorded completion does not start a process.
do.not.disturb = Local simulated chips; source review is not automatic whole-program proof or public deployment.
