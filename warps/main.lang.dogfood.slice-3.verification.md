# Catalog inspector integration observation — 2026-09-10

Contract: `contract:main.lang.dogfood.slice-3:v1`.
Gate: `server-inspector-integration`.

Implemented `main.lang.api.jl`, the `/api/lang` route and `/lang.html` with its
plain JS/CSS. The adapter reuses query_report/build_view/render_view and preserves
the original packet. Server-owned roots and scopes are separate from request
parameters; the only process call uses the existing argument-vector adapter.
No existing greeting or inspector implementation behavior was changed.

Observed first green standalone `main.lang.api.test.jl`, Julia 1.12.7 with
`--startup-file=no -C generic --project=demos/web-evidence-lab`: exit 0,
103 fixture assertions and 53 real loopback assertions. Unlike the slice-2 red
run, all guarded behavior assertions executed. Both demo trees (including
specifications and receipts) and repository config were byte-identical across
the real queries. All eight catalog scopes succeeded; unknown ID returned 404.

Moved green suites into `julia-tests/main.demo.lang-dogfood.test.jl`, included
by the normal runner. Focused integration then passed with this command:

```powershell
$env:RECUR_BIN='C:/src/recur/target/release-safe/recur.exe'
& C:/Users/marcn/AppData/Local/Microsoft/WindowsApps/julia.exe --startup-file=no -C generic --compile=min --project=demos/web-evidence-lab -e 'include("julia-tests/main.demo.lang-inspector.test.jl"); include("julia-tests/main.demo.lang-dogfood.test.jl")'
```

Exit 0: inspector 33, server 33, greeting 111, adapter 103, real HTTP 53; all
333 passed. Captured summary: `main.lang.dogfood.slice-3.output.txt`.
Some normal-compilation and disabled-compiled-module attempts crashed within
Julia (including installed 1.12.0). Those runs are not acceptance evidence.

Started the same Julia server on ephemeral loopback port 61773; ran:

```powershell
& target/web-evidence-venv/Scripts/python.exe demos/web-evidence-lab/main.lang.page.test.py http://127.0.0.1:61773
```

Exit 0, Chrome 152.0.7977.83, existing Playwright environment. The browser checks
exercise real model.m aliases/composition and packet detail, separate evidence,
inert markup, empty header, retained CLI error, hidden stale report, and loading
with controls disabled. An initial label lookup failure led to explicit form
labels. Visual review found verbose JSON in the footer; added and observed a
failing compact-event assertion before rendering event/transition lines instead.
Final browser test passes. Raw packet details remain expandable.

Observed test evidence is deliberately empty in the API: no source-bound runtime
receipt loader was specified. The page says so, independently of recorded state
and static findings. WIR1 display does not claim CIR1/SGR1 or runtime acceptance.

produces: main.lang.dogfood.inspector-integration observed fixture and browser behavior

Input fingerprints:

demos/web-evidence-lab/main.server.jl: sha256:da7d28964109079bcaf36ccf6b34dde055e8bf2508f4e3aa2ffdc4045ccebb3d

demos/web-evidence-lab/main.lang.api.jl: sha256:781521354a2b33a8133aa9c9dbcddbca95d21403a952d5afe2bddbc1b8e3a972

demos/web-evidence-lab/main.lang.api.test.jl: sha256:52ecd076072b4f1bdd24becb0f16ddd1b91e59964bc9fc6892defb4e3c4f649d

demos/web-evidence-lab/lang.html: sha256:a4ea32db2d52010d928f8d9fdb015f3986f2de0d45f07c4724247ecf12d0189a

demos/web-evidence-lab/main.lang.page.js: sha256:b32e879950f503a3dc5008895fb4c0b0ad4b29a1831b082b94f0ca8a1088882f

demos/web-evidence-lab/main.lang.page.css: sha256:42bde8704c1af01be146985922afbc89c22f338b1b9adee595edc714ff55d548

demos/web-evidence-lab/main.lang.page.test.py: sha256:6521efce955160e95418c5adc4a34d35e57ffa3174226c9c238068884603ac1e

demos/lang-inspector/main.lang.inspector.jl: sha256:cddc31cb333eaccf6b34ccc842a831d8213d995ff9642976e1cb107e8f21ecdc

demos/lang-inspector/main.lang.inspector.recur: sha256:5937074eb591bbcafd553c71fc41bac6e712e002bb859cc64c7bed3f0f2ea4df

julia-tests/main.demo.lang-dogfood.test.jl: sha256:963c336093bfe40feef11b790a0f9e6623bb8cb424e788eb1366d568633654e6

julia-tests/runtests.jl: sha256:4f4f1cc46bb71e7aeaf98cd5f3ef924ab2bdc57f2ba24929a771f8e7a4bd7ac5

target/release-safe/recur.exe: sha256:0c73312bbe08489efc36f4fb286d4c12e0e1c9f3ebfd393620a833e1ff1f47b0
