# pour.ps1 - Compose a drink using recur merge
#
# Usage:
#   .\pour.ps1 mug stout     # Pour a stout into a mug
#   .\pour.ps1 wine red      # Pour red wine into a glass

param(
    [string]$Glass = "mug",
    [string]$Drink = "empty"
)

$ScriptDir = $PSScriptRoot
$Recur = if ($env:RECUR_BIN) { $env:RECUR_BIN } else { "recur" }

Write-Host ""
Write-Host "  Pouring $Drink into $Glass..."
Write-Host ""

# Show the recur merge view - structure [.] and contents [_] unified
Write-Host "  [recur merge view]"
$struct = & $Recur tree "drink.$Glass" -d $ScriptDir --sep . | Out-String
$fill = & $Recur tree "drink_${Glass}_${Drink}" -d $ScriptDir --sep _ | Out-String
Write-Output $struct $fill | & $Recur merge --stdin --base drink --sep . --sep _ --show-sep |
    ForEach-Object { "  $_" }
Write-Host ""

# Compose ASCII art by sorting files by their number prefix
$structFiles = Get-ChildItem "$ScriptDir\drink.$Glass.*.txt" -ErrorAction SilentlyContinue
$fillFiles = Get-ChildItem "$ScriptDir\drink_${Glass}_${Drink}_*.txt" -ErrorAction SilentlyContinue

$allFiles = @()
if ($structFiles) { $allFiles += $structFiles }
if ($fillFiles) { $allFiles += $fillFiles }

$sorted = $allFiles | Sort-Object {
    if ($_.Name -match '(\d+)[a-z]+\.txt$') { [int]$Matches[1] } else { 0 }
}

foreach ($f in $sorted) {
    Get-Content $f.FullName
}
Write-Host ""
