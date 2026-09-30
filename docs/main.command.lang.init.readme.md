# recur-lang init

defines: recur.lang.init.configuration additive project-local companion preferences

`recur-lang init [-d ROOT] [--dry-run] [--json]` installs missing opinionated Lang
preferences in the nearest ancestor `.recur/config.toml`, or creates that file
under ROOT when none exists. ROOT must already be a directory.

```powershell
recur-lang init -d . --dry-run --json
recur-lang init -d .
```

Preview writes nothing. Normal invocation adds absent known keys, preserving
explicit values (including `false`), unknown keys, comments and unrelated
configuration. Repeat invocation is byte-identical. Inline tables are supported.

```toml
[recur-lang]
schema_version = 1
target = "unspecified"

[recur-lang.planning]
specification_first = true
prioritize_graph_findings = true
include_test_plan = true
```

Target names are editable preferences; setting `julia`, `rust` or a custom name
does not install a backend. These settings do not execute a planner or bindings.
The core config loader reserves `recur-lang` as policy, so unknown `dir` or `sep`
keys cannot accidentally register it as a file lane.

The companion rejects malformed nearest configuration, invalid known fields,
unsupported schema versions and escaping config paths without changing bytes.
Publication stages and flushes in the destination directory, rechecks the original
bytes and preserves detected concurrent edits. A Windows sharing-lock failure
preserves the original file and removes this invocation's staging file. This is
not a lock against an adversarial writer racing the final filesystem operation.

Success exits 0 with `recur-lang-init-v1`: `project_root`, `config_path`, `dry_run`,
`changed`, `state`, `preview`, `writes`. State is `planned`, `written` or
`unchanged`; preview contains the complete proposed TOML. Operational errors
exit 2 with `recur-lang-init-error-v1` and code/message diagnostics:

| Code | Meaning |
| --- | --- |
| LINIT001 | Missing, unreadable or non-directory ROOT |
| LINIT002 | Unreadable/malformed config, invalid known field or schema |
| LINIT003 | Config path escapes the project boundary |
| LINIT004 | Concurrent change, staging or publication failure |

See the [contract](../warps/main.lang.init.contract.md) and query live Warp
evidence separately. Initialization is a configuration operation, not acceptance.
