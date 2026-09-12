# Greeting boundary observation — 2026-09-10

Contract: `contract:main.lang.dogfood.slice-1:v1`.
Gate: `greeting-boundary-fixtures`.

The hand-authored table in `demos/web-evidence-lab/main.greeting.fixtures.test.jl`
derives from `main.greeting.recur`: absent/empty/whitespace, trimmed Unicode,
1/40/41 characters, combining marks, exact Spanish and locale fallback. Every
row checks status, exact one-field JSON, MIME/cache headers, HEAD status/body/
content length, and POST rejection through an ephemeral loopback HTTP server.
40 multibyte characters are accepted; 21 two-character graphemes are rejected.

Observed on `recur-lang`, Windows, Julia 1.12.7:

```powershell
& C:/Users/marcn/AppData/Local/Microsoft/WindowsApps/julia.exe --startup-file=no -C generic --project=demos/web-evidence-lab -e 'println(VERSION); include("demos/web-evidence-lab/main.server.test.jl"); include("demos/web-evidence-lab/main.greeting.fixtures.test.jl")'
```

Exit 0: existing server 33/33; new greeting table 111/111. Greeting
implementation unchanged. Separate request/domain/view structs would only wrap
the URI, two strings and HTTP response here; retain the existing small function.
Fixtures are human-derived examples, not parser execution of prose.

The first attempt using `--compiled-modules=no --compile=min -O0` crashed in
Julia's parser/interpreter (ReadOnlyMemoryError/EXCEPTION_ACCESS_VIOLATION).
It produced no acceptance evidence. Generic CPU with normal compilation passed.

produces: main.lang.dogfood.greeting-fixtures observed boundary behavior

Input fingerprints:

demos/web-evidence-lab/main.server.jl: sha256:097894b1ef0179068261998f068761f90ede1c55b27abe0f78fe9d15603d8f14

demos/web-evidence-lab/main.greeting.recur: sha256:d4247a577f61ad1336fcea25213d367c28213b3541a2900e349a6b1d9e0c77e3

demos/web-evidence-lab/main.greeting.fixtures.test.jl: sha256:731042bc8a35a39a13efab07bfdabf644d03f2da30ab99347eea19383490ec01

demos/web-evidence-lab/Project.toml: sha256:76e40e4bb2000989fb67b0942319e93b41ab24c0b1a7a81223c548a7e741743c

demos/web-evidence-lab/Manifest.toml: sha256:b391bb1f71cb77b8fbc8ad95db2a407781c410277ce8bb14facdd94d560caa53
