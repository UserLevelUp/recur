# Homebrew Packaging Future Plan

Status: `todo.future-plan` (not active yet)

## Intent

Add a Homebrew formula after the Cargo-first baseline is working.

## Why Later

- Homebrew is useful, but not the shortest path to broad installability
- formula maintenance should come after the crate/release story is stable

## Future Steps

1. Stabilize Cargo publication and release artifacts
2. Decide whether to ship bottles later or start with source install
3. Add formula instructions and update path
4. Add a recurring maintenance lane once it is real

## Discovery

```bash
recur files "main.package.homebrew.**" -d docs/
recur files "main.package.crates-io.**" -d docs/
```

