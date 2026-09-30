artifact.type = warp
recur.gift = Initialize opinionated Lang policy through the existing companion conventions.
status = complete
warp.id = main.lang.init
warp.map = warps/main.lang.init.warp-map.json
pull.first = read warps/main.lang.init.contract.md and warps/main.lang.init.baseline.md; query recur warp show main.lang.init -d .
pull.then = run julia-tests/main.command.recur-lang.init.test.jl standalone against the selected candidate binary
verify = Cargo publication tests and Julia init suite; query live gates separately and preserve historical red observations
ready.state = implementation and focused tests present; use current verification evidence and full-root Warp projection

observed.state = complete
readiness.slice = none
evidence = warps/main.lang.init.verification.md; live merge retains declared gate policy
