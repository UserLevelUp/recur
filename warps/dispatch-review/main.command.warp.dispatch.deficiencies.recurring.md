artifact.type = lane
defines: main.command.warp.dispatch.deficiencies observed improvement candidates from live multi-provider acceptance
consumer: main.command.warp.dispatch.review.coordination requests responses and reviewed outcomes

# Deficiencies observed during acceptance

Observed October 7, 2026. These are follow-up candidates, not newly implemented
capabilities. Preserve raw attempts and distinguish execution errors from test
assertion failures.

- `main.command.warp.dispatch.deficiency.argv`: full source context through `-p`
  exceeds Windows' command-line limit. Gemini and Copilot launch failures reported
  OS error 206. Stdin or a bounded file handoff avoids it. Default adapters need
  a tested transport contract, including large prompts.
- `main.command.warp.dispatch.deficiency.retry`: launch errors immediately consume
  the bounded attempt budget. Classify nonretryable errors and back off transient
  failures instead of launching the same invalid argv repeatedly.
- `main.command.warp.dispatch.deficiency.dead-claim`: a Copilot worker PID vanished
  while its record remained running. Explicit recovery preserved attempt 3.
  Consider safe stale-worker diagnostics, with ownership and PID identity checks.
- `main.command.warp.dispatch.deficiency.copilot-preset`: Copilot updated from
  1.0.83 to 1.0.93 and advertises `--reasoning-effort`; initialized old presets use
  `--effort`. Existing custom configuration must remain authoritative; capability
  detection and versioned suggested migrations would help.
- `main.command.warp.dispatch.deficiency.timing`: records have claim and terminal
  Unix timestamps and whole-second durations, but no precise worker-start or review timeline.
  A separate 500 ms observer is recording this trial. Add companion-owned durable
  events, precise timestamps and observed versus measured labels.
- `main.command.warp.dispatch.deficiency.cross-talk`: current coordination uses
  files and polling. There is no implemented live bidirectional message bus.
  Handoffs are now retained as trace/Eventness observations inside the review Warp.
- `main.command.warp.dispatch.deficiency.build-lock`: Windows refuses replacing
  a running Recur executable. Serialize builds/installations after query/worker
  exit; consider immutable per-run binaries and explicit binary fingerprints.
- `main.command.warp.dispatch.deficiency.snapshot`: recur-git `checkpoint
  --snapshot` prints Git/lane state and can append a checkpoint entry; it does
  not create a recoverable Git commit or save ignored files. Keep that distinction
  explicit when designing a future snapshot-storage command.
- `main.command.warp.dispatch.deficiency.package-build`: local pnpm installation
  of Gemini/Codex returned a build-script-policy error for optional keytar/pty
  packages. Headless version/help worked without allowing scripts. Record actual
  provider authentication/runtime results before assuming those optional modules
  are either necessary or harmless to omit.
- `main.command.warp.dispatch.deficiency.provider-access`: Gemini CLI 0.62.0
  reached the cached OAuth provider when configured process-locally, but received
  IneligibleTierError / UNSUPPORTED_CLIENT. The provider requested migration to
  Antigravity. No Gemini code review was produced; the revised availability
  disposition gate and Codex fallback preserve this distinction.
- `main.command.warp.dispatch.deficiency.observer-restart`: the initial observer
  recalculated historical handoff duration at restart. The local observer now uses
  recorded claim/finish timestamps and rehydrates seen states from saved JSONL.
  Durable native companion events remain a future Warp candidate.
- `main.command.warp.dispatch.deficiency.trace-token`: trace-id extracts dotted
  identifiers with alphanumeric/underscore segments and truncates a hyphenated
  slice token. Canonical trace aliases now use underscores while retaining original
  slice IDs and raw observations. Exact response query confirmed producer and consumer.
- `main.command.warp.dispatch.deficiency.publication-root`: standalone evidence
  assessment at repository root and inventory assessment at map directory can
  disagree. The first checked publication was blocked by its reference root;
  its bytes are retained under observations/rejected. Corrected acceptance is
  declared, with separately checked repository-root evidence. Preflight publication
  against the intended live reader's root before writing completion layers.

Critical reviewed defects resolved in this pass: nonrecursive argv substitution,
startup/recovery transition fencing, feedback history after repeated recovery,
context-only drift, timeout observations and partial verification result retention.
Actual Cargo assertions cover these boundaries and positive dependency unlocking.
Independent and coordinator reviews remain preserved even where root disagreed
or repaired the earlier source. Remaining gaps are candidates for later Warps.

All current adapters and package installations are local to `.recur/cli-tools`
and `.recur/dispatch-acceptance`; global persona/model settings are preserved.
Provider review findings will be added after review, with blocking status and
their own proposed Warp scope.

## Retry follow-up, October 7, 2026

publish: main.command.warp.dispatch.deficiency.retry bounded retry and intervention follow-up
consumer: main.command.warp.dispatch.retry.provider_blocked unsupported provider/client instruction
consumer: main.command.warp.dispatch.retry.auth_required human authorization instruction
consumer: main.command.warp.dispatch.retry.transient capped retry deadline

New attempts now retain a pinned retry policy, classified failure, instruction
trace and Warp/slice/attempt trace. Auth and unsupported-client diagnostics pause
without relaunching; transient and unknown execution failures back off within the
existing attempt budget. Explicit recovery preserves original outcomes and does
not reset that budget. Runtime failures remain separate from test intelligence.
This addresses the retry candidate above for newly recorded classified failures;
historic attempts were preserved, and unknown errors still need inspection.

The installed deterministic mock retained six attempts, three produced outcomes,
bounded transient waiting and explicit recovery, with no implicit gate acceptance.
Evidence is `.recur/dispatch-acceptance/retry-installed-smoke-result.json`, with
scoped Eventness observations under its recorded mock root. No new Gemini request
or successful human authorization is claimed by this simulation.

Parallel Windows tests also exposed temporary access denial during atomic record
replacement. Publication now retries the same staged bytes for a bounded interval;
unresolved terminal publication errors are retained in `*.worker.error.txt`.
A deliberate exclusive-reader test covers this behavior. This fixes one observed
cause of dead-looking attempts; automatic stale-process recovery remains pending.

The prior accepted layers and source manifests remain historical. New retry
checks supplement that evidence; they do not make earlier manifests current.
