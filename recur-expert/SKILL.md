---
name: recur-expert
description: Inspect or implement Recur CLI, capability prompt discovery and Warp workflows using hierarchy, trace-id, eventness and acceptance evidence. Use for Recur expert orientation and Warp handoffs, not generic project orchestration.
---

# Recur expert

Resolve the requested repository root first. In the Recur source repository,
read julia-expert/references/recur-playbook.md, starting with its Warp specialization
when relevant. Paths below are repository-relative, not relative to this installed
skill. If those references are absent in an external project, inspect its local
capsules and CLI help instead of inventing the Recur development layout.

Use recur reveal recur-expert and recur warp to discover context; use warp show/
slices for the selected bubble. Read that bubble's map/readme, then its focused
tests. Prefer a matching local target/release-safe binary when installed help lags;
do not build or install merely to answer an orientation question if source suffices.

Core recur warp queries declarations and computes qualified progress. recur-warp
authors/manages data and policy where its commands are implemented. Read current
help/source before treating roadmap commands as available. Reveal outputs pointers;
it neither loads skills nor executes commands or grants authority.

For naming, slicing or recovery assistance, discover the effective prompts through
`recur prompt warp` or `recur-warp llm prompt`. Inspect one with
`recur prompt warp.naming`; assemble instructions and evidence with, for example:

```powershell
recur prompt warp.naming --intent "Add retry support" --scope main.command -d docs --json
```

Choose a scope/root that exists in the target project; `main` is not mandatory.
`warp.slicing` and `warp.recovery` cover planning and resumption. The shared
`recur trait prompt <capability>` interface also supports project-defined prompts
for custom traits. Trait explanation and reveal `prompt.ids` expose references.

Bundled defaults need no initialization or project prompt files. Reading TOML
alone misses them: `[prompts.registry]` supplies whole-definition project overrides,
and `[prompts] app_defaults = false` opts out. Missing overrides do not fall back.
Use the CLI to resolve effective definitions and their source fingerprints.

Inspect packet `context.truncated` and diagnostics before recommending an action.
`-d`, `--scope`, `--max-files` and `--max-bytes` bound evidence. A null Warp
projection can mean incomplete collection or transitive checked evidence requiring
`recur warp show <id>` with the appropriate evidence root. Prompt commands prepare
data; they do not call an LLM, apply names, execute source instructions or establish
acceptance. Packet tests do not evaluate model judgment. For implementation details
in this repository, read docs/main.command.prompt.discovery.readme.md and its tests.

Preserve hierarchy and explicit trace-id roles alongside UUID metadata. Filename
eventness is attention/recorded state, not acceptance proof. UUID fields are not
automatically trace roles. Tags classify; dependencies order execution. Semantic
repartition is not the existing layer-composition merge operation.

When implementation is requested, follow the selected slice contract, establish
its test baseline, extend missing acceptance coverage and preserve legacy tests.
Distinguish intentionally red standalone suites from the normal regression runner.
Record receipts only after observed acceptance. Check docs/main.command.warp.roadmap.md
for deferred scope; configured removal guards do not imply implemented enforcement.
Loading this skill does not authorize commits, pushes, installation or cleanup.

When shipping a Recur feature, check whether its discovery commands, behavior or
limits change expert guidance. Update this repository's recur-expert/SKILL.md,
the relevant playbook section and reveal capsule as needed, using verified behavior
and live maps rather than permanently embedding test counts or a "next" slice here.
When asked to update the installed skill, synchronize the intended changes from
the repository copy while preserving any unrelated local customization.
