# Ten Hello World apps: semantic boundaries, Eventness, and evidence

**The experiment worked: ten runnable web apps, 80 passing browser case executions, and ten Warp slices accepted with checked external evidence.** Recur helped locate related artifacts and reassess recorded acceptance. It did not run the browser or establish whether our chosen tests and file scope were sufficient.

Joe, Barney has located the laser pointer. Three lines on the board:

1. **The name tells us where:** `main.history` owns history.
2. **Eventness tells us what needs attention:** `main.history.integration.current.md` records work in progress.
3. **Evidence tells us what we observed:** the parent sent a greeting and the browser displayed a history entry, against specified source inputs.

The short arms are particularly relevant here: a child passing its own test cannot reach across the board and declare the parent finished.

## What was actually built and run

These are ten independently runnable, cumulative vanilla HTML/CSS/JavaScript examples in `apps/01` through `apps/10`. Each uses **`main` as its semantic root**. The numbered directories isolate different versions; they do not rename the project to `app01`, `app02`, etc. A Julia 1.12.7 server using HTTP.jl and JSON3 supplies the loopback development host and, at levels 08–10, a real HTTP greeting endpoint. Its dependencies are recorded in the local `Project.toml` and `Manifest.toml`.

The original Python server was replaced to follow the project's Julia preference. The active server has no Python dependency. The generator, discovery probes, and Playwright browser-test helpers still use Python; they are development tools, and the lab as a whole is not yet Python-free. Historical Python-server runs remain historical evidence, while the current run explicitly records the Julia executable, version, launch command, package manifest, and server source. The Julia server contract suite checks 33 assertions, including API responses, static pages, HEAD requests, and file-serving boundaries.

Playwright drove installed Chrome headlessly at 1100 × 850. Every case received a fresh browser context. Later examples inherited relevant earlier checks. Thus **80 means case executions across the ladder, not 80 unrelated test designs**. There are ten captured app screenshots and a screenshot of the deliberately broken integration.

| Level | Added behavior | Extracted responsibility | App source files | Browser cases passed |
|---|---|---|---:|---:|
| 01 | Static Hello World | `main.html` | 1 | 1 |
| 02 | Presentation | `main.style` | 2 | 2 |
| 03 | Module-based rendering | `main.greeting.view` | 4 | 3 |
| 04 | Input, trimming, validation, safe text | `main.greeting.model` | 5 | 6 |
| 05 | English/Spanish greeting | `main.greeting.locale` | 6 | 7 |
| 06 | Preferences survive reload | `main.preferences.store` | 7 | 9 |
| 07 | Greeting/About routes and reload | `main.navigation.router` | 8 | 10 |
| 08 | HTTP greeting and visible service errors | `main.greeting.api.client` | 9 | 12 |
| 09 | One retry and per-language offline cache | `main.greeting.delivery` | 10 | 14 |
| 10 | Recent history, five-entry bound, clear | `main.history.model`, `main.history.view` | 12 | 16 |

Source counts include HTML, CSS, and JavaScript within each example, excluding contracts, receipts, and the shared server. Real HTTP success and server-side invalid-input rejection were exercised. Transient errors and offline failures were injected through browser request interception. This does not constitute testing an actual remote outage.

The canonical completed run is identified in [main.results.json](main.results.json). It contains commands, exit statuses, browser/runtime versions, per-case outcomes, experiment observations, and the final Warp projection. Its `run_id` locates immutable result/receipt files and screenshots under `evidence/`. The earlier interrupted run is retained as history; its repair check exposed browser caching, which the development server now disables.

## How the hierarchy helped

The useful final shape is:

```text
main                         parent startup and composition
├── greeting
│   ├── model                input contract
│   ├── view                 text rendering
│   ├── locale               formatting
│   ├── api.client           HTTP boundary
│   └── delivery             retry/cache policy
├── preferences.store        persistence boundary
├── navigation.router        URL/visibility boundary
└── history
    ├── model                bounded list behavior
    └── view                 consumes parent greeting events
```

The physical filenames encode this shape; JavaScript imports still implement actual dependencies. Recur neither extracts modules nor rewrites imports. I performed the decomposition and used Recur to inspect the resulting project.

At level 10, `recur files 'main.greeting.**'` selected **six artifacts**: five source modules plus their contract. A settled `main.**` query returned **16 artifacts**. This narrows reading to a feature, but deliberately excludes the parent and history consumer. For an integration change, expand back to the parent and follow references. A smaller file list is not automatically a complete change scope.

A discovery-only control copied the same JavaScript content to neutral `file01.js` names. The greeting filename query returned no matches. Those aliases were not a functioning alternative build: imports were deliberately not rewritten. This demonstrates the query advantage of semantic names, **not a measured productivity or performance advantage over other development tools**.

My recommendation is to add a level when a responsibility gains its own contract, state, failure behavior, or consumer. Level 01 needs no feature bureaucracy. Rendering becomes a module at 03; input and language become distinct concerns at 04–05; persistence and routing become siblings rather than disappearing inside greeting; the HTTP client sits below greeting because that feature owns its protocol. At 10 the parent explicitly owns the greeting-to-history connection.

Preserve meaningful subject identities while evolving their implementation. Filename moves alone do not preserve a stable UUID or record why a split happened. A production extraction should retain a short lineage note and explicit old/new contract relationship; this lab demonstrates names and trace sites, not UUID migration.

## Eventness and tracing: observed behavior

Each run expanded `main.acceptance.current.md`, queried it, ran acceptance, and collapsed successful work into `main.acceptance.complete.md` with its receipt reference. During a rerun an old complete marker and a new current marker can coexist: historical acceptance and current reassessment are different facts.

The supplemental probe also recorded a child-level sequence in a disposable copy:

```text
main.history.integration.current.md
    → main.history.integration.strange.md
    → main.history.integration.complete.md
```

The notes explain the intent, observed parent failure, and observed repair from the browser experiment. Recur found each state through `main.history.**.<state>`. The probe authors the notes; Recur queries them. The stable subject remains `main.history.integration`.

Two discovery limitations were observable:

- **Trace roles require vocabulary appropriate to the project.** Default `trace-id main.greeting.sent` found five textual sites and one producer classification, but no consumer classification despite the JavaScript listener being present. In a copy configured with `produces:,dispatchevent` and `consumes:,addeventlistener`, it reported two producer and four consumer sites. Comments and executable sites can both match. These are search/classification observations, not runtime event-delivery proof.
- **JSON tree depth did not bound this output.** `tree main --depth 1 --json` and `--depth 5 --json` returned equal trees. Also, a tree node has a single path: `main.html` and `main.js` share the semantic node, so use `files` for the complete artifact inventory. Do not build an evidence scope from tree leaves alone.

Exact supplemental outputs are in `evidence/<run_id>/discovery-probes.json`. These are reported findings; this investigation did not change core Recur to address them.

## What the negative experiments established

| Deliberate condition | Observed result | Meaning |
|---|---|---|
| Parent emits `main.greeting.lost` | Greeting visible; child model passes; history has 0 entries instead of 1 | Child success does not imply parent integration |
| Old complete marker retained in broken copy | Marker still exists | Eventness is recorded state, not a live health check |
| Restore the correct event | Parent history receives an entry | The integration regression and repair were both exercised |
| Change a fingerprinted source file | Evidence becomes `stale` | Checked evidence can detect covered source drift |
| Change an omitted dependency | Evidence remains `checked`; browser history breaks | The producer must supply adequate scope |
| Change result bytes | Evidence becomes `stale` | Result integrity is checked |
| Supply a failed structured result | Evidence becomes `failed` | A failed observation cannot satisfy this checked gate |
| Change level 10's required contract | Warp becomes `exploded` with stale contract | Historical acceptance does not satisfy a different contract |
| Restore the contract and use intact full-scope results | Warp reports `complete`, evidence `checked`, 10/10 covered | Qualified acceptance is recoverable |

The omitted-dependency fixture is **intentionally incomplete evidence**, adapted from a passing observation and narrowed to the parent source alone. It is a negative control, not an acceptable release receipt. The lab's real stage receipts enumerate the entire stage's sources and contracts, generated local config, server, generator, test runner, and dependency requirement. They do not infer dependency closure automatically.

`warp evidence` performs read-only structured-result and scoped fingerprint validation; it explicitly reports that the producer was not rerun. The current schema uses FNV-1a fingerprints. Treat these as change detection, not cryptographic attestation. Neither a receipt nor a Git tag establishes that an arbitrary producer told the truth.

## Where evidence should belong

**Keep execution with the project, acceptance policy with Warp, and Git provenance with recur-git. Reuse the existing external evidence format before adding another evidence subsystem.**

| Owner | Responsibility | Recommendation grounded in this lab |
|---|---|---|
| Project test runner | Execute tests; capture outcomes, logs, screenshots, inputs and environment | Browser runner for a web app; Cargo/Julia for Recur development. This lab's Python adapter already feeds the existing format. |
| Core `recur` | Discover artifacts, trace declared relationships, project Warp state, check referenced result integrity | Keep queries read-only. Improve depth behavior and make trace vocabulary clearer. Do not make core launch every ecosystem's tests. |
| `recur-warp` | Author contracts, bind evidence to gates, record accepted attempts, manage reassessment | Existing checked gates and `complete` already accepted these browser receipts. Add focused child and parent gates for real feature work. |
| `recur-git` | Identify tested revision/worktree, bind portable evidence to a snapshot, support preservation/publication workflows | Its current `test-receipt` help exposes Cargo and Julia targets. Add a bounded external-runner/import adapter if needed; do not require a web project to adopt Julia. |
| Project `main.server` | Present task, evidence, reports, and demos through one local hub | Use the project's server as a viewer. The included Julia server and board demonstrate this for the lab; a solution-wide hub remains separate work. |

The simulated Warp has ten independent checked slices, one per app, each with a browser gate. It does **not** claim to demonstrate nested Warp rings. Within a real app, use separate child-contract and parent-integration gates: a history unit receipt should never be silently promoted into whole-app acceptance.

The next bounded improvement I would build is an **external evidence adapter**, shared by companion workflows. Its input is a runner invocation or an existing result plus an explicit input manifest; its output is a portable receipt bundle compatible with Warp. Record the working directory, executable/version, arguments, relevant nonsecret environment, before/after source manifest, contract identity, result/log/screenshot references and cryptographic digests. Fail or mark inconclusive if source changes during execution. The current lab records runtime details and source fingerprints but does not yet implement a before/after mutation guard or cryptographic binding of every supplementary artifact.

Then let `recur-git` attach the bundle to a tested snapshot and optionally an annotated tag **after** evidence exists. The tag names the snapshot and evidence manifest; it should not contain a vague “tests passed” assertion or point solely at private ignored files. Warp consumes that same bundle to assess its gates. One observation can support multiple explicitly compatible gates without rerunning or duplicating execution policy.

Do not make `main.evidence` another parent for all functionality. Use it as an index if useful; keep subjects such as `main.greeting.delivery` and `main.history.integration` in the receipt's identity/scope. Evidence belongs to a claim about a subject, configuration, and version.

## Run and inspect it

From `C:\src\recur` using the environment created for this investigation:

```powershell
# Serve the board and all ten apps; stop with Ctrl+C.
$juliaExe = 'C:\Users\marcn\AppData\Local\Microsoft\WindowsApps\julia.exe'
& $juliaExe --startup-file=no --project=demos/web-evidence-lab -e 'using Pkg; Pkg.instantiate()'
& $juliaExe --startup-file=no --project=demos/web-evidence-lab demos/web-evidence-lab/main.server.jl --port 8765
# Open http://127.0.0.1:8765

# In another terminal: regenerate, test, and produce a new evidence run.
.\target\web-evidence-venv\Scripts\python.exe demos/web-evidence-lab/main.test.py
.\target\web-evidence-venv\Scripts\python.exe demos/web-evidence-lab/main.probe.py

# The server's own contract tests run directly in Julia.
& $juliaExe --startup-file=no --project=demos/web-evidence-lab demos/web-evidence-lab/main.server.test.jl

.\target\release-safe\recur.exe tree main -d demos/web-evidence-lab/apps/10
.\target\release-safe\recur.exe files 'main.greeting.**' -d demos/web-evidence-lab/apps/10
.\target\release-safe\recur.exe warp show main.web-lab -d demos/web-evidence-lab --json
```

For a fresh checkout, serving the existing examples requires Julia and `Pkg.instantiate()` as shown above. To regenerate the examples or rerun browser automation, create a Python environment, install `requirements.txt`, and run `main.build.py` to regenerate the ignored local `.recur/config.toml` files. `main.test.py` does this automatically and launches the Julia server as a child process. Set `RECUR_JULIA` to override the Julia launcher; pass `--browser`, `--recur`, and `--recur-warp` for other executable locations. The companion binaries used here are local `target/release-safe` builds; their SHA-256 digests are recorded.

The generated examples are small deterministic teaching apps, not production frameworks. There was no cross-browser, mobile, load, full accessibility, or comprehensive security audit. Persistence testing covers reload and malformed JSON, not every valid-but-unexpected storage value. Initial greeting rendering is local even in the API examples; submissions exercise the server. Offline caching is in-memory within the loaded page. No service worker or background retry system is implied.

The repository base revision is recorded separately from the **uncommitted source actually tested**. Stage receipts say `dirty: true` and fingerprint those inputs; this work has not been committed or pushed. Tests of these additions do not replace Recur's own Cargo/Julia regression suites, which were not rerun because core code was unchanged.

The browser setup follows the [Playwright Python documentation](https://playwright.dev/python/docs/intro). Browser storage behavior is described in [MDN's localStorage reference](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage), and the client uses the [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch). All experiment conclusions above come from the saved local runs, not those documentation pages.
