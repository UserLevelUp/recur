# Lang builds a Lang inspector

This demo uses a high-level Lang specification to build a small Recur capability:
inspect a selected function's meaning, input/output contracts and evidence limits.
It queries **its own specification** using the real `recur lang` command.

The specification came first, followed by hand-authored mocks and acceptance tests,
then the Julia implementation. An LLM interpreted the specification to choose the
models and code; Recur did not generate or execute the implementation. This is a
worked example, not a comparative evaluation of LLMs or proof of reduced effort.

## Try it

From the repository root, use a binary containing the Lang baseline. An older
installed executable may report 0.2.8 while lacking `lang`.

```powershell
# Use the extracted candidate, or substitute your current build's executable.
$env:RECUR_BIN = "$PWD/target/lang-final/choco-smoke/tools/recur.exe"
& $env:RECUR_BIN lang show main.lang.inspector.recur --scope model.m -d demos/lang-inspector
julia --startup-file=no --project=demos/web-evidence-lab demos/lang-inspector/main.lang.inspector.jl demos/lang-inspector main.lang.inspector.recur model.m
```

The inspector shows `model.m`, including `model.i(b) = query.o(b)`. This makes the
shared packet contract visible without carrying its fields through every function.
The footer says execution is `not-run` and whole-source validation is `false`:
those describe the queried Lang specification, not whether this Julia program ran.

Try another existing capability with the same implementation:

```powershell
julia --startup-file=no --project=demos/web-evidence-lab demos/lang-inspector/main.lang.inspector.jl demos/main.lang main.lang.algorithm-lab.recur merge.f
```

To reproduce the acceptance checks:

```powershell
& $env:RECUR_BIN lang check main.lang.inspector.recur -d demos/lang-inspector
julia --startup-file=no --project=demos/web-evidence-lab julia-tests/main.demo.lang-inspector.test.jl
```

The Julia environment is shared with the existing evidence demos; it needs JSON3.
No additional server, installation, package or LLM call is involved in running this
demo. The tests are also included in `julia-tests/runtests.jl`.

## How the specification became models and tests

| Lang scope | Inferred implementation | Acceptance examples |
| --- | --- | --- |
| `query.q` | Query packet data model; injectable process adapter | Exact argument boundaries, failed command, malformed JSON, real scoped query |
| `model.m` | `FunctionCard` and `InspectorView`; supported-schema rules | Canonical alias retained, authored order, empty selection, unsupported IR, missing coverage, original packet preserved |
| `view.v` | Deterministic plain-text presentation | Meaning, boundary edges, findings and coverage visible; recorded completion never called acceptance |

Here the domain is inspecting declared contracts: it does not need an artificial
class hierarchy. `QueryPacket` is represented by the existing CLI JSON schema;
the view contains typed function cards plus a copy of the entire original packet.
Bindings name responsibilities; choosing Julia structs, dictionaries and an
injected adapter was an implementation decision, not a new Lang grammar feature.

Natural-language expectations in the body explain intended behavior to an LLM.
The bounded parser checks the supported declarations and aliases; Julia tests
check the implemented behavior. These are separate sources of evidence.

## Observed development evidence

- Before implementation, the acceptance suite failed its missing-implementation
  assertion. Remaining behavior assertions were not yet exercised.
- The first specification check returned `RLIR010`: completion transitions lacked
  matching declared state events. Adding those declarations corrected the spec.
- The corrected specification passed `lang check` within WIR1 coverage.
- The implementation then passed all 33 assertions, including real self-query,
  exact alias identity, and an unchanged demo-file inventory.
- After the final display changes, the 33 demo assertions and 15 existing Lang
  CLI assertions passed together. Normal compilation hit a Julia/LLVM access
  violation; generic CPU targeting also failed internally during compilation.
  The successful combined run used Julia 1.12.7 with
  `--startup-file=no --compiled-modules=no --compile=min -O0`, the existing
  `demos/web-evidence-lab` environment and the extracted Windows candidate.
  These compiler failures are not passing test evidence. Full repository
  regression was not rerun for this demo-only change.

This demo intentionally accepts WIR1 packets only. CIR1 fan-in has a different
contract shape and is rejected explicitly. No runtime acceptance, automatic test
generation, skill loading or worker execution is claimed. The complete query
packet remains available as `view.packet` for a future Julia server adapter.

defines: demo.lang.inspector specification-first inspection capability
consumes: recur.lang.query.v1 source-bound query packet
produces: demo.lang.inspector.view function cards and original evidence
consumes: demo.lang.inspector.tests behavioral acceptance
