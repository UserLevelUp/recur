# Recur Lang Pathing Demo

Status: red-first design fixture

`main.lang.pathing.recur` specifies a deterministic Pac-Man-like path
generator. It is intentionally ahead of the current Recur Lang parsers and
runtime.

The demo exercises two different parallel forms:

```text
one MazeGraph
  -> distance + dead-end + symmetry + coverage scorers

one PowerPlan
  -> one route and breadcrumb branch per power node

one validated ScenarioDraft
  -> current route + shoe-technology progression route
  -> comparison -> score receipt -> next-map decision
```

The formal source separates:

- `header`: contracts, function ports, aliases, fan-out, scatter, and joins;
- `body`: algorithm bindings and non-executable memos;
- `footer`: static checks, reports, Eventness, and the desired Warp.

## First Tech-Tree Slice

The additive `tech_tree_slice` graph preserves the original Pac-Man generator
and freezes one small creator-to-score game loop:

1. `creator` validates image-region labels and publishes an immutable map.
2. `route_now` evaluates only capabilities already active on the runner.
3. `progression` researches and equips one affordable shoe technology.
4. `compare` chooses the open detour or the newly available glide route.
5. `score` binds component scores and the next-map decision to the map hash.

The first formal fixture is intentionally one map with two routes, one
glide-gated transition, and one shoe technology. Parsing the new contracts,
deriving its graph, and executing it through Recur Lang remain future
capabilities.

## Executable Layered Prototype

The second slice supplies a standalone Julia implementation for the future
`pathing.proto_map.generate` binding:

```powershell
julia --startup-file=no demos/pathing/proto_map.jl
julia --startup-file=no demos/pathing/proto_map.jl --width=41 --height=17 --research-cost=9
```

The checked-in image-backed fixture is [city-pass](maps/city-pass/):

```text
maps/city-pass/
  city-pass.ppm       140 x 60 bitmap, aligned at 4 x 4 pixels per cell
  map.manifest.toml   dimensions, image alignment, and glyph definitions
  topology.txt        city and lane topology
  terrain.txt         capability-gated terrain
  current-route.txt   baseline route overlay
  optimum-route.txt   researched-route overlay
```

Regenerate it deterministically after changing the generator:

```powershell
julia -O0 --startup-file=no demos/pathing/proto_map.jl --write-fixture=demos/pathing/maps/city-pass
```

`city-pass.ppm` is a plain portable bitmap. Its manifest proves that one ASCII
cell represents a $4 \times 4$ pixel image rectangle; the layer files, not the
bitmap, remain the authoritative pathing model.

It dynamically generates four independent ASCII layers:

- topology: `O` city or powerup, `.` one lane, `+` crossing, `=` two shared
  lanes, and `E` three shared lanes;
- terrain: `~` glide-gated ravine and `^` stepped terrain;
- current route: the baseline runner's city-to-city detour;
- optimum route: the shorter route after researching and equipping glide shoes.

The default map evaluates to a current-route cost of `38`, glide travel cost of
`22`, and glide travel plus research cost of `28`. Terrain negotiation remains
explicit: baseline stages record accepted `climb` decisions, while the optimum
route records accepted `glide` decisions.

Contract labels are sidecar anchors instead of terrain glyphs:

```text
i(a) player and active runner capabilities
i(b) map, terrain, and technology parameters
f(a) evaluate terrain against capability
f(b) compare current and researched routes
o(c) verified route and scoring input
```

Routes may share or cross intermediate tiles, but discovery starts and ends at
`O` cities. The generated network does not require a loop.

## Current Test Posture

Run the isolated executable specification:

```powershell
julia --startup-file=no julia-tests/main.lang.pathing.test.jl
julia --startup-file=no julia-tests/main.lang.pathing.proto-map.test.jl
```

Source-shape tests pass now. Unsupported parser, graph, local-view, and
execution contracts use Julia's `@test_broken` convention. They remain visible
roadmap debt without making the established regression suite falsely red.

When a capability lands, its `@test_broken` assertion should become a normal
`@test`. An unexpected pass is therefore a signal to rotate that contract
forward rather than leave it marked as missing.

## Intended Implementation Order

1. Parse the 0.3 contracts, reusable scopes, and graph block.
2. Freeze broadcast, execution-mode, scatter, qualified-instance, and join IR.
3. Produce static and one-function-local graph views.
4. Bind deterministic Julia pathing algorithms.
5. Let `recur-lang` coordinate execution only after the read-only contracts are
   stable.

## Discovery

```powershell
recur files "main.lang.pathing" -d demos/pathing/
recur trace-id "recur.lang.pathing.graph" --scope "**" -d .
recur trace-id "demo.pathing.tests" --scope "**" -d .
```

## Trace-Id Lines

```text
defines: demo.pathing red-first Recur Lang fan-out scatter join and graph fixture
defines: recur.lang.pathing.graph contract-addressed parallel Pac-Man-like path generation
defines: demo.pathing.tech.tree creator route progression score and unlock graph fixture
consumes: recur.lang.concurrent.ir.v1 exact typed message and await conventions
produces: demo.pathing.tests Julia executable specification for future parser graph and runtime behavior
produces: demo.pathing.tech.tree.score.receipt source-bound first-slice result
produces: demo.pathing.proto.map.layers executable topology terrain current-route and optimum-route evidence
triggers: recur.lang.static.graph.report.v1 directed and local graph projections
```
