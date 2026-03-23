# main.choco.readme

Persistent reference for Chocolatey packaging in this repo.

## Core Rules

- Keep nuspec `<id>` lowercase: `recur`
- Keep nuspec `<title>` in title case: `Recur`
- Follow Chocolatey CCR guideline `CPMR0050`: do not leave `<title>` as the same lowercase value as `<id>`
- Keep nuspec `<version>` aligned with `VERSION` and `Cargo.toml`
- Keep the nuspec description, command list, and tags aligned with `recur --help` and `recur-git --help`

## Branch + Eventness

Use one release branch per version.
The branch name should match the version lane, for example `a.0.2.8`.
Feature branches roll into the version branch before publication.
Example: merge a feature branch like `trace-id` into `a.0.2.8` before cutting the package.

Version eventness should stay easy to find:

- Active integration cursor: `docs/main.version.a.X.Y.Z.todo.current.md`
- Permanent shipped record: `docs/main.version.a.X.Y.Z.complete.md`
- Recurring Chocolatey install guide: `docs/main.choco.install.recurring.md`

## When Creating a New Version

1. Create or switch to the version branch `a.X.Y.Z`
2. Merge the feature branches intended for that release into the version branch
3. Update `VERSION`, `Cargo.toml`, `README.md`, and `choco/recur.nuspec`
4. In the nuspec, keep `<id>recur</id>` and `<title>Recur</title>`
5. Bump `<version>` in the nuspec to `X.Y.Z`
6. Recheck the nuspec feature list against `recur --help` and `recur-git --help`
7. Review `choco/tools/VERIFICATION.txt` if installation or verification steps changed
8. Create or update the version eventness doc for that branch

## Discovery

```bash
recur files "main.choco.**" -d docs/
recur files "main.choco.install.**" -d docs/
recur files "main.version.**" -d docs/
recur find "Chocolatey" --scope "main.choco.**" -d docs/
recur --help
recur-git --help
cat choco/recur.nuspec
```

## Files Usually Touched

- `choco/recur.nuspec`
- `choco/tools/VERIFICATION.txt`
- `VERSION`
- `Cargo.toml`
- `README.md`
- `docs/main.version.a.X.Y.Z.todo.current.md`
- `docs/main.version.a.X.Y.Z.complete.md`