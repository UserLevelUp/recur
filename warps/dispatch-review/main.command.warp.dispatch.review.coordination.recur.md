artifact.type = lane
consumes: main.recur.reveal.type.lane typed work-context capsule
defines: main.command.warp.dispatch.review.coordination reviewed coordinator handoffs
consumer: main.command.warp.dispatch.review.copilot_review.attempt.11.response actual Copilot review assessed by parent
consumer: main.command.warp.dispatch.review.codex_independent.attempt.1.response independent Codex proposals integrated and verified by parent
consumer: main.command.warp.dispatch.review.codex_coordinator.attempt.1.response high-reasoning acceptance recommendation assessed by parent
pull.first = read observations/decisions.jsonl and the three actual review JSON files
pull.then = inspect handoffs.jsonl, per-attempt Eventness files and actual private attempt records
do.not.disturb = failed provider attempts remain blocked observations; completion suffixes describe capture rather than task acceptance

Trace pattern: main.command.warp.dispatch.review.<slice_token>.attempt.<number>.request
and the corresponding .response. Each request names the host and requested
reasoning. Each response preserves actual output or failure. Review decisions
are appended explicitly; a worker's produced state never substitutes for review.
Slice trace tokens replace hyphens with underscores, retaining the original
slice ID in the attempt metadata. The current trace parser truncates identifiers
at hyphens; canonical aliases preserve queryable lineage without rewriting raw
requests, responses or old observation events.

The live local view is http://127.0.0.1:8795/. It is a companion observation
prototype using file handoffs and polling. It is not a bidirectional agent bus.
Durable events, stale-owner recovery, transport negotiation and precise start/
first-response/review timing are retained as later Warp candidates.
