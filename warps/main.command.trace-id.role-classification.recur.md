# Repair scoped recur trace-id role classification

artifact.type = warp
defines: main.command.trace-id.role-classification
consumer: main.command.trace-id
warp.id = main.command.trace-id.role-classification
warp.uuid = 01a1cee4-c2d3-7ced-b032-3c4da7ec4595
warp.map = warps/main.command.trace-id.role-classification.warp-map.json
origin.project = C:/src/kicad_ra
status = planned; slice-0 ready; no implementation
pull.first = read warps/main.command.trace-id.role-classification.contract.md
pull.then = recur warp show main.command.trace-id.role-classification -d .
verify = cargo test --locked && julia julia-tests/runtests.trace-id.jl && julia julia-tests/main.improvement.34.trace.test.jl
ready.state = backwards-compatible scoped role-classification repair contract locked (legacy JSON schemas and C# symbol resolution preserved); ready for slice-0 red baseline
