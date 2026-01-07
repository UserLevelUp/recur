Yes! `recur` does several things that `ripgrep` doesn't:

## What `recur` Does That `rg` Doesn't

| Feature | `recur` | `rg` (ripgrep) |
|---------|---------|----------------|
| **Hierarchy-aware tree view** | `recur tree "Service" --count` | ❌ No |
| **Scoped search by hierarchy** | `recur find "x" --scope "Module.Sub.*"` | ❌ No (searches everything) |
| **Find sibling files** | `recur related "File.Ops.cs"` | ❌ No |
| **Find child files** | `recur children "Module.Sub"` | ❌ No |
| **Dot-notation identifier search** | `recur id "ulu.executor.*"` | ⚠️ Can do with regex, but not semantic |
| **Understands file naming hierarchy** | ✅ Built for `A.B.C.cs` patterns | ❌ Treats dots as literal characters |

## Examples of What Only `recur` Can Do

```powershell
# 1. Visualize hierarchical structure (ripgrep can't do this)
recur tree "DynamicGameComponentService" --count
# Output:
# DynamicGameComponentService (base)
# ├── Cache.cs
# ├── Ops.cs
# │   ├── Batch.cs
# │   ├── Crud.cs
# │   └── Hierarchy.cs
# └── SoftDelete.cs

# 2. Search ONLY within a hierarchy branch
recur find "async" --scope "DynamicGameComponentService.Ops.*" --ext ".cs"
# ripgrep would search EVERYTHING

# 3. Find related/sibling files
recur related "DynamicGameComponentService.Ops.cs" --exclude-self
# Returns: Cache.cs, Private.cs, SoftDelete.cs (same parent)

# 4. Find dot-notation identifiers semantically
recur id "exec.admin.*" --ext ".cs"
# Understands this is a hierarchical ID, not just text
```

## Summary

| Tool | Best For |
|------|----------|
| `rg` | **Fast text search across entire codebase** |
| `recur` | **Navigating/searching hierarchically-named files and identifiers** |

**`recur` is novel because it was designed specifically for your naming convention** - hierarchical file names (`LevelController.CreateWizard3.Templates.cs`) and dot-notation identifiers (`ulu.executor`, `exec.admin.version`).

`rg` is a better grep. `recur` is a **hierarchy-aware search tool** - a different category entirely.
