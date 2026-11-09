# Trace sets and contextual inspection

artifact.type = warp
defines: main.improvement.34.implementation.trace.context
consumer: main.improvement.34.implementation.trace.review
warp.id = main.improvement.34.implementation.trace.context
warp.uuid = 01a1bf18-a8d3-76ac-9d08-1eb4d211739d
warp.map = warps/improvement34/trace-context/main.improvement.34.implementation.trace.context.warp-map.json
parent.context = main.improvement.34.implementation
status = implemented; five checked slices locally accepted; re-query live evidence
pull.first = read warps/improvement34/trace-context/main.improvement.34.implementation.trace.context.final.observed.complete.md
pull.then = query live Warp progress from the repository root
ready.state = trace repair, inventory, selection and context verified; graph and coordinator acceptance remain separate
