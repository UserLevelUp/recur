# Crates.io Package Trigger Events

Status: `todo.trigger.event` (manual release checklist)

## Use

Run this when opening or closing the active crates.io packaging lane.

## Start Trigger

1. Confirm the active branch/version lane
2. Read `docs/main.package.crates-io.todo.current.md`
3. Check `Cargo.toml`, `VERSION`, and `README.md`
4. Verify what `cargo package` would include
5. Record any blockers before publishing

## Complete Trigger

1. Publish to crates.io
2. Verify `cargo install recur` on a clean path
3. Update version or packaging docs if the real outcome differed
4. Convert active notes into a stable record if needed
5. Remove or collapse `todo.current` when this packaging window closes

## Discovery

```bash
recur files "main.package.crates-io.**" -d docs/
cat Cargo.toml
cargo package --allow-dirty --list
```

## Related

- `docs/main.package.crates-io.todo.current.md`
- `docs/main.package.crates-io.recurring.md`

