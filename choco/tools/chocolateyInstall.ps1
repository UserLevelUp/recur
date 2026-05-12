$ErrorActionPreference = 'Stop'

$toolsDir = Split-Path -Parent $MyInvocation.MyCommand.Definition

$recur = Join-Path $toolsDir 'recur.exe'
if (!(Test-Path $recur)) {
    throw "Expected recur.exe in package tools directory."
}

Install-BinFile -Name 'recur' -Path $recur

$recurGit = Join-Path $toolsDir 'recur-git.exe'
if (Test-Path $recurGit) {
    Install-BinFile -Name 'recur-git' -Path $recurGit
}
