# recur Config Family

Status: `readme` (permanent)
Date: 2026-03-08

Commands for project setup and trait configuration.

```bash
recur children "main.command.config" -d docs/ --sep .
recur related "main.command.config.trait" -d docs/ --sep .
```

## Commands

main.command.config.trait — list/get/set trait parameters in `.recur/config.toml`
main.command.config.init — initialize `.recur/` directory with full default config

## trait

`recur trait` manages configurable behavior for each command family.
Traits are stored under `[traits.*]` sections in `.recur/config.toml`.

```bash
recur trait list                                              # all trait sections
recur trait get trace_id.producer_keywords                   # read a value
recur trait set trace_id.producer_keywords "publish,emit"    # write a value
```

Current trait sections:
- `[traits.trace_id]` — keyword vocabulary for trace-id classification
- `[traits.traversal_budget]` — depth cap and guardrail defaults

## init

`recur init` scaffolds the `.recur/` directory with:
- `config.toml` — full default config including all `[traits.*]` sections
- Ready for `recur trait set` customization per project

## References

- `docs/main.command.trait.readme.md`
- `docs/main.command.init.readme.md`
- `docs/main.command.map.readme.md`
- `src/main_command_trait_impl.rs`
- `src/trait/trace_id.rs`
