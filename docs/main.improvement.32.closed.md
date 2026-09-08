# Improvement 32: Agent Discovery — Closed in Favor of Reveal

Status: `closed` (superseded proposal; not implemented)
Date: 2026-09-07

Decision: handle agent and skill discovery through the shared reveal route rather
than separate core improvements. No standalone `recur agent` command or Warp
should be scheduled from Improvement 32.

Retain hierarchical artifacts, scoped private `.recur/` contexts, depth/reporting
boundaries and the distinction between child completion and parent acceptance as
design history. Shared typed reveal remains proposed. Describing an agent does
not launch it; opinionated execution remains outside the core query engine.

Historical design: [README.CORE.IMPROVEMENT32.md](../README.CORE.IMPROVEMENT32.md).
Related closure: [Improvement 31 / skills](main.improvement.31.closed.md).
Follow-on direction: [Improvement 29 / reveal](../README.CORE.IMPROVEMENT29.md),
coordinated with [the persona/skill proposal](main.command.reveal.persona-skills.readme.md).
These references do not amend existing Warp contracts automatically. Decide
the combined scope and implementation later, after current Warp and Lang work.

This record closes a proposal; it does not claim implementation, passing tests
or acceptance of a new feature. No active cursor or Warp receipt is created.

consumes: recur.agent.discovery superseded standalone query proposal
consumes: recur.agent.coordination historical hierarchical coordination design
