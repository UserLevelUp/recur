# Recur Expert Recurring

This file is the rediscovery point for "be a recur expert" behavior in this repo.

## What this means

Work as a Rust expert who dogfoods `recur` to build `recur`.
Use `recur` first for discovery instead of manually browsing for files.

Pick the right layer before choosing commands:

- Search and analysis: `find`, `id`, `callers`, `callees`, `trace`, `trace-stats`, `flatten`
- State and workflow: `files`, `tree`, `stats`, `related`, `children`, `merge`

## Naming Basics

Start with a simple prefix like `main` or `README`.
That gives the hierarchy room to add a base, suffix, and eventness cleanly over time.

Useful mental models:

- `prefix.base.suffix.[eventness-expanded].[ext]`
- `prefix.base.suffix.[eventness-collapsed].[ext]`

Examples in this repo:

- `main.command.trace-id.readme.md`
- `main.command.trace-id.test.todo.current.md`
- `main.improvement.9.trace-id.todo.future-plan.md`

The point of the simple prefix is not minimalism for its own sake.
It is to preserve extra hierarchical layers for the interesting parts that come later.

## Eventness Reminder

`recurring` is a valid rediscovery/eventness concept, but most work in this repo is non-recurring improvement work.
In practice, expect more files like `todo`, `current`, `complete`, `future-plan`, `reference`, and `trigger.event` than `recurring`.

Use `recurring` when you want a durable "remember this workflow next time" rediscovery point.
Use the other eventness forms for active project progress and one-off improvement tracking.

## Canonical references

- `docs/AGENT.PROMPT.recur-expert.md`
- `julia-expert/references/recur-playbook.md`
- `ulu_docs/recur-agent.md`
- `ulu_docs/recur-agent.workflow.md`
- `ulu_docs/recur-agent.static-analysis.md`

## Fast start

Run these first when returning to the repo:

```powershell
recur files "**.recurring" -d docs/
recur find "recur expert" --scope "**" -d docs/ -i
recur files "**.current" -d docs/
recur files "**.reference" -d docs/
recur files "**agent**" -d docs/
recur find "trace-id" --scope "**" -d docs/ -i
```

## Repo-specific reminders

- Rust source usually uses `_` separators, for example `recur files "main_command_*" -d src/ --sep _`
- Docs and workflow files usually use `.` separators, for example `recur files "main.**" -d docs/`
- Before deep work, skim the recur guidance docs for 5 minutes, then explore with real queries
- Prefer existing references and current work items before inventing a new pattern

## Why this file exists

The goal is to make the recur-expert context easy to rediscover with one suffix-based search:

```powershell
recur files "**.recurring" -d docs/
```
