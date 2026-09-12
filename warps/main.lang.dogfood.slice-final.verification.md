# Final dogfood integration observation — 2026-09-10

Contract: `contract:main.lang.dogfood.slice-final:v1`.
Gates: `walkthrough-and-regression`, `lang-gap-assessment`.

Observed in `C:/src/recur`, branch `recur-lang`, with uncommitted task changes.
No Rust changes were introduced; the contract does not require Rust checks for
this demo-only change. This record does not claim a build from committed HEAD.

## Commands and observations

The focused integration command in slice-3 passed all 333 assertions: existing
inspector 33, existing server 33, greeting fixtures 111, adapter fixtures 103,
real loopback/read-boundary 53. No old assertions were weakened. New green suites
are included by the normal runner through `main.demo.lang-dogfood.test.jl`.

```powershell
$env:RECUR_BIN='C:/src/recur/target/release-safe/recur.exe'
& C:/Users/marcn/AppData/Local/Microsoft/WindowsApps/julia.exe --startup-file=no -C generic --project=demos/web-evidence-lab julia-tests/runtests.jl
```

Julia 1.12.7, exit 0, observed after warming the package cache:

```text
Test Summary:             | Pass  Broken  Total     Time
Recur Complete Test Suite | 4133      73   4206  1m41.8s
```

The 73 broken cases are existing explicit expected-broken contracts. All new
dogfood tests executed and passed. The first complete run had 4,132 pass / 1 fail /
73 broken: an existing pipeline's empty-stderr assertion received Julia package
precompile messages (Parsers/JSON3). No test was changed or filtered for the
warmed-cache rerun. Earlier focused-run interpreter/compiler crashes are recorded
in slice-1/3. Runtime flags and cache conditions matter for reproduction here.

The runnable `demos/web-evidence-lab/main.lang.walkthrough.md` was exercised:

- `lang show main.greeting.recur --scope greeting.g -d demos/web-evidence-lab`:
  exit 0; `fnv1a64:9a4f9796c1e5d4ef`; declared URI/response bundles and `g(a)`.
- `lang report main.lang.inspector.recur -d demos/lang-inspector`: exit 0;
  `fnv1a64:a620679006d3da65`; query.q → model.m → view.v contracts/boundaries.
- `lang check main.lang.api.recur -d demos/web-evidence-lab --json`: exit 0;
  `fnv1a64:dba65c1b438cb118`; sound-within-coverage; no findings;
  whole_source_validated false. All three queries used the local Recur binary.
- Real greeting HTTP: trimmed José + es returned exactly `{"message":"Hola, José!"}`;
  empty name returned 400 and exactly `{"error":"Invalid name"}`.
- `target/web-evidence-venv/Scripts/python.exe demos/web-evidence-lab/main.lang.page.test.py
  http://127.0.0.1:61773`: exit 0. Chrome 152.0.7977.83; real model.m report,
  alias/body/footer display, inert markup, empty/error/loading states and disabled
  controls. See slice-3 for the red-to-green browser corrections.
- Desktop 1100px and mobile 390px screenshots visually reviewed; mobile document
  width did not exceed viewport width. Footer retains compact event/transition
  lines; full source spans remain available in expandable packet data.
- `git diff --check`: exit 0 (only Git's configured LF/CRLF notices).

## Scope and residue

The new Lang specification declares bundles in its header, composes scoped
function symbols in its body, and describes events/state in its footer.
The API protocol and hand-derived models/fixtures are linked implementation
choices. Neither Lang nor this page executes prose, bindings or acceptance tests.
Query packets retain original source/hash/coverage/findings and unknown fields.
The API supplies no runtime evidence; recorded state and static results remain
separate. CIR1/SGR1 support and whole-architecture cycle freedom are not claimed.

`main.lang.dogfood.gap-assessment.md` records query sufficiency, presentation
friction, authoring choices and the distinct future runtime-evidence boundary.
No grammar expansion or companion behavior was needed. Repository expert skill,
playbook and reveal pointer now link the worked example; installed personal
skills were not altered. Historical acceptance records are retained.

Completion layers use the existing map's declared evidence mode. The hashes
below bind the listed observed inputs for human reassessment; this Markdown is
not a checked external-evidence manifest and does not assert dependency closure.

produces: main.lang.dogfood.acceptance observed final integration and useful residue

Base Git revision (dirty task workspace): 77d8f226bf191dff11d2d2ffcfe2a06be9f7c594

Input fingerprints:

warps/main.lang.dogfood.contract.md: sha256:2bea880983395b7982dc69a6b28754994f9032edb70610c538af06202b197999

warps/main.lang.dogfood.gap-assessment.md: sha256:92c98af1b79d47f5b58237270a558e145476a17ecf06a675bbd64ee55d6a77c6

demos/web-evidence-lab/main.greeting.recur: sha256:d4247a577f61ad1336fcea25213d367c28213b3541a2900e349a6b1d9e0c77e3

demos/web-evidence-lab/main.greeting.fixtures.test.jl: sha256:731042bc8a35a39a13efab07bfdabf644d03f2da30ab99347eea19383490ec01

demos/web-evidence-lab/main.server.jl: sha256:da7d28964109079bcaf36ccf6b34dde055e8bf2508f4e3aa2ffdc4045ccebb3d

demos/web-evidence-lab/main.server.test.jl: sha256:e9ac8df8e4013822128a724ea98cdf22bd14f2ed10de6e6c53d48e3023b54dac

demos/web-evidence-lab/main.lang.api.contract.md: sha256:bfcee6ce6d146c365a4f008b14b66bf67f04110277683530d5e7eaf8d16067c5

demos/web-evidence-lab/main.lang.api.recur: sha256:d997b2d36969a56361f4454d4f439e98192b4ea58ccce70ed644fb32dc30eb5d

demos/web-evidence-lab/main.lang.api.jl: sha256:781521354a2b33a8133aa9c9dbcddbca95d21403a952d5afe2bddbc1b8e3a972

demos/web-evidence-lab/main.lang.api.test.jl: sha256:52ecd076072b4f1bdd24becb0f16ddd1b91e59964bc9fc6892defb4e3c4f649d

demos/web-evidence-lab/lang.html: sha256:a4ea32db2d52010d928f8d9fdb015f3986f2de0d45f07c4724247ecf12d0189a

demos/web-evidence-lab/main.lang.page.js: sha256:b32e879950f503a3dc5008895fb4c0b0ad4b29a1831b082b94f0ca8a1088882f

demos/web-evidence-lab/main.lang.page.css: sha256:42bde8704c1af01be146985922afbc89c22f338b1b9adee595edc714ff55d548

demos/web-evidence-lab/main.lang.page.test.py: sha256:6521efce955160e95418c5adc4a34d35e57ffa3174226c9c238068884603ac1e

demos/web-evidence-lab/main.lang.walkthrough.md: sha256:b3d490f2f86a0035c14dd231b033c00a6b7f422833eb2c21141cde2fcbd57f17

demos/web-evidence-lab/Project.toml: sha256:76e40e4bb2000989fb67b0942319e93b41ab24c0b1a7a81223c548a7e741743c

demos/web-evidence-lab/Manifest.toml: sha256:b391bb1f71cb77b8fbc8ad95db2a407781c410277ce8bb14facdd94d560caa53

demos/lang-inspector/main.lang.inspector.jl: sha256:cddc31cb333eaccf6b34ccc842a831d8213d995ff9642976e1cb107e8f21ecdc

demos/lang-inspector/main.lang.inspector.recur: sha256:5937074eb591bbcafd553c71fc41bac6e712e002bb859cc64c7bed3f0f2ea4df

julia-tests/main.demo.lang-inspector.test.jl: sha256:2339c859fb542b7fdb306867f04d50ec8def910ced8a4a22bc8c10a5662feb57

julia-tests/main.demo.lang-dogfood.test.jl: sha256:963c336093bfe40feef11b790a0f9e6623bb8cb424e788eb1366d568633654e6

julia-tests/runtests.jl: sha256:4f4f1cc46bb71e7aeaf98cd5f3ef924ab2bdc57f2ba24929a771f8e7a4bd7ac5

target/release-safe/recur.exe: sha256:0c73312bbe08489efc36f4fb286d4c12e0e1c9f3ebfd393620a833e1ff1f47b0
