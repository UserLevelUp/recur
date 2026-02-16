# Improvement 7: Expelliarmus to Embeddings

Status: `phase3.todo.current` (Phase 3 active)

## Combined Phased Approach

Improvement 7 combines three previously separate specs into a unified pipeline:

| Phase | Name | Source Spec | Status |
|-------|------|-------------|--------|
| 1 | `.recur/config.toml` | `README.CORE.IMPROVEMENT7.recur-git.md` | **complete** |
| 2 | More flatten formats | `README.CORE.IMPROVEMENT12.md` | **complete** |
| 3 | `trace-stats` | `README.CORE.IMPROVEMENT7.md` | **active** |
| 4 | Farming tools | `README.CORE.IMPROVEMENT13.md` | planned |
| 5 | Embedding bridge | (new) | vision |

## Tracking

- TOML complete: `docs/main.improvement.7.phase2.flatten.toml.complete.md`
- YAML complete: `docs/main.improvement.7.phase2.flatten.yaml.complete.md`
- CSV complete: `docs/main.improvement.7.phase2.flatten.csv.complete.md`
- Phase 2 complete: `docs/main.improvement.7.phase2.complete.md`
- Active work: `docs/main.improvement.7.phase3.todo.current.md`
- Phase 3 references: `docs/main.improvement.7.phase3.todo.current.reference.md`
- Phase 3 triggers: `docs/main.improvement.7.phase3.todo.trigger.event.md`
- Phase 3 completed patch: `docs/main.command.trace.force.guardrails.complete.md`
- Phase 3 completed patch: `docs/main.command.trace-stats.cli-surface.complete.md`
- Active phase 3 patch: `docs/main.command.trace-stats.metrics.todo.current.md`
- Phase 1 completion: `docs/main.improvement.7.phase1.complete.md`
- Phase 1 test snapshot: `docs/main.improvement.7.phase1.julia-tests.complete.md`

## Discovery

```bash
recur files "main.improvement.7.**" -d docs/
recur tree "main.improvement.7" -d docs/
```
