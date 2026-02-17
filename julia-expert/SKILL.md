---
name: julia-expert
description: Julia and recur workflow expertise for developing the recur codebase. Use when working in recur to discover state via hierarchical queries, find reference implementations, and implement or validate command behavior.
---

# Julia Expert

## Overview
Use recur as external memory for current work, next actions, and reusable patterns while writing Julia and Rust tooling.

## Core Workflow
1. Run discovery queries first (`**.current`, `**.todo`, `**.reference`, `**.trigger.event` in `docs/`).
2. Use `--sep _` for queries in `src/`.
3. Reuse known patterns via reference files before implementing.
4. Clean up ephemeral `.current` and `.trigger.event` files when work is complete.

## References
Load `references/recur-playbook.md` for full commands and examples.

`references/recur-playbook.md` is the canonical recur-expert prompt source in this repo.
