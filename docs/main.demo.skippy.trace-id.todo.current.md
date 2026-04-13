# Demo: Skippy Adaptive Comms + trace-id

Status: `todo.current`
Date: 2026-04-06

## Goal

Show that adaptive persona selection can use the real `recur trace-id` command
over plain-text protocol files instead of relying only on an internal mental
rubric.

## Demo Layout

- tracked demo files live in `demos/skippy-adaptive-comms/`
- `.txt` files encode relationship phase and cue-specific adaptive cases
- `publish` lines emit jab family, boast, lament, or phase overlays
- `subscribe` lines show downstream selections
- `trigger` lines represent tone activation
- a local ignored `.recur/config.toml` is seeded from
  `trace-id.config.example.toml` because repo `.recur/` paths are gitignored

## What Landed

- relationship phase fixture:
  `skippy.relationship.playful.precise.current`
- cue fixtures:
  - `skippy.case.separator.correction`
  - `skippy.case.strong.insight`
  - `skippy.case.release.admin`
- PowerShell walkthrough:
  `demos/skippy-adaptive-comms/demo.ps1`
- Julia test lane:
  `julia-tests/runtests.demo.skippy.jl`

## Test Coverage

The demo test proves:

1. the tracked `.txt` files are discoverable via `recur files`
2. the active relationship phase is traceable as define/produce/trigger
3. separator-correction cues resolve to the expected jab family and tone
4. strong-insight cues resolve to a different jab family and approval tone
5. release-admin cues resolve to paperwork/package-oriented mockery

## Repro

```powershell
powershell -ExecutionPolicy Bypass -File demos/skippy-adaptive-comms/demo.ps1
julia julia-tests/main.demo.skippy.trace-id.test.jl
```

## Why It Matters

- proves that adaptive comms can move from "trace-id inspired" to actual
  `trace-id` execution
- keeps the selection logic auditable in plain text
- leaves the final synthesis step with the LLM, where it belongs
- keeps room for a Sudoku-style delivery later:
  Julia can build the demo-local `.recur/` folder and eventually host a browser
  flow that runs `recur` on behalf of the UI

## Next Useful Expansions

- add a caution-heavy or pink-mist case
- add a tired/supportive relationship phase
- optionally emit a tiny rendered opening from a wrapper script on top of the
  trace-id JSON
- replace the simple bootstrap path with a Julia-owned demo builder that creates
  the local `.recur/` folder for the demo
- follow the Sudoku pattern for optional Julia server + browser delivery
- track the wrapper/inference layer separately in
  `docs/main.demo.skippy.trace-id.inference.todo.future-plan.md`

## Related

- `docs/main.command.trace-id.readme.md`
- `docs/main.demo.skippy.trace-id.inference.todo.future-plan.md`
- `docs/main.demo.sudoku.trace-id.todo.current.md`
- `.recur/skippy/skippy.persona.adaptive-comms.test.recurring.md`
