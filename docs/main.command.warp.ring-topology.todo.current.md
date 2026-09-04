# Warp Ring Topology & Complete Warp Command Suite

Status: `todo.current`
Priority: `active primary priority — finish recur warp query suite and complete recur-warp companion app`
Target: Recur v0.2.8 follow-on schema slice and complete writer toolchain

## Goal

Finish the complete `recur warp` read-only query suite and the `recur-warp`
companion application so Warp can be fully utilized in real-world settings:

1. **`recur warp` (Pure Query Suite)**:
   - Extend `warp-bubble-map-v1` with the recursive `warp-ring-map-v1` topology:
     ```text
     outer coordinator/orchestrator ring (flagship LLM / human coordinator)
       -> inner specialized directory domains (small, cheap, hyper-focused models)
          -> independently initialized local Recur realities (.recur per subdirectory)
          -> explicit subscriptions and public completion boundaries via recur-watch
     ```
   - Enable heterogeneous model tiers: outer coordinator manages the macro
     converging bubble, while subdirectories host lightweight, fast specialist
     LLMs (or scripts/monkeys) bounded by local `.recur/config.toml`, local reveal
     capsules, and minimal context windows.
   - Closed-loop verification & evolutionary pivot:
     1. Coordinator authors/focuses on integration tests and test harness gates.
     2. Coordinator sics focused worker LLMs on bounded subdirectory Warp bubbles.
     3. Worker returns receipt; coordinator tests parent integration.
     4. On track: coordinator records accepted slice layer.
     5. Off track or broken assumption: coordinator triggers evolution ($W_0 \to W_1$)
        via `recur-warp evolve`, preserving valid slices, pruning stale ones, and
        prompting for human input only when an operator decision is truly required.
   - Consume companion maps in `recur warp merge`, `recur warp map`, and
     `recur warp status`.
   - Enforce bounded projection depth, path containment, and separate parent
     acceptance from child completion.

2. **`recur-warp` (Stateful Companion Writer Application)**:
   - Complete the full writer suite:
     - `recur-warp complete`: atomic Slice completion layer persistence (`--confirm`).
     - `recur-warp evolve`: confirmed Warp supersession ($W_0 \to W_1$) when a
       bubble explodes, carrying forward still-valid slices and retiring invalidated ones.
     - `recur-warp collapse`: confirmed execution of `recur warp collapse-plan` to safely
       collapse known eventness residue into `.complete` or archive.

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

## Active Implementation Roadmap

We are actively transitioning from frozen schema to implementation:

1. **Schema Binding in Rust Core (`src/warp_bubble.rs`)**:
   - Define `WARP_RING_MAP_SCHEMA = "warp-ring-map-v1"`.
   - Add deserialization structs: `WarpRingMap`, `WarpRingDomain`, `WarpRingParentAcceptance`, and `WarpRingSubscription`.
2. **Pure Ring Composition Engine (`src/main_command_warp_impl.rs`)**:
   - Detect `<warp>.warp-ring.json` companion files alongside or within `<warp>.warp-map.json` workflows.
   - Implement recursive domain inspection: resolve `relative_root`, verify child domain status against `required_state`, and check parent acceptance Slices.
   - Convert `recur.warp.ring.projection.red` into `recur.warp.ring.projection.green`.
3. **Execution & Boundary Safety**:
   - Guard against path traversal escaping workspace root and enforce `projection_depth`.
   - Wire `recur-warp evolve` contract hooks for handling exploded bubbles and superseding maps.

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
defines: recur.warp.commands.full-suite complete implementation of pure query and companion writer surfaces
consumes: recur.warp.ring.topology outer coordinator ring and recursive initialized domain model
consumes: recur.warp.subscription.edge explicit directional inter-domain event contract
produces: recur.warp.ring.schema frozen domain subscription boundary traversal and projection contract
produces: recur.warp.ring.projection.green pure recursive ring merge implementation
produces: recur.warp.ring.engine.impl rust deserialization and composition of warp-ring-map-v1
produces: recur.warp.companion.evolve confirmed supersession of exploded bubbles into evolved target maps
produces: recur.warp.companion.collapse confirmed execution of read-only collapse plans
triggers: recur.warp.ring.fixtures red-first complete missing-acceptance stale-hash blocked-subdomain stale-subscription cycle escape and depth cases
consumes: main.command.watch.query.impl pure ACK and NAK subscription-state query
consumes: main.command.watch.impl active asynchronous task and completion event runner
consumes: main.command.warp.ring-topology.test.complete first schema and async harness verification receipt
```
