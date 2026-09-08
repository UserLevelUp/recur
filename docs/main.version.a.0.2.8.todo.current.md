# a.0.2.8 release preparation

Status: current preparation; release not finalized.
Updated: 2026-09-08.

The intended release target for the current Recur updates, including Recur Lang,
is `a.0.2.8` / package version `0.2.8`. Recur Lang remains under development;
build and accept its bounded Warps before choosing the final release cutoff.
The historical `main.version.a.0.2.8.complete.md` record does not certify the
current candidate or a Chocolatey submission.

## Observed integration state

Commit `d291d65` added the Julia evidence lab, refreshed expert guidance and
clarified the Recur Lang scope. It was pushed to `recur-lang`, then `a.0.2.8`
was fast-forwarded and pushed to that same commit after fetching remote state.
The workspace switched to `a.0.2.8` for the baseline Warp setup.
Reassess branch heads and the working tree before release; this is a recorded
integration checkpoint, not a live status assertion.

The refreshed demo run `20260908T085126685355Z` passed 80 browser checks and its
Julia server suite passed 33 assertions. The board passed ten selections and
31 artifact links. Lab attributes preserve the exact bytes covered by evidence
fingerprints. These scoped checks do not establish Recur Lang release acceptance.

## Lang work

Start with `warps/main.lang.baseline.warp-map.json` and its contract/readme.
It covers the first pure query surface, compact header/body/footer explanation,
scope and Eventness filtering, formal checks and packaged Lang smoke tests.
It consumes `docs/main.improvement.30.static-graph.warp-map.json` as an existing
prerequisite with separate parent acceptance. All baseline slices start pending.
Grid snapshots, live coordination and broader language grammars remain later work.

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
