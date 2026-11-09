# recur Config Family

Status: `readme` (permanent)
Date: 2026-03-08

Commands for project setup and trait configuration.

```bash
recur children "main.command.config" -d docs/ --sep .
recur related "main.command.config.trait" -d docs/ --sep .
```

## Commands

main.command.config.trait - list/get/set trait parameters in `.recur/config.toml`
main.command.config.init - initialize `.recur/` directory with full default config
main.command.config.reveal - open lane-local `*.recur.md` reveal capsules using `.recur/config.toml`
main.command.config.version - expose artifact version policy/schema from `.recur/config.toml`

## trait

`recur trait` manages configurable behavior for each command family.
Traits are stored under `[traits.*]` sections in `.recur/config.toml`.

```bash
recur trait list                                              # all trait sections
recur trait get trace_id.producer_keywords                   # read a value
recur trait set trace_id.producer_keywords "publish,emit"    # write a value
```

Current trait sections:
- `[traits.content_search]` - file-size and binary-file policy for content search commands
- `[traits.separator_merge]` - merge-specific project policy lane
- `[traits.stdin]` - stdin path resolution and missing-path policy
- `[traits.trace_id]` - keyword vocabulary for trace-id classification
- `[traits.traversal_budget]` - depth cap and guardrail defaults

## Companion workflow defaults

Shared query controls belong in `[traits.*]`. For example, `[traits.trace_id]`
holds role keywords and `[traits.content_search]` governs content reads. A
companion chooses how to use those capabilities for its workflow. Its local
choices belong in the companion’s own config section.

| Companion | Setup | Config it owns |
| --- | --- | --- |
| Trace ID | `recur-trace-id init --dry-run --json` | `[recur-trace-id]`, with inventory and excerpt preferences |
| Git | `recur-git init --dry-run --json` | `[checkpoint]`, with the log path and lane root pattern |
| Warp | `recur-warp init --dry-run --json` | Warp creation, removal, dispatch, and reasoning preferences. Its preview includes the starter template where applicable. |
| Lang | `recur-lang init --dry-run --json` | `[recur-lang]`, with target and planning preferences |

Use `-d ROOT` to choose the starting directory. Each initializer fills missing
settings in the nearest project configuration and preserves existing values.
Omit `--dry-run` to apply the preview. Trace ID and Git init write only
`.recur/config.toml` and create its parent directory if needed. Git checkpoint
appends happen through the explicit checkpoint command.

The overview and drill-down report shows how the query surfaces fit together.
[Eventness and trace-ID discovery](main.command.trace-id.eventness.discovery.report.md)

## init

`recur init` scaffolds the `.recur/` directory with:
- `config.toml` - full default config including lane sections, `[reveal]`, and `[traits.*]`
- Ready for `recur trait set` customization per project
- Deduplicated lane section names when different directories normalize to the same key

`recur reveal` uses the `[reveal]` section to decide:
- which suffix counts as a reveal capsule (default: `.recur.md`)
- which ordered fields should be shown first
- whether persona can be skipped when already known

`recur version policy` and `recur version schema` read `[artifact."<subject>"]`
sections to expose project-specific versioning semantics without hardcoding a
domain into recur.

## References

- `docs/main.command.trait.readme.md`
- `docs/main.command.init.readme.md`
- `docs/main.command.reveal.readme.md`
- `docs/main.command.version.readme.md`
- `docs/main.command.init.lane-name-collision.complete.md`
- `docs/main.command.map.readme.md`
- `julia-tests/main.command.init.test.jl`
- `julia-tests/main.command.trait.test.jl`
- `julia-tests/main.command.version.test.jl`
- `src/main_command_init_impl.rs`
- `src/main_command_trait_impl.rs`
- `src/main_command_version_impl.rs`
- `src/trait/trace_id.rs`
