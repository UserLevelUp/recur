# Opt-in hierarchy segment-depth bounding for recur tree

artifact.type = warp
defines: main.command.tree.hierarchy-depth
consumer: main.command.tree
warp.id = main.command.tree.hierarchy-depth
warp.uuid = 01a1cee4-c2ca-781d-b410-d51cf2022c65
warp.map = warps/main.command.tree.hierarchy-depth.warp-map.json
origin.project = C:/src/kicad_ra
status = planned; slice-0 ready; no implementation
pull.first = read warps/main.command.tree.hierarchy-depth.contract.md
pull.then = recur warp show main.command.tree.hierarchy-depth -d .
verify = cargo test --locked --test tree_configured_lanes && julia julia-tests/runtests.tree.jl && julia julia-tests/main.command.tree.wildcard-current.test.jl
ready.state = backwards-compatible opt-in --levels / --tree-depth contract locked (--depth filesystem walk preserved); ready for slice-0 red baseline
