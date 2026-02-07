# main.dogfooding.readme

## Purpose

This file defines the dogfooding process for hierarchical files in this repo.

Goal:
- Keep humans and LLMs aligned on one naming contract.
- Make missing artifacts obvious by running `recur tree "main"`.
- Use suffixes to capture next actions, priority, and status without separate tooling.

## Core Convention

Use `main` as the root prefix for cross-folder metadata.

Dot notation model:
- `main.<area>.<unit>.<artifact>[.<qualifier>]`

Examples:
- `main.command.files.test.jl`
- `main.command.files.readme.md`
- `main.command.files.todo.md`
- `main.command.files.todo.priority.md`

Rust source keeps underscore naming where needed:
- `main_command_files_impl.rs`
- `main_command_files_stdin.rs`

## Folder Roles

- `src/`: Rust implementation artifacts (underscore hierarchy with `--sep _`).
- `julia-tests/`: test artifacts under `main.command.*.test.jl`.
- `docs/`: docs and planning artifacts under `main.command.*`.

## Suffix Contract

Use suffixes intentionally:
- `readme`: behavior and usage docs
- `test`: executable validation
- `todo`: open work
- `todo.priority`: urgent open work

Optional chain/status suffixes are allowed:
- `todo.next`
- `todo.blocker`
- `todo.priority.p1`
- `todo.owner.<name>`

If a suffix is missing, that absence is signal.
Example: if `todo` exists but `todo.priority` does not, it is not currently priority.

## Daily Workflow

1. Inspect tree shape:
   - `recur tree "main" -d docs/`
   - `recur tree "main" -d julia-tests/`
2. Inspect Rust command inventory:
   - `recur files "main_command_*_impl" -d src/ --sep _`
3. Compare expected branches:
   - command exists in `src` but missing `main.command.<x>.test.jl` -> add test
   - command exists in `src` but missing `main.command.<x>.readme.md` -> add docs
   - todo exists but priority missing -> not priority (or undocumented urgency)
4. Update suffixes as work moves:
   - `todo` -> `todo.priority` when escalated
   - remove `todo*` when completed

## Human + LLM Collaboration Rules

- Humans decide naming policy and semantic meaning of suffixes.
- LLMs must follow existing prefix/base/suffix structure exactly.
- LLMs should not invent alternate roots when `main` already exists.
- Any new artifact should be added in a way that makes `recur tree "main"` more complete.

## Quick Commands

```bash
# Tree view by folder
recur tree "main" -d docs/
recur tree "main" -d julia-tests/

# Tests and docs coverage
recur files "main.command.**.test" -d julia-tests/
recur files "main.command.**.readme" -d docs/
recur files "main.command.**.todo" -d docs/
recur files "main.command.**.todo.priority" -d docs/

# Rust command inventory
recur files "main_command_*_impl" -d src/ --sep _
recur files "main_command_*_stdin" -d src/ --sep _
```

## Success Criteria

- `main` tree is understandable at a glance.
- Missing test/doc/todo branches are obvious by absence.
- Suffixes communicate what to do next without extra explanation.
