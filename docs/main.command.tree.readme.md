# main.command.tree.readme

Command overview for `tree`.

`tree` accepts either a literal hierarchy base or a full wildcard pattern. When
the query has a literal prefix before its first wildcard, that prefix anchors
the rendered tree. For example, this renders every `current` state under
`main`, including a leaf that ends at `current`:

```powershell
recur tree "main.**.current.**" -d docs/
```

Use `**` as a complete hierarchy segment for zero or more segments. `****` is
only a wildcard within one segment and is not a recursive hierarchy operator.

## Configured directory lanes

After `recur init` records directory separator mappings in `.recur/config.toml`,
`recur tree main` from the project root interprets each file using its most
specific configured directory mapping. For example, underscore-named source
`src/main_command_watch_impl.rs` and dot-named documentation
`docs/main.command.watch.readme.md` appear together under `main.command.watch`.
JSON paths retain the actual filenames, not normalized synthetic source paths.

The nearest config to `-d` applies throughout the query; descendant configs
are not merged. Unmapped directories default to dots. Subdirectory and stdin
queries use the same per-file lane policy. Malformed configuration fails
explicitly rather than silently dropping a lane.

An explicit `--sep` overrides automatic lane selection and retains the existing
single/multi-separator behavior. Without configuration, the default remains
the dot separator. `--show-sep` marks configured origins in text output;
`--sep-replace-default` selects the display separator without renaming files.
Queries never initialize or update configuration.

defines: recur.tree.configured-lanes.root root queries honor descendant lane separator mappings
consumes: recur.separator.lane-policy nearest configuration and most specific directory mapping
produces: recur.tree.configured-lanes.acceptance tests/tree_configured_lanes.rs
