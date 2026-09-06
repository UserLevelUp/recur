> Archived: misplaced tourist-game plan. Closed administratively in Recur on 2026-09-05; not implemented or runtime-verified here. Original acceptance criteria below are historical, not satisfied.

# Warp 10 Sub-6: Journey Reliability, Polish, and Accessibility

## Scope

Unify the two scenes into one intentional, observable journey after their
interaction contracts are stable.  Apply the SRE contract to continuity,
failure recovery, latency, focus, presentation depth, and player orientation;
polish the surfaces that remain necessary.

## Required behavior

- Consistent typography, spacing, hierarchy, color, focus rings, buttons,
  prompts, overlay treatment, animation timing, and audio feedback.
- Clear hover, pressed, selected, disabled, loading, acquired, and error states.
- Responsive layouts avoid clipping and preserve readable dialogue and map
  controls at supported sizes.
- Keyboard/controller focus order, contrast, scalable text, reduced motion,
  subtitle/dialogue pacing, and input remapping follow the product's declared
  accessibility support.
- UI polish does not create additional navigation destinations or duplicate
  underlying state.
- The presentation hierarchy prefers world response and anchored context over
  HUD feedback, panels, and full-screen modals in that order.
- Journey states and transitions emit privacy-safe development/playtest facts
  sufficient to diagnose hesitation, duplicate input, focus loss, slow
  transitions, fallback use, and failed restoration.
- Transition, input-lock, content-loading, generated-dialogue, and save failures
  have bounded recovery paths that return the player to a safe spatial state.
- Numeric journey SLOs are measured on declared builds and device classes;
  meeting them cannot override consistent observed player confusion.

## Acceptance evidence

- Component/state inventory and removed inconsistencies.
- Representative visual captures at supported layouts and input modes.
- Runtime accessibility pass with focus, scaling, contrast, reduced motion,
  and dialogue readability observations.
- SRE journey receipt covering transition usability, panel depth, input-lock
  recovery, fallback behavior, state restoration, and relevant frame stalls.
- Human observation showing that the route feels continuous and understandable
  rather than like screens appearing in succession.
