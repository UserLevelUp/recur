# Inspect a greeting, then inspect its inspector

From the repository root on Windows, select the local Lang-capable Recur binary
and the Julia runtime. This run used Julia 1.12.7. The demo project pins HTTP and
JSON3; instantiate it if this is a fresh environment.

```powershell
$env:RECUR_BIN = "$PWD/target/release-safe/recur.exe"
$juliaExe = 'C:/Users/marcn/AppData/Local/Microsoft/WindowsApps/julia.exe'
& $env:RECUR_BIN lang show main.greeting.recur --scope greeting.g -d demos/web-evidence-lab
& $juliaExe --startup-file=no -C generic --compile=min --project=demos/web-evidence-lab demos/web-evidence-lab/main.greeting.fixtures.test.jl
& $env:RECUR_BIN lang report main.lang.inspector.recur -d demos/lang-inspector
& $juliaExe --startup-file=no -C generic --compile=min --project=demos/web-evidence-lab demos/web-evidence-lab/main.server.jl --port 8765
```

Open `http://127.0.0.1:8765/lang.html`. Select **Greeting boundary → greeting.g**.
The header declares one URI input and one response output. The body composes
`i(a) → g(a) → o(b)`. The footer describes events and requested state; it does
not report a test pass. The greeting specification was retrospective.

The fixture table shows a deliberate distinction: 40 `界` characters pass even
though they occupy 120 UTF-8 bytes. 21 `e` + combining-acute graphemes fail because
Julia counts 42 characters. To exercise the simpler boundary directly:

```powershell
Invoke-RestMethod 'http://127.0.0.1:8765/api/greeting?name=%20Jos%C3%A9%20&locale=es'
# message: Hola, José!
Invoke-WebRequest 'http://127.0.0.1:8765/api/greeting?name=' -SkipHttpErrorCheck
# status 400, exactly {"error":"Invalid name"}
```

Select **Lang inspector → model.m**, then **query.q** and **view.v**. The model
input retains `model.i(b) = query.o(b)`; the view similarly consumes `model.o(c)`.
The CLI report above shows all three stages and their boundary edges together.
Expand bundle declarations and the complete packet to inspect exact source hash,
canonical identities, findings and excluded coverage. Source text is inert.

Compare `main.lang.api.recur` with `main.lang.api.contract.md` and the page:
bundles live in the header, symbol composition in the body, events/state in the
footer. The HTTP contract, catalog choices, fixture cases and small adapter model
were human/LLM interpretations. They were not mechanically generated proof.
The adapter reuses the existing FunctionCard/InspectorView; the greeting needs
no extra request/domain/view structs. API fixtures were written before routes.

Static findings, recorded filenames and observed tests are separate evidence.
The API supplies no runtime receipts, so its observed-test area explicitly says
none were supplied. The observed passing tests live in the dogfood verification
records under `warps/`. WIR1 excludes whole-document/runtime semantics; this demo
does not establish CIR1 support or the absence of architectural cycles.

Reproduce adapter, server and browser verification while the server runs:

```powershell
& $juliaExe --startup-file=no -C generic --compile=min --project=demos/web-evidence-lab -e 'include("julia-tests/main.demo.lang-inspector.test.jl"); include("julia-tests/main.demo.lang-dogfood.test.jl")'
& target/web-evidence-venv/Scripts/python.exe demos/web-evidence-lab/main.lang.page.test.py http://127.0.0.1:8765
& $juliaExe --startup-file=no -C generic --project=demos/web-evidence-lab julia-tests/runtests.jl
```

The browser suite uses the existing lab Playwright environment and installed
Chrome (override `CHROME_BIN` for another executable). It does not rewrite prior
demo receipts. Stop the foreground server with Ctrl+C when done.
