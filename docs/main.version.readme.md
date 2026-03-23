# main.version.readme

Persistent guidance for version branches and release records.

## Branch Strategy

Use one branch per releasable version.
The branch name should be the version lane, for example `a.0.2.8`.
This keeps release integration work isolated and easy to search.
If a packaging or moderation fix is for an older submitted version, branch from that older release point and name it for that version, for example `a.0.2.5` from `v0.2.5`.

Feature branches should roll into the version branch before the version is finalized.
Example: merge `trace-id` into `a.0.2.8`, then do release checks and packaging there.

## Eventness Pattern

Recommended version eventness:

- `docs/main.version.a.X.Y.Z.todo.current.md` for the active release integration cursor
- `docs/main.version.a.X.Y.Z.complete.md` for the permanent shipped release record

If the release is small, it is fine to go straight to the permanent `complete` record.
If the release has active coordination work, use `todo.current` first and close it into `complete` when shipped.

## Minimum Release Surfaces

When creating a new version, review these together:

- `VERSION`
- `Cargo.toml`
- `README.md`
- `choco/recur.nuspec`
- `docs/main.version.a.X.Y.Z.complete.md`

For Chocolatey, keep nuspec `<id>` lowercase and nuspec `<title>` in title case.

## What to Record in the Version Doc

Capture the parts that will matter later:

- Branch name
- What landed
- Which feature branches were rolled in
- Test baseline at release
- What was deferred to the next version
- References to the active improvement lanes

## Discovery

```bash
recur files "main.version.**" -d docs/
recur tree "main.version" -d docs/
recur find "main.version.a.0.2.8" --scope "**" -d docs/
```

## Related Docs

- `docs/main.choco.readme.md`
- `docs/main.choco.todo.current.md`
- `docs/main.version.a.0.2.8.complete.md`
