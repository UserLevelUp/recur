# Lang runtime evidence: baseline and contract

Date: 2026-09-12. Historical planning task, subsequently accepted via recur-warp.
See main.lang.runtime-evidence.slice-0.verification.md and the current handoff;
this original task is retained for its starting observations.
Warp: `main.lang.runtime-evidence`. Slice: `slice-0`.
Contract: `contract:main.lang.runtime-evidence.slice-0:v1`.
Gate: `baseline-and-contract` (declared planning observations).

## Intent

Move from general project specificity to one exact Lang specification and its
test evidence. Freeze the association, compatibility and path boundaries before
writing implementation. See [the full contract](main.lang.runtime-evidence.contract.md).

## Known starting observations

- Baseline and dogfood maps reported complete with declared evidence on 2026-09-12.
- The API intentionally returns no observed evidence; query footers do not check
  receipt contents. A native Lang ACK is not checked external test evidence.
- The readiness analysis passed three Lang query tests and one evidence-checker
  test. It did not rerun Julia/browser suites. These are historical observations,
  not acceptance of this new slice.
- `main.web-lab` reported ten stale/blocked slices. Preserve the old observations
  and investigate relevant current inputs instead of editing receipt fingerprints.
- The `warps/` map root cannot reference `../src` or `../demos` in checked
  evidence. An actual supported layout decision is still required.

## Next bounded work

1. Discover the current lane and read Improvement 30 plus the linked query/API
   contracts; inspect the owning loader/checker and neighboring tests.
2. Reproduce the applicable baseline using the runtime guidance in the dogfood
   final verification. Record tool/binary identities and any current failures.
3. Resolve the development Warp evidence-root layout and fixed product catalog
   roots. Do not accept snapshots as live-source checks or relax containment.
4. Freeze explicit scope/slice/test associations, red/green history, freshness
   outcomes, compatibility, path limits and the intended compact/expanded views.
5. Record observations and remaining decisions. Only then may slice-0 be accepted
   and slice-1 begin; writing this file does not satisfy the gate.

```powershell
recur warp show main.lang.runtime-evidence -d . --json
recur warp show main.web-lab -d demos/web-evidence-lab --json
recur lang report main.lang.api.recur --scope query.q -d demos/web-evidence-lab
cargo test --locked --test lang_query
cargo test --locked --lib warp_evidence::tests
```

The workflow also uses tree/files/trace-id/source trace and recorded context
recovery as appropriate. `recall` is not an assumed shipped command.
Do not start GRID0/COORD, alter legacy receipt contracts, refresh all old Warps,
or implement new syntax as part of this baseline.

consumes: main.lang.runtime-evidence.contract.v1 slice-0 boundary
produces: main.lang.runtime-evidence.baseline planned baseline and frozen association contract
