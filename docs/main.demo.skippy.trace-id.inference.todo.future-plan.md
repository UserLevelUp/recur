# Demo: Skippy trace-id Inference

Status: `todo.future-plan`
Date: 2026-04-06

## Purpose

Track the future layer that sits on top of the current Skippy adaptive-comms
demo:

- `recur trace-id` provides the auditable role view
- protocol files remain the queryable source material
- a thin inference or rendering layer turns that trace into a final Skippy
  opening or adaptive reply bias

This is intentionally future work. The current demo proves traceability; this
lane captures how to turn that into actual Skippy-side inference.

## Current Truth

- The tracked demo in `demos/skippy-adaptive-comms/` already uses the real
  `recur trace-id` command on plain-text protocol files.
- The current output is still raw trace data plus human interpretation.
- The LLM still performs the final synthesis step manually.
- The current proof is shell-first, but the likely productized path is
  Sudoku-style: Julia builds the demo-local `.recur/` folder and can later host
  a browser flow that runs `recur` for the UI.

## Future Goal

Move from:

```text
protocol files -> recur trace-id -> human/LLM reads JSON -> final opening
```

To:

```text
protocol files -> recur trace-id -> thin inference layer -> suggested opening components
```

Where suggested components may include:

- preferred jab family
- preferred boast family
- preferred timewaster lament family
- bite level
- approval or caution bias
- active relationship-phase override

## Design Constraints

- Keep protocol truth in plain text files, not buried in prompt-only memory.
- Do not make `trace-id` responsible for natural-language generation.
- Let `trace-id` classify and expose roles; let a separate wrapper map those
  roles to Skippy opening choices.
- Preserve configurability: other projects can rename cues and tone families via
  their own text protocol and `.recur/config.toml` trait vocabulary.
- Keep the inference layer inspectable enough that a user can understand why a
  certain jab family was selected.

## Candidate Input Pattern

Possible stable identifiers:

- `skippy.case.*`
- `skippy.relationship.*`
- `skippy.tone.*`
- `skippy.jab.family.*`
- `skippy.boast.*`
- `skippy.lament.*`
- `skippy.trigger.*`

Possible wrapper rule:

1. trace the active `skippy.case.*` identifier
2. collect produced jab/boast/lament identifiers
3. collect relationship-phase overlays
4. collect triggered tone modifiers
5. rank or filter the candidate opener parts
6. return a compact suggestion object for the LLM to render

## Suggested Output Contract

The first useful wrapper output could stay machine-readable:

```json
{
  "case": "skippy.case.strong.insight",
  "relationship_phase": "skippy.relationship.playful.precise.current",
  "jab_family": "skippy.jab.family.annoyingly.correct.mammal",
  "boast_family": "skippy.boast.unfair.advantage",
  "lament_family": "skippy.lament.cosmic.paperwork",
  "tone_triggers": ["skippy.trigger.grudging.respect"]
}
```

That keeps the inference auditable before adding any direct text rendering.

## Phase Sketch

### Phase A

- freeze one minimal suggestion-object schema
- define deterministic ranking when multiple jab families appear
- define fallback behavior when no clear case is active

### Phase B

- add a tiny wrapper script or helper that runs `recur trace-id`
- emit suggestion JSON from current demo fixtures
- add focused tests for ranking and fallback
- prefer a Julia helper or service boundary if this starts looking like a real
  demo/app instead of a shell walkthrough

### Phase C

- add optional text rendering on top of the suggestion object
- verify the rendered opening still respects relationship phase and safety
  rules
- keep raw suggestion output available for debugging

## Exit Criteria

- a wrapper can convert protocol files into a deterministic suggestion object
- test cases prove different Skippy cue families map to different outputs
- relationship phase actually changes the suggested tone
- users can inspect why a suggestion was made without reading the whole prompt

## Repro Seeds

```powershell
recur files "main.demo.skippy.trace-id.**" -d docs/
recur trace-id "skippy.case.separator.correction" --scope "skippy.**" --ext .txt --json -d demos/skippy-adaptive-comms/
recur trace-id "skippy.relationship.playful.precise.current" --scope "skippy.**" --ext .txt --json -d demos/skippy-adaptive-comms/
```

## Related

- `docs/main.demo.skippy.trace-id.todo.current.md`
- `docs/main.command.trace-id.readme.md`
- `docs/main.demo.sudoku.trace-id.todo.current.md`
- `demos/skippy-adaptive-comms/`
- `.recur/skippy/skippy.persona.adaptive-comms.test.recurring.md`
