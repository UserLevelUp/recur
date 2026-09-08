# Recur Lang baseline for a.0.2.8

Status: planned; six pending slices, no implementation acceptance recorded.
Warp: `main.lang.baseline`.

Deliver the first useful Recur Lang query experience: select a bounded part of
work, understand its inputs/functions/outputs through compact symbols, inspect
header/body/footer detail, and see relevant Eventness and dependency findings.
Recur Lang is the tightest formal layer in Recur; the surrounding project keeps
its flexible names, files and conventions. All opinionated language behavior
belongs in `recur-lang`, including read-only advice and workflow selection.

## Starting point

- `recur-lang warp` already provides a bounded receipt-backed transition.
- WIR1 and CIR1 exist in `src/recur_lang_ir.rs` and
  `src/recur_lang_concurrent_ir.rs`; they are distinct versioned boundaries.
- Core `recur lang` is still a proposed CLI family.
- `main.improvement.30.static-graph` is an existing pending prerequisite Warp.
  It owns the shared analyzer over CIR1. This baseline consumes its result;
  it does not duplicate its implementation slices or inherit acceptance.
- The Julia language demos supply reference behavior within their documented
  prototype limits. They are not proof that the proposed Rust query CLI ships.

## Slices

| Slice | Observable result | Depends on |
| --- | --- | --- |
| slice-0 | Reproducible baseline and exact initial command/schema contract | none |
| slice-1 | Accepted SGR1 is usable through a source-bound query projection | slice-0 and reviewed static-graph prerequisite |
| slice-2 | Pure `recur lang list` and `show` discover and explain supported symbols | slice-1 |
| slice-3 | Compact and expanded header/body/footer reports preserve exact contracts | slice-2 |
| slice-4 | Scope/Eventness filtering and `check` expose relevant boundary and graph findings | slice-3 |
| slice-final | Regressions, companion boundary, docs and packaged CLI smoke checks pass | slice-4 |

Read `main.lang.baseline.contract.md` for each gate's concrete checks and failure
cases. The map records slice dependencies within this bubble. The cross-bubble
SGR1 prerequisite is a review gate, not an automatically enforced dependency
merely because metadata names it. Inspect the child projection and its exact
contract before accepting slice-1.

## First action

Read the contract, inspect current binaries/source, and capture the existing
Rust/Julia baseline. Freeze the bounded source-version support and initial
command schema in slice-0. Resume the existing static-graph Warp for analyzer
implementation before baseline slice-1 can be accepted.

```powershell
recur reveal main.lang.baseline
recur warp show main.lang.baseline -d .
recur warp slices main.lang.baseline -d .
recur warp slices main.improvement.30.static-graph -d docs
```

Use repository root `-d .` for acceptance so both `warps/` and the prerequisite
in `docs/` are available. No test execution, implementation or release is implied
by creating this map. Live coordination, a grid UI, new language grammars and
general subsystem imports require later contracts.

defines: recur.lang.baseline first useful scoped formal query experience
consumes: recur.lang.static.graph.report.v1 shared graph findings
consumes: recur.lang.warp.ir.v1 existing bounded Warp model
consumes: recur.lang.concurrent.ir.v1 existing concurrent model
produces: main.lang.baseline.plan bounded a.0.2.8 implementation and acceptance plan
