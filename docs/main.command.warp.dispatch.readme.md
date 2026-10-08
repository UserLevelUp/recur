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

## Provider intervention and bounded retries

Recover the instruction lane with `recur reveal main.command.warp.dispatch.retry`.
New execution failures record a `failure` object with instruction `trace_id`,
Warp-scoped `attempt_trace_id`, category, action and nullable `retry_at_unix`.
Authorization and unsupported-client/model markers pause automatic dispatch until
explicit recovery after verified intervention. Transient and unknown execution
errors retry with exponential backoff, still bounded by `max_attempts`. The
coordinator keeps polling a pending retry instead of treating its delay as idle.
Configured assertion failures keep their existing intelligence-feedback behavior.

```toml
[warp.dispatch.retry]
delay_seconds = 5
max_delay_seconds = 60
auth_required_markers = ["AUTH_REQUIRED", "UNAUTHENTICATED", "LOGIN_REQUIRED", "INVALID_GRANT", "Please set an Auth method"]
provider_blocked_markers = ["UNSUPPORTED_CLIENT", "MODEL_NOT_FOUND", "UNSUPPORTED_MODEL"]
transient_markers = ["RATE_LIMIT_EXCEEDED", "RESOURCE_EXHAUSTED", "SERVICE_UNAVAILABLE", "ECONNRESET", "ETIMEDOUT"]
```

Missing retry policy uses these defaults; init adds missing fields without
overwriting custom marker lists. Delay must be positive, no larger than the cap,
and capped at 3600 seconds. Lists may be empty to disable a marker category.
Parsing is advisory; inspect retained diagnostics. No automatic provider fallback,
authentication flow or model permission bypass occurs. Older attempt records are
preserved; this policy is attached to new attempts.

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
