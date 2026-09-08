[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$Version,

    [string]$ZipPath,

    [string]$OutputDirectory = "choco"
)

$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent $PSScriptRoot
$requestedVersion = $Version.Trim()
$normalizedVersion = $requestedVersion
if ($normalizedVersion.StartsWith('v')) {
    $normalizedVersion = $normalizedVersion.Substring(1)
}
if ($normalizedVersion -match '^[A-Za-z]\.(.+)$') {
    $normalizedVersion = $Matches[1]
}

$tag = "v$normalizedVersion"
$outputDir = Join-Path $repoRoot $OutputDirectory
New-Item -ItemType Directory -Force -Path $outputDir | Out-Null

$tempRoot = Join-Path $repoRoot (".tmp/choco-pack-" + $normalizedVersion + "-" + [guid]::NewGuid().ToString("N"))
$tempChoco = Join-Path $tempRoot "choco"
New-Item -ItemType Directory -Force -Path $tempChoco | Out-Null
Copy-Item -Path (Join-Path $repoRoot "choco\*") -Destination $tempChoco -Recurse -Force

try {
    if ($ZipPath) {
        $resolvedZip = (Resolve-Path $ZipPath).Path
    } else {
        $zipName = "recur-$tag-x86_64-pc-windows-msvc.zip"
        $zipUrl = "https://github.com/UserLevelUp/recur/releases/download/$tag/$zipName"
        $resolvedZip = Join-Path $tempRoot $zipName
        Invoke-WebRequest -Uri $zipUrl -OutFile $resolvedZip
    }

    $hash = (Get-FileHash $resolvedZip -Algorithm SHA256).Hash
    $installPath = Join-Path $tempChoco "tools\chocolateyInstall.ps1"
    $install = Get-Content $installPath -Raw
    $install = $install -replace '%CHECKSUM64%', $hash
    Set-Content -Path $installPath -Value $install -NoNewline

    $nuspecPath = Join-Path $tempChoco "recur.nuspec"
    choco pack $nuspecPath --version $normalizedVersion --output-directory $outputDir | Out-Host
    if ($LASTEXITCODE -ne 0) { throw "Chocolatey pack failed with exit code $LASTEXITCODE" }

    $packagePath = Join-Path $outputDir ("recur.$normalizedVersion.nupkg")
    Write-Host "Requested version $requestedVersion"
    Write-Host "Normalized package version $normalizedVersion"
    Write-Host "Created $packagePath"
    Write-Host "SHA256 $hash"
} finally {
    if (Test-Path $tempRoot) {
        $resolvedTemp = (Resolve-Path -LiteralPath $tempRoot).Path
        $allowedTemp = [IO.Path]::GetFullPath((Join-Path $repoRoot '.tmp')) + [IO.Path]::DirectorySeparatorChar
        if (-not $resolvedTemp.StartsWith($allowedTemp, [StringComparison]::OrdinalIgnoreCase)) {
            throw "Refusing cleanup outside the repository .tmp directory: $resolvedTemp"
        }
        Remove-Item -LiteralPath $resolvedTemp -Recurse -Force
    }
}
