artifact.type = lane
publish: demo.blackjack.skill.coordination.eventness signature transport contract
consumer: main.command.watch.dispatch readiness signaling
consumer: main.recur.eventness scoped intelligence rehydration

Worker intelligence belongs to typed Eventness artifacts inside the Warp: findings, reasoning summaries, implementation decisions, validation evidence and unresolved questions. Publish and persist the substantive receipt before notification. Watch carries a trace-id signature so the coordinator can discover readiness and rehydrate the relevant receipt; it does not become the knowledge store.

Eventness may occupy one file or multiple files. Empty files may serve as presence/state markers; other files may contain trace declarations, descriptions, Recur commands, instructions or evidence. A notification may identify any relevant set of these artifacts. Do not require every file to contain prose or force all material into one JSON receipt. Naming, hierarchy and configured artifact roles supply structure; gate acceptance separately requires the appropriate evidence. Stored commands/instructions remain material for the coordinator to interpret within the authorized workflow, not commands executed by Watch.

This trial publishes durable worker result leaves under eventness/ with signatures like demo.blackjack.skill.replay.attempt.1.result. The Warp slug remains metadata because the current identifier lexer does not accept hyphens. Earlier hyphenated aliases are preserved but are superseded by trace-queryable signatures. The signature-only notifications are retained in .recur/dispatch-acceptance/blackjack-skill-signatures.jsonl. Exact source report hashes and raw execution references live in the Eventness receipt. Producer completion does not accept a gate; parent and independent coordinator dispositions remain separately recorded.

The publisher consumes the current actual Recur dispatch snapshot, creates complete/blocked Eventness results first and then emits only {trace_ids: [...]}, with one or several signatures. Earlier single trace_id notifications remain valid historical records. Existing Recur Watch still emits its legacy full scheduler-pass JSON to private diagnostic files. A follow-up core transport change should expose this compact Eventness-signature form natively, retaining legacy output compatibility. This adapter does not falsely claim that the Watch core already changed.

Further opinionated requirements: deterministic attempt identity, immutable historical receipts, retry lineage, scoped source/test provenance, explicit unknowns, terminal publication acknowledgement and discovery of stored artifacts after a lost notification. Do not embed arbitrary agent conversation or instructions in scheduler notifications.
