# Warp 10 Tourist Experience

Status: closed in Recur — misplaced external-project plan, not implemented here.
warp.id = warp10.tourist-experience
warp.root = docs
observed.state = complete
readiness.slice = none
goals.now = administrative closure only; no tourist-game implementation is claimed.
pull.first = read docs/warp10.tourist-experience.recur-closeout.complete.md

The text below is the archived game brief, not current Recur work. The original
seven-slice map is retained as warp10.tourist-experience.original-map.reference.json;
slice documents are now .reference.md. Only administrative disposition is accepted.

## Reveal

Warp 10 turns the current tourist prototype into one sharp, continuous player
experience.  The product has two durable scenes--the map and the selected
site--and prefers spatial, contextual interaction over screens and overlay
chains.  The player can directly manipulate the map, enter and leave a site
without losing context, talk with a person, and leave the conversation with a
visible, persistent idea.

## Current reality

- The mapping experience has improved, but empty-map drag/pan and cursor
  feedback are still incomplete.
- Scene changes and UI transitions feel clunky and appear broader than the two
  player destinations require.
- Talking to a person and gaining ideas has not received the corresponding
  interaction, persistence, and payoff improvements.
- Focus-group requests have not yet been reconciled against the playable build
  and source tree.
- This Recur workspace currently contains the coordination artifacts but no
  discoverable Godot `.tscn`, `.gd`, or `project.godot` files.  Runtime work
  must not be claimed complete until the owning game workspace/build is bound
  to this Warp.

## Desired final Eventness

The Warp 10 final is defined in
`warp10.tourist-experience.warp-final.md`, with its operational perspective in
`warp10.journey-reliability.sre-contract.md`.  Completion means a player can:

1. orient on and directly pan the map;
2. select a site and transition into it cleanly;
3. interact with a person through responsive dialogue;
4. discover a meaningful, persistent idea with clear feedback; and
5. return to the same map position, zoom, and selection without a loading
   flash, accidental object movement, or navigation confusion.

## Slice order

1. `warp10.sub1.focus-group-reality-audit` -- archived, not implemented here
2. `warp10.sub2.two-scene-foundation`
3. `warp10.sub3.map-direct-manipulation`
4. `warp10.sub4.map-site-transitions`
5. `warp10.sub5.conversation-idea-discovery`
6. `warp10.sub6.interface-polish-accessibility`
7. `warp10.sub7.playable-runtime-acceptance`

Only one mutating slice is current under the configured single-thread policy.
Later slices may be refined after Sub-1 identifies the real scenes, scripts,
autoloads, input actions, focus-group evidence, and playable baseline.

## Eventness rules

- Filenames record coordination state; receipts establish reality.
- Every completion must cite its source revision, changed files, verification
  commands, results, runtime observations, residuals, and limitations.
- Static or structural checks cannot substitute for the required Godot runtime
  receipt.
- Dialogue completion requires both system behavior and authored playable
  content; an empty branching framework is not the player experience.
- A sequence of functioning panels is not equivalent to a reliable journey;
  hesitation, lost context, overlay stacking, and unclear return paths remain
  unfinished player-facing Eventness.
- Preserve surprising or incomplete evidence as `strange`; do not silently
  promote partial work.

## Rehydrate

```powershell
recur reveal warp10
recur warp explain "warp10" -d docs
recur tree "warp10" -d docs
git status -sb
```

To resume game work elsewhere, read the original-map reference, final contract
and archived slice references. There is no current game slice in Recur. Verify
claims in the owning game workspace before accepting any original game gate.

defines: warp10.tourist-experience two-scene map site dialogue and idea-discovery player loop
defines: warp10.final receipt-backed sharp tourist experience with preserved navigation context
defines: warp10.journey-reliability SRE contract for continuity recovery observability and player orientation
consumes: focus-group.observations incomplete mapping transitions dialogue and UI feedback
produces: warp10.runtime.acceptance playable end-to-end tourist loop receipt
