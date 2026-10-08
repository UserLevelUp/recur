artifact.type = lane
defines: main.command.watch.eventness.native durable optional Eventness topic implementation
consumer: main.command.watch.eventness.contract frozen native behavior

# Eventness topics

Use a topic when a coordinator needs a small signal that persisted work changed.
Intelligence, instructions, evidence and attachments stay in Eventness files.
Trace IDs, Watch, Warp bindings and Lang are optional. Ordinary files and existing
filesystem subscriptions need no conversion. This feature requires a binary whose
`recur-watch topic --help` exposes the commands below; version alone is insufficient.

Create an `eventness` directory under your project, then register interest:

```text
recur-watch topic create project.results --filter "task.**" --eventness-dir eventness -d ROOT --confirm --json
recur-watch topic subscribe project.results --id coordinator -d ROOT --confirm --json
```

Persist a file such as `eventness/task.one.complete.md`:

```text
publish: task.one.ready
The useful work, description, commands or instructions go here.
```

Reconcile it with a bounded drain:

```text
recur-watch topic drain project.results --id coordinator --max-events 10 -d ROOT --confirm --json
```

The response is exactly `[{"trace_ids":["task.one.ready"]}]`, or `[]` when nothing
new is available. It carries no intelligence or delivery metadata. One producer
may declare several distinct IDs with comma-separated IDs or several declaration
lines; that producer is one notification. Producer roles come from the existing
`traits.trace_id.producer_keywords` policy at topic creation (defaults include
`publish`, `send`, `emit`, `enqueue`). Declarations use `ROLE: ID[,ID...]` followed
by optional description. IDs are ASCII dot-separated nonempty words, matching the
configured hierarchy pattern. Arbitrary body text is inert and never executed.

Omitting `--confirm` on create, subscribe or drain previews without writing. A
preview drain reports lifecycle/count metadata, not a successful notification
array. `recur watch topics -d ROOT --json` is a pure inventory query. Registration
starts no daemon, worker or LLM. The caller chooses when to drain again; each call
reconciles durable files, including work written before subscription or while no
listener was running.

Optional `--warp WARP` binds the discovered bubble UUID. Producer `warp.id` and
`warp.uuid` fields may be omitted. Matching explicit metadata is checked; clearly
foreign work is ignored, while contradictory ID/UUID metadata fails the drain.
Attempt and revision fields are optional and do not prove execution. Topic
bindings and producer-role policy are immutable: changed bindings require another
topic identity. Repeated subscribe preserves the existing cursor.

Optional attachments use `artifact.refs = ["root/relative/path", ...]` on a producer.
Referenced files can be empty or arbitrary bytes. Publication identity covers the
producer path/bytes and reference paths/bytes; a changed body or attachment wakes
again without a revision field. Use immutable producer/attachment filenames when
historical content matters. Private journals preserve observations and signatures,
not old copies of mutable intelligence. Resolve announced IDs with `recur trace-id`
within the topic's Eventness directory, then read the files and their references.

## Commit and recovery protocol

Private state lives under `.recur/watch/topics/<SHA256(topic)>/`. Registration
publishes a synced temporary file without replacing an existing binding or journal.
A subscription has an append-only checksum journal and an OS advisory lock.
Confirmed drains take an exclusive lock; preview, repeated-subscribe preview and
replay take shared locks without creating or modifying state. Busy operations fail
with a retry message. Locks release when their process exits, including crashes.

Drain validates the entire bounded scope before choosing at most `--max-events`
unseen publications. Missing references, invalid declarations, escaped paths or
contradictory bindings cause no cursor advance. Endpoint rereads reject artifacts
that change while the snapshot is assembled. Writers should finish attachments
before publishing their producer, preferably by rename from a temporary file
outside the subscribed directory. This is cooperative filesystem coordination,
not a sandbox or a guarantee against hostile concurrent path replacement.

Each nonempty batch appends a length-bounded JSON line with sequence, binding,
observations and compact notifications, then syncs the file before output. An
unterminated final line is uncommitted; the next confirmed drain truncates it.
A malformed or checksum-invalid complete line fails closed. Failed write/sync
attempts roll back and sync the previous valid length; failed rollback reports
indeterminate persistence and requires inspection. Never infer acceptance from a
failed command or from a notification.

A process can die after commit but before stdout reaches its caller. Recover the
saved compact batch without changing the cursor:

```text
recur-watch topic replay project.results --id coordinator --sequence 1 -d ROOT --json
```

Sequence 1 is the first nonempty committed drain; subsequent batches increment it.
A preview drain reports `next_sequence`; subscribe reports `committed_batches`.
Replay does not repair a torn tail. The protocol covers process interruption and
local cooperative concurrency; it does not promise power-loss durability of
directory entries, exactly-once external side effects, or an acknowledged consumer
transport. The coordinator must make its own downstream processing idempotent.

Bounds: 1–1000 notifications per drain, 10,000 scanned entries, 16 MiB per file,
64 MiB total reads per reconciliation (including consistency rereads), 64 refs per
producer and 64 MiB journal. Exhaustion fails explicitly; narrow the directory or
use a fresh subscriber while preserving the old history. Paths must remain within
the explicit root and outside Watch private state. Eventness scope cannot contain
that state. Linked directories must be bound explicitly; linked files are checked
for containment. Producer paths must have valid UTF-8 names to avoid lossy identity.

## Coordinator boundary and verification

An application can perform a bounded loop: dispatch authorized workers, drain
signatures, resolve and review Eventness, then query live Warp gates. Produced work
does not become accepted solely because Watch announced it. Existing
`recur-watch dispatch` retains polling and its current outputs; it is not silently
converted into a topic-driven scheduler. No new Lang grammar is required.

This repository's Watch map declares `evidence_root = ".."` because it lives in
`warps/` while its checked source files live in the project. Inspect and complete
it with `-d` at the project root. That optional Warp metadata keeps inventory,
completion and refresh bound to the same real source files; it does not copy
source into a receipt folder or widen a narrower requested scope.

Core tests: `node --test warps/watch-eventness/main.command.watch.eventness.native.red.test.cjs`
and `julia-tests/main.command.watch.eventness.test.jl`. The integration suite also
requires an isolated `watch-test-hooks` build, selected by `WATCH_TEST_BIN`, to test
before-commit, partial-commit, write-failure and after-commit interruption points.
Those environment fault hooks are unavailable in normal builds. Tests cover a real
deterministic CLI worker, native reconciliation and produced-versus-accepted gates;
they do not establish live Gemini/Copilot/Codex provider authentication or cost.
