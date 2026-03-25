# main.package.readme

Persistent reference for packaging and distribution lanes in this repo.

## Intent

Use `main.package.*` as the umbrella hierarchy for package-manager setup,
release workflow, and future distribution channels.

Keep version history separate:

- `main.package.*` tracks how packaging works
- `main.version.a.X.Y.Z.*` tracks what shipped in a specific version

## Current Lane Layout

- `docs/main.package.crates-io.recurring.md`
- `docs/main.package.crates-io.todo.current.md`
- `docs/main.package.crates-io.todo.trigger.event.md`
- `docs/main.package.aur.todo.future-plan.md`
- `docs/main.package.homebrew.todo.future-plan.md`
- `docs/main.package.nix.todo.future-plan.md`

Existing adjacent packaging lanes:

- `docs/main.choco.readme.md`
- `docs/main.choco.todo.current.md`
- `docs/main.choco.install.recurring.md`

## Recommended Use

- Use `crates-io` as the cross-platform baseline install lane
- Use `choco` as the Windows package lane
- Use `aur`, `homebrew`, and `nix` as future Linux/macOS distribution lanes
- Keep one active `todo.current` only where real work is live
- Use `future-plan` when a lane matters but is not yet active

## Discovery

```bash
recur files "main.package.**" -d docs/
recur tree "main.package" -d docs/
recur files "main.choco.**" -d docs/
recur files "main.version.**" -d docs/
```

## Related Docs

- `docs/main.choco.readme.md`
- `docs/main.version.readme.md`

