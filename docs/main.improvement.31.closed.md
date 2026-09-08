# Improvement 31: Skill Discovery — Closed in Favor of Reveal

Status: `closed` (superseded proposal; not implemented)
Date: 2026-09-07

Decision: use the reveal route for skills instead of a separate core skill
improvement. No standalone `recur skill` command or Warp should be scheduled from
Improvement 31.

Retain hierarchical artifacts such as `skill.recur.expert.recur.md`, discoverable
through ordinary `tree` queries. Shared reveal type recognition/presentation for
skills, personas and agents remains proposed; recognition does not activate them.
Opinionated content and actions remain the responsibility of companions.

Historical design: [README.CORE.IMPROVEMENT31.md](../README.CORE.IMPROVEMENT31.md).
Follow-on direction: [Improvement 29 / reveal](../README.CORE.IMPROVEMENT29.md),
coordinated with [the persona/skill proposal](main.command.reveal.persona-skills.readme.md).
Those references do not imply that existing Warp contracts now include every
typed-reveal idea; scope and implementation will be decided later.

[Improvement 32](main.improvement.32.closed.md) is also closed in favor of the
shared reveal route for agents and skills. This closure records the user's
design decision, not passing tests, implemented functionality or an acceptance receipt.

consumes: recur.skill.discovery superseded standalone query proposal
consumes: recur.skill.management historical companion design
