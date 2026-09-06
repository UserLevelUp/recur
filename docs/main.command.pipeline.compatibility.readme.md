# Composing Recur with pipes

Recur commands can compose through standard streams, provided the producer's
output matches the consumer's accepted input. Capability traits describe available
operations; they do not make unrelated JSON schemas interchangeable.

## Tested paths

```powershell
recur tree main -d src --json | recur merge --stdin --base main --json
git -c core.quotepath=false ls-files --others --exclude-standard |
    recur files "main.**" --stdin --json |
    recur merge --stdin --base main --json
recur trait explain warp --json | ConvertFrom-Json
```

The Git example reads untracked filenames; use the appropriate Git query for your
workflow. It does not stage, commit or push. Line-oriented filename lists cannot
represent filenames containing embedded newlines safely.

- Tree/merge exchange a supported hierarchy JSON shape.
- `files --stdin` consumes line-oriented paths; `files --json` produces a JSON path array.
- `merge --stdin` accepts supported path/hierarchy JSON, not arbitrary status/trait JSON.
- External JSON-aware programs may validate/filter/transform between stages.
- Trait, Warp and watcher outputs need a consumer that understands their schemas.
- Query pipelines are not authorization to execute a writer afterward.

## PowerShell UTF-8

The regression run found that a shell's default native-pipe encoding could change
`café` to `caf?`. Direct process pipes preserved it. Set encoding explicitly in
the PowerShell session when passing Unicode through native commands:

```powershell
[Console]::InputEncoding = [Console]::OutputEncoding = $OutputEncoding =
    [System.Text.UTF8Encoding]::new($false)
```

This is a shell setting, not a Recur output-format change. The PowerShell pipe test
retains exact Unicode equality after this setup.

## Check every stage before execution

A producer can emit usable JSON and then fail. A downstream consumer may still
exit successfully. The last stage's success is not proof that the pipeline succeeded.
The regression explicitly covers producer exit 7 with consumer exit 0.

For PowerShell workflows that must gate later execution, capture and check each
native query separately before composing its output:

```powershell
$treeJson = recur tree main -d src --json
if ($LASTEXITCODE -ne 0) { throw "Tree query failed" }

$mergedJson = $treeJson | recur merge --stdin --base main --json
if ($LASTEXITCODE -ne 0) { throw "Merge query failed" }

$projection = $mergedJson | ConvertFrom-Json -ErrorAction Stop
# Inspect/validate the projection, then obtain the required authority before a writer.
```

Bash users can use `set -o pipefail` and inspect stage statuses as needed.
Keep diagnostics on stderr; do not merge stderr into a machine-readable input stream.

## Preserved limitation

An empty `merge --stdin --json` currently succeeds with the text
`No files found in stdin`, not a JSON object. Tests preserve that existing behavior.
A JSON consumer must handle that case explicitly or reject it. A future normalized
empty-result contract would be a separate compatibility decision.

## Regression suite

`julia julia-tests/main.command.pipeline.compatibility.test.jl` runs real OS pipes,
external Julia JSON adapters, Git path input, and a real PowerShell `|` when
PowerShell is installed. It checks exact output equivalence, spaces/Unicode,
clean stderr on success, invalid-input errors, stage exit codes and unchanged fixture
bytes. It does not authorize or test automatic writer execution.

defines: recur.pipeline.compatibility tested standard-stream composition without changing existing contracts
