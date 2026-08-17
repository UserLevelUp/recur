# Warp 10 Tourist Experience

## Reveal

Warp 10 turns the current tourist prototype into one sharp, continuous player
experience.  The product has two durable scenes--the map and the selected
site--with overlays for transient UI.  The player can directly manipulate the
map, enter and leave a site without losing context, talk with a person, and
leave the conversation with a visible, persistent idea.

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
`warp10.tourist-experience.warp-final.md`.  Completion means a player can:

1. orient on and directly pan the map;
2. select a site and transition into it cleanly;
3. interact with a person through responsive dialogue;
4. discover a meaningful, persistent idea with clear feedback; and
5. return to the same map position, zoom, and selection without a loading
   flash, accidental object movement, or navigation confusion.

## Slice order

1. `warp10.sub1.focus-group-reality-audit` -- current
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
- Preserve surprising or incomplete evidence as `strange`; do not silently
  promote partial work.

## Rehydrate

```powershell
recur reveal warp10
recur warp explain "warp10" -d docs
recur tree "warp10" -d docs
git status -sb
```

Then open the final contract and the one `.todo.current.md` slice.  Verify all
claims against the actual game workspace and playable build before advancing
Eventness.

defines: warp10.tourist-experience two-scene map site dialogue and idea-discovery player loop
defines: warp10.final receipt-backed sharp tourist experience with preserved navigation context
consumes: focus-group.observations incomplete mapping transitions dialogue and UI feedback
produces: warp10.runtime.acceptance playable end-to-end tourist loop receipt
