# main

`main` is the root prefix for dogfooding metadata in this repository.

Use `recur tree "main"` in `docs/` and `julia-tests/` to inspect coverage.

Primary process guide:
- `docs/main.dogfooding.readme.md`

Packaging and release guides:
- `docs/main.choco.readme.md`
- `docs/main.choco.install.recurring.md`
- `docs/main.version.readme.md`
- `docs/main.recur.expert.recurring.md`

Active queue discovery:
- `recur files "**.current" -d docs/`
- Treat `*.current` files as the live queue and `*.complete` files as release/history records

State history logs:
- `docs/main.dogfooding.history.md`
- `docs/main.dogfooding.parallel.history.md`
- `docs/main.separator.history.md`
- `docs/main.git.checkpoint.readme.md`
