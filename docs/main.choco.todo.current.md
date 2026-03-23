# Choco Package: Keep Current

Status: `todo.current` (ongoing maintenance task)

## Purpose

Keep the Chocolatey package metadata in sync with the current release branch and command surface.

## Current Baseline

- Command coverage is synced at v0.2.8
- Chocolatey CCR guideline `CPMR0050` is handled by keeping `<id>recur</id>` and `<title>Recur</title>`
- The current permanent version record is `docs/main.version.a.0.2.8.complete.md`
- The recurring install rediscovery point is `docs/main.choco.install.recurring.md`

## Trigger: When to Update

Update Chocolatey metadata whenever:

- A new version branch is created, for example `a.X.Y.Z`
- A feature branch is rolled into the active version branch and the package description should reflect it
- A new command is added to `recur`
- A command description changes
- Tags should be updated to reflect new capabilities
- `recur-git` adds or removes commands that should be mentioned
- `choco/tools/VERIFICATION.txt` needs different verification steps

## When Creating a New Version

1. Work from the version branch `a.X.Y.Z`
2. Merge the intended feature branches into that version branch
3. Update `VERSION`, `Cargo.toml`, `README.md`, and `choco/recur.nuspec`
4. Keep nuspec `<id>` lowercase as `recur`
5. Keep nuspec `<title>` title-cased as `Recur`
6. Bump nuspec `<version>` to the release version
7. Create or update the version eventness doc for that branch
8. Recheck package text against `recur --help` and `recur-git --help`
9. Use `docs/main.choco.install.recurring.md` as the recurring install checklist

## Discovery

```bash
recur files "main.choco.**" -d docs/
recur files "main.choco.install.**" -d docs/
recur files "main.version.**" -d docs/
recur --help
recur-git --help
cat choco/recur.nuspec
cat VERSION
cat Cargo.toml
```

## Files to Update

- `choco/recur.nuspec` - package id, title, version, description, command list, tags
- `choco/tools/VERIFICATION.txt` - if verification steps change
- `VERSION` - repo version marker
- `Cargo.toml` - crate version
- `README.md` - displayed version
- `docs/main.version.a.X.Y.Z.todo.current.md` - active release cursor when needed
- `docs/main.version.a.X.Y.Z.complete.md` - permanent version record

## References

- `docs/main.choco.readme.md`
- `docs/main.choco.install.recurring.md`
- `docs/main.version.readme.md`