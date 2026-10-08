artifact.type = lane
publish: main.command.watch.eventness.initial initial scope and work order
consumer: main.command.watch.eventness.contract frozen companion boundary
trigger: main.command.watch.eventness.tests explicitly selected conformance suite

Implement a native companion boundary for topics/subscriptions described by
Eventness, with trace-ID-only notifications and durable reconciliation. The
coordinator follows signatures to relevant artifacts and checks existing gates.
Keep CLI verification deterministic; spend model reasoning on ambiguous contracts,
implementation and independent review. Preserve private dispatch policy and prior
Warps. Initial setup is deliberately separate from feature completion.

The initial contract defines a bounded drain command for testable subscriptions.
A long-running coordinator may call it asynchronously. Freeze interruption
handling and notification/cursor persistence before implementing that runtime.
Remaining integration cases include cursor write failure, interruption after
publication/before acknowledgement, bounded backlog, subscription self-updates,
scope rejection, pure-query snapshots and legacy dispatch compatibility.

Run the passing setup suite with:
`node --test warps/watch-eventness/main.command.watch.eventness.scaffold.test.cjs`

Run the intentionally red native suite explicitly with:
`node --test warps/watch-eventness/main.command.watch.eventness.native.red.test.cjs`

RECUR_BIN and RECUR_WATCH_BIN may select exact executables. Otherwise the helper
uses target/release-safe; Node's standard library is the only harness dependency.
No default Cargo/Julia runner is modified by this initial setup. Native tests are
core feature tests, not optional demo tests. Implementation/integration must add
appropriate Rust/Julia coverage once the native protocol exists.
