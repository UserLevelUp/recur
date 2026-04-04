# Recur Expert Recurring

This file is the rediscovery point for "be a recur expert" behavior in this repo.

## What this means

Work as a Rust expert who dogfoods `recur` to build `recur`.
Use `recur` first for discovery instead of manually browsing for files.

Pick the right layer before choosing commands:

- Search and analysis: `find`, `id`, `trace-id`, `callers`, `callees`, `trace`, `trace-stats`, `flatten`
- State and workflow: `files`, `tree`, `stats`, `related`, `children`, `merge`, `init`, `trait`

## Naming Basics

Start with a simple prefix like `main` or `README`.
That gives the hierarchy room to add a base, suffix, and eventness cleanly over time.

Useful mental models:

- `prefix.base.suffix[.expanding.eventness][.ext]`
- `prefix.base.suffix[.collapsing.eventness][.ext]`

`.ext` is optional. Most files in this repo have one; some do not.

Examples in this repo:

- `main.command.trace-id.readme.md`
- `main.command.trace-id.test.todo.current.md`
- `main.improvement.9.trace-id.todo.future-plan.md`

The point of the simple prefix is not minimalism for its own sake.
It is to preserve extra hierarchical layers for the interesting parts that come later.

## Interest Marker Note

Eventness in this repo is a naming convention layered on stable hierarchical names.
It is not recur's core ontology.

Treat it as a visible marker of interest:

- stable identity lives in `prefix.base.suffix`
- expanded eventness carries live working context
- during expansion, use `recur` commands to discover what is interesting and store exact `recur` commands inside the eventness files when that helps resume work
- collapsed or closed eventness keeps only the amount of signal that still matters
- collapse usually ends in `complete`, `future-plan`, `recurring`, or full removal of the ephemeral file

## Recurring Lifecycle

Treat eventness as a recurring operating pattern:

1. Expand interest around a stable subject.
2. Use `recur` commands to discover the next useful questions and commands.
3. Store those commands in the expanded eventness files when they improve rediscovery.
4. Collapse once the interest window closes.
5. Keep only the durable residue that still matters, or remove the file entirely.

## Eventness Reminder

`recurring` is a valid rediscovery/eventness concept, and it is also a good collapsed form when the useful outcome is "remember this pattern next time."
Most work in this repo is still non-recurring improvement work.
In practice, expect more files like `todo`, `current`, `complete`, `future-plan`, `reference`, and `trigger.event` than `recurring`.

Use `recurring` when you want a durable "remember this workflow next time" rediscovery point.
Use the other eventness forms for active project progress and one-off improvement tracking.

## Active Queue Rule

When deciding what is still active, trust `**.current` results before `complete` records.
`complete` is a release/history record; `current` is the live queue.

In practice:

- Start with `recur files "**.current" -d docs/`
- Treat those files as the official active lanes unless a newer rediscovery note says otherwise
- Use `complete` docs for baseline/history, not for deciding what is left
- If a feature branch has already been merged, the remaining work usually lives in the `current` improvement and test lanes, not in the old feature branch name itself

## Fast Rehydration Order

When training Skippy or another LLM back into this repo, rehydrate in this order:

1. `docs/eventness_explained_whitepaper.docx` for the deeper eventness theory and equation framing.
2. `README.CORE.EVENTNESS.md` for the operational model used in this repo.
3. `docs/main.recur.expert.recurring.md` for the repo-specific rediscovery rules.
4. `julia-expert/references/recur-playbook.md` for the concrete command workflow.

Then switch from reading to query-time discovery:

```powershell
recur files "**.current" -d docs/
recur files "**.recurring" -d docs/
recur files "**.reference" -d docs/
recur tree "main" --sep . --sep _ --show-sep
```

## Canonical references

- `docs/eventness_explained_whitepaper.docx` - deeper eventness theory and equation framing
- `README.CORE.EVENTNESS.md` - operational eventness model for this repo
- `julia-expert/references/recur-playbook.md` - canonical prompt/playbook
- `docs/AGENT.PROMPT.recur-expert.md` - pointer to canonical prompt
- `ulu_docs/recur-agent.md` - supplemental older agent notes
- `ulu_docs/recur-agent.workflow.md` - supplemental workflow notes
- `ulu_docs/recur-agent.static-analysis.md` - supplemental static-analysis notes

## Fast start

Run these first when returning to the repo:

```powershell
recur files "**.current" -d docs/
recur files "**.recurring" -d docs/
recur find "recur expert" --scope "**" -d docs/ -i
recur files "**.reference" -d docs/
recur files "**agent**" -d ulu_docs/
recur tree "main" --sep . --sep _ --show-sep
recur find "trace-id" --scope "**" -d docs/ -i
```

## Repo-specific reminders

- Rust source usually uses `_` separators, for example `recur files "main_command_*" -d src/ --sep _`
- Docs and workflow files usually use `.` separators, for example `recur files "main.**" -d docs/`
- Before deep work, skim the recur guidance docs for 5 minutes, then explore with real queries
- Prefer existing references and current work items before inventing a new pattern
- For release/package work, use the version branch lane `a.X.Y.Z`
- If moderation or packaging feedback is for an older already-submitted version, branch from that historical release point, for example `a.0.2.5` from `v0.2.5`
- The main workspace may stay on the current version branch while an older version branch lives in a separate worktree

## Why this file exists

The goal is to make the recur-expert context easy to rediscover with one suffix-based search:

```powershell
recur files "**.recurring" -d docs/
```
