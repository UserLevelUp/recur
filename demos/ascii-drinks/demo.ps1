# demo.ps1 - Quick demo of recur merge composability with ASCII drinks

$ScriptDir = $PSScriptRoot
$Recur = if ($env:RECUR_BIN) { $env:RECUR_BIN } else { "recur" }

function Compose-Drink($Glass, $Drink) {
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
}

Write-Host ""
Write-Host "============================================"
Write-Host "  recur merge demo: Hold My Beer Edition"
Write-Host "============================================"
Write-Host ""
Write-Host "  Files use TWO naming conventions:"
Write-Host "    Structure (dots):       drink.mug.1top.txt"
Write-Host "    Contents (underscores): drink_mug_stout_2fill.txt"
Write-Host ""
Write-Host "  recur merge unifies them with --sep . --sep _"
Write-Host ""

Write-Host "--------------------------------------------"
Write-Host "  1. recur tree (dot notation - structure)"
Write-Host "--------------------------------------------"
& $Recur tree drink.mug -d $ScriptDir --sep .
Write-Host ""

Write-Host "--------------------------------------------"
Write-Host "  2. recur tree (underscore notation - stout)"
Write-Host "--------------------------------------------"
& $Recur tree drink_mug_stout -d $ScriptDir --sep _
Write-Host ""

Write-Host "--------------------------------------------"
Write-Host "  3. recur merge (unified view!)"
Write-Host "--------------------------------------------"
$struct = & $Recur tree drink.mug -d $ScriptDir --sep . | Out-String
$fill = & $Recur tree drink_mug_stout -d $ScriptDir --sep _ | Out-String
Write-Output $struct $fill | & $Recur merge --stdin --base drink --sep . --sep _ --show-sep
Write-Host ""

Write-Host "--------------------------------------------"
Write-Host "  4. Composed ASCII art: Stout in a Mug"
Write-Host "--------------------------------------------"
Compose-Drink mug stout
Write-Host ""

Write-Host "--------------------------------------------"
Write-Host "  5. Same mug, different drink: Lager"
Write-Host "--------------------------------------------"
Compose-Drink mug lager
Write-Host ""

Write-Host "--------------------------------------------"
Write-Host "  6. Wine glass: Red Wine"
Write-Host "--------------------------------------------"
Compose-Drink wine red
Write-Host ""

Write-Host "--------------------------------------------"
Write-Host "  7. Same glass, different wine: Rose"
Write-Host "--------------------------------------------"
Compose-Drink wine rose
Write-Host ""

Write-Host "============================================"
Write-Host "  The container stays the same."
Write-Host "  Only the contents change."
Write-Host "  recur merge makes it all composable."
Write-Host "============================================"
Write-Host ""
