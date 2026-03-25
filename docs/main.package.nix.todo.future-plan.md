# Nix Packaging Future Plan

Status: `todo.future-plan` (not active yet)

## Intent

Add a Nix/Nixpkgs distribution path once the baseline Cargo package is stable.

## Why Later

- Nix is valuable for reproducible installs
- it is still a separate maintenance lane from the first Cargo baseline

## Future Steps

1. Publish and verify crates.io first
2. Decide whether to package from crate source or release source
3. Add Nix expression / package definition
4. Document install and update flow
5. Promote this lane from future-plan when active work begins

## Discovery

```bash
recur files "main.package.nix.**" -d docs/
recur files "main.package.crates-io.**" -d docs/
```
