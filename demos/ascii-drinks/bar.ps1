# bar.ps1 - Interactive ASCII drinks bar
#
# Use arrow keys or number keys to browse drinks.
# Press Q to quit.

$ScriptDir = $PSScriptRoot

$drinks = @(
    @{ Glass="mug";  Drink="empty"; Label="Empty Mug";        Quip="Somebody forgot to order. Classic.";       FillColor="Gray" },
    @{ Glass="mug";  Drink="half";  Label="Half Pint";        Quip="Half full or half empty? Yes.";             FillColor="DarkYellow" },
    @{ Glass="mug";  Drink="stout"; Label="Irish Stout";      Quip="Dark as my commit history at 3am.";         FillColor="DarkRed" },
    @{ Glass="mug";  Drink="lager"; Label="Golden Lager";     Quip="Fizzy yellow confidence juice.";            FillColor="Yellow" },
    @{ Glass="mug";  Drink="ipa";   Label="Hoppy IPA";        Quip="For people who enjoy suffering. Respect.";  FillColor="DarkYellow" },
    @{ Glass="mug";  Drink="water"; Label="Just Water";       Quip="Designated driver. The real MVP.";          FillColor="Cyan" },
    @{ Glass="wine"; Drink="empty"; Label="Empty Wine Glass";  Quip="The saddest thing I've ever rendered.";    FillColor="Gray" },
    @{ Glass="wine"; Drink="white"; Label="Chardonnay";       Quip="Pairs well with merge conflicts.";          FillColor="White" },
    @{ Glass="wine"; Drink="rose";  Label="Rose";             Quip="It's not pink, it's *light red*. Sure.";    FillColor="Magenta" },
    @{ Glass="wine"; Drink="red";   Label="Cabernet";         Quip="Bold. Like force-pushing to main.";         FillColor="DarkRed" }
)

function Get-DrinkArt($Glass, $Drink) {
    $structFiles = Get-ChildItem "$ScriptDir\drink.$Glass.*.txt" -ErrorAction SilentlyContinue
    $fillFiles = Get-ChildItem "$ScriptDir\drink_${Glass}_${Drink}_*.txt" -ErrorAction SilentlyContinue

    $allFiles = @()
    if ($structFiles) { $allFiles += $structFiles }
    if ($fillFiles) { $allFiles += $fillFiles }

    $sorted = $allFiles | Sort-Object {
        if ($_.Name -match '(\d+)[a-z]+\.txt$') { [int]$Matches[1] } else { 0 }
    }

    $lines = @()
    foreach ($f in $sorted) {
        $lines += Get-Content $f.FullName
    }
    return $lines
}

function Draw-Screen($index) {
    $d = $drinks[$index]
    $art = Get-DrinkArt $d.Glass $d.Drink

    # Fixed-height display area
    $host.UI.RawUI.CursorPosition = @{ X=0; Y=0 }

    $width = 58
    $blank = " " * $width

    # Header
    Write-Host ""
    Write-Host ("  {0}" -f ("=" * ($width - 4))) -ForegroundColor DarkCyan
    Write-Host "  Skippy & Joe's Bar & Merge" -ForegroundColor Cyan -NoNewline
    Write-Host "$(' ' * ($width - 32))" -ForegroundColor Cyan
    Write-Host "  Hold My Beer Edition" -ForegroundColor DarkCyan -NoNewline
    Write-Host " -- powered by recur merge$(' ' * 2)" -ForegroundColor DarkGray
    Write-Host ("  {0}" -f ("=" * ($width - 4))) -ForegroundColor DarkCyan
    Write-Host ""

    # Drink label
    $label = "  === $($d.Label) ==="
    Write-Host "$label$(' ' * [Math]::Max(0, $width - $label.Length))" -ForegroundColor Yellow
    Write-Host ""

    # ASCII art (pad to fixed height of 8 lines) with colorized fill
    $fillColor = $d.FillColor
    $glassColor = "White"
    $handleColor = "DarkGray"

    for ($i = 0; $i -lt 8; $i++) {
        if ($i -lt $art.Count) {
            $line = $art[$i]
            $padded = "$line$(' ' * [Math]::Max(0, $width - $line.Length))"

            # Colorize: frame in white, fill content in drink color, handle in gray
            if ($line -match '^(\s*\|)([^|]+)(\|.*)$') {
                $leftPipe = $Matches[1]
                $fill = $Matches[2]
                $rest = $Matches[3]
                $pad = ' ' * [Math]::Max(0, $width - $line.Length)

                Write-Host -NoNewline $leftPipe -ForegroundColor $glassColor
                Write-Host -NoNewline $fill -ForegroundColor $fillColor
                # Split rest into glass part and handle part
                if ($rest -match '^(\|\s+)(\|.+)$') {
                    Write-Host -NoNewline $Matches[1] -ForegroundColor $glassColor
                    Write-Host -NoNewline $Matches[2] -ForegroundColor $handleColor
                } else {
                    Write-Host -NoNewline $rest -ForegroundColor $glassColor
                }
                Write-Host "$pad"
            } elseif ($line -match '\\|/|~') {
                # Rim, bottom, stem, base lines
                Write-Host "$padded" -ForegroundColor $glassColor
            } else {
                Write-Host "$padded"
            }
        } else {
            Write-Host $blank
        }
    }

    Write-Host ""

    # Skippy's quip
    $quip = "  `"$($d.Quip)`""
    Write-Host "$quip$(' ' * [Math]::Max(0, $width - $quip.Length))" -ForegroundColor Green
    $attr = "  -- Skippy the Magnificent"
    Write-Host "$attr$(' ' * [Math]::Max(0, $width - $attr.Length))" -ForegroundColor DarkGreen
    Write-Host ""

    # File info
    $structPat = "drink.$($d.Glass).* [.]"
    $fillPat = "drink_$($d.Glass)_$($d.Drink)_* [_]"
    Write-Host "  Structure: $structPat$(' ' * [Math]::Max(0, $width - 14 - $structPat.Length))" -ForegroundColor DarkGray
    Write-Host "  Contents:  $fillPat$(' ' * [Math]::Max(0, $width - 14 - $fillPat.Length))" -ForegroundColor DarkGray
    Write-Host ""

    # Footer
    Write-Host ("  {0}" -f ("-" * ($width - 4))) -ForegroundColor DarkGray
    $nav = "  [$($index + 1)/$($drinks.Count)]  Left/Right to browse, Q to quit"
    Write-Host "$nav$(' ' * [Math]::Max(0, $width - $nav.Length))" -ForegroundColor DarkGray

    # Drink menu
    Write-Host -NoNewline "  "
    for ($j = 0; $j -lt $drinks.Count; $j++) {
        if ($j -eq $index) {
            Write-Host -NoNewline "[" -ForegroundColor Yellow
            Write-Host -NoNewline "$($j+1)" -ForegroundColor Yellow
            Write-Host -NoNewline "] " -ForegroundColor Yellow
        } else {
            Write-Host -NoNewline " $($j+1)  " -ForegroundColor DarkGray
        }
    }
    Write-Host "$(' ' * 10)"

    # Bottom bar
    Write-Host ""
    Write-Host "  Joe says: `"Two naming conventions walk into a bar...$(' ' * 2)`"" -ForegroundColor DarkYellow
    Write-Host "  recur merge: `"I'll have what they're both having.`"$(' ' * 4)" -ForegroundColor DarkYellow
}

# Setup
Clear-Host
[Console]::CursorVisible = $false
$current = 0

try {
    Draw-Screen $current

    while ($true) {
        $key = $host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

        switch ($key.VirtualKeyCode) {
            37 { # Left arrow
                $current = if ($current -gt 0) { $current - 1 } else { $drinks.Count - 1 }
                Draw-Screen $current
            }
            39 { # Right arrow
                $current = if ($current -lt $drinks.Count - 1) { $current + 1 } else { 0 }
                Draw-Screen $current
            }
            default {
                $ch = $key.Character
                if ($ch -eq 'q' -or $ch -eq 'Q') {
                    break
                }
                # Number keys 1-9, 0=10
                if ($ch -ge '1' -and $ch -le '9') {
                    $idx = [int]::Parse($ch) - 1
                    if ($idx -lt $drinks.Count) {
                        $current = $idx
                        Draw-Screen $current
                    }
                }
                if ($ch -eq '0' -and $drinks.Count -ge 10) {
                    $current = 9
                    Draw-Screen $current
                }
            }
        }
        if ($key.Character -eq 'q' -or $key.Character -eq 'Q') { break }
    }
} finally {
    [Console]::CursorVisible = $true
    Clear-Host
    Write-Host ""
    Write-Host "  Thanks for visiting Skippy & Joe's!"
    Write-Host "  Remember: different naming conventions, one unified view."
    Write-Host "  That's recur merge. Don't Panic."
    Write-Host ""
}
