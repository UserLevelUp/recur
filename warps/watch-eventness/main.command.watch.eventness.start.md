artifact.type = lane
consumer: main.command.watch.eventness.initial first-use orientation
consumer: main.command.watch.eventness.contract opt-in native behavior

# Start here

Keep coding normally. Use only the Recur features that help this task. Trace IDs,
Watch, Warps and Lang are optional; existing files need no wholesale renaming,
metadata conversion or language translation. Skills explain workflows; code and
observed tests establish what is actually implemented.

For this repository's next Watch Warp, inspect the current state:

```text
recur warp show main.command.watch.eventness -d . --json
recur warp slices main.command.watch.eventness -d . --json
```

Read the contract and the latest baseline under observations/. The topic commands
in the contract are proposed and expected-red today. Existing `recur watch list`
is a query; `recur-watch --filter PATTERN -d ROOT` is the active file runner.
Use each executable's --help before running it. No subscription is started by
this initial setup.

For an agent: load recur-expert for orientation, recur-eventness for scoped
rehydration, recur-watch for subscriptions, recur-warp for this work order, and
recur-trace-id when lineage is useful. Load recur-lang only for actual Lang work.
Do not load every skill or ingest every descendant artifact. Reveal returns
pointers; it does not load skills or grant execution authority. Start with the
slice's contract, current source, tests and blocking findings, then expand as needed.

The first native example should work with a short producer declaration and body.
Additional metadata and explicit attachment references are useful when a task
needs stronger lineage. An empty file remains a valid attached marker. Existing
file subscriptions remain available when no trace IDs are needed.
