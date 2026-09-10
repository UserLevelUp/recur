[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)][string]$ZipPath,
    [Parameter(Mandatory = $true)][string]$PackagePath,
    [Parameter(Mandatory = $true)][string]$OutputDirectory
)
$ErrorActionPreference = 'Stop'
# Exercise the actual packed scripts with Chocolatey helpers mocked. No install,
# download, shim mutation, elevated action or uninstallation takes place.
$zip = (Resolve-Path -LiteralPath $ZipPath).Path
$package = (Resolve-Path -LiteralPath $PackagePath).Path
$work = [IO.Path]::GetFullPath($OutputDirectory)
New-Item -ItemType Directory -Path $work | Out-Null
Add-Type -AssemblyName System.IO.Compression.FileSystem
[IO.Compression.ZipFile]::ExtractToDirectory($package, $work)
$tools = Join-Path $work 'tools'
[IO.Compression.ZipFile]::ExtractToDirectory($zip, $tools)
$script:installed = @()
$script:uninstalled = @()
$script:expectedHash = (Get-FileHash -LiteralPath $zip -Algorithm SHA256).Hash
$script:downloadChecked = $false
function Install-ChocolateyZipPackage {
    param($packageName, $unzipLocation, $url64bit, $checksumType64, $checksum64)
    if ($packageName -ne 'recur' -or $checksumType64 -ne 'sha256' -or $checksum64 -ne $script:expectedHash) {
        throw 'Packed installer does not bind the tested archive checksum'
    }
    if ($url64bit -ne 'https://github.com/userlevelup/recur/releases/download/v0.2.8/recur-v0.2.8-x86_64-pc-windows-msvc.zip') {
        throw "Unexpected package URL: $url64bit"
    }
    $script:downloadChecked = $true
}
function Install-BinFile {
    param($Name, $Path)
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) { throw "Missing binary $Name" }
    $script:installed += $Name
}
function Uninstall-BinFile {
    param($Name)
    $script:uninstalled += $Name
}
$previousVersion = $env:chocolateyPackageVersion
try {
    $env:chocolateyPackageVersion = '0.2.8'
    . (Join-Path $tools 'chocolateyInstall.ps1')
    . (Join-Path $tools 'chocolateyUninstall.ps1')
} finally {
    $env:chocolateyPackageVersion = $previousVersion
}
$expected = @('recur-git', 'recur-lang', 'recur-reveal', 'recur-version', 'recur-warp', 'recur-watch')
if (-not $script:downloadChecked) { throw 'Download/checksum helper was not exercised' }
if (($script:installed | Sort-Object) -join ',' -ne ($expected -join ',')) { throw 'Install shim set differs' }
if (($script:uninstalled | Sort-Object) -join ',' -ne ($expected -join ',')) { throw 'Uninstall shim set differs' }
[xml]$metadata = Get-Content -LiteralPath (Join-Path $work 'recur.nuspec') -Raw
if ($metadata.package.metadata.version -ne '0.2.8') { throw 'Nuspec version differs' }
$report = [ordered]@{
    schema = 'recur-lang-chocolatey-smoke-v1'
    version = '0.2.8'
    archive_sha256 = $script:expectedHash
    package_sha256 = (Get-FileHash -LiteralPath $package -Algorithm SHA256).Hash
    installed_shims = $script:installed
    uninstalled_shims = $script:uninstalled
    helper_mode = 'mocked; no installation or publication'
}
$report | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath (Join-Path $work 'chocolatey-smoke.json')
Write-Output 'PASS: packed version, URL/checksum binding and six companion install/uninstall shims'
