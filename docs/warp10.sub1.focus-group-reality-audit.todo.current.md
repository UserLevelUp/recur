# Warp 10 Sub-1: Focus-Group Reality Audit

## Scope

Bind Warp 10 to the actual game workspace and playable build.  Reconcile the
focus-group observations with the current source and capture a precise E0
before implementation begins.

## Work

- Locate and record the owning Godot project, branch, base revision, launch
  scene, map scene, site scene(s), dialogue UI, navigation code, autoloads,
  input actions, save model, and tests.
- Collect the available focus-group requests and classify each as verified,
  already addressed, partially addressed, not addressed, contradictory, or
  not reproducible.
- Run the current playable loop and record map manipulation, scene count,
  transition behavior, dialogue behavior, idea acquisition, focus behavior,
  debugger output, and save/load reality.
- Identify coupling or asset constraints that require the later slice
  contracts to change.

## Invariants

- Do not infer behavior from filenames, mockups, or focus-group prose alone.
- Do not restructure scenes while establishing the baseline.
- Preserve unrelated local work and private participant information.
- Record unknown or unavailable runtime evidence honestly.

## Acceptance evidence

- Exact project path/repository, branch, commit, and dirty-worktree status.
- Focus-group residual matrix linked to observations or approved summaries.
- Scene/script/input/save ownership map.
- Current-build runtime receipt with screenshots or video references where
  useful and debugger observations.
- Proposed refinements to Sub-2 through Sub-7, without silently expanding
  their scope.

## Transition

On accepted evidence, rename this artifact to `.complete.md` and activate
Sub-2.  If the Godot project or playable build remains unavailable, record the
impediment and keep the slice current or move it to the configured blocked
state; do not claim an implementation baseline.
