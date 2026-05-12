$ErrorActionPreference = 'Stop'

$packageName = 'recur'

Uninstall-BinFile -Name 'recur-git'
Uninstall-BinFile -Name 'recur-watch'
