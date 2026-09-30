# Lang verification demonstrations

defines: recur.lang.verification.demo.guide walkthrough of tests and implementation slices
consumes: recur.lang.verification.tests executable fault demonstrations

The lab maps **all 32 verification todos** to targeted executable demonstrations,
existing suites, remaining test work and an implementation slice. A catalog entry
does not claim that all its proposed cases exist or pass.

From `C:\src\recur`, select explicit matching binaries. On this machine the
Chocolatey `julia` on PATH resolves to 1.12.0 and crashed during preparation;
Juliaup resolves to 1.12.7 and completed the red run. Other machines should use
their verified Julia 1.12-compatible executable and instantiate the web-lab
project before running these commands.

The original tests-first demonstration completed with `-O0`: 58 cases, 44 passing
and 14 deliberately failing, with no errors/broken/empty cases. Normal
optimization and `--compile=min` later faulted inside Julia; their logs are
preserved separately. See the [observed baseline](../../warps/main.lang.verification.baseline.md).

```powershell
$juliaExe = 'C:\Users\marcn\AppData\Local\Microsoft\WindowsApps\julia.exe'
$env:RECUR_BIN = 'C:\Users\marcn\.cargo\bin\recur.exe'
$env:RECUR_LANG_BIN = 'C:\Users\marcn\.cargo\bin\recur-lang.exe'
$lab = 'demos/lang-verification/main.lang.verification.jl'

# Explain all todos or one concern; list exact runnable case IDs.
& $juliaExe --startup-file=no -O0 -C generic --project=demos/web-evidence-lab $lab catalog
& $juliaExe --startup-file=no -O0 -C generic --project=demos/web-evidence-lab $lab catalog V15
& $juliaExe --startup-file=no -O0 -C generic --project=demos/web-evidence-lab $lab list

# Show the sound graph and deliberately introduced cycles.
& $juliaExe --startup-file=no -O0 -C generic --project=demos/web-evidence-lab $lab demo graph.

# Demonstrate status corruption and path-helper failures.
& $juliaExe --startup-file=no -O0 -C generic --project=demos/web-evidence-lab $lab demo checked.status.
& $juliaExe --startup-file=no -O0 -C generic --project=demos/web-evidence-lab $lab demo path.oracle.

# All new cases; choose a NEW observation filename on each run.
& $juliaExe --startup-file=no -O0 -C generic --project=demos/web-evidence-lab $lab demo --json-output work/lang-verification-attempt.json
$LASTEXITCODE

# Existing suites run separately and retain their actual exit code.
& $juliaExe --startup-file=no -O0 -C generic --project=demos/web-evidence-lab $lab suite init
& $juliaExe --startup-file=no -O0 -C generic --project=demos/web-evidence-lab $lab suite loader
```

Point both binary variables at a new candidate to retest it. The seven-binary
smoke reads siblings of `RECUR_BIN`; it can smoke an extracted package directory
but does not yet validate the archive contents or native CI.

| Prefix | Demonstration |
| --- | --- |
| `query.` | Configured suffixes, CIR empty recorded state, simultaneous states, errors, linked containment and inert bindings |
| `graph.` | Exact nodes/edges/waits, positive control, self/dependency/wait cycles and missing join under scoped queries |
| `parser.` | Comments, malformed declarations/policies, ambiguity codes and Unicode/LF/CRLF byte spans |
| `actor.arguments.` | Invalid combinations reject without mutation |
| `checked.control.` | Real fixture assessment, pure preview, confirmed temporary transition and exact replay |
| `checked.status.` | Four identity mutations must invalidate accepted status |
| `checked.conflict.` | Corrupt durable records must reject confirmation and recovery |
| `checked.drift.` | Every assessed input role invalidates current acceptance on change |
| `checked.attempt.` | 80/81-character, empty, dot and Unicode ID boundaries |
| `path.oracle.` | Adjacent/reversed routes pass; disconnected, malformed, empty and singleton routes fail |
| `delivery.` | Help/version for seven selected binaries |
| `catalog.` | Every checkbox maps to real artifacts/slices and no new case is orphaned |

Product demonstrations use isolated temporary fixtures. Synthetic producer
receipts test qualification; they never certify this repository. Deliberate
product failures exit nonzero. The lab uses no Python, changes no implementation,
starts no watcher and executes no Lang binding. All 58 cases now pass;
the suite is included in `julia-tests/runtests.jl`; historical red
observations remain unchanged.

Other existing suite names: `baseline`, `language`, `inspector`, `dogfood`,
`protomap`, `api`, `holdem`, and the expensive `holdem-exhaustive`. Listing a suite
does not establish a pass. Native follow-up commands include:

```powershell
cargo test --locked --test lang_query --test lang_checked --test lang_checked_transition --test lang_checked_reentry
cargo test --locked --test warp_refresh --test warp_refresh_bounds
cargo test --locked --lib recur_lang
```

Use the repository's proven toolchain/profile workaround if the compiler faults;
preserve that fault separately from assertion failures.

## Work ownership

- [Verification contract](../../warps/main.lang.verification.contract.md):
  bounded repairs and explicit tests-first prerequisites for broader coverage.
- [Init contract](../../warps/main.lang.init.contract.md): additive configuration
  initialization, demonstrated with `suite init`.
- [Runtime evidence](../../warps/main.lang.runtime-evidence.contract.md): existing
  loader, then binding/UI/integration; acceptance is separate.
- Advanced [identity](../../warps/main.lang.advanced.identity.contract.md),
  [grid](../../warps/main.lang.advanced.grid.contract.md),
  [watch](../../warps/main.lang.advanced.watch.contract.md),
  [retry](../../warps/main.lang.advanced.retry.contract.md),
  [scatter/gather](../../warps/main.lang.advanced.scatter.contract.md) and
  [imports](../../warps/main.lang.advanced.imports.contract.md) each have a separate
  contract-first Warp. Their feature semantics and executable tests are deferred.

Planner/scaffolding proposals remain future work. The existing Hold'em graph
test demonstrates a cycle hidden behind a Julia binding; static Lang analysis
does not yet discover arbitrary implementation behavior.

```powershell
recur warp slices main.lang.verification -d .
recur warp show main.lang.advanced.retry -d .
recur trace-id 'recur.lang.verification.**' --scope 'main.lang.verification.**' -d warps --format full
recur trace-id 'recur.lang.verification.**' --scope 'main.lang.verification.**' -d demos/lang-verification --format full
```
