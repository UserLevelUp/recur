# main.choco.install.recurring

Recurring rediscovery point for creating or testing a new Chocolatey installable version.

## Use This When

Use this file when a new Chocolatey package version is being prepared, installed, upgraded, or re-checked.
This is the interesting recurring install eventness for the package lane.

## Core Reminders

- Work from the release branch `a.X.Y.Z`
- If Chocolatey feedback is for an older submitted version, branch from that exact release first
- Merge the feature branches for that version into the release branch first
- Keep nuspec `<id>` as `recur`
- Keep nuspec `<title>` as `Recur`
- Lowercase applies to the package `<id>`; keep the display `<title>` in title case
- Bump nuspec `<version>` to the release version
- Keep nuspec description and commands aligned with `recur --help`
- Review `choco/tools/VERIFICATION.txt` if install or verification steps changed
- Create or update the matching version doc under `docs/main.version.a.X.Y.Z.*.md`

## Suggested Flow

1. Start from the version branch `a.X.Y.Z`
2. If the active Chocolatey submission is older than current development, create that version branch from the matching tag first
3. Update `VERSION`, `Cargo.toml`, `README.md`, and `choco/recur.nuspec`
4. Recheck the nuspec command list against `recur --help` and `recur-git --help`
5. Build or collect the release artifact that Chocolatey will install
6. Review install and uninstall scripts in `choco/tools/`
7. Test the install or upgrade path you care about
8. Update the version eventness record for what actually shipped

## Discovery

```bash
recur files "**.recurring" -d docs/
recur files "main.choco.install.**" -d docs/
recur files "main.version.**" -d docs/
cat choco/recur.nuspec
```

## Related Docs

- `docs/main.choco.readme.md`
- `docs/main.choco.todo.current.md`
- `docs/main.version.readme.md`
