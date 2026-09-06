# Warp 10 Journey Reliability Contract

> Archived external-project requirements, not satisfied implementation claims.
> Warp 10 is administratively closed in Recur; see its recur-closeout.complete.md record.

## SRE perspective

Warp 10 treats the player journey as a stateful, observable service.  Its
reliability target is not merely that every button opens the requested panel;
it is that the player preserves orientation, intent, control, and progress
through the complete experience.

The interaction model must not degrade into a technically functioning chain
of screens.  Every modal change imposes cognitive load and introduces failure
modes: lost focus, stale state, stacked overlays, accidental dismissal,
duplicate actions, obscured world context, and uncertainty about how to
continue.  The preferred presentation is therefore spatial, contextual, and
progressively disclosed inside the map or site.

## Journey state model

```text
MapExplore
  -> SiteApproach
    -> SiteExplore
      -> PersonApproach
        -> Conversation
          -> IdeaMoment
        -> SiteExplore
  -> MapReturn
-> MapExplore
```

These are interaction states within two durable scenes, not eight screens.
State changes should be communicated through camera movement, character or
marker response, contextual prompts, anchored dialogue, sound, and concise
feedback before considering a full-screen surface.

## Presentation hierarchy

Use the least disruptive surface that can communicate the required state:

1. response in the world--animation, movement, lighting, sound, or character;
2. contextual element anchored to the relevant person, site, or map marker;
3. small transient HUD feedback that dismisses itself safely;
4. one focused panel when comparison, reading, or explicit choice requires it;
5. full-screen modal only for true application-level interruption such as
   pause, accessibility, save recovery, or an explicit collection view.

Do not chain panels, open a second modal over a first, or make closing UI the
primary form of travel.  Dialogue should retain the person and site as visual
context wherever readability allows.  Idea acquisition should emerge from
the conversation and settle into the player's persistent state without
routing through several confirmation screens.

## Initial journey SLOs

The Sub-1 baseline may refine numeric budgets for the supported hardware, but
it may not remove the user-visible objective.

- Every accepted input produces visible or audible acknowledgement by the
  next rendered frame under the supported performance target.
- Normal map/site transitions have no blank frame and reach usable input state
  within the declared transition budget.
- Acceptance runs have zero stuck input locks, orphaned overlays, duplicate
  idea grants, lost map transforms, or unrecoverable focus states.
- At most one focused transient panel interrupts world interaction at a time.
- Closing or completing an interaction returns the player to a predictable
  spatial context in one action.
- Save/load reproduces the selected site, discovered ideas, person memory, and
  a safe resumable destination; it never resumes inside a half-committed
  transition.
- A failed optional content or generated-dialogue dependency falls back to an
  authored path without losing progress or trapping the player.

## Observability

Record structured, privacy-safe journey facts in development and approved
playtests:

- transition requested, started, usable, cancelled, timed out, and recovered;
- interaction state entered/exited and the previous/next state;
- input mode and focus owner changes;
- overlay/panel depth and orphan detection;
- conversation start, meaningful choice, exit reason, and fallback use;
- idea grant/refinement attempt, idempotent replay, persistence result, and
  visible acknowledgement;
- map transform capture and restoration result;
- frame stalls, resource-load failures, script errors, and recovery path.

Do not record private dialogue text, personal participant information, or raw
free-form input merely to obtain interaction metrics.  A receipt must bind
observations to source revision, build, device class, input mode, and test
route.

## Reliability mechanisms

- One explicit journey/navigation state machine owns transitions.
- State changes are idempotent and reject duplicate or impossible requests.
- Input locks have an owner, a bounded lifetime, and a recovery path.
- Panels register with one presentation owner; illegal stacking fails visibly
  in development rather than silently confusing the player.
- Idea grants use stable identities and commit once, independently from
  animation replay.
- Map context is captured before leaving and restored only after the map is
  ready to receive input.
- Loading and generated content have timeouts, authored fallbacks, and useful
  diagnostics.
- Save data records stable gameplay state, never an uncommitted animation or
  half-finished transition state.

## SRE review gate

The SRE review consumes runtime journey traces plus human observation.  It
asks where players hesitate, backtrack, close surfaces, lose context, repeat
inputs, or abandon the route.  Passing unit tests or meeting latency budgets
does not override consistent player confusion; that is a reliability failure
of the interaction contract and must remain visible Eventness.
