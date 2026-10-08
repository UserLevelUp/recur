artifact.type = lane
publish: demo.blackjack.skill.coordination.watch.topic proposed Eventness subscription contract
consumer: demo.blackjack.skill.coordination.eventness persisted intelligence
consumer: demo.blackjack.skill.flow optional Lang dependencies

User refinement: a Watch itself can be represented in Eventness and trace IDs, especially within a Warp and alongside Lang. Watch establishes or subscribes to a named topic; publication of relevant Eventness triggers the subscription. Intelligence stays in the discoverable artifacts, so notification need not transfer its contents. A subscription declaration, its state and receipt can each be one file or several files, including empty presence markers where appropriate.

Proposed topic identity: demo.blackjack.skill.replay.ready. Bind it to this Warp bubble and to the replay lane's produced receipt, separately from its reviewed acceptance. A subscriber follows the published trace ID(s), reads the relevant current artifacts and applies the existing contract/gates. Subscription declarations describe interest; a declaration alone does not prove a running Watch or completed work. A file edit is not automatically worker completion or gate acceptance.

Current implementation boundary: recur-watch --filter subscribes to hierarchical file paths and persists runtime status when --id is supplied. Its emitted event identifies the changed path and event type. The private Warp adapter persists substantive receipts before emitting trace_ids. There is no demonstrated native trace-ID topic creation command or Lang topic/subscription syntax yet. The existing CIR1 fork/await graph describes dependencies; it does not execute a topic broker. Do not introduce invented executable syntax into that frozen, tested graph.

Next implementation should keep topic registration/subscription scoped by Warp identity, use existing configured trace declaration roles to resolve artifacts, make repeated or recovered notifications harmless, and recover current Eventness after a missed wake-up. It should avoid triggering on its own subscription/status updates. Companion/init configuration owns opinionated dispatch and intelligence policy. Watch only supplies interest, readiness and references; it does not interpret stored instructions or select reasoning levels.
