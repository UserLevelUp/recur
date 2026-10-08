artifact.type = lane
publish: main.command.watch.eventness.final.review independent high-reasoning source review
warp.id = main.command.watch.eventness
warp.uuid = 01a11a3f-14c9-76f0-a4f6-62956194944b

Independent Codex review found and drove these corrections before acceptance:

- Replay and previews now hold shared locks, so they cannot observe an unsynced
  batch while another drain owns the cursor. Confirmed subscribe keeps its lock.
- Cursor-write injection now exercises partial writes and rollback. Rust tests
  wrap a real file to exercise sync failure and retry after rollback.
- Non-UTF-8 producer paths fail before commit instead of losing filename identity.
  A Unix-only regression covers this; this Windows run cannot execute it.
- Windows private-state scope and reference checks compare paths with ASCII case
  folding. A Windows regression covers differently cased `.Recur/Watch` paths.
- Final gate validation caught a manifest-folder/project-source mismatch. Optional
  root-bounded `evidence_root` metadata aligns actual checked source resolution
  across inventory, show, merge, completion and refresh without changing defaults.

The parent checks the final source with Rust, Julia, frozen Node conformance and
integration suites. This review is source evidence, not proof of test execution.
Actual command outcomes and fingerprints are recorded separately under observations/.

Future Warp candidates / known bounds:

- Measure useful verified output per uncached token across authenticated hosts.
  Deterministic worker fixtures here do not prove live provider access or cost.
- Native topic-driven scheduling can be added separately. Existing dispatch stays
  polling; callers can already build a bounded drain/review/gate loop.
- Add consumer acknowledgements only if needed. Durable replay handles lost stdout;
  the caller still owns idempotence and downstream side effects.
- Add validated retention/rotation when 64 MiB cursor journals become limiting.
  Use a new subscriber identity while preserving history in the current version.
- Current snapshot checks detect endpoint drift, not change-then-restore. Path
  checks assume cooperative local filesystems, not malicious concurrent retargeting.
- Process interruption is covered; directory-entry durability after OS/power loss
  and Unix execution remain unverified on this Windows host.

No Lang grammar change, provider reauthentication, model fallback or automatic
intelligence selection is introduced by this Warp. Those policies remain optional
and companion-owned. Notifications remain exactly trace_ids arrays.
