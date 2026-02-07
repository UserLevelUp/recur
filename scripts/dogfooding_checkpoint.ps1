param(
    [string]$SrcSeparator = "_",
    [switch]$RunTests,
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

& recur @args

if ($LASTEXITCODE -ne 0) {
    throw "recur checkpoint failed with exit code $LASTEXITCODE"
}
