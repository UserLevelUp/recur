# Checked transition: contract and baseline

Date: 2026-09-12. Slice-0 planning, not accepted.
Contract: `contract:main.lang.checked-transition.slice-0:v1`.
Gate: `contract-and-baseline` (declared observations).

Freeze the exact verified handoff between one WIR1 scope, external test evidence
and a confirmed companion transition. Read the full
[contract](main.lang.checked-transition.contract.md) and
[readme](main.lang.checked-transition.readme.md) first.

## Current knowledge and limits

At creation, live runtime-evidence progress showed slices 0/1 accepted, slice-2
pending and ready, and later slices pending. Its separate blocker records Julia
runtime failures. Its unaccepted loader is prior art, not an accepted dependency.
Earlier focused Rust tests and a companion dry run passed; these are historical
observations, not a baseline run for this new contract. Re-query/reassess.

The existing companion validates receipt identity fields but only requires
nonempty artifact/test_receipt references. No production checked transition
schema or new evidence query option has been selected yet.

## Next bounded work

1. Recover live state and inspect current query/companion help, source and tests.
2. Record the focused Rust baseline and binary/source provenance.
3. Freeze the shared assessment and explicit checked-transition wire/CLI contract,
   including root/limits, diagnostic precedence, replay and interruption recovery.
4. Exercise the root layout against actual inputs and settle earlier-gate
   freshness/refresh handling before accepting any checked slice.
5. Write the hand-authored case matrix and record remaining uncertainties. Only
   then accept slice-0; start actual-boundary red tests before slice-1 code.

```powershell
recur reveal recur-expert
recur reveal recur-eventness
recur reveal main.lang.checked-transition -d warps
recur warp show main.lang.checked-transition -d . --json
recur warp slices main.lang.checked-transition -d .
recur tree main.lang.checked-transition -d warps
recur files "main.lang.checked-transition.**" -d warps
recur trace-id "recur.warp.evidence.integrity.external" --scope "**" -d src --format full
recur trace assess --scope warp_evidence -d src --sep _ --depth 1
recur-lang warp --help
recur lang --help
cargo test --locked --test lang_query
cargo test --locked --lib warp_evidence::tests
cargo test --locked --bin recur-lang
```

Read `src/warp_evidence.rs`, `src/recur_lang_query.rs`,
`src/recur_lang_main.rs`, `docs/main.command.lang.query.readme.md` and the shipped
companion sections of `docs/main.command.lang.readme.md`. Continue recorded
recovery through runtime-evidence's contract/blocker and Improvement 30 before
selecting the exact Lang view. Check help rather than assuming recall exists.

produces: main.lang.checked-transition.baseline planned contract freeze
