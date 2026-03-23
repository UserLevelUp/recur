# Recur Agent: Julia Scripts Lane

> Extracted from `recur-agent.md` — run `recur tree "recur-agent" -d docs/agents/` to see all sections.

## Julia Scripts as Database Verification Lane

**Always create `.jl` files — never inline Julia in PowerShell.** PowerShell mangles multiline Julia with escaping issues.

### The `--fix` Pattern

Julia verification scripts should be **read-only by default**, with `--fix` to repair:

```julia
# jl/Level.Game.ContentType.Content.check-nulls.jl
# Run without args: check only (safe)
# Run with --fix:   backfill NULLs

if "--fix" in ARGS
    execute(pg, "UPDATE ulu_levels.level SET contenttype = 'ulu.level' WHERE contenttype IS NULL")
    println("  Done. Re-run without --fix to verify.")
end
```

```bash
julia jl/Level.Game.ContentType.Content.check-nulls.jl          # Check only
julia jl/Level.Game.ContentType.Content.check-nulls.jl --fix    # Fix + verify
```

### Julia Script Naming in jl/ Lane

Scripts follow the same hierarchical naming as docs:

| Pattern | Purpose | Example |
|---------|---------|---------|
| `*.verify.jl` | Cross-layer code verification | `Level.Game.ContentType.Content.verify.jl` |
| `*.check-nulls.jl` | DB data verification + `--fix` | `Level.Game.ContentType.Content.check-nulls.jl` |
| `*.patch-*.jl` | One-shot code patches | `DashboardController.patch-signout-v2.jl` |
| `mongo.*.jl` | MongoDB operations | `mongo.check-ownership-collections.jl` |
| `postgres.*.jl` | PostgreSQL operations | `postgres.describe-table.jl` |

### Mongoc.jl Gotcha: Use COALESCE, Not `something()`

LibPQ returns `missing` for NULL, but `something(missing, "fallback")` can crash Julia. Use SQL-side COALESCE instead:

```julia
# ? Crashes: something(row.contenttype, "NULL")
# ? Works:  COALESCE in SQL
r = execute(pg, "SELECT COALESCE(contenttype, 'NULL') as ct FROM ulu_levels.level")
```

## Cross-Lane
- Parent: `docs/agents/recur-agent.md`
