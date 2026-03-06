# Command: trait — Complete

Status: `complete`
Date: 2026-03-06
Version: a.0.2.8

## What Landed

`recur trait` promoted from side-channel `maybe_execute_from_args()` dispatch
to a first-class `Commands::Trait` enum variant in `main.rs`.

- `recur --help` now lists `trait` alongside `init`, `files`, etc.
- `recur trait --help` shows `list`, `get`, `set` subcommands
- `TraitSubcommand` made `pub`; `execute(command, dir, json)` is the public entry point
- Side-channel removed; `maybe_execute_from_args()` deleted from trait impl
- `trait` removed from `after_help` Additional commands note

## Files Changed

- `src/main_command_trait_impl.rs` — pub TraitSubcommand, new execute(), removed side-channel
- `src/main.rs` — Trait variant in Commands enum, match arm, after_help updated
- `docs/main.command.trait.readme.md` — created (permanent docs)
- `docs/main.improvement.8.trace-id.todo.current.md` — corrected deferred/implemented status

## Test Baseline

- Rust: 120 passed, 0 failed
- Julia trait tests: `julia-tests/runtests.trait.jl` — CLI contract + traversal budget passing,
  `traversal_budget.min_depth` correctly marked `@test_broken` (future work)

## Related

- `docs/main.command.trait.readme.md`
- `docs/main.improvement.8.trace-id.todo.current.md`
