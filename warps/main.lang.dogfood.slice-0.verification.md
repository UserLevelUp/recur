# Lang dogfood baseline observation — 2026-09-10

Contract: `contract:main.lang.dogfood.slice-0:v1`.
Gate: `baseline-and-specification`.

Observed on Windows from `C:/src/recur`, branch `recur-lang`, with local
`target/release-safe/recur.exe`. Existing Reveal changes are uncommitted;
this observation does not claim that HEAD alone reproduces this binary.

```powershell
$env:RECUR_BIN='C:/src/recur/target/release-safe/recur.exe'
& C:/Users/marcn/AppData/Local/Microsoft/WindowsApps/julia.exe --startup-file=no --compiled-modules=no --compile=min -O0 --project=demos/web-evidence-lab -e 'include("julia-tests/main.demo.lang-inspector.test.jl"); include("demos/web-evidence-lab/main.server.test.jl")'
```

Exit 0. Observed summaries:

```text
Lang inspector: specification before implementation | 33 / 33 pass
main.server Julia HTTP contract                    | 33 / 33 pass
```

The server test started an ephemeral loopback server and closed it afterward.
No existing source implementation was changed for this baseline.

```powershell
& target/release-safe/recur.exe lang check main.greeting.recur -d demos/web-evidence-lab --json
& target/release-safe/recur.exe lang show main.greeting.recur --scope greeting.g -d demos/web-evidence-lab --json
```

Both exited 0. Source fingerprint: `fnv1a64:9a4f9796c1e5d4ef`.
The scoped report resolved `greeting.g`, `greeting.i(a)`, `greeting.o(b)` and
binding `MainServer.greeting`. Check: `sound-within-coverage`, no findings,
`whole_source_validated: false`, capability diagnostic `LANG101`.
Excluded coverage includes runtime behavior, receipt contents, worker semantics
and whole-document composition. Prose expectations have not been executed by
Lang. The greeting specification is retrospective; new integration tests are
still future work under slice-2.

The slicing prompt reported omitted context due to its explicit file/byte
bounds. Planning also used direct reads of the existing inspector source/spec,
server source/tests and the previous Lang baseline map. Prompt output was
guidance only, not complete repository evidence or acceptance.

No full regression, new API route or browser UI is claimed by this gate.

produces: main.lang.dogfood.baseline observed starting behavior
consumes: main.greeting.api.response existing API expectations
