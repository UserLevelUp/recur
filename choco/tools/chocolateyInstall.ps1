$ErrorActionPreference = 'Stop'

$packageName = 'recur'
$version = $env:chocolateyPackageVersion

$url64 = "https://github.com/userlevelup/recur/releases/download/v${version}/recur-v${version}-x86_64-pc-windows-msvc.zip"

$installDir = "$(Split-Path -Parent $MyInvocation.MyCommand.Definition)"

$packageArgs = @{
    packageName    = $packageName
    unzipLocation  = $installDir
    url64bit       = $url64
    checksumType64 = 'sha256'
    checksum64     = '%CHECKSUM64%'
}

Install-ChocolateyZipPackage @packageArgs

# Add shim for recur-git as well if it exists in the zip
$recurGit = Join-Path $installDir 'recur-git.exe'
if (Test-Path $recurGit) {
    Install-BinFile -Name 'recur-git' -Path $recurGit
}