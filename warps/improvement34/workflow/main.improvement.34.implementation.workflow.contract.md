# Workflow guidance and settled attention

artifact.type = contract
defines: main.improvement.34.implementation.workflow.contract
consumer: main.improvement.34.workflow
consumer: main.improvement.34.attention
consumer: main.improvement.34.skill_guidance
consumer: main.improvement.34.compatibility
warp.id = main.improvement.34.implementation.workflow
warp.uuid = 01a1ba42-9d6d-756c-bd82-762ae3a6b195
status = planned; no implementation or acceptance

## Initial Eventness

Completion already supports preview, confirmation and matching-attempt retry. Checkpoint can display and append together. A completed recorded slice currently produces a stale-selection warning.

Inspect: src/main_command_warp_impl.rs, src/recur_warp_main.rs, src/recur_git_main.rs, existing Warp usability tests and focused skills. Use the [root proposal](../../../README.CORE.IMPROVEMENT34.md) and [integration plan](../main.improvement.34.implementation.initial.current.md).

Predecessor handoffs: None beyond reviewing the baseline. These are cross-bubble review prerequisites, not automatic scheduler edges. Confirm live child acceptance and inspect the relevant interfaces before applying dependent changes.

## Slices and gates

### baseline — scope and expected-red review

Gate `scope-and-expected-red-reviewed` is a declared review. Confirm the active contract and exact source inputs, run the relevant current Julia baseline, and capture a standalone expected-red run for behavior that is missing. A missing binary or malformed invocation does not establish a behavioral counterexample. Preserve failed runs and explain the cause. No passing test is inferred from this gate.

### guide — Teach the supported short workflow

Checked test gate: `guide-julia` (no skipped checks). Exercise a clean-room walkthrough: scoped lineage, plan, preview, checked completion, combined snapshot/append and handoff. Distinguish UUIDs, semantic IDs and paths. Verify exact rejection examples; document append semantics and optional lifecycle receipts. Leave installed skill customizations alone.

### attention — Separate settled attention from stale or invalid selection

Checked test gate: `attention-julia` (no skipped checks). Keep legacy current_slice meaning and expose recorded value and reason additively. Test completed selection, unknown ID, real blockers/conflicts, multiple ready slices, and stale next notes without map mutation.

### update — Confirm explicit companion selection changes

Checked test gate: `update-julia` (no skipped checks). Preview an explicit clear or selected ready slice. On confirm revalidate the map and readiness; reject stale preview and unknown or not-ready choices. Never select automatically or rewrite acceptance layers.

### verify — Verify workflow and compatibility

Checked test gate: `verify-julia` (no skipped checks). Run completion retry and rejection cases, the combined checkpoint fixture, the beginner projection suite and the walkthrough against legacy maps. Document current behavior and future policy separately.

Add the selected Julia CLI cases to `julia-tests/main.improvement.34.workflow.test.jl` during implementation. Proposed focused command:

```powershell
julia --project=demos/web-evidence-lab julia-tests/main.improvement.34.workflow.test.jl
```

Use temporary synthetic fixtures and the matching local executables. The Julia project supplies dependencies; no demo is selected. Keep expected-red work out of the normal runner. Capture executed checks, real result counts, commands, and actual source/config/executable identities; use checked evidence manifests rooted at the repository. See [evidence contract](../../../docs/main.command.warp.evidence.readme.md).

## Predicted final Eventness

Users have a runnable short workflow and understandable attention projection. The displayed ready selection and original recorded selection remain separate; optional updates are confirmed and guarded.

This is the prediction for this bubble. Record observed results, residual uncertainty and deviations separately. If a slice or the bubble falls short, retain the failure and its cause so a subsequent Warp can make a better prediction. Parent acceptance requires a separate integration assessment.
