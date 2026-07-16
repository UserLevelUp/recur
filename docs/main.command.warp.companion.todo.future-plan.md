# `recur-warp` companion boundary

Status: `todo.future-plan`
Date: 2026-07-15

`recur warp status`, `recur warp explain`, and `recur warp next` are read-only
queries. Their suggested actions are evidence, never authority to change the
workspace.

`recur-warp` is a reserved companion name, not an executable in this release.
It may be implemented only for one user-requested, bounded writer after the
core status contract remains useful and fixture-backed.

## Required invocation contract

Any future companion invocation must include all of the following:

- an action selected from a documented, finite action set;
- the lane and evidence that justify that action;
- an explicit `--confirm` from the operator; and
- a default `--dry-run` that reports its proposed receipt without mutation.

It must reject arbitrary shell commands, broad glob-based mutation, implicit
approval from `warp next`, and silent background/watch execution.

## Receipt contract

Every non-dry-run attempt must write exactly one durable eventness receipt:

```text
.recur/warp/recur-warp.<id>.status.current.md
```

The receipt must state `ACK` or `NAK`, the requested action, confirmation
evidence, lane, input evidence references, intended output paths, observed
post-action evidence, and a timestamp. A `NAK` receipt is required for an
attempt that is refused after validation starts; no workspace mutation may
occur before confirmation is accepted.

## Entry conditions

Implement a companion only when all conditions hold:

1. A concrete core `next` action maps to one bounded writer.
2. Its read-only evidence and expected output have fixture coverage.
3. The operator requests that exact writer and supplies `--confirm`.
4. The writer can produce the receipt above and be tested for ACK and NAK.

Trace-id lines:

```text
defines: recur.warp.companion.boundary confirmation-gated future writer with durable ACK NAK eventness receipts
defines: recur.warp.companion.dry.run default non-mutating proposal mode for a bounded future warp action
consumes: recur.warp.next read-only suggested-action evidence that never grants mutation authority
produces: recur.warp.companion.receipt future durable ACK or NAK status record for each attempted confirmed action
triggers: main.improvement.27.command-boundary preserve core query and companion actor separation
```
