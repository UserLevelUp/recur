---
name: recur-warp
description: Plan, dispatch, review and accept Recur Warp slices using live gate evidence and companion-owned policy. Use for Recur Warp work, not generic agent orchestration.
---

# Recur Warp

Resolve the project root and inspect the chosen executable's help. Installed
and local builds may differ. Core `recur warp` queries remain read-only;
`recur-warp` owns policy, publication and dispatch. In the Recur repository,
read the relevant sections of `docs/main.command.warp.readme.md` and
`julia-expert/references/recur-playbook.md`; external projects use their own
contracts and capsules.

Start with `recur warp list -d ROOT`, then `show WARP` and `slices WARP`.
Read the map, selected slice contract, dependencies and required gates. Treat
Eventness and declared evidence as recorded claims until their supporting
observations are checked. Keep child acceptance and parent integration separate.

Use `recur prompt warp.naming|warp.slicing|warp.recovery` for bounded guidance
when useful. Prompt packets prepare context; they do not execute or grant authority.
Preview writer commands and preserve the user's already authorized scope.
`recur-warp init --dry-run` reveals missing companion defaults; preserve
existing host choices, test commands and explicit policy opt-outs.
For skill discovery, `recur-reveal init --dry-run` previews missing registry
defaults. `--local -d ROOT` selects a nested root's own config; otherwise init
uses the nearest existing config. It registers pointers, not skill bodies.

For delegated slices, read `docs/main.command.warp.dispatch.readme.md` in this
repository. Configure exact `WARP.SLICE` assignments with existing, root-bounded
context and inputs, scoped test argv and disjoint writable workspaces. Verify
local host/model support before enabling an adapter. Core cannot set reasoning
policy: host level lists, baseline, slice ticks and bounded feedback belong to
the companions. Execution failures are not test-failure intelligence signals.
Lang design is optional; it must not become a requirement for ordinary work.

```powershell
recur-warp llm plan WARP --slice SLICE -d ROOT --json
recur-warp dispatch WARP -d ROOT --json
recur-watch dispatch WARP --confirm -d ROOT --cycles 10 --json
recur warp dispatch WARP -d ROOT --json
```

Read actual attempts and test outputs. `produced` means the configured checks
succeeded under recorded stable inputs, not that a gate is accepted or tests
are sufficient. Review the implementation and relevant integration behavior,
then preview `recur-warp complete` with explicit attempt, result hash and
gate evidence; publish within the authorized task and re-query live progress.
Do not rewrite accepted historical layers to make new results look current.

For stale evidence under an unchanged contract, inspect `refresh --help` and
`docs/main.command.warp.evidence-refresh.readme.md`. Changed contracts need
their own assessment. For interrupted workers, inspect `recover --help`;
recovery preserves attempts and does not terminate a live process. Do not
launch retries indefinitely or remove locks without checking their ownership.

If a slice concerns a demo, select only that demo's tests. In this repository
use `julia julia-tests/runtests.jl --demo NAME`; add core coverage explicitly
when the change also affects core. Report observed tests, evidence limits,
pending gates and the next useful action without inferring automatic completion.
