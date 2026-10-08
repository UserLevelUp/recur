param([ValidateRange(1,65535)][int]$Port = 8791)
$ErrorActionPreference = 'Stop'
$webJulia = Join-Path $env:LOCALAPPDATA 'Microsoft\WindowsApps\julia.exe'
if (-not (Test-Path -LiteralPath $webJulia)) { $webJulia = (Get-Command julia -ErrorAction Stop).Source }
$webProject = Join-Path $PSScriptRoot '..\web-evidence-lab'
$webServer = Join-Path $PSScriptRoot 'main.blackjack.web.server.jl'
Write-Host "The Green Room: http://127.0.0.1:$Port (Ctrl+C stops the server)"
& $webJulia --startup-file=no -O0 -C generic "--project=$webProject" $webServer --port $Port
exit $LASTEXITCODE
