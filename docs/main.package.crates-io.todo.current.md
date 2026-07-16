# Crates.io Package Setup

Status: `todo.current` (active packaging lane)

## Purpose

Get `recur` into crates.io so the default cross-platform install path becomes:

```bash
cargo install recur
```

## Why This Is Worth Doing

- one install story for Windows and Linux
- lower friction than per-distro packaging
- good baseline before AUR/Homebrew/Nix
- portable workflow for any machine with Rust

## Current Goal

Prepare the repo so crates.io publication is straightforward and repeatable.

## Readiness Check 2026-05-23

Package-content verification exposed and fixed one release blocker:

- `cargo package --allow-dirty --list` originally included local scratch/build
  folders such as `.tmp/` and `target2/`, producing a very large crate archive.
- `Cargo.toml` now excludes scratch folders, generated release archives, local
  temp fixtures, and cached demo output from crates.io packaging.
- Current package list: `442` files.
- Current package size: `2.7MiB` unpacked, `906.5KiB` compressed.
- `cargo package --allow-dirty` verifies successfully when using a separate
  `CARGO_TARGET_DIR`; the normal `target/debug/recur-watch.exe` can be locked
  by active local watcher processes on Windows.

Publication and fresh install verification are still intentionally open.

## Checklist

1. Verify crate metadata in `Cargo.toml`
2. Confirm version alignment across `Cargo.toml`, `VERSION`, and release docs
3. Recheck README install instructions for Cargo-first workflow
4. Verify the package contents that would ship
5. Publish the crate
6. Verify fresh install from Cargo on this machine
7. Collapse this lane into a stable recurring or complete record once the baseline works

## Likely Files

- `Cargo.toml`
- `VERSION`
- `README.md`
- `docs/main.package.crates-io.recurring.md`
- `docs/main.version.a.X.Y.Z.complete.md`

## Discovery

```bash
recur files "main.package.crates-io.**" -d docs/
cat Cargo.toml
cat VERSION
recur find "cargo install recur" --scope "README.**" -i
cargo package --allow-dirty --list
```

## References

- `docs/main.package.readme.md`
- `docs/main.package.crates-io.recurring.md`
- `docs/main.package.crates-io.todo.trigger.event.md`

