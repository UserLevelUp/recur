# recur watch

Status: `readme`
Date: 2026-05-11

`recur watch` is the pure watcher-state query surface.

It does not arm a filesystem listener, run a polling loop, or stream events.
The active runner is `recur-watch`. Core `recur watch` reads watcher eventness
left under `.recur/watch/` and exits.

## Command Split

```text
recur-watch  = active subscription runner; blocks, polls/streams, emits events
recur watch  = pure watcher-state query; reads eventness, reports, filters, exits
```

## Usage

```powershell
recur watch
recur watch list
recur watch list --filter "**.active"
recur watch status docs-monkey
recur watch status docs-monkey --json
recur watch explain
```

## State Files

`recur watch` reads files shaped like:

```text
.recur/watch/recur-watch.<id>.status.current.md
```

The first supported record format is simple key/value text:

```text
state = active
ack = accepted
nak_reason = ""
filter = monkey.**
dir = .recur/docs-monkey
mode = poll
poll_framing = 5
format = json
pid = 12345
started_at = 2026-05-11T00:00:00Z
last_event_at = 2026-05-11T00:00:12Z
events_seen = 12
filtered_out = 43
```

## Filtering

`list --filter` matches a virtual watcher hierarchy:

```text
<id>.<state>
```

Examples:

```powershell
recur watch list --filter "**.active"
recur watch list --filter "docs-monkey.**"
recur watch list --filter "**.stale"
```

## ACK/NAK

Watcher records carry both ACK and NAK information.

- ACK says what was accepted and is now active.
- NAK says what was understood, what was rejected, and why.

This lets `recur watch` answer both "what is running?" and "what failed to
arm?"

## Related

- `README.CORE.IMPROVEMENT23.md`
- `docs/main.recur.purity.decision.md`
- `docs/main.command.watch.cli-art.todo.current.md`
