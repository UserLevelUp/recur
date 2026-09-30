# recur-lang init v1

Status: proposed implementation contract; standalone tests intentionally red.
Warp: `main.lang.init`. Scope is configuration initialization, not plan/scaffold
generation, arbitrary execution, or a change to the Lang grammar.

## CLI and configuration ownership

`recur-lang init [-d ROOT] [--dry-run] [--json]` follows `recur-warp init` and
`recur-reveal init`. Without `--dry-run` it writes the reviewed additive defaults;
there is no extra confirmation flag or force/overwrite mode. Default ROOT is `.`.
`recur lang` stays pure; it gains no `init` writer alias.

Resolve the nearest ancestor `.recur/config.toml` from the canonical existing
directory ROOT. If none exists, use ROOT/.recur/config.toml. Report the resolved
project root and config path so an invocation from a child is transparent.
Relative configuration paths are based on that project root. Reject a config
file or `.recur` directory resolving outside that project, including a symlink
escape. Do not fall back past an invalid nearest config. No user/global config,
second TOML file, precedence layers, templates or generated code are introduced.

Use the existing TOML-editing and staged-publication patterns in
`src/recur_warp_init.rs` and `src/reveal_profiles.rs`; reuse the project config
discovery policy where its boundary matches this contract. Avoid a new generic
configuration framework. Explicitly reserve `recur-lang` as a policy section in
the core loader so a custom `dir` or `sep` key cannot accidentally create a lane.

Install only absent known keys:

```toml
[recur-lang]
schema_version = 1
target = "unspecified"

[recur-lang.planning]
specification_first = true
prioritize_graph_findings = true
include_test_plan = true
```

`unspecified` is an explicit language-neutral default; a later plan command must
resolve a real target from project policy or an explicit request. Nonblank target
names such as `julia`, `rust` and `csharp` are preserved, including custom targets.
Initialization does not claim that those backends exist. These planning settings
are stored preferences only; this Warp implements no planner or policy execution.

## Preservation and validation

- `schema_version` must be integer 1 (not boolean, float or a future version).
- `target` must be a nonblank string; all three planning flags must be booleans.
- Existing `recur-lang` and `planning` values must be tables, including inline
  TOML tables. Absent known keys are added even to empty tables. Explicit `false`
  is an opt-out and must survive. This uses Warp's missing-key convention;
  Reveal's empty association-table convention is not copied to scalar settings.
- Preserve unknown keys/subtables and all unrelated configuration semantically.
  Preserve user comments. An already initialized file is a byte-for-byte no-op.
  Unknown fields are inert; they are not commands and do not grant authority.
- Do not duplicate `[status]`, `[warp]`, `[reveal]` or lane defaults. Do not change
  fresh `recur init` output in v1; installation is opt-in through the companion.
- Read and validate the complete candidate before any publication. A malformed
  config, invalid known field, path error or missing/file ROOT leaves all bytes
  and directories unchanged. Never overwrite malformed user content.
- Stage in the destination directory, flush, recheck original bytes, then publish
  atomically. On detected concurrent modification, refuse without clobbering it.
  On failure, remove only this invocation's temporary files/empty directories.
  A Windows file-sharing conflict must preserve the original config.
- Dry-run creates nothing, including `.recur`, lock files and receipts. Existing
  configs and templates belonging to other companions remain untouched.

## Results

Success exits 0. JSON schema `recur-lang-init-v1` includes `project_root`,
`config_path`, `dry_run`, `changed`, `state`, `preview`, and `writes`.
`preview` is the complete proposed TOML string. `writes` is the single resolved
config path if changed, otherwise an empty array. State is `planned` on dry-run,
`written` after a change, or `unchanged` for a no-op write invocation. No timestamps
or random IDs: the same input produces the same dry-run packet.

Operational failures exit 2 and, with `--json`, use
`recur-lang-init-error-v1` with a nonempty `diagnostics` array of code/message:

| Code | Meaning |
| --- | --- |
| LINIT001 | ROOT missing, unreadable, or not a directory |
| LINIT002 | Config unreadable/not a file, malformed TOML, invalid known field or schema |
| LINIT003 | Config or `.recur` resolves outside the selected project boundary |
| LINIT004 | Detected concurrent change, staging or publication failure |

Malformed CLI arguments retain Clap's normal exit 2/help behavior. Human output
must state the resolved config path and whether it planned, wrote, or preserved
settings; it must not claim a planner was executed.

## Slices and acceptance

| Slice | Purpose | Required gates |
| --- | --- | --- |
| slice-0 | Freeze contract, map and executable tests; observe unsupported init | contract-and-red-baseline |
| slice-1 | Real preview/install command for fresh and existing projects | defaults-preview-and-nearest-root, preservation-and-repeatability |
| slice-2 | Fail safely across malformed inputs, escapes and publication failure | invalid-inputs-no-mutation, publication-and-policy-isolation |
| slice-final | Document, discover, package and integrate only green tests | focused-and-legacy-regressions, docs-and-installed-smoke |

Each slice depends on the preceding slice. Existing checked-transition and
runtime-evidence Warps are context, not prerequisites or inherited acceptance.
Existing failed/stale receipts must not be refreshed by this planning work.

The Julia CLI suite covers ordinary external behavior with temporary fixtures.
Cargo tests added with implementation must deterministically exercise changed
config detection and rollback, including the established Windows sharing-lock
pattern. Filesystem race acceptance cannot be inferred from sequential CLI tests.
Final gates require legacy core init, Warp init, Reveal init, Lang query and
checked-transition tests, plus the main Julia suite and appropriate Cargo tests.
Compiler/runtime faults are recorded as failed validation attempts, not passes.
Do not wire the standalone red suite into `julia-tests/runtests.jl` until green.

defines: recur.lang.init.v1 additive project-local opinionated policy initialization
consumes: recur.lang.query.v1 pure-query boundary remains intact
produces: recur.lang.init.configuration editable preferences without execution
