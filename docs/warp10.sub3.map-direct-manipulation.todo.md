# Warp 10 Sub-3: Map Direct Manipulation

## Scope

Make empty map space feel physically manipulable while preserving the distinct
behavior of labels, inputs, markers, and controls.

## Required behavior

- Empty map space uses an open-hand/grab cursor.
- Active background drag uses a closed-hand/grabbing cursor and translates the
  viewport.
- Release ends the gesture without selection, click-through, or node movement.
- Interactive descendants exclude their gestures from background panning.
- Mouse, trackpad, controller, and applicable touch behavior are explicit.
- Reset/fit behavior is discoverable, and normal UI updates do not reset the
  current map transform.

## Acceptance evidence

- Focused input and hit-testing checks.
- Runtime receipt covering empty space, every interactive exclusion class,
  drag threshold, viewport boundaries, and repeated gestures.
- Observation of cursor feedback and map responsiveness at representative
  window sizes.
