# tasting.ps1 - Animated drink tasting using recur
#
# Usage:
#   .\tasting.ps1           # Full tasting
#   .\tasting.ps1 mug       # Beer only
#   .\tasting.ps1 wine      # Wine only

param(
    [string]$Glass = "all",
    [int]$Delay = 2
)

$ScriptDir = $PSScriptRoot

function Compose-Drink($G, $D) {
    $structFiles = Get-ChildItem "$ScriptDir\drink.$G.*.txt" -ErrorAction SilentlyContinue
    $fillFiles = Get-ChildItem "$ScriptDir\drink_${G}_${D}_*.txt" -ErrorAction SilentlyContinue

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

function Show-Drink($G, $D, $Label) {
    Clear-Host
    Write-Host ""
    Write-Host "  === $Label ==="
    Write-Host ""
    Compose-Drink $G $D
    Write-Host ""
    Start-Sleep -Seconds $Delay
}

Write-Host ""
Write-Host "  Hold my beer... recur is about to show off."
Write-Host ""
Start-Sleep -Seconds 1

switch ($Glass) {
    "mug" {
        Show-Drink mug empty  "Empty Mug"
        Show-Drink mug half   "Half Pint"
        Show-Drink mug stout  "Irish Stout"
        Show-Drink mug lager  "Golden Lager"
        Show-Drink mug ipa    "Hoppy IPA"
        Show-Drink mug water  "Just Water"
    }
    "wine" {
        Show-Drink wine empty "Empty Glass"
        Show-Drink wine white "Chardonnay"
        Show-Drink wine rose  "Rose"
        Show-Drink wine red   "Cabernet"
    }
    "all" {
        Show-Drink mug empty  "Empty Mug"
        Show-Drink mug stout  "Irish Stout"
        Show-Drink mug lager  "Golden Lager"
        Show-Drink mug ipa    "Hoppy IPA"
        Show-Drink wine empty "Empty Glass"
        Show-Drink wine white "Chardonnay"
        Show-Drink wine rose  "Rose"
        Show-Drink wine red   "Cabernet"
    }
}

Clear-Host
Write-Host ""
Write-Host "  Cheers! That's recur merge - different naming conventions, one view."
Write-Host ""
