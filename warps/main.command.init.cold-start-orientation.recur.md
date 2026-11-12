# Cold-start base-prefix discovery and Eventness orientation

artifact.type = warp
defines: main.command.init.cold-start-orientation
consumer: main.recur.skill.expert
warp.id = main.command.init.cold-start-orientation
warp.uuid = 01a1cee4-c2bf-7c28-bdef-408be1ca631e
warp.map = warps/main.command.init.cold-start-orientation.warp-map.json
origin.project = C:/src/kicad_ra
status = planned; slice-0 ready; no implementation
pull.first = read warps/main.command.init.cold-start-orientation.contract.md
pull.then = recur warp show main.command.init.cold-start-orientation -d .
verify = cargo test --locked --test reveal_profiles && julia julia-tests/runtests.init.jl
ready.state = backwards-compatible additive prefix discovery and cold-start Eventness/trace-id orientation contract locked; ready for slice-0 red baseline
