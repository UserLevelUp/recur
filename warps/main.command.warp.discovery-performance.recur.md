# Cross-functional Warp discovery performance and evidence memoization

artifact.type = warp
defines: main.command.warp.discovery-performance
consumer: recur.warp.discovery.inventory
warp.id = main.command.warp.discovery-performance
warp.uuid = 01a1cf0b-9f99-7604-aca8-887f3b353673
warp.map = warps/main.command.warp.discovery-performance.warp-map.json
origin.project = C:/src/recur
status = complete; implementation and compatibility evidence observed
pull.first = read warps/main.command.warp.discovery-performance.verification.md
pull.then = recur warp show main.command.warp.discovery-performance -d .
verify = cargo test --locked && julia julia-tests/main.command.warp.discovery.test.jl
ready.state = baseline recorded; all implementation slices covered; re-query live evidence after source changes
