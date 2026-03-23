# Recur Agent: Piping & Composability

> Extracted from `recur-agent.md` — run `recur tree "recur-agent" -d docs/agents/` to see all sections.

## stdin/stdout Piping

**Recur is a Unix-style composable tool** — all commands support `--stdin` to read file paths and output to stdout.

### Pipe Recur ? Recur (Multi-stage filtering)
```bash
recur files "**" -d docs/ | recur files "**.stdin.**" --stdin
```

### Pipe Git ? Recur
```bash
git diff --name-only | recur files "**" --stdin
git diff --name-only | recur find "TODO" --scope "**" --stdin
```

### Pipe rg ? Recur
```bash
rg -l "async" src/ | recur files "main_command_**" --stdin --sep _
```

### Pipe Recur ? Unix Tools
```bash
recur files "**.readme" -d docs/ | wc -l
recur files "**" -d src/ | grep "stdin"
```

### Pipe Recur ? PowerShell

```powershell
# Sort files by modified date
recur files "GITHUB-ISSUE**" | ConvertFrom-Json | ForEach-Object { Get-Item $_ } | Sort-Object LastWriteTime -Descending

# Count
recur files "**.todo" -d docs/ | ConvertFrom-Json | Measure-Object

# Filter
recur files "**.current" -d docs/ | ConvertFrom-Json | ForEach-Object { Get-Item $_ } | Where-Object { $_.Length -gt 1000 }
```

**PowerShell key insight:** Recur's default output is a JSON array of strings. Always pipe through `ConvertFrom-Json` to get clean PowerShell objects.

## Structured Flattening (`flatten`)

```bash
recur flatten config.xml                             # XML -> path=value
recur flatten config.json --json                     # JSON array output
recur flatten config.json --filter "config.db"       # Filter to prefix
recur flatten config.json --max-depth 2              # Limit depth
recur --sep _ flatten config.json --json             # Merge-friendly
```

**Flatten + merge:** `merge` can ingest `flatten --json`. Use `recur --sep _ flatten` when planning to merge flattened inputs (dot-separated paths are lossy in merge).

## Cross-Lane
- Parent: `docs/agents/recur-agent.md`
