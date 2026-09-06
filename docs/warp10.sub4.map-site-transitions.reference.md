> Archived: misplaced tourist-game plan. Closed administratively in Recur on 2026-09-05; not implemented or runtime-verified here. Original acceptance criteria below are historical, not satisfied.

# Warp 10 Sub-4: Map/Site Transitions

## Scope

Create a short, reversible transition that visually and spatially connects the
selected map site to the playable site and back.

## Required behavior

- The selected marker/site is the semantic origin of the forward transition.
- Reverse navigation restores map translation, zoom, selected marker, and
  appropriate input focus.
- Conflicting input is guarded only while transition state changes.
- No black frame, loading flash, camera jump, double activation, or stale site
  content is visible.
- Reduced-motion mode replaces movement without removing state feedback.
- Transition duration and easing are centralized rather than scattered among
  scenes.

## Acceptance evidence

- Automated state and duplicate-request checks where practical.
- Runtime capture of forward, reverse, repeated, interrupted, reduced-motion,
  keyboard, mouse, and controller paths.
- Frame/debugger observations for loading and focus regressions.
