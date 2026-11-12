# Trace-ID budget-starvation diagnostics and opt-in directory exclusions

artifact.type = warp
defines: main.command.trace-id.scan-coverage
consumer: main.command.trace-id
warp.id = main.command.trace-id.scan-coverage
warp.uuid = 01a1cee4-c2e7-7d22-a307-014577204420
warp.map = warps/main.command.trace-id.scan-coverage.warp-map.json
origin.project = C:/src/kicad_ra
status = planned; slice-0 ready; no implementation
pull.first = read warps/main.command.trace-id.scan-coverage.contract.md
pull.then = recur warp show main.command.trace-id.scan-coverage -d .
verify = cargo test --locked && julia julia-tests/main.improvement.34.trace.test.jl && julia julia-tests/main.improvement.34.trace-context.test.jl
ready.state = additive coverage starvation diagnostics, opt-in --exclude-dir, and recur-trace-v1 guidance contract locked; ready for slice-0 red baseline
