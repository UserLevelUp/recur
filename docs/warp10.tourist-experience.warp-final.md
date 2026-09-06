# Warp 10 Final: A Sharp Tourist Loop

> Historical target for another solution. Administratively closed in Recur;
> not implemented or runtime-verified here. See warp10.tourist-experience.recur-closeout.complete.md.

## Final player experience

Warp 10 is complete when the map, site, conversation, and idea systems feel
like one intentional journey rather than separate prototypes or a sequence of
screens to dismiss.

The player starts on a legible map.  Empty map space advertises direct
manipulation with an open-hand cursor; dragging changes it to a closed hand and
translates the viewport.  Labels, inputs, markers, and other controls retain
their own cursor and interaction.  Background panning never selects text,
moves a marker, or steals a control click.

Selecting a site moves the player from the persistent map scene to the
persistent site scene through a short, coherent transition.  Returning reverses
that relationship and restores map position, zoom, selected site, and input
focus.  There are no unnecessary destination scenes, black frames, loading
flashes, camera jumps, or stacked navigation states.  Interaction remains in
the world or anchored to its person, marker, or site whenever possible.
Settings, inventory, help, pause, and explicit collection views may use
focused panels, but panels are exceptional tools rather than the player's
route through the experience.

At a site, the player approaches a visible person and begins a responsive,
contextual conversation without feeling transported into a separate
application screen.  Speaker identity, readable pacing, advance/skip behavior,
choices, and exit behavior are clear.  Choices and prior conversations can
change what the person says.  When a conversation discovers or refines an
idea, presentation grows naturally from that exchange, gives immediate
acquisition feedback, persists the idea, and shows what it enables.  Returning
to site exploration restores control without a dead click, focus error, or
panel-closing obstacle course.

## Product boundary

The durable navigation model has exactly two player destinations:

```text
Map scene <---- reversible transition ----> Site scene
    |                                         |
    +-- spatial map interaction               +-- people and contextual interaction
```

Different sites and people are data/resources loaded into those destinations;
they do not require a new top-level scene-management concept.  A site may use
internal child scenes for authored composition, but those are not independent
navigation destinations.  The presentation hierarchy and reliability budgets
are defined by `warp10.journey-reliability.sre-contract.md`.

## Required persistent state

- active destination and selected site;
- map translation, zoom, selection, and return focus;
- transition state with duplicate navigation guarded;
- person identity, met state, remembered topics, relationship or conversation
  flags, and exhausted/repeatable branches;
- idea identity, source person, discovery/refinement state, player-facing
  description, and unlocked consequence;
- save-schema version and migration behavior for the above state.

## Quality contract

- Mouse, keyboard, controller, trackpad, and applicable touch paths do not
  compete for ownership of the same gesture.
- Transition timing is brief and consistent, input is guarded while state is
  changing, and reduced-motion behavior remains fully understandable.
- Dialogue remains usable with fast reveal, skip/advance, choice navigation,
  readable focus, conversation history where appropriate, and safe exit.
- AI-backed dialogue, if used, has bounded latency, safe authored fallback,
  and never fabricates acquisition state outside the conversation contract.
- The map-to-site-to-conversation-to-idea-to-map loop is exercised in the real
  Godot runtime with no new debugger errors.
- Journey traces and human observation show no stuck input, orphaned or stacked
  panels, duplicate idea grants, lost spatial context, or ambiguous return path.
- Technical success does not overrule repeated player hesitation or confusion;
  those are observable reliability failures.

## Final acceptance receipt

One end-to-end receipt must bind the exact source revision and record:

1. map background cursor and drag-to-pan behavior;
2. exclusion of labels, inputs, markers, and interactive controls from
   background drag;
3. forward and reverse scene transitions with state restoration;
4. one complete person conversation containing a meaningful choice;
5. one idea discovery or refinement, persistence across save/load, and visible
   player payoff;
6. keyboard/controller focus and reduced-motion observations;
7. journey timing, panel-depth, recovery, and state-restoration observations;
   and
8. debugger output plus remaining known limitations.

Passing static tests, a video without its source revision, or the existence of
an idea record alone is not sufficient proof of this final Eventness.
