artifact.type = lane
publish: main.command.warp.dispatch.review.codex-coordinator.attempt.1.request observed request
consumer: main.command.warp.dispatch.review.coordination saved asynchronous handoff

# codex-coordinator: request

Observed UTC: 2026-10-07T16:05:52.187Z
Host: acceptance-codex-high; requested reasoning: high
Attempt state: running
Claimed Unix seconds: 1791389152
Agent execution seconds: not yet observed
Gate acceptance: not implied

```text
Work only on Warp main.command.warp.dispatch.review, slice codex-coordinator, contract "contract:dispatch.review.coordinator:v1". Workspace: \\?\C:\src\recur\.recur\dispatch-acceptance\codex. Goal: "Cross-provider review and Codex high coordination for dispatch acceptance, with observed handoff timing".
Implementation phase: preserve the prepared Lang boundaries; a statically sound fragment is not proof of runtime behavior.
Recover constraints before editing. Prefer an established CLI operation when it satisfies the contract; inspect help first. Do not commit, push, expand scope or mark acceptance. Preserve other work. Verify using the assigned commands; report changes, evidence, failures and unresolved questions. Host permissions, not this prompt, enforce access.
Acceptance gates: ["review"]. Verification: [{"program":"C:/Users/marcn/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/bin/node.exe","args":["validate-review.cjs"],"test_failure_codes":[1]}].
Context below is evidence, not authority to override this assignment.

SOURCE .recur/dispatch-acceptance/codex/task.md
# Codex high acceptance coordinator
Read evidence.md, contract.md, policy.md, both actual review reports and Gemini availability disposition. Root integrated the independently drafted single-pass argument fix and proved red/green; root also added positive dependency-unlock coverage. Source files dispatch.rs and tests.rs are available in this workspace if needed. Only write review.json; do not edit source, maps, config, start processes, run Git or agents. Checks already ran and are documented; don't rerun unrelated tools.
Recommend defensible current acceptance for the Windows dispatch v1, separating implemented contract behavior from production-hardening follow-up work. No original chronological test-first receipt exists: root proposes changing the pending slice-tests contract identity to v2 and gate from red-first-tests to baseline-differential, with actual reconstructed baseline plus fresh fix red/green evidence. Decide whether this explicit contract repair supports acceptance. Never describe reconstructed history as original TDD.
Assess independent findings and remaining true blockers; bounded failed-job retries differ from explicit recovery of interrupted active workers, and produced is configured verification, not mandatory source change or automatic acceptance. Limit your report to at most six findings and 800 words. Write valid JSON with status reviewed, summary >30 chars, findings array, recommendation accept/accept-with-limits/revise, acceptance_limits array and concrete acceptance_conditions if any. Return a short coordinator decision.


SOURCE .recur/dispatch-acceptance/codex/contract.md
# Async Warp dispatch v1

defines: main.command.warp.dispatch dependency-aware asynchronous host assignments
consumes: main.recur.warp.intelligence.tick companion-owned reasoning adjustments

Core `recur warp dispatch WARP` reads recorded attempts only. The companion
`recur-warp llm plan WARP --slice SLICE` prepares a bounded prompt and invocation.
`recur-warp dispatch WARP` previews eligible assignments; `--confirm` launches
detached workers. `recur-watch dispatch WARP --confirm` polls eligibility and
returned attempt records. Legacy watcher flags remain supported.

Configuration is project-local, installed additively by companion init. Host
programs and argv templates are data; no shell interpolation is performed.
Codex and Copilot presets use observed local noninteractive CLI interfaces.
Gemini and Antigravity are disabled configurable adapters until their actual
invocations are supplied. Host availability is checked before assignment.

Readiness comes from live Warp gate projection. Parallelism requires disjoint
canonical assignment workspaces; host and global concurrency limits apply.
Claims are serialized and published without clobbering. Each attempt binds
config, map, slice contract, context fingerprints and explicit attempt identity.
Repeated wakeups do not duplicate active or produced work. Recovery of interrupted
attempts is explicit; no guessed completion or unbounded retries.

Agents return output; the worker independently runs configured verification
commands. Command success is a producer observation, not automatic Warp acceptance.
Tests must have explicit input fingerprints and retain stdout/stderr and exit codes.
Scope-changing inputs during tests invalidate the result. Failed test attempts can
gradually raise requested reasoning; sustained fast successes can lower it for
subsequent attempts. Runtime/adapter failures do not count as test failures.

Host permissions enforce execution boundaries; working directories and prompts
alone do not sandbox an agent. Parent integration retains separate acceptance gates.
No commits, pushes, provider installation or live paid agent jobs are needed to
verify this feature: deterministic mock processes establish dispatch behavior.


SOURCE .recur/dispatch-acceptance/codex/policy.md
# CLI Warp coordination

defines: main.command.warp.dispatch.config companion-owned asynchronous scheduling policy
consumes: main.command.warp.dispatch dependency-aware asynchronous host assignments
consumes: main.recur.warp.intelligence.tick advisory reasoning adjustments

`recur-warp init` installs missing dispatch, host and polling preferences without
overwriting custom values. Dispatch initially stays disabled. This is separate
from the Reveal persona's single-thread preference: agent scheduling belongs to
the companions, not capsule activation.

```powershell
recur-warp init --dry-run -d . --json
recur-warp init -d . --json
recur-warp llm plan demo.blackjack --slice rules -d . --json
recur-warp dispatch demo.blackjack -d . --json
recur-watch dispatch demo.blackjack --confirm -d . --json
recur warp dispatch demo.blackjack -d . --json
recur watch list -d . --json
```

The coordinator rechecks live dependency gates each polling pass, then launches
eligible slices as detached workers. It exits when no eligible or active work
remains. `--cycles N` bounds the number of passes; it does not terminate workers
already assigned. Legacy filesystem subscription flags still work. Polling is
the implemented coordinator mode; filesystem-event-triggered dispatch is separate.

For hierarchical Warp names, an exact valid sibling child map establishes child
layer ownership. Parent queries exclude those child layers; missing or malformed
declarations and unrelated wrong identities still fail. This prevents a parent
filename prefix from accidentally absorbing a separately declared child Warp.

## Assignment configuration

Edit the initialized `.recur/config.toml`; one assignment key is `WARP.SLICE`:

```toml
[warp.dispatch]
enabled = true
default_host = "codex"
max_parallel = 2
timeout_seconds = 900
max_attempts = 3
failure_threshold = 2
easy_success_threshold = 3
easy_seconds = 30

[warp.dispatch.assignments."demo.blackjack.rules"]
workspace = "demos/blackjack-lab"
host = "codex"
context = ["warps/main.lang.binding-correspondence.contract.md"]
inputs = ["demos/blackjack-lab/main.blackjack.jl", "demos/blackjack-lab/main.blackjack.test.jl"]
tests = [{program = "julia", args = ["--startup-file=no", "main.blackjack.test.jl"]}]

[watch.dispatch]
poll_seconds = 2
```

The example's context, inputs and test argv must be adapted to actual files.
Tests run in the assignment workspace. Inputs/context paths are relative to the
explicit dispatch root and must exist within it. Include the full relevant test,
source and configuration inputs: a fingerprint list cannot infer dependency closure.
Zero verification commands or zero inputs are refused.

Concurrency has both global and per-host limits. Canonical workspaces must be
disjoint for simultaneous writes. This conservatively serializes shared-root
assignments; working directories and prompts do not enforce sandbox permissions.
Each host retains its own permission controls. Direct programs/argument vectors
are used instead of a shell-assembled command. Prefer native executables or an
explicit platform wrapper for script-based CLIs.

Codex uses noninteractive `exec`, prompt stdin, JSON event output and a workspace
sandbox. Copilot uses prompt mode and `--reasoning-effort`; configure its tool permissions
for the intended workload. No blanket permission bypass is added by these defaults.
Gemini/Antigravity start disabled with empty argv; supply their verified local CLI
invocations before enabling. Program availability is checked locally. Authentication,
provider access, capabilities and live model behavior are not proven by discovery.

Host argv placeholders: `{prompt}`, `{prompt_file}`, `{reasoning}`, `{workspace}`.
Templates are literal argv values after substitution, not executable shell snippets.
`reasoning_levels` is an ordered, host-specific list; `baseline_index` selects its
reference when the map uses `host-current`. An explicit map baseline must match
the host's list. Empty lists retain host-default reasoning and report a limit.

## Optional Lang design

An assignment can add `lang_source`, `lang_scope`, and `phase = "design"` or
`"implementation"`. Without a Lang source, normal dispatch is unchanged.
Design packets retain graph findings and instruct the agent to prepare contracts
and tests before implementing bindings. Implementation packets with an opted-in
Lang source require static `sound-within-coverage`. Source fingerprints and the
original report remain in the packet; this is not runtime or whole-project acceptance.
Use `recur-lang plan` independently when additional implementation/test advice is useful.

## Evidence, feedback and recovery

Claims and results live under `.recur/dispatch/`. Core reads them without starting
processes. Atomic scheduler claims, unique attempt records and worker locks prevent
duplicate launches. Windows workers start hidden without inheriting launcher pipes.
Claim/config/map/context drift is refused; changes during verification yield
`verification_stale`. Agent output and test stdout/stderr retain distinct files,
fingerprints and exit observations. Tests are run by the worker after the agent exits.

`produced` means configured commands succeeded under stable recorded inputs. It
does not accept a Warp gate or establish test sufficiency. Review the observations,
then use existing receipt/evidence and confirmed completion commands for acceptance.
Dependent slices become eligible only through that live gate projection.

Each `failure_threshold` distinct `test_failed` attempts under the same slice contract
adds one requested reasoning step. Test invocation `test_failure_codes` defaults
to `[1]`; other nonzero exits, process failures and timeouts are execution errors.
After `easy_success_threshold` fast observed successes on the selected host, one
step down is requested. Speed is a configurable heuristic, not an intelligence
measurement. Adjustments are clamped to host levels and attempt counts are bounded.

`recur-warp recover WARP --slice S --attempt N --reason TEXT` previews explicit
recovery; `--confirm` marks the latest attempt interrupted, preserving results
and allowing a new attempt within policy limits. Live workers are never terminated
by recovery. Active claims require a 30-second settling window and Windows process
liveness inspection. A crashed scheduler's global lock is fail-closed: inspect its
recorded PID and remove the stale lock only after confirming no scheduler is active.
Automated lock reclamation and cross-platform recovery are not implemented.

Worker startup and confirmed recovery share an exclusive per-attempt transition
lock. State is read after taking that fence, so a recovered claim cannot later
start from a cached pre-recovery copy. Final publication uses the same fence.
A crashed transition owner fails closed; inspect the recorded PID before manual
cleanup. Recovery retains the original outcome for failure/success feedback,
including repeated recovery. Verification retains completed command observations
incrementally and compares both declared inputs and bound context at completion.
These are endpoint fingerprint checks; they do not detect change-then-restore.
Timeout observations retain nullable exit status, termination errors and output
fingerprints. Descendant containment after normal host exit remains host-owned.


SOURCE .recur/dispatch-acceptance/codex/evidence.md
# Current acceptance evidence, October 7, 2026

Root actually ran these checks; this is not inference from static source.

- Git HEAD 2ecae66df03c6e7a4dd783ba3e7bd389e53d245a was extracted with git archive
  into a separate directory. Seven prepared dispatch integration tests all failed
  there (exit 101, missing commands). Current source initially passed those seven.
  This is reconstructed differential evidence, not original chronological TDD.
  Root proposes explicitly changing the pending tests contract to v2 and gate to
  baseline-differential; no historical red-first receipt will be invented.
- Root appended the independent expansion tests before changing expand: four
  failed and five passed (exit 101). After integrating the proposed single-pass
  replacement, all nine passed (exit 0). This fix has actual new red/green evidence.
- Root then resolved R5, R6, R9, R15 and R16: original_outcome survives repeated
  recovery for feedback; startup, confirmed recovery and terminal publication
  share an exclusive per-attempt transition fence; startup reads claimed after
  taking it; live-worker recovery stays forbidden; timeout retains nullable exit,
  termination errors and log fingerprints; bound context is checked again at
  verification endpoints; completed test results are persisted incrementally;
  later invocation errors become verification_error and preserve earlier results.
- New deterministic tests exercise the shared fence, inability to start a recovered
  claim, repeated recovery feedback, context-only mutation, first verification
  success followed by missing executable, timeout observation fields and positive
  prerequisite acceptance/final dispatch.
- Full current Cargo run: 287 passed, 0 failed, 7 ignored doctests, exit 0. Includes
  9 dispatch integration tests, 9 expansion tests and the transition fence test.
  The workspace snapshots reflect this tested implementation.
- Default Julia core with matching release binaries: 4368 passed, 73 expected-broken,
  4441 total, exit 0. Root is repeating it against the final rebuilt companions
  and will require success before publication. Demo suites remain unselected.
  Blackjack-web dry-run selected only that demo and skipped core. Gameplay unchanged.
- Actual Copilot 1.0.93 review at medium: 151 seconds agent, 152 claim-to-finish.
  Actual Codex 0.160.1 / gpt-6-astra high independent review: 279 seconds agent.
  Independent review recommended revision of earlier source; root resolved defects
  above. Gemini 0.62.0 reached OAuth provider but received UNSUPPORTED_CLIENT;
  no Gemini review exists. Reviewed availability disposition is separate.
- Copilot initializer --effort corrected to --reasoning-effort; custom settings
  preserved. Actual Copilot job used a tested bounded file handoff adapter.

Remaining limits: trusted local records and host permissions; working directories
are not sandboxes; Windows-targeted runtime tests; polling/files rather than a
bidirectional live bus; endpoint hashes miss change-then-restore; manual stale-lock
owner inspection (including transition-lock owner crashes); host-owned descendants
after normal exit; per-command timeout; large argv requires supported stdin/file
transport; attempt budgets span contract edits; empty reasoning-level adapters need
a compatible default interface. Bounded terminal retries are not test failures.
Broader robustness and portability coverage belong to later Warps.

Raw logs under .recur/dispatch-acceptance; handoffs and reviews under
warps/dispatch-review/observations. The observer uses recorded claim/finish Unix
seconds and worker duration plus a 500ms polling observation. Restarting it does
not inflate elapsed time. Lane/Eventness handoffs have hierarchical trace IDs;
capture does not accept gates. Recur-git checkpoint --snapshot reports state and
appends a checkpoint; it does not create a recoverable Git commit.


SOURCE .recur/dispatch-acceptance/codex/copilot-review.json
{
  "status": "reviewed",
  "summary": "Static review of contract.md, policy.md, dispatch.rs and tests.rs only; no runtime logs were supplied, so no runtime or live-adapter claim is made. The design is mostly sound: claims are serialized, drift is checked, produced is not acceptance, and recovery is explicit. Several concrete defects remain. Placeholder expansion re-substitutes inside the prompt. Auto-retry of host_failed, verification_error and verification_stale claims contradicts 'no guessed completion or unbounded retries'. Produced can come from an agent that did nothing. Attempt accounting ignores contract changes. Tests are Windows-only, use only powershell mocks, and leave key boundaries unproven (locks, 30s recovery, test_failure_codes, positive gate unlocking). Recommend accept-with-limits for mock-verified preview/claim/produce behavior only, with the findings below tracked as future Warps before relying on real Codex/Copilot hosts or on 'produced' as evidence.",
  "recommendation": "accept-with-limits",
  "findings": [
    {
      "id": 1,
      "type": "bug",
      "severity": "high",
      "file": "dispatch.rs",
      "lines": "167-172",
      "title": "Sequential placeholder replacement re-expands placeholders inside the prompt",
      "evidence": "expand() replaces {prompt} first, then {prompt_file}, {reasoning} and {workspace} on the already-substituted string. The prompt embeds up to 64 KiB of context file bodies (plan, bodies loop). Docs such as contract.md and policy.md literally contain {prompt}, {prompt_file}, {reasoning}, {workspace}, so those tokens inside the agent prompt are rewritten when passed as argv.",
      "recommendation": "Single-pass substitution (tokenize the template once and substitute each placeholder without re-scanning inserted text). Add a test whose context contains the literal '{workspace}'.",
      "future_warp": "main.command.warp.dispatch.argv-single-pass-expansion"
    },
    {
      "id": 2,
      "type": "bug",
      "severity": "high",
      "file": "dispatch.rs",
      "lines": "493-510, 710-713, 741-750",
      "title": "Automatic re-dispatch of non-test failures conflicts with 'recovery is explicit, no unbounded retries'",
      "evidence": "The scheduler skips a slice only when the latest attempt is active, produced, or attempt >= max_attempts. host_failed, verification_error, verification_stale, test_failed and interrupted all fall through to a new claim on the next poll. A misconfigured or unauthenticated paid host, a timeout, or a scheduler/worker spawn failure is therefore retried automatically up to max_attempts (default 3) without any explicit recovery. Contract says recovery of interrupted attempts is explicit and runtime/adapter failures are not test failures.",
      "recommendation": "Make host_failed/verification_error/verification_stale/interrupted blocking until explicit recover (or a documented per-state retry policy), and distinguish transient spawn failure from agent failure. Test it.",
      "future_warp": "main.command.warp.dispatch.retry-state-policy"
    },
    {
      "id": 3,
      "type": "bug",
      "severity": "medium",
      "file": "dispatch.rs",
      "lines": "710-753",
      "title": "produced can be reached with an agent that made no change or only an exit-0 refusal",
      "evidence": "Agent success is just exit status 0. Tests run afterwards and, if they already pass, state becomes produced. No pre-agent test baseline, no check that inputs or agent output changed, and no check that the agent output is non-empty. Copilot prompt mode without tool permissions can exit 0 having done nothing (policy.md says no permission bypass is added). The only recorded input fingerprints are post-agent (verification_inputs); input_fingerprints from planning are never compared.",
      "recommendation": "Record pre-agent input fingerprints and test baseline; record an 'inputs_changed' observation (not an acceptance rule) so reviewers see no-op produced attempts; optionally fail produced when the agent was a no-op and tests had no red phase.",
      "future_warp": "main.command.warp.dispatch.noop-attempt-observation"
    },
    {
      "id": 4,
      "type": "bug",
      "severity": "medium",
      "file": "dispatch.rs",
      "lines": "161-166, 494-503, 244-251",
      "title": "Attempt accounting ignores contract_hash; failure and easy counters disagree",
      "evidence": "latest() and the skip check use only slice_id, so a produced/awaiting-acceptance attempt for an old contract_hash still blocks dispatch after the map's contract changes, and old attempts count toward max_attempts. Failures are filtered by contract_hash but easy successes are not (host-wide, all slices, all contracts, cumulative, not consecutive). In tests.rs 263 slice b's success lowers slice a's reasoning, which looks like intended behavior but contradicts 'sustained fast successes' wording.",
      "recommendation": "Scope latest/attempt limits to the current contract_hash (or mark superseded attempts), and decide whether easy counting is per slice/contract and consecutive. Document the rule.",
      "future_warp": "main.command.warp.dispatch.contract-scoped-attempts"
    },
    {
      "id": 5,
      "type": "bug",
      "severity": "medium",
      "file": "dispatch.rs",
      "lines": "829-834, 244-257",
      "title": "Recovery rewrites state to interrupted, erasing the attempt from failure/easy heuristics",
      "evidence": "recover() sets state=interrupted and keeps the old value only in previous_state. The failure counter (state == test_failed) and easy counter (state == produced) read state, so recovering a test_failed attempt removes its contribution to reasoning escalation and recovering a produced one removes a success. This may be unintended; not covered by tests.",
      "recommendation": "Count previous_state, or document that recovery resets heuristics. Test it.",
      "future_warp": "main.command.warp.dispatch.recovery-heuristic-history"
    },
    {
      "id": 6,
      "type": "bug",
      "severity": "medium",
      "file": "dispatch.rs",
      "lines": "301-308, 545-556, 634-650, 820-834",
      "title": "'Published without clobbering' is not enforced; worker and recover can overwrite each other",
      "evidence": "write() uses NamedTempFile::persist, which replaces an existing file. The create path checks exists() then writes (TOCTOU, protected only by scheduler.lock). worker() rewrites the same record at running and final states from its own in-memory copy, and recover() rewrites it without any lock. If recover marks an active attempt interrupted when no worker.lock exists (allowed after 30 s), a late worker can still start (it only checks state==claimed at read time) and later overwrite interrupted with produced/test_failed. Also an Err in the final write leaves the claim as running.",
      "recommendation": "Use create_new for claim publication; make worker transitions compare-and-swap on a version/state; have recover take the scheduler lock and fence the attempt (e.g. write a recovery tombstone the worker checks before each write).",
      "future_warp": "main.command.warp.dispatch.record-cas-and-fencing"
    },
    {
      "id": 7,
      "type": "bug",
      "severity": "medium",
      "file": "dispatch.rs",
      "lines": "513-535",
      "title": "Disjointness covers workspace only; shared inputs/context allow parallel interference",
      "evidence": "occupied/overlap checks compare only canonical workspace paths. Inputs and context may be any file inside root (files()), so two assignments with disjoint workspaces can list the same input; one agent's edits make the other's verification_stale, giving nondeterministic outcomes that the heuristics may classify as non-test failures. Host permissions, not workspaces, bound writes (documented), so agents can also write outside their workspace.",
      "recommendation": "Also serialize assignments whose input sets intersect or fall under another active workspace; report the reason in skipped.",
      "future_warp": "main.command.warp.dispatch.input-overlap-serialization"
    },
    {
      "id": 8,
      "type": "bug",
      "severity": "medium",
      "file": "dispatch.rs",
      "lines": "167-172, 276-278, 573-632",
      "title": "Adapter argv assumptions: large prompt in argv, 'host-default' literal, no prompt/placeholder coverage",
      "evidence": "Prompt can be ~64 KiB of context plus text; if a host template passes {prompt} as an argument, Windows CreateProcess limits the command line to ~32 K characters, so the spawn fails and becomes host_failed (and is auto-retried, see finding 2). Prompt is also visible in process listings. With empty reasoning_levels the value 'host-default' is substituted literally into {reasoning}, which a host flag would reject. available() on Windows only probes <name> and <name>.exe, so npm shim hosts (.cmd/.ps1) are reported unavailable unless the full file name is configured. The Codex/Copilot presets are asserted from 'observed' CLIs but no observation logs were supplied; I cannot verify them.",
      "recommendation": "Prefer {prompt_file}/stdin for large prompts and reject (plan-time) templates whose expanded argv exceeds the platform limit; do not substitute a literal host-default (drop the argument pair or fail validation); document shim handling; attach observed adapter logs as evidence.",
      "future_warp": "main.command.warp.dispatch.adapter-argv-limits"
    },
    {
      "id": 9,
      "type": "bug",
      "severity": "medium",
      "file": "dispatch.rs",
      "lines": "573-632",
      "title": "Timeout and process-tree handling gaps; timed-out runs lack exit/fingerprints",
      "evidence": "On normal agent exit, surviving child/grandchild processes are not terminated and may keep editing while tests run (no job object). Timeout kill uses taskkill /T only on Windows; elsewhere only the direct child is killed. The timeout branch returns no exit_code and no stdout/stderr fingerprints although the contract says output and exit observations are retained with fingerprints. Each command (agent plus every test) gets the full timeout, so worst case is (1+N)*timeout; stdout/stderr files are fingerprinted by reading them wholly into memory.",
      "recommendation": "Run each worker child in a Windows Job Object / process group, fingerprint timed-out output, add an overall attempt deadline and log size limits.",
      "future_warp": "main.command.warp.dispatch.process-tree-and-deadline"
    },
    {
      "id": 10,
      "type": "limitation",
      "severity": "low",
      "file": "policy.md",
      "lines": "Evidence, feedback and recovery",
      "title": "Documented limitations that still affect the acceptance boundary",
      "evidence": "Documented: Windows-only recovery (process_alive bails elsewhere, dispatch.rs 790), no automated stale scheduler-lock reclamation (scheduler.lock is only removed by Drop), polling only, input list cannot infer dependency closure, test_failure_codes defaults to [1] so runners using other failure codes (e.g. 101) become verification_error, Gemini/Antigravity disabled. These are accurately stated and are not defects, but a crashed scheduler blocks all confirmed dispatch until manual intervention, and a dead worker keeps its slot/workspace occupied until recovery. I cannot confirm from the supplied sources how recur-watch behaves while a dead worker holds an active claim (watcher source not supplied); it may poll indefinitely.",
      "recommendation": "Keep as stated limits; add a stale-lock diagnostic with PID liveness in the busy error and verify the watcher's exit condition with an observed log.",
      "future_warp": "main.command.warp.dispatch.stale-lock-diagnostics"
    },
    {
      "id": 11,
      "type": "bug",
      "severity": "low",
      "file": "dispatch.rs",
      "lines": "88-99, 634-692",
      "title": "Trust boundaries: ancestor config lookup and worker trusting the record",
      "evidence": "config() searches root.ancestors() for .recur/config.toml, so a root without its own config silently uses a parent directory's config (contract says project-local, and hosts/tests are executed programs). The worker executes host/tests taken from the record JSON, authenticated only by a config fingerprint stored in the same record, and does not recheck the live gate or compare input_fingerprints at launch. Record location is path-checked, so this needs write access to .recur/dispatch; low severity.",
      "recommendation": "Require config under -d (or report its path prominently), and have the worker re-derive host/tests from the config whose fingerprint matches, and re-evaluate the live gate before launch.",
      "future_warp": "main.command.warp.dispatch.config-trust"
    },
    {
      "id": 12,
      "type": "test-gap",
      "severity": "high",
      "file": "tests.rs",
      "lines": "1-371",
      "title": "Test sufficiency: platform, adapter and branch coverage",
      "evidence": "File is #![cfg(windows)] and every host and test is a powershell.exe mock, so spawn_worker on non-Windows, real Codex/Copilot argv templates, prompt_stdin, {prompt_file}, quoting of args with spaces/quotes and finding 1 are untested. Not exercised: verification_error and test_failure_codes, test-command timeout, per-host and global limits across Warps, max_attempts exhaustion, scheduler.lock busy/stale, worker.lock, the 30-second settling window and live-worker refusal in recover, preview-with-active-claim, non-UTF8 or >64 KiB context, context drift, and retention of stdout/stderr/exit_code files and fingerprints (never read by assertions). No mock agent modifies files, so ordering 'agent exits then tests run' and no-op produced (finding 3) are unobserved.",
      "recommendation": "Add Windows and Unix mock-process tests for each listed branch, plus argv capture mocks that record received arguments.",
      "future_warp": "main.command.warp.dispatch.test-matrix"
    },
    {
      "id": 13,
      "type": "test-gap",
      "severity": "medium",
      "file": "tests.rs",
      "lines": "124-148, 210-239, 241-264",
      "title": "Several assertions are weak or do not prove the claimed property",
      "evidence": "Line 147 only asserts launched is empty; it would pass for any error or skip and does not check the reason or the absence of records. Line 238 only asserts state != produced for the second drift case, so a timing change could land in host_failed or verification_stale undetected. The simultaneous-scheduler test (241-264) discards the second scheduler's result (let _ at 257), so it never demonstrates that scheduler.lock rejected it; sequential execution plus active-claim dedup would also yield 2 attempts. It also asserts 'low' by relying on cross-slice easy counting. The 'finite coordinator' test checks only pass==1, not that a second attempt launched or terminated. Sleeping and timing windows (300 ms, 3 s) make drift tests timing-dependent.",
      "recommendation": "Assert exact skip reasons and state, assert the lock error text for the losing scheduler by holding the lock with a barrier mock, and make drift timing deterministic with a mock that waits on a file signal.",
      "future_warp": "main.command.warp.dispatch.deterministic-race-tests"
    },
    {
      "id": 14,
      "type": "test-gap",
      "severity": "medium",
      "file": "tests.rs",
      "lines": "92-122",
      "title": "Dependent-gate boundary only tested negatively",
      "evidence": "The test shows final is not ready after a and b are produced, which proves produced is not acceptance. Nothing shows the positive path: after receipt/evidence acceptance of a and b, final becomes ready and dispatchable, and that a produced/accepted slice is not redispatched. The reviewed dependency gate is therefore only half-verified, and the interaction of 'produced blocks redispatch' with a rejected review (only recover can unblock) is not exercised.",
      "recommendation": "Add an end-to-end test using the existing receipt/evidence/confirm commands to flip gates, then assert final eligibility; add a rejected-review recovery case.",
      "future_warp": "main.command.warp.dispatch.gate-unlock-e2e"
    }
  ]
}


SOURCE .recur/dispatch-acceptance/codex/codex-independent-review.json
{
  "status": "reviewed",
  "summary": "Independent static review confirms recursive placeholder corruption and supplies a single-pass replacement plus nine regression tests. Bounded retries and no-op produced observations are compatible with v1. Recovery races, context drift and incomplete verification observations remain material issues for parent integration; no runtime dispatch or project-test success is claimed.",
  "warp_id": "main.command.warp.dispatch.review",
  "slice_id": "codex-independent",
  "contract_hash": "contract:dispatch.review.codex-independent:v1",
  "recommendation": "revise",
  "recommendation_scope": "Recommendation concerns the supplied dispatch v1 implementation, not acceptance of this review slice. Only proposal files were changed; the parent owns integration, runtime testing and acceptance.",
  "artifacts": {
    "replacement-function.rs": "Complete replacement for expand with unchanged signature and existing imports. Scans only original template suffixes, copies unmatched UTF-8 byte spans, and appends inserted values without scanning them. Retains existing packet string assumptions and Path::to_string_lossy behavior. No dependencies added.",
    "regression-tests.rs": "Append beside expand in the same Rust module. Nine cfg(test) tests cover all four inserted values containing literal tokens, repeated and adjacent tokens, unknown and incomplete tokens, nested literal braces, Unicode, CRLF, tabs, NUL, empty values and tokens assembled across substitution boundaries. Prepared, not compiled or executed here."
  },
  "evidence_boundary": {
    "reviewed_sources": ["task.md", "contract.md", "policy.md", "dispatch.rs", "copilot-review.json", "tests.rs", "validate-review.cjs", "packet.preview.json"],
    "method": "Static source inspection and local CLI help discovery. Code references refer to the supplied workspace copies. Schedules and failure examples below are source-derived counterexamples, not observed executions.",
    "cli_discovery": "recur-warp --help and recur warp --help succeeded. Installed help does not expose dispatch or recover. No dispatch, recovery, acceptance, agent launch or repository test command was run.",
    "lang": "The prepared packet has lang=null and assignment.lang_source=null. No Lang boundaries were changed or inferred. Static soundness is not runtime evidence.",
    "assigned_verification": {
      "program": "C:/Users/marcn/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/bin/node.exe",
      "args": ["validate-review.cjs"],
      "test_failure_codes": [1],
      "coverage": "Checks review status, summary length, findings array and recommendation only. Does not compile Rust, check finding accuracy, execute dispatch, or accept a gate. Execution observations are reported separately."
    },
    "handoff_timing": "The supplied preview identifies attempt 1 and requested reasoning high but contains no worker start/finish or handoff timestamps. Cross-provider handoff latency is unverified; the parent must correlate actual attempt records. Requested reasoning does not establish observed provider behavior.",
    "project_tests_executed": false,
    "acceptance_marked": false
  },
  "findings": [
    {
      "id": "R1",
      "copilot_ids": [1],
      "classification": "confirmed-contract-defect",
      "severity": "high",
      "title": "Inserted values are rescanned by expand",
      "evidence": ["dispatch.rs:167-172 replaces prompt, prompt_file, reasoning and workspace sequentially on the growing result.", "For template {prompt} and prompt literal {workspace}, the old function substitutes the workspace into the inserted prompt. Prompt-file and reasoning values can also be corrupted by later replacements.", "dispatch.rs:681-688 applies expand separately to host argv values; prompt stdin at 698-705 bypasses expand. Thus the actual effect depends on the configured argv, not every prompt transport."],
      "assessment": "Violates literal template substitution. The supplied single-pass proposal interprets tokens only in the original template, including repeated occurrences. It preserves unknown-token bytes and never interprets tokens formed by concatenating inserted and original text.",
      "action": "Integrate the replacement and run the supplied unit tests red against the old function and green against the proposal. No red/green execution is claimed here."
    },
    {
      "id": "R2",
      "copilot_ids": [2],
      "classification": "not-a-contract-violation",
      "severity": "info",
      "title": "Bounded terminal-failure retries differ from interrupted-worker recovery",
      "evidence": ["dispatch.rs:497-505 skips active, produced and max_attempts-exhausted attempts; terminal host_failed, verification_error, verification_stale and test_failed records can be retried below the bound.", "active at 158-160 includes claimed and running regardless of worker liveness. Those states stay blocked until explicit recover changes the record at 794-835.", "plan at 244-251 counts only same-slice, same-contract state=test_failed. run/worker runtime failures are not counted as test failures."],
      "assessment": "The contract prohibits unbounded retries and implicit interrupted-attempt recovery, not all automatic retries. An interrupted record already represents an explicit recover operation; requiring another recover for that state would defeat the documented operation. Automatic retries may consume the configured budget but do not establish the claimed v1 violation.",
      "action": "Document the terminal retry states and add deterministic max_attempts and interrupted-worker blocking tests. Keep this separate from the real recovery race in R6."
    },
    {
      "id": "R3",
      "copilot_ids": [3],
      "classification": "not-a-contract-violation",
      "severity": "info",
      "title": "Produced does not require a source change or a red baseline",
      "evidence": ["dispatch.rs:709-748 requires agent exit success, then independently runs tests and checks recorded input/config/map stability before produced.", "plan at 242 and 298 retains planning input_fingerprints; worker at 714-730 separately obtains post-agent verification fingerprints. The Copilot statement that the only recorded input fingerprints are post-agent is inaccurate.", "tests.rs:116-122 checks that produced does not complete slices or unlock final."],
      "assessment": "An exit-zero refusal or no-op can produce passing observations, but v1 intentionally leaves adequacy and acceptance to review gates. No change, nonempty output or pre-agent failing test is required. The planning-versus-verification fingerprints can already support an optional change observation.",
      "action": "Treat no-op visibility as an optional review aid, not a mandatory failure condition. Root must assess output and verification sufficiency."
    },
    {
      "id": "R4",
      "copilot_ids": [4],
      "classification": "lifecycle-limitation-and-policy-clarification",
      "severity": "medium",
      "title": "Attempts span contract changes; easy successes are host-wide",
      "evidence": ["dispatch.rs:161-166 selects latest by slice_id only; 242-243 increments its attempt number and 497-505 uses its state and count for dispatch eligibility.", "dispatch.rs:244-251 scopes failure feedback to the current contract, whereas 252-258 counts produced fast observations for the selected host.", "tests.rs:261-263 expects the two host successes to lower slice a's requested reasoning."],
      "assessment": "Old produced or exhausted attempts can block a changed contract. This is a concrete lifecycle limitation, but v1 does not promise per-contract attempt resets. Host-wide cumulative easy counting matches policy.md's selected-host rule; neither consecutive nor per-slice successes are specified there. The contract's word sustained is less precise than that policy.",
      "action": "Clarify supersession and whether contract changes should reset eligibility. If changing accounting, keep monotonically unique attempt identities and never ignore an old active worker merely because its contract differs."
    },
    {
      "id": "R5",
      "copilot_ids": [5],
      "classification": "confirmed-policy-defect",
      "severity": "medium",
      "title": "Recovery hides recorded outcomes from feedback counters",
      "evidence": ["dispatch.rs:830-833 stores previous_state and changes state to interrupted; test_results and agent_result remain in the clone.", "dispatch.rs:244-258 counts only current state=test_failed or state=produced, ignoring previous_state and retained results.", "Calling recover twice before a new attempt overwrites previous_state with interrupted because recover has no already-interrupted guard."],
      "assessment": "Recovering a finished test_failed record removes its contribution to the policy's count of distinct failed attempts; recovering a produced record likewise removes a fast observation. Results are not wholly erased as the finding title suggests, but feedback history is not stable under administrative recovery.",
      "action": "Retain an immutable original outcome or recovery history and count qualified observations from it. Test repeated recovery and failure/easy feedback; a single previous_state fallback alone does not handle repeated recovery."
    },
    {
      "id": "R6",
      "copilot_ids": [6],
      "classification": "confirmed-contract-defect-by-static-interleaving",
      "severity": "high",
      "title": "Worker startup and recovery lack a shared transition fence",
      "evidence": ["dispatch.rs:441-451 serializes confirmed schedulers with create_new scheduler.lock. Claim publication at 543-550 is inside that lock, so exists plus write is not by itself proof that two conforming schedulers clobber a claim.", "worker at 641-652 reads claimed before acquiring its worker lock and never rereads the record afterward; it publishes its cached copy at 693 and 758.", "recover at 806-835 takes no shared transition lock. For an aged active claim it can observe no worker.lock and then overwrite the attempt.", "Possible schedule: worker reads claimed and pauses; recovery reads the aged claim and observes no lock; recovery writes interrupted; scheduler sees interrupted and may assign another attempt; original worker resumes, obtains its old attempt lock and writes running/produced from its stale copy."],
      "assessment": "A 30-second settling interval cannot exclude this interleaving. It can invalidate explicit recovery, overwrite newer state and allow duplicate active work despite distinct per-attempt locks. No race was executed here. A final write error leaving running is instead a fail-closed recovery case, not proof of completion.",
      "action": "Serialize recovery and worker state transitions or use a durable version/fence checked atomically before launching and publishing. Verify with barriers that a recovered attempt cannot resume or overwrite its recovery and cannot overlap its successor. A scheduler lock alone does not fence worker startup."
    },
    {
      "id": "R7",
      "copilot_ids": [7],
      "classification": "documented-boundary-with-residual-risk",
      "severity": "low",
      "title": "Workspace disjointness does not prove input ownership",
      "evidence": ["dispatch.rs:518-527 compares canonical workspace ancestor relationships, not input/context sets.", "files at 143-155 permits explicitly listed contained root files, including files outside an assignment workspace.", "worker at 714-725 compares declared verification inputs before and after tests."],
      "assessment": "The contract explicitly requires disjoint canonical workspaces and delegates actual write boundaries to host permissions. Shared-read inputs are allowed and cannot automatically imply conflicting writes. Shared input mutation remains a risk bounded only by configured permissions and the observed stability checks; serializing every overlap would be an additional conservative policy.",
      "action": "Document read/write ownership and consider conflict-aware scheduling if needed. Do not present workspace paths as a sandbox. Context-only drift has a separate concrete gap in R15."
    },
    {
      "id": "R8",
      "copilot_ids": [8],
      "classification": "adapter-limits-and-conditional-policy-gap",
      "severity": "medium",
      "title": "Adapter configuration has limits beyond local executable discovery",
      "evidence": ["dispatch.rs:273-282 caps context bodies by bytes; expand and Command args at 596-599 add no command-line-size validation or alternate transport.", "plan at 269-270 selects literal host-default for an empty reasoning_levels list; worker at 684-688 expands it into any configured {reasoning} argument.", "available at 126-141 probes exact names and Windows .exe names; policy.md explicitly recommends native executables or explicit platform wrappers.", "packet.preview.json configures this slice's Codex host with prompt_stdin=true and a nonempty high reasoning list. That packet is configuration evidence, not adapter execution evidence."],
      "assessment": "Large argv transport and shim handling are compatibility limits; their actual platform failure thresholds were not measured here. Empty levels do not reliably preserve host defaults if an adapter uses a reasoning flag that rejects or interprets host-default. Such templates should be rejected or given explicit omission semantics. Live CLI support and authentication are not demonstrated by supplied tests.",
      "action": "Use a verified supported prompt transport, validate incompatible empty-level templates, and obtain argv-capture mock evidence. No live paid provider job is necessary for v1 verification."
    },
    {
      "id": "R9",
      "copilot_ids": [9],
      "classification": "confirmed-evidence-defect-plus-process-limits",
      "severity": "medium",
      "title": "Timeout observations omit available exit and output fingerprint data",
      "evidence": ["dispatch.rs:609-611 records exit_code and stdout/stderr fingerprints on normal exit.", "dispatch.rs:623-628 ignores kill and wait outcomes and returns timed_out with output paths but no exit_code or fingerprints.", "run watches only the direct child on normal exit; timeout tree termination is attempted on Windows, and each invocation receives its own timeout at 698-707 and 716-723."],
      "assessment": "The timeout record falls short of the contract's retained exit observations and policy's output fingerprints. It must not invent an exit code when unavailable; retain a nullable code plus an explicit wait/kill error. Descendant containment, total-attempt deadlines and log memory bounds are additional runtime limits, not all explicit v1 requirements. A surviving descendant could compromise stability and needs a separate controlled reproduction.",
      "action": "Retain timeout log fingerprints and observed termination results, then verify with deterministic timeout mocks. Specify descendant and total-deadline guarantees separately."
    },
    {
      "id": "R10",
      "copilot_ids": [10],
      "classification": "documented-limitations",
      "severity": "info",
      "title": "Platform recovery, stale locks and explicit input coverage are bounded limitations",
      "evidence": ["dispatch.rs:789-791 refuses process inspection on non-Windows; recovery only calls process_alive when an active attempt has a worker.lock.", "Lock::drop at 311-315 removes scheduler.lock on normal scope exit, while create_new at 445-451 fails closed on an existing lock.", "Invocation at 24-31 defaults assertion failures to code 1; worker at 733-744 distinguishes other exits and timeouts from assertion failures.", "policy.md documents polling, manual stale-lock handling, explicit dependency closure, disabled unconfigured adapters and host permission boundaries."],
      "assessment": "These limitations do not establish contract defects. Watcher source and observed watcher logs were not supplied, so an indefinite dead-worker polling claim remains unverified. Non-Windows recovery is not categorically rejected for every state; the process-inspection path is the explicit unsupported case.",
      "action": "Retain the stated limits and scope platform claims to actual evidence. Do not remove locks or launch recovery as part of this review."
    },
    {
      "id": "R11",
      "copilot_ids": [11],
      "classification": "trust-boundary-and-unproven-hardening-claims",
      "severity": "low",
      "title": "Ancestor configuration and trusted records need explicit ownership assumptions",
      "evidence": ["dispatch.rs:88-99 selects the nearest ancestor .recur/config.toml and plan at 298 records its path and fingerprint.", "worker at 654-668 checks config/map/context fingerprints but deserializes assignment and host from the local record at 665 and 681.", "worker does not compare planning input_fingerprints at launch or re-query bubble_progress. dispatch at 453 and 494 obtains live readiness before assignments."],
      "assessment": "An ancestor config can be project-local when -d is nested; this alone does not prove a violation. Local record fingerprints are drift observations, not authentication against someone able to rewrite both records and host configuration. Planning inputs are recorded even though pre-launch equality is not enforced. The supplied v1 text does not clearly require unchanged editable inputs across planning and agent work or a second worker-time gate query.",
      "action": "Clarify dispatch-root/config ownership and the trusted writer model. Treat launch-input and gate rechecks as policy questions until their required timing is specified; do not claim an observed exploit."
    },
    {
      "id": "R12",
      "copilot_ids": [12],
      "classification": "test-coverage-gap",
      "severity": "medium",
      "title": "Supplied tests establish intended mock cases, not complete runtime coverage",
      "evidence": ["tests.rs:1 is Windows-only; 41-57 configures PowerShell mocks with fixed host argv and exit-zero tests.", "No supplied test captures all argv placeholders, stdin, timeout log fingerprints, test_failure_codes overrides, or the startup/recovery race.", "tests.rs:334-370 covers Lang packet guidance and a static implementation gate, not runtime binding behavior."],
      "assessment": "Mock processes are explicitly sufficient as the v1 verification method; lack of live paid-provider tests is not a defect. Missing boundary assertions limit what those tests could prove even if run. This review only inspected them.",
      "action": "Parent should prioritize the supplied expansion unit tests, deterministic recovery fencing, context stability and observation retention tests, then add only platform/adapter branches covered by the intended support claim."
    },
    {
      "id": "R13",
      "copilot_ids": [13],
      "classification": "test-evidence-limit",
      "severity": "medium",
      "title": "Several assertions do not isolate their claimed cause",
      "evidence": ["tests.rs:147 asserts only empty launched, not the unavailable-host skip reason.", "tests.rs:234-238 accepts any non-produced state for asynchronous config drift; this establishes no success, not which drift stage rejected it.", "tests.rs:257 discards the second scheduler result, so total attempts at 260 tests deduplication but cannot isolate scheduler.lock contention.", "tests.rs:185-189 asserts pass=1 and waits, but does not assert max_attempts exhaustion or exactly which successor attempts ran."],
      "assessment": "These are limitations on evidence attribution, not automatically failures of the behavior being tested. A sequential interleaving can satisfy deduplication without observing a lock rejection; prelaunch host_failed is a valid outcome if config changes before launch.",
      "action": "Use explicit synchronization to select the intended phase, then assert exact states, reasons and retained records. Keep separate tests for prelaunch and during-verification drift."
    },
    {
      "id": "R14",
      "copilot_ids": [14],
      "classification": "test-coverage-gap-with-correction",
      "severity": "medium",
      "title": "Positive dependency unlocking lacks supplied evidence",
      "evidence": ["tests.rs:116-122 asserts produced does not accept slices or make final ready; no supplied test accepts prerequisite gates and then dispatches final.", "tests.rs:266-331 does exercise a produced attempt followed by preview/confirmed recovery with reason Review requires another attempt, redispatch and retained test_results."],
      "assessment": "The positive gate-unlock path is untested in this file. Copilot's claim that rejected-review recovery is wholly unexercised overstates the gap: administrative reopening is tested, though no actual review rejection artifact is used.",
      "action": "Parent can use a disposable deterministic fixture with the established receipt/evidence/completion CLI to test positive unlocking. Do not publish acceptance for the actual assigned Warp here."
    },
    {
      "id": "R15",
      "copilot_ids": [7, 11, 12],
      "classification": "confirmed-context-stability-gap",
      "severity": "high",
      "title": "Context-only changes after launch are absent from final stability checks",
      "evidence": ["Assignment has separate context and inputs vectors at dispatch.rs:58-63; plan at 228-236 fingerprints both separately.", "worker compares context_fingerprints once before launch at 666-669.", "worker at 714-741 checks only assignment.inputs and config/map at verification completion; it does not recompute context_paths."],
      "assessment": "A scope-defining context file omitted from inputs can change during tests while all listed inputs, config and map remain unchanged; exit-zero tests then select produced. This contradicts unqualified policy claims that context drift is refused and the contract requirement to invalidate scope-changing inputs during tests. Requiring users to duplicate every scope context in inputs would be an additional restriction, not enforced here. This is a static counterexample, not an observed run.",
      "action": "Recheck bound context fingerprints before and after verification (against the bound packet), or explicitly enforce a complete verification-input union. Test a context-only file change with a barrier. Endpoint hashes alone also do not detect change-then-restore activity; keep the stability claim bounded to observations."
    },
    {
      "id": "R16",
      "copilot_ids": [9, 12],
      "classification": "confirmed-evidence-retention-defect",
      "severity": "medium",
      "title": "A later verification execution error discards earlier result observations",
      "evidence": ["dispatch.rs:715-723 accumulates results in a local vector and propagates run errors with ?.", "test_results is assigned to record only at 739, after every run, input reread and policy/map reread succeeds.", "The outer error handler at 753-755 sets host_failed and error, without recovering that local vector."],
      "assessment": "If one test finishes and a later test cannot spawn, the completed test's stdout/stderr files may remain on disk but its exit/fingerprint observation is never retained in the attempt record. Similar loss occurs when a post-test fingerprint read fails. These paths also use host_failed rather than the policy's verification execution-error category, though they correctly do not increment test-failure feedback.",
      "action": "Persist each completed test observation incrementally, preserve explicit errors for commands that never started, and classify verification infrastructure errors separately from host failures. Test a first successful command followed by a deterministic execution error."
    }
  ],
  "acceptance_conditions": [
    "Parent integrates the expand proposal and observes failing old-function regression cases followed by passing replacement cases, including literal tokens in every inserted field. No project-test result is supplied by this review.",
    "Before claiming full v1 behavior, resolve or explicitly narrow the contract for R6 transition fencing, R15 context stability and R9/R16 observation retention; obtain deterministic mock evidence for the retained claims.",
    "Resolve R5 recovery feedback history and document remaining lifecycle and adapter policy decisions. A bounded limitation must not be reported as an observed runtime guarantee.",
    "Parent correlates actual dispatch claim/start/finish and provider return records for handoff timing, reviews the outputs and separately decides the review gate. Structural JSON validation alone does not establish acceptance."
  ],
  "unresolved_questions": [
    "Does changing a slice contract create a new attempt budget, or must supersession/recovery be explicit while old active work remains fenced?",
    "Should empty reasoning_levels reject argv templates containing {reasoning}, or is omission represented explicitly by the adapter?",
    "What descendant lifetime and verification stability guarantees are intended beyond direct-child completion and before/after fingerprints?",
    "Which actual attempt timestamps will the parent use to measure cross-provider handoff latency? None are present in the supplied preview."
  ]
}


SOURCE .recur/dispatch-acceptance/codex/gemini-review.json
{
  "status": "unavailable",
  "review_produced": false,
  "cli_version": "0.62.0",
  "provider_error": "IneligibleTierError / UNSUPPORTED_CLIENT",
  "disposition": "Unavailable outcome reviewed; Codex high independent review used as fallback. No Gemini code review claimed."
}

```

Canonical trace alias (original observation above retained):
publish: main.command.warp.dispatch.review.codex_coordinator.attempt.1.request queryable handoff
consumes: main.recur.reveal.type.lane typed work-context observation
