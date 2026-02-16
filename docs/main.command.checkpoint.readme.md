# main.command.checkpoint.readme

Workflow overview for checkpointing lane state via `recur-git`.

Purpose:
- capture a repeatable lane snapshot (git + active todo leaves + separator policy)
- optionally append a parallel-lane checkpoint entry

Common usage:
- `recur-git checkpoint --snapshot`
- `recur-git checkpoint --emit-parallel --checkpoint-id ck-<id>`
- `recur-git checkpoint --append-parallel --checkpoint-id ck-<id>`
- `recur-git checkpoint --append-parallel --checkpoint-id ck-<id> -f <path>`
- `powershell -ExecutionPolicy Bypass -File scripts/dogfooding_checkpoint.ps1 -AppendParallelEntry -CheckpointId ck-<id>`

Config behavior:
- If `.recur/config.toml` exists, checkpoint discovery uses configured lanes plus:
  - `[checkpoint].root_pattern`
  - `[status].current_suffix`
- `--append-parallel` writes to `--file` when provided; otherwise it uses `[checkpoint].file` from config.

Design note:
- `recur` remains pure hierarchy semantics.
- `recur-git` is the extension that runs git/test workflow commands.
