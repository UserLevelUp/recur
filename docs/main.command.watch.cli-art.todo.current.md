# main.command.watch.cli-art.todo.current

Status: `current`
Date: 2026-04-25
Lane: `recur-watch` (binary extracted in commit 2d77f2d)
Parent doc: `README.CORE.IMPROVEMENT23.md` (ADDENDUM section)

## Idea

Add a `--format art` mode to `recur-watch` that renders the polling loop
as a small live terminal face — countdown to next poll, last-event age,
filtered event count, rolling tick spinner.

The premise is thematic.  `recur-watch` is a tick-driven loop with three
observable quantities; exposing those quantities directly is more legible
than a stream of timestamped lines for the human-in-a-tab use case.

## Sketch

```text
recur-watch  .recur/docs-monkey   filter: monkey.**
   clock 03   next poll                    framing: 5s
   tick spin                                mode: poll
   ---------------------------------------------------
   t-12s  modify  monkey.section-2.response.md
   t-47s  create  monkey.section-1.response.md
   t-2m   create  coord.section-1.instruction.md
   ---------------------------------------------------
   events: 3   filtered-out: 7   uptime: 4m 12s
```

## Constraints (load-bearing)

- `--format art` is opt-in only.  Default `oneline` and `json` modes stay
  pipe-safe.  No escape codes in non-art modes.
- Renderer-only.  The same internal event stream feeds all three formats.
- Multi-filter / multi-panel visualization is a separate `--lanes` or
  `--multi` consideration, NOT a `recur merge` invocation.  Merge operates
  on data shape, not visual composition.

## Non-goals

- NOT a TUI dashboard with mouse, scrollback, or input handling.
- NOT a daemon mode.  Same process-lifetime contract as the other formats.
- NOT a replacement for log capture.  Use `--format json` for that.

## Discovery

This file is the durable rediscovery anchor.  Find it via:

```bash
recur files "**.todo.current" -d docs/
recur files "main.command.watch.**" -d docs/
recur find "cli-art" --scope "main.command.watch.**" -d docs/
```

## Collapse condition

When `--format art` ships, rename this file to
`main.command.watch.cli-art.complete.md` (or `.resolved.md`) and remove
the ADDENDUM block from `README.CORE.IMPROVEMENT23.md` or replace it with
a one-line "shipped in vX.Y.Z" pointer.
