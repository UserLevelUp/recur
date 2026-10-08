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
