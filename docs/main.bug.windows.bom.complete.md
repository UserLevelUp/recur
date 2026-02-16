# Windows BOM Interop Bug Patch Complete

Status: `complete`
Date: 2026-02-16

## Closed Items

- `docs/main.command.merge.stdin.windows.bom.todo.md`
- `docs/main.command.flatten.json.windows.bom.todo.md`

## Summary

All currently known Windows BOM parsing gaps for merge/flatten JSON paths are fixed:
- merge file mode JSON with BOM (already fixed in previous patch)
- merge stdin JSON stream with BOM
- flatten JSON (file + stdin) with BOM

## Verification Commands

```bash
recur files "main.command.**.windows.bom.todo" -d docs/
recur files "main.command.**.windows.bom.complete" -d docs/
```
