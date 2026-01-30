# stdin Functionality: recur vs ripgrep Comparison

## Overview

Both `recur` and `ripgrep` support `--stdin` for pipeline workflows, but they serve different purposes:

- **ripgrep**: Filters file paths OR searches content from stdin
- **recur**: Filters file paths from stdin using **hierarchical patterns**

## Key Difference

**ripgrep** treats filenames as flat strings:
```powershell
ls *.cs | rg "UserService"  # Matches any file containing "UserService" substring
```

**recur** understands hierarchical structure:
```powershell
ls *.cs | recur files "UserService.**" --stdin  # Matches hierarchical pattern UserService.**
```

## Common Workflows

### 1. Git Integration - Filter Changed Files

**With ripgrep** (substring matching):
```powershell
git diff --name-only | rg "\.cs$"  # All .cs files
```

**With recur** (hierarchical patterns):
```powershell
# Filter changed files by hierarchy
git ls-files "*.cs" | recur files "UserService.**" --stdin

# Find only modified UserService handler files
git diff --name-only | recur files "UserService.Handlers.*" --stdin
```

### 2. Filesystem Exploration

**With ripgrep**:
```powershell
Get-ChildItem -Recurse -Name | rg "Service"  # Any file with "Service" in name
```

**With recur**:
```powershell
# Filter by hierarchical pattern
Get-ChildItem *.cs -Name | recur files "*.Service" --stdin

# Match deep hierarchies
Get-ChildItem *.cs -Name | recur files "UserService.**" --stdin
```

### 3. Chaining recur Commands

```powershell
# Find all UserService files, then search for "async" in them
recur files "UserService.**" | recur find "async" --scope "**" --stdin

# Get files at specific depth, filter further
recur files "**" | recur files "*.Handlers.*" --stdin
```

## Test Results (feature-stdin branch)

### ✅ Working Examples

```powershell
PS C:\src\recur\test_stdin> ls *.cs | Select-Object -ExpandProperty Name | ..\target\release-safe\recur.exe files "UserService.**" --stdin
.\UserService.cs
.\UserService.Handlers.cs
.\UserService.Models.cs
```

```powershell
PS C:\src\recur\test_stdin> ls *.cs | Select-Object -ExpandProperty Name | ..\target\release-safe\recur.exe files "Api*" --stdin
.\ApiController.cs
```

### Pattern Matching with stdin

| Input Files | Pattern | Matches |
|------------|---------|---------|
| `UserService.cs`<br>`UserService.Handlers.cs`<br>`UserService.Models.cs`<br>`ApiController.cs` | `UserService.**` | `UserService.cs`<br>`UserService.Handlers.cs`<br>`UserService.Models.cs` |
| Same files | `Api*` | `ApiController.cs` |
| Same files | `**.Handlers.*` | `UserService.Handlers.cs` |

## Comparison Matrix

| Feature | ripgrep | recur --stdin |
|---------|---------|---------------|
| **Input Type** | File paths OR content | File paths only |
| **Pattern Type** | Regex/substring | Hierarchical (`*`, `**`) |
| **Use Case** | Text search | Hierarchy filtering |
| **Hierarchy Awareness** | ❌ | ✅ |
| **Example** | `\| rg "Service"` | `\| recur files "*.Service" --stdin` |

## When to Use Each

### Use ripgrep when:
- Searching file **content** from stdin
- Simple substring matching on filenames
- Need regex patterns on flat strings
- Maximum speed is critical

### Use recur when:
- Filtering by **hierarchical patterns** (e.g., `Module.SubModule.**`)
- Working with dotted naming conventions
- Need to understand parent/child relationships
- Filtering git output by code structure

## Real-World Example

### Scenario: Find all modified UserService handler files

**With ripgrep** (limited):
```powershell
# Can only do substring matching
git diff --name-only | rg "UserService.*Handlers"
```

**With recur** (hierarchical):
```powershell
# Precise hierarchical filtering
git diff --name-only | recur files "UserService.Handlers.*" --stdin

# Or get all UserService descendants
git diff --name-only | recur files "UserService.**" --stdin
```

## Stdin Status on feature-stdin Branch

### ✅ Implemented & Tested
- `recur files --stdin` - **WORKING**
- CLI flag recognized
- Help text documented
- Pipeline compatibility verified

### 🔧 To Test
- `recur find --stdin`
- `recur stats --stdin`
- `recur tree --stdin`
- Other commands with stdin

### 📋 Implementation Notes

The stdin functionality reads file paths from stdin (one per line) and applies hierarchical pattern matching to filter them. This differs from filesystem scanning and allows integration with:

1. **Git commands**: `git ls-files`, `git diff --name-only`
2. **File explorers**: PowerShell `Get-ChildItem`, Unix `find`
3. **Other recur commands**: Chain multiple recur operations

## Conclusion

`recur --stdin` complements ripgrep by adding **hierarchical awareness** to pipeline workflows. While ripgrep excels at fast text search, recur provides structure-aware filtering for modern codebases with dotted naming conventions.

**Best Practice**: Use both together:
```powershell
# Find UserService files (recur), search for async patterns (ripgrep)
recur files "UserService.**" | rg "async.*Task"

# Or: Filter by structure (recur), search content (recur find)
git ls-files "*.cs" | recur files "UserService.**" --stdin | xargs recur find "async" --scope "**"
```
