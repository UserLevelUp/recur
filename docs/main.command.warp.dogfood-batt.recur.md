artifact.type = warp
defines: recur.warp.dogfood.batt
consumes: main.recur.skill.warp, main.recur.skill.eventness
warp.id = main.command.warp.dogfood-batt
warp.map = warps/main.command.warp.dogfood-batt.warp-map.json
status = planned; slice-0 ready; no implementation authorized yet
observed.state = pending
readiness.slice = slice-0
pull.first = recur warp show main.command.warp.dogfood-batt -d .
pull.then = recur warp slices main.command.warp.dogfood-batt -d .; read recur-expert/SKILL.md and recur-eventness/SKILL.md

# Dogfood feedback from batt

Gaps observed while using Recur on `C:\src\batt` (engineering-review movie Warp):

1. `recur warp evidence` rejects an absolute manifest path even when it lies inside
   `-d` (`src/warp_evidence.rs` `contained_file`). Relative form checks fine.
2. A completed Warp whose map still declares `current_slice` warns
   "no longer ready; selection not inferred" without saying the slice is already
   complete or what is ready (`src/main_command_warp_impl.rs`).
3. `recur-eventness/SKILL.md` has no Reveal capsule, unlike `docs/recur-expert.recur.md`
   and `docs/recur-warp.recur.md`, so `recur reveal recur-eventness` finds nothing.
4. `recur-warp refresh` rejects Windows backslash map/layer paths as
   "unsafe root-relative path"; forward slashes work. Candidate for evidence-reference.

## Slices

slice-0 (red baselines) -> evidence-reference | current-slice-diagnostic |
eventness-discovery (tick -1, docs-only) -> slice-final (regressions, docs, expert guidance).

## Intended trace gap

`main.recur.skill.eventness` is consumed but not yet defined; the
eventness-discovery slice owns defining it in a `docs/recur-eventness.recur.md`
reveal capsule. Verify with `recur-git checkpoint --snapshot` and `trace-id`.

## Out of scope

Dispatch already raises reasoning one step per `failure_threshold` `test_failed`
attempts and steps down after easy successes; provider/runtime errors never
escalate. Tier escalation from Auto mode is a host concern, not core Recur.

## Next bounded action

With user authorization, write failing tests for all three gaps (slice-0), record
declared baseline evidence, then implement the three independent slices.
