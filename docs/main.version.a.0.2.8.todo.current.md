# a.0.2.8 release preparation

Status: current preparation; release not finalized.
Updated: 2026-09-08.

The intended release target for the current Recur updates, including Recur Lang,
is `a.0.2.8` / package version `0.2.8`. Recur Lang remains under development;
build and accept its bounded Warps before choosing the final release cutoff.
The historical `main.version.a.0.2.8.complete.md` record does not certify the
current candidate or a Chocolatey submission.

## Observed integration state

On inspection, local `a.0.2.8` and `recur-lang` both pointed to `e3f7741`.
The workspace remains on `recur-lang`. Existing Lang implementation and the
committed Warp, prompt and reveal updates are therefore already in both local
branch histories. Remote-tracking refs matched locally; no remote refresh was
performed during this inspection.

Uncommitted expert guidance changes and `demos/web-evidence-lab/` still need
review and inclusion with the intended release changes. Reassess branch heads
and the working tree before integration; do not use this note as a live status.

## Lang work

Start with `main.improvement.30.static-graph.warp-map.json` and its readme.
The map organizes the existing active contract into pending slices. Later pure
queries, grid snapshots and coordination need their own bounded contracts;
their inclusion in 0.2.8 is not decided by this record.

## Packaging gaps observed

- `.github/workflows/release.yml` builds the crate but copies only `recur`,
  `recur-git` and `recur-warp` into Windows/Linux archives. Include `recur-lang`
  and review the omitted `recur-watch` and `recur-version` companions before release.
- Chocolatey install/uninstall scripts have no `recur-lang` shim handling.
- The nuspec needs the current Lang, Warp, prompt and typed-reveal surface.
- The release workflow currently submits to Chocolatey automatically after a
  release-tag push. Reconcile that with the intended separate submission step
  before triggering a release.
- `VERSION`, Cargo metadata, README and nuspec already identify 0.2.8.

## Release exit evidence still required

Integrate the intended work on `a.0.2.8`; build and test the selected release
candidate; inspect archive and nupkg contents; verify that packaged companion
commands run; bind package checksums to the final artifacts. Record actual
results before declaring release readiness. Publication and Chocolatey
submission remain later actions.

defines: main.version.a.0.2.8.preparation current release target and outstanding integration evidence
consumes: main.improvement.30.static-graph.warp pending Recur Lang foundation work
consumes: main.choco.todo.current package maintenance lane
