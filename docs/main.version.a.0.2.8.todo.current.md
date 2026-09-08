# a.0.2.8 release preparation

Status: current preparation; release not finalized.
Updated: 2026-09-08.

The intended release target for the current Recur updates, including Recur Lang,
is `a.0.2.8` / package version `0.2.8`. Recur Lang's bounded query baseline is
implemented; inspect its live acceptance before choosing the final release cutoff.
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
prerequisite with separate parent acceptance. The verification record binds the
candidate implementation, actual tests and archives to its gates.
Grid snapshots, live coordination and broader language grammars remain later work.

## Candidate packaging

- `.github/workflows/release.yml` packages all six declared binaries and tests
  their extracted help/version, pure Lang queries and companion dry run. Windows
  uses `release-safe`; Linux uses `release`, both with Rust 1.85.0.
- Chocolatey install/uninstall includes `recur-lang`; the actual packed scripts
  were exercised with mocked helpers and the candidate archive checksum.
- `warps/main.lang.baseline.verification.md` records the zip, tar.gz and nupkg
  checksums. The nupkg requires its exact zip at the declared release URL.
- A manual Release workflow builds and tests candidates without publishing.
- The release workflow currently submits to Chocolatey automatically after a
  release-tag push. Reconcile that with the intended separate submission step
  before triggering a release.
- `VERSION`, Cargo metadata, README and nuspec already identify 0.2.8.

## Release exit evidence still required

Choose the final release cutoff and inspect the baseline's acceptance and candidate
checksums. Rebuilding the zip requires repacking its checksum-bound nupkg. The
manual candidate workflow does not build a Debian package; verify that separately
if it is part of the final submission. Publication and Chocolatey submission
remain later actions; no release tag or submission was made for this baseline.

defines: main.version.a.0.2.8.preparation current release target and outstanding integration evidence
consumes: main.improvement.30.static-graph.warp Recur Lang foundation acceptance
consumes: main.choco.todo.current package maintenance lane
