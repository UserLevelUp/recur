artifact.type = lane
defines: main.command.watch.eventness.contract native Eventness subscriptions v1
consumer: demo.blackjack.skill.coordination.watch.topic earlier design motivation
consumer: recur.lang.advanced.watch optional future Lang readiness work

# Native Eventness subscriptions: initial contract

Warp: main.command.watch.eventness. Status: initial contract and tests; native
topic behavior is not implemented by this setup. This is a core Watch feature,
not a blackjack demo test or a new Lang grammar.

All of Warp binding, trace-ID signaling, Watch and Lang remain optional. Ordinary
Recur queries and coding do not require them. Hierarchy, skill guidance and trace
declarations remain loose discovery aids; native code implements and enforces the
explicitly selected behavior. No filename, empty marker or skill is executable
proof. Preserve untyped/unannotated files and existing first-use workflows.

Intelligence lives in project Eventness. Watch establishes interest and emits
trace-ID references. Eventness may consist of one file, several files, empty
markers and files containing descriptions, instructions, commands or evidence.
Watch does not execute that content. A wake-up establishes changed artifacts,
not successful tests, reviewed acceptance or authority to dispatch.

## Proposed companion CLI, frozen for the red suite

These commands are proposed v1 interfaces, not examples of shipped behavior:

```text
recur-watch topic create TOPIC [--warp WARP] --filter TRACE_PATTERN --eventness-dir DIR -d ROOT --confirm --json
recur-watch topic subscribe TOPIC --id SUBSCRIBER -d ROOT --confirm --json
recur-watch topic drain TOPIC --id SUBSCRIBER --max-events N -d ROOT --confirm --json
```

Create binds a topic to an explicit root-bounded Eventness directory and trace
pattern. An optional --warp binds the discovered Warp UUID; without it the topic
is project-scoped. Subscribe persists interest and its cursor;
registration does not launch an LLM, task, shell or background daemon. Drain is
a bounded reconciliation pass, usable by an asynchronous loop. The default
subscription includes matching existing publications so missed wake-ups can be
recovered. Repeating subscribe preserves its cursor; changed bindings are refused.
Unconfirmed writer operations preview without mutation.

Create/subscribe return lifecycle metadata. A successful drain writes a JSON
array of notifications; each notification has exactly `trace_ids`, a nonempty
array of distinct, valid hierarchical identifiers. No intelligence, report body,
stored command or conversation travels in that notification. Empty drain is `[]`.
Delivery/cursor bookkeeping belongs to private persisted state, not the payload.

## Publication and identity

Use configured producer trace roles. The fixtures use `publish:` and readable
dot-separated IDs. Warp/attempt/revision metadata is optional. When supplied,
`warp.id` and `warp.uuid` must agree with an explicitly bound topic; omission
inherits the topic's scope without inventing execution evidence. A minimal
producer declaration and body is sufficient. A content change can be discovered
without a revision field. One rich publication can associate root-bounded files with
`artifact.refs = ["relative/path", ...]`. Those referenced files may be empty or
contain arbitrary bytes. This explicit association is the first v1 input format;
do not require the same metadata or a JSON receipt in every file.

The observed publication fingerprint covers its producer file and referenced
artifact bytes. Publish files/attachments before signaling. Missing references,
traversal, symlinks escaping the root, invalid IDs and conflicting Warp bindings
must fail without advancing the subscriber cursor or emitting successful signals.
Foreign-Warp publications in the directory must not wake this topic.
Existing path subscriptions continue to work without trace IDs; trace filtering
is an explicit topic mode, not a universal requirement imposed on project files.

Deduplicate an unchanged publication across processes and restart. A new revision
under the same semantic trace ID can wake again. Preserve prior observations;
do not overwrite accepted Warp layers. Deduplication is bounded to notification
handling; this contract does not promise exactly-once external task effects.

Drain must limit notifications to N (1 through 1000), persist only acknowledged
observations and leave further publications discoverable. Reconcile from durable
artifacts rather than relying on transient filesystem notifications. Changes to
the subscription's own private status/cursor must not generate self-wake loops.
Failure to persist a cursor must not report a successful drain. Implementation
must freeze a crash-safe publication/cursor protocol and test interruption points
before accepting the recovery gate.

## Ownership and compatibility

Core `recur watch` remains a deterministic, read-only query. The companion owns
topics, subscriptions, reconciliation and runtime state. Opinionated host choices,
intelligence ticks, budgets and dispatch policy remain with init/recur-warp;
Watch never selects reasoning effort. Produced worker results and accepted gates
remain separate. Preserve legacy file subscriptions and polling dispatch.

Lang stays optional. Existing CIR1 fork/await can describe dependencies; no new
topic syntax or executable Lang semantics is included in this Warp. The older
main.lang.advanced.watch plan remains separate and may consume the native results.

## Acceptance and boundaries

Initial: review this contract, validate scaffolding and observe the legacy control.
Tests: preserve native expected-red results, exact binaries and the first blocker;
tests that stop at missing CLI setup do not prove downstream semantics.
Implementation: all native conformance cases pass with nonzero eligible tests.
Integration: actual bounded coordinator loop, interruption/cursor failure injection,
backlog draining, pure-query no-write checks and affected Rust/Julia regressions.
Final: docs/discovery, source-bound checked evidence and separate parent review.

No remote broker, arbitrary content execution, automatic provider fallback, general
Lang extension or blackjack changes are included. The intended success criterion
is retained useful work and bounded intelligence overhead, not parallel speed.
