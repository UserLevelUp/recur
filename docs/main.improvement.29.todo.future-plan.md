# Improvement 29: recur-reveal next Orientation Packet

Status: `todo.future-plan` (proposal / future direction)
Date: 2026-07-03

## Objective

Keep the `recur-reveal next` idea visible in the eventness tree and preserve
the boundary: core `recur reveal` stays a pure capsule reader, while the
`recur-reveal` executable owns the higher-level next packet and any optional
command execution.

## Canonical Proposal

- `README.CORE.IMPROVEMENT29.md`

## Current Posture

- `recur-reveal next` is the proposed command shape.
- It should reveal the next coherent orientation packet for the current
  workspace, domain, or lane.
- By default it should not execute `pull.first`, `verify`, shell commands, file
  edits, approvals, watchers, or commits.
- Optional execution should stay on the same `next` command behind explicit
  flags such as `--run verify --confirm`.

## Command Boundary

```text
recur reveal        = pure capsule listing/showing in core recur
recur-reveal next   = orientation packet plus optional ACK/NAK execution
```

Core `recur reveal` should inspect and explain reveal capsules. The
`recur-reveal` executable should own next-packet composition and any
operator-confirmed command execution, leaving state under `.recur/reveal/` for
core recur to inspect later.

## Packet Goal

A good packet answers:

```text
What should I read now, what should I ignore, and what is the first safe pull?
```

Expected packet fields include:

- persona / agent role;
- north-star;
- active index / focus gate;
- current next action;
- small `read_now` set;
- paused lanes excluded from planning;
- first pull command as text only;
- verification command as text only;
- trusted receipts;
- short trace-id relationship edges.

## Discovery

```powershell
recur files "main.improvement.29.**" -d docs/
recur files "README.CORE.IMPROVEMENT29" -d ./
recur files "main.command.reveal.**" -d docs/
recur capability explain reveal -d .
```

## Related

- `README.CORE.IMPROVEMENT29.md`
- `README.CORE.IMPROVEMENT22.md`
- `README.CORE.IMPROVEMENT27.md`
- `README.CORE.IMPROVEMENT28.md`
- `docs/main.command.reveal.readme.md`
- `docs/main.recur.purity.decision.md`
- `docs/main.command.version.readme.md`

## Trace-Id Lines

```text
defines: main.improvement.29.todo future-plan bridge for recur-reveal next orientation packet
defines: recur.reveal.core-boundary pure capsule listing and showing remains in core recur reveal
defines: recur-reveal.next companion command for next focus packet and confirmation-gated ACK/NAK records
consumes: README.CORE.IMPROVEMENT29 canonical reveal-next proposal
consumes: main.recur.purity.decision core recur pure query and companion actor split
produces: main.improvement.29.discovery recur files queries for future implementation work
triggers: main.improvement.29.contract packet schema fixtures and reveal-next CLI behavior tests
```
