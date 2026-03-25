# AUR Packaging Future Plan

Status: `todo.future-plan` (not active yet)

## Intent

Add an Arch User Repository package after the Cargo-first baseline is stable.

## Why Later

- crates.io gives a faster cross-platform win first
- AUR adds another maintenance surface
- it is easier once release/version flow is already repeatable

## Future Steps

1. Publish and verify crates.io first
2. Create a clean PKGBUILD flow
3. Decide whether AUR should build from source or consume release artifacts
4. Document install and update flow
5. Add a recurring maintenance note if the package goes live

## Discovery

```bash
recur files "main.package.aur.**" -d docs/
recur files "main.package.crates-io.**" -d docs/
```

