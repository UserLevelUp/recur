---
name: recur-watch
description: Inspect Recur watcher state or coordinate asynchronous Warp workers through recur-watch polling and bounded recovery. Use for Recur subscriptions and dispatch, not recurring Codex reminders.
---

# Recur Watch

Resolve the project root and distinguish the requested mode. Core `recur watch`
reads status records and exits. The separate `recur-watch` companion owns active
subscriptions and coordinator polling. A status file or configured host is not
proof of a running or authenticated process. Check current help and inspect the
recorded PID, timestamps, ACK/NAK and actual outputs when liveness matters.

For filesystem subscriptions in this repository, consult
`docs/main.command.watch.readme.md`. Use an explicit root, filter, subscription
ID and appropriate polling interval; retain unrelated existing watchers.
`recur watch list/status/explain` inspects state without arming a listener.
Do not confuse subscription events with test results or gate acceptance.

For Warp coordination, consult `docs/main.command.warp.dispatch.readme.md` and
the selected map/contract. Preview with `recur-watch dispatch WARP -d ROOT`.
Within authorized delegation, add `--confirm` and optionally `--cycles N`.
The coordinator polls live dependency gates and asynchronous attempt records;
filesystem-event-triggered dispatch is not the implemented coordinator mode.
`--cycles` bounds scheduler passes and does not terminate assigned workers.

Assignments need explicit host argv, root-bounded context/inputs and scoped
verification commands. Simultaneous writable workspaces must be disjoint;
that condition is not a sandbox. Respect global and host parallel limits.
Select a demo's checks only when that demo is selected for the task. A CLI-only
check worker is useful when the work needs deterministic execution rather than
an agent. Avoid shell-built commands and unnecessary model invocations.

Read terminal attempts through `recur warp dispatch WARP -d ROOT --json` and
their observations under `.recur/dispatch/`. `produced` is successful configured
verification, not automatic gate acceptance. Coordinator review and companion
completion publication unlock dependent slices. Quiescence can also mean blocked
or exhausted work: inspect live gates rather than equating idle with complete.

On failure, distinguish test failures from provider/authentication, launch,
timeout and input drift errors. Bounded feedback can request reasoning changes;
it cannot fix a missing executable or invalid model. Preserve failure evidence.
Inspect `recur-warp recover --help` for explicit interrupted-attempt recovery.
Do not delete active locks, kill unrelated processes or retry beyond policy.
