A `.recur/` folder would solve most of the gaps in one shot:

```
.recur/
├── config.toml          # Project-specific settings
├── checkpoints.md       # Default checkpoint log location
└── (future: cache, indices)
```
```
# .recur/config.toml

[docs]
dir = "docs/"
sep = "."

[src]
dir = "User Level Up/"
sep = "."

[checkpoint]
file = ".recur/checkpoints.md"
root_pattern = "**"          # not hardcoded to "main.*"

[status]
# recur status reads these
current_suffix = ".current.md"
todo_suffix = ".todo.md"
complete_suffix = ".complete.md"
```

That fixes:
- **Lane detection** — `root_pattern = "**"` means checkpoint finds `CreateWizard3.**.current`, not just `main.**`
- **Checkpoint file location** — no more guessing, `recur-git checkpoint --append-parallel` just knows where to write
- **Multi-dir awareness** — `docs/` uses `.` separator, `src/` uses `.` too but different root
- **`recur status`** — knows which suffixes to count without flags

And it follows the convention every developer already knows: `.git/`, `.vscode/`, `.github/`. You `recur init` once, it creates `.recur/config.toml`, and every command after that just works without flags.

Attempt to implement where the recur-git logs to and how it works.  Use recur-git commands to configure recur and recur-git to specific solution or project working on.