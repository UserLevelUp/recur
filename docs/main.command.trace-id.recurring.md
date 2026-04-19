# main.command.trace-id.recurring

Recurring rediscovery point for using `trace-id` as the handoff contract between lanes.

## Doctrine

**trace-id is the handoff contract between lanes.**

In the multi-lane peer-programming model (see `memory/multi_lane_coordination.md`):

- Each lane's output is a set of trace-id edges (`publish`, `subscribe`, `trigger`, `define`)
- Cross-lane synchronization happens through those edges, not through git merges or shared mutable state
- The coordinator (human or future agent) verifies the trace-id cascade before allowing merge
- A lane that publishes an edge is a green-light signal to subscribers
- A lane that subscribes stays red-lighted until its dependency publishes

## Relation To Ignition-Capsule Eventness

The `ignition-capsule` family (`.recur.md`, see `memory/eventness_conventions.md`) is how a lane wakes up and rediscovers its role. The trace-id edge family is how a lane hands off to other lanes.

Ignition is rehydration. `trace-id` is contract.

A live multi-lane session uses both:

- `recur reveal <lane>` fires the ignition capsule for one lane
- `recur trace-id` and related checks verify the publish/subscribe cascade across lanes

## When This Doctrine Applies

Use this doctrine when multiple lanes run in parallel and their outputs must converge.

It does not apply to single-lane work.
It does not apply to git-merge workflows.

## Recurring Reminder

If a future Skippy is tempted to coordinate lanes through git branches or mutable shared state, stop. The doctrine is trace-id contracts in-session, in-vault, on one box. See `memory/multi_lane_coordination.md` for the full multi-lane vision.
