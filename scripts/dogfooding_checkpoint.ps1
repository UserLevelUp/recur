param(
    [string]$SrcSeparator = "_",
    [switch]$RunTests,
    [switch]$RunJuliaTests,
    [string]$CheckpointId,
    [switch]$EmitParallelEntry,
    [switch]$AppendParallelEntry,
    [string]$ParallelLogPath = "docs/main.dogfooding.parallel.history.md"
)

$ErrorActionPreference = "Stop"

$args = @("checkpoint", "--snapshot", "--src-sep", $SrcSeparator)

if ($RunTests) {
    $args += "--run-tests"
}
if ($RunJuliaTests) {
    $args += "--run-julia-tests"
}
if ($EmitParallelEntry) {
    $args += "--emit-parallel"
}
if ($AppendParallelEntry) {
    $args += "--append-parallel"
    $args += "--parallel-log"
    $args += $ParallelLogPath
}
if (-not [string]::IsNullOrWhiteSpace($CheckpointId)) {
    $args += "--checkpoint-id"
    $args += $CheckpointId
}

$recurGit = "recur-git"
$cmd = Get-Command $recurGit -ErrorAction SilentlyContinue
if (-not $cmd) {
    if (Test-Path ".\target\release\recur-git.exe") {
        $recurGit = ".\target\release\recur-git.exe"
    } elseif (Test-Path ".\target\debug\recur-git.exe") {
        $recurGit = ".\target\debug\recur-git.exe"
    } else {
        throw "Could not find recur-git binary. Build with `cargo build --bin recur-git` or install it."
    }
}

& $recurGit @args

if ($LASTEXITCODE -ne 0) {
    throw "recur-git checkpoint failed with exit code $LASTEXITCODE"
}
