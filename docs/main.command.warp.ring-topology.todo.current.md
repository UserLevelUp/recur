# Warp Ring Topology

Status: `todo.current`
Target: Recur v0.2.8 follow-on schema slice

## Goal

Extend the flat `warp-bubble-map-v1` model into a recursively renderable bubble:

```text
outer coordinator/orchestrator ring
  -> inner specialized directory domains
     -> independently initialized local Recur realities
     -> explicit subscriptions and public completion boundaries
```

Each inner domain may run `recur init`, use its nearest `.recur/config.toml`,
maintain local reveal/Eventness/watch/Warp state, and host any bounded human,
script, monkey, bird-brain agent, specialist model, or sub-orchestrator. The
outer ring consumes only declared public contracts and receipts.

## First schema questions

- Freeze domain identity, relative root, child Warp identity, public contract
  hash, required child state, and parent acceptance Slice.
- Freeze subscription identity, direction, endpoints, filter, event contract,
  freshness, and ACK/NAK projection.
- Use the `warp-ring-map-v1` imported-domain companion schema; do not overload
  `warp-bubble-map-v1` implicitly.
- Define recursive traversal, path-containment, cycle, and depth-budget rules.
- Preserve the distinction between child completion, child integration
  readiness, and parent integration acceptance.

## Frozen first test contract

`julia-tests/main.command.warp.ring-topology.test.jl` now freezes the first
coordinator/worker boundary before runtime composition changes:

- the outer coordinator and each worker are independently initialized Recur
  domains with their own nearest `.recur/config.toml`;
- `recur-watch` subscriptions carry a task from coordinator to worker and a
  completion receipt from worker to coordinator asynchronously;
- `recur watch status` queries the ACK Eventness at the domain that owns each
  subscription endpoint;
- child required state and parent acceptance remain separate schema facts; and
- `recur warp merge` consumption of the companion map remains explicitly red.

The first committed fixtures are:

```text
julia-tests/fixtures/warp-ring-v1/complete/coordinator.release.warp-ring.json
julia-tests/fixtures/warp-ring-v1/missing-acceptance/coordinator.release.warp-ring.json
```

## Verification state

The schema and asynchronous harness are verified by
`main.command.warp.ring-topology.test.complete.md`: 29 assertions pass and the
single `recur warp merge` companion-map assertion remains intentionally broken
until the query implementation consumes `warp-ring-map-v1`.

## Red-first fixtures

1. Outer ring with two complete child domains and accepted integration Slices.
2. Child complete but parent acceptance missing.
3. Child public contract hash changed after prior acceptance.
4. Child blocked while unrelated sibling completes.
5. Stale child-to-parent subscription.
6. Recursive domain cycle.
7. Domain root escaping the parent workspace.
8. Three nested levels with bounded projection depth.

## First pull

```powershell
recur tree "main.command.warp" -d docs/
recur find "Ring topology" --scope "README.CORE.IMPROVEMENT27.Appendum" -d . -C 3
recur files "main.command.warp.ring-topology.**" -d docs/
recur warp status main.command.warp.ring-topology -d docs/ --json
```

## Trace-id lines

```text
defines: main.command.warp.ring-topology.todo.current recursive coordinator and specialist-domain bubble schema slice
consumes: recur.warp.ring.topology outer coordinator ring and recursive initialized domain model
consumes: recur.warp.subscription.edge explicit directional inter-domain event contract
produces: recur.warp.ring.schema frozen domain subscription boundary traversal and projection contract
triggers: recur.warp.ring.fixtures red-first complete missing-acceptance stale-hash blocked-subdomain stale-subscription cycle escape and depth cases
consumes: main.command.watch.query.impl pure ACK and NAK subscription-state query
consumes: main.command.watch.impl active asynchronous task and completion event runner
consumes: main.command.warp.ring-topology.test.complete first schema and async harness verification receipt
```
