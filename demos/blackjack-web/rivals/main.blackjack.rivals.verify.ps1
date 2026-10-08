$ErrorActionPreference = 'Stop'
$rivalWebRoot = Split-Path -Parent $PSScriptRoot
$rivalJulia = 'C:/Users/marcn/.julia/juliaup/julia-1.12.7+0.x64.w64.mingw32/bin/julia.exe'
$rivalNode = 'C:/Users/marcn/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/bin/node.exe'
# consumes: demo.blackjack.rivals.coordination existing CLI checks instead of another model invocation
Push-Location $rivalWebRoot
try {
    $env:RECUR_BIN = 'C:/src/recur/target/debug/recur.exe'
    & $rivalNode --test main.blackjack.rivals.test.cjs
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    & $rivalJulia --startup-file=no -O0 -C generic --project=../web-evidence-lab main.blackjack.web.test.jl
    exit $LASTEXITCODE
} finally { Pop-Location }
