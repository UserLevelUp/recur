# Capability traits

Warp, watch, merge, unmerge and Git are discoverable capability traits. This
catalog does not introduce Rust trait interfaces or replace existing commands.
`unmerge` is explicitly proposed: there is no implemented unmerge command.
Implemented status describes the source capability, not whether its companion
binary is installed on PATH.

```powershell
recur trait list
recur trait explain warp
recur trait explain git --json
recur trait get warp.preference
recur trait set warp.preference preferred
recur trait set watch.notes "Use only for explicitly requested live monitoring"
```

For another root, place `-d` before the trait subcommand:
`recur trait -d ../project explain warp`.

## Configuration and effects

`recur init` writes these sections for the five catalog entries:

```toml
[traits.warp]
preference = "unspecified"
notes = ""
```

Preference accepts `unspecified`, `preferred`, or `discouraged`. These fields
record project intent for humans/agents. They do not enable, disable, authorize,
execute or schedule anything. In particular, a preferred watcher is not a request
to start it, and a preferred Git capability is not permission to commit or push.
`enabled` is intentionally unsupported for these capability entries.

Runtime configuration remains where it is implemented: for example
`[warp.discovery]` and `[warp.suffixes]`, or a companion's command-line options.
`recur trait explain` supplies references, not a duplicate runtime configuration.
Generic `merge` merges hierarchy results; it is distinct from `recur warp merge`.

## Compatibility and discovery

List/get/explain use the nearest ancestor `.recur/config.toml`, with virtual
defaults for missing catalog entries. With no config they can still discover
built-in capabilities without creating files. Set requires an existing config
(`recur init`), changes that nearest config, and preserves unrelated values.
Existing set serialization may reformat TOML/comments; this is not a lossless editor.

Existing custom traits remain listable/configurable; their settings are not
automatically interpreted or enforced by Recur. Built-in capability names reserve
only preference/notes as writable fields. Legacy custom fields under those names
produce an actionable validation error, rather than silently implying enforcement.
Catalog status, commands and effect are read-only derived facts.

List JSON retains its trait-name object shape and adds the built-ins. Explain JSON
uses `recur-trait-explain-v1` and includes catalog facts, effective fields, config
path, source and `mutation: none`. This is separate from `recur capability`, which
continues to inspect authored root `.recur-*` capability cards.

## Verification

For actual shell composition, see [Pipeline compatibility](main.command.pipeline.compatibility.readme.md).
Discoverability is distinct from stdin/JSON contract compatibility.

Focused CLI coverage: `julia julia-tests/main.command.trait.capabilities.test.jl`.
Tests cover no-config discovery, old/new configs, round trips, validation,
nonmutating reads, inherited config, custom traits and proposed unmerge status.

defines: recur.trait.capabilities catalog-backed capability discovery and descriptive project preferences
