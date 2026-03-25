# main.package.crates-io.recurring

Recurring rediscovery point for publishing and maintaining `recur` on crates.io.

## Why This Lane Matters

Crates.io is the cleanest cross-platform install surface for a Rust CLI.
It gives `recur` one portable install story for both Windows and Linux:

```bash
cargo install recur
```

## Core Goal

Make `recur` easy to install from any machine that already has Rust/Cargo.

## Recurring Checks

- crate metadata in `Cargo.toml` stays accurate
- version stays aligned with `VERSION`
- README install section matches the current recommended Cargo flow
- published crate contents are the files we actually want to ship
- release docs and package docs stay aligned

## When To Revisit

Revisit this lane when:

- preparing a new public release
- changing install instructions
- adding or removing packaged assets
- publishing the crate for the first time
- fixing crates.io metadata or packaging warnings

## Discovery

```bash
recur files "main.package.crates-io.**" -d docs/
cat Cargo.toml
cat VERSION
recur find "cargo install recur" --scope "README.**" -i
```

## Related Docs

- `docs/main.package.readme.md`
- `docs/main.package.crates-io.todo.current.md`
- `docs/main.package.crates-io.todo.trigger.event.md`
- `docs/main.version.readme.md`

