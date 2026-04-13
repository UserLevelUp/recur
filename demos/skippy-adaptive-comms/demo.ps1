# demo.ps1 - trace-id adaptive comms walkthrough

$ScriptDir = $PSScriptRoot
$Recur = if ($env:RECUR_BIN) { $env:RECUR_BIN } else { "recur" }
$ConfigDir = Join-Path $ScriptDir ".recur"
$ConfigPath = Join-Path $ConfigDir "config.toml"
$ConfigExample = Join-Path $ScriptDir "trace-id.config.example.toml"

if (-not (Test-Path $ConfigPath)) {
    New-Item -ItemType Directory -Path $ConfigDir -Force | Out-Null
    Copy-Item $ConfigExample $ConfigPath -Force
    Write-Host "Seeded local .recur/config.toml from trace-id.config.example.toml"
    Write-Host ""
}

Write-Host "============================================"
Write-Host "  recur trace-id demo: Skippy Adaptive Comms"
Write-Host "============================================"
Write-Host ""
Write-Host "1. Demo files"
& $Recur files "skippy.**" -d $ScriptDir
Write-Host ""

Write-Host "2. Active relationship phase"
& $Recur trace-id "skippy.relationship.playful.precise.current" --scope "skippy.**" --ext .txt --json -d $ScriptDir
Write-Host ""

Write-Host "3. Separator correction cue"
& $Recur trace-id "skippy.case.separator.correction" --scope "skippy.**" --ext .txt --json -d $ScriptDir
Write-Host ""

Write-Host "4. Strong insight cue"
& $Recur trace-id "skippy.case.strong.insight" --scope "skippy.**" --ext .txt --json -d $ScriptDir
Write-Host ""

Write-Host "5. Release admin cue"
& $Recur trace-id "skippy.case.release.admin" --scope "skippy.**" --ext .txt --json -d $ScriptDir
Write-Host ""

Write-Host "The protocol stays in text files."
Write-Host "trace-id provides the auditable role view."
Write-Host "Skippy still has to synthesize the final line."
