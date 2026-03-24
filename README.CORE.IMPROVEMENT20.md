RECUR IMPROVEMENT 20
Pipe-Friendly Filter Command
============================
Date: March 24, 2026
Status: Proposal / future direction
Author: Proposed from real recur usage while composing `files`, `tree`, and `merge`


INTENT
------
This document proposes a first-class `recur filter` command.

The goal is not to replace shell pipes.
The goal is to give recur a native filtering stage that works cleanly
inside pipelines, especially when the input is already structured as
recur paths or recur JSON.

Today recur is strong at:
- discovering paths
- shaping hierarchy
- merging across separators
- rendering trees

What is missing is a native middle stage for:
- excluding noisy folders
- excluding generated artifacts
- keeping only one separator domain
- narrowing a path stream before `merge`
- narrowing a tree/file JSON stream without dropping to ad hoc shell logic


SUMMARY
-------
Improvement 20 proposes:
- a new `recur filter` command
- stdin-first behavior so it fits naturally in pipelines
- support for both newline path input and recur JSON input
- path-based include/exclude rules
- optional separator/provenance filtering
- output that preserves pipeline shape whenever possible

In short:

```bash
recur files ... --json | recur filter ... | recur merge --stdin ...
```

Instead of:
- `ConvertFrom-Json`
- `Where-Object`
- `ConvertTo-Json`
- repeated shell-specific glue


THE PROBLEM
-----------
Today the user can compose recur successfully, but the filter stage lives
outside recur.

Typical current pattern:

```powershell
@(
  (
    recur files "main.command.**" -d docs --sep . --json |
    ConvertFrom-Json |
    Where-Object { $_ -notmatch '^docs\\main.command.merge.phase' } |
    ConvertTo-Json -Compress
  )
  (
    recur files "main_command_**" -d src --sep _ --json |
    ConvertFrom-Json |
    Where-Object { $_ -notmatch 'tmp|test_quick|target2|\.tmp' } |
    ConvertTo-Json -Compress
  )
) -join "`n" |
recur merge --stdin --base main --sep . --sep _ --show-sep
```

That works.
It is also awkward.

Problems with the current state:

1. Filtering logic becomes shell-specific.
   PowerShell and Unix pipelines diverge immediately.

2. Structured recur output must be unpacked and repacked manually.
   This adds ceremony and makes scripts noisier.

3. Provenance-aware filtering is hard.
   If the user wants "keep only `_` lane results" or "drop `[.]` docs lane"
   after merge-oriented discovery, the shell has no native understanding of
   separator provenance.

4. There is no recur-native place to express path exclusions.
   The user can scope with `-d`, but cannot say:
   - exclude `.tmp`
   - exclude `test_quick`
   - exclude `target2`
   - exclude `*.todo.current.md`

5. It weakens composability.
   Recur already has `files`, `tree`, `merge`, `flatten`, and stdin support.
   A native filter stage would complete that pipeline model.


DESIGN GOAL
-----------
`recur filter` should be:

1. Stdin-first
2. Pipe-friendly
3. Format-aware
4. Path-oriented first, hierarchy-aware second
5. Simple enough to be useful immediately

This is not a query language proposal.
This is a pragmatic filtering primitive.


CORE IDEA
---------
Add a command:

```bash
recur filter
```

It reads one of the following from stdin:
- newline-delimited paths
- JSON array of paths
- JSON object with a `files` field
- recur tree JSON with `path` fields
- merge/file-mode JSON streams that ultimately contain paths

It emits:
- newline-delimited paths by default for text/path input
- JSON by default when the input is recur JSON
- optionally forced `--json` or `--paths`


MINIMAL CLI PROPOSAL
--------------------
```text
recur filter [OPTIONS]

OPTIONS:
  --include-path <GLOB>       Keep only matching paths (repeatable)
  --exclude-path <GLOB>       Drop matching paths (repeatable)
  --include-name <GLOB>       Keep only matching filenames (repeatable)
  --exclude-name <GLOB>       Drop matching filenames (repeatable)
  --include-sep <CHAR>        Keep only paths/provenance from a separator domain
  --exclude-sep <CHAR>        Drop paths/provenance from a separator domain
  --include-ext <EXTS>        Keep only matching extensions
  --exclude-ext <EXTS>        Drop matching extensions
  --json                      Force JSON output
  --paths                     Force plain newline path output
  --stdin                     Explicit stdin mode (optional if defaulted)
```

The minimal MVP does not need all of these.
But this is the intended direction.


MVP
---
The smallest useful version is:

```text
recur filter \
  --exclude-path <GLOB> \
  --include-path <GLOB> \
  --json \
  --paths
```

That alone unlocks most real usage:
- remove `.tmp/**`
- remove `target/**`
- remove `test_quick/**`
- keep only `src/**`
- keep only `docs/**`

Suggested MVP behavior:

1. Read stdin.
2. Detect whether stdin is JSON or newline path text.
3. Extract paths.
4. Apply include/exclude filters.
5. Re-emit in the same format unless overridden.


WHY THIS SHOULD BE A COMMAND, NOT JUST A FLAG
---------------------------------------------
This should be a separate command because it is a pipeline stage.

Examples:

```bash
recur files "main.command.**" --sep . --json | recur filter --exclude-path ".tmp/**"
```

```bash
recur tree main --sep . --json | recur filter --exclude-name "*.todo.*"
```

```bash
recur flatten config.json --json | recur filter --include-path "config.db.**"
```

That is different from adding `--exclude-*` to every command.

A separate command:
- keeps the model composable
- reduces feature duplication across commands
- gives users a standard mid-pipeline narrowing step
- fits Unix philosophy better than sprinkling flags everywhere


RELATIONSHIP TO EXISTING COMMANDS
---------------------------------
`recur files`
- discovers candidate paths
- `recur filter` narrows them

`recur tree`
- renders hierarchy directly
- `recur filter` can preprocess the path set before rendering

`recur merge`
- rebuilds a unified hierarchy from multiple path streams
- `recur filter` sits naturally before merge, and maybe after merge JSON too

`recur flatten`
- produces path-bearing JSON records
- `recur filter` can keep only certain subtrees before downstream use

`recur trait`
- not directly related in MVP
- future filter defaults could eventually live in config, but not required now


EXAMPLES
--------
1. Exclude generated and temp folders before merge

```powershell
@(
  (
    recur files "main.command.**" -d . --sep . --json |
    recur filter --exclude-path ".tmp/**" --exclude-path "target2/**" --exclude-path "test_quick/**" --json
  )
  (
    recur files "main_command_**" -d . --sep _ --json |
    recur filter --exclude-path ".tmp/**" --exclude-path "target2/**" --exclude-path "tmp_multi_sep/**" --json
  )
) -join "`n" |
recur merge --stdin --base main --sep . --sep _ --show-sep
```

2. Keep only source lane paths

```bash
recur files "main_command_**" -d . --sep _ --json | \
recur filter --include-path "src/**"
```

3. Drop eventness/task files from docs lane

```bash
recur files "main.command.**" -d docs --sep . --json | \
recur filter \
  --exclude-name "*.todo.md" \
  --exclude-name "*.current.md" \
  --exclude-name "*.reference.md"
```

4. Keep only underscore provenance before merge output

```bash
recur merge --stdin --base main --sep . --sep _ --show-sep --json | \
recur filter --include-sep "_"
```

This is especially useful if the merge stream contains both docs and src
but the user wants to narrow to one domain without regenerating inputs.


INPUT/OUTPUT SHAPE
------------------
This part matters a lot.

`recur filter` should not force one format if it can preserve the existing one.

Recommended behavior:

1. Path text in -> path text out
2. JSON path array in -> JSON path array out
3. Tree JSON in -> tree JSON out if possible, otherwise extracted-path JSON out
4. Merge stdin JSON stream in -> merge-compatible JSON stream out if possible

Important practical note:
an MVP can legitimately choose a simpler contract:

- accept newline paths and JSON arrays of paths
- always emit newline paths by default
- emit JSON arrays when `--json` is specified

That would still be useful.


SEPARATOR / PROVENANCE FILTERING
--------------------------------
This is where recur can do something shells cannot do elegantly.

If the input contains marker/provenance data such as:
- `[.]`
- `[_]`
- normalized paths with remembered original separator

Then `recur filter` should support:

```bash
recur filter --include-sep "_"
recur filter --exclude-sep "."
```

This is the recur-native answer to:
- show only source lane
- show only docs lane
- drop one separator domain from a merged set

This likely requires a stable internal representation for separator provenance
instead of relying only on the rendered filename marker text.


PATH MATCHING RULES
-------------------
Suggested rules:

1. Match against normalized relative paths when possible.
   Example:
   `src/main_command_tree_impl.rs`

2. Use simple glob semantics first.
   Examples:
   - `src/**`
   - `**/*.md`
   - `.tmp/**`
   - `test_quick/**`

3. Keep include/exclude evaluation predictable.

Recommended order:
- start with all paths
- apply all include rules, if any
- apply all exclude rules

This is the least surprising behavior.


NON-GOALS
---------
Improvement 20 is not trying to:

1. Replace shell filtering entirely.
2. Add a full SQL-like query language.
3. Force every recur command to grow its own `--exclude-*` options.
4. Solve every structured-data filtering problem in the first version.
5. Make `merge` or `tree` responsible for all filtering concerns.


IMPLEMENTATION SHAPE
--------------------
A practical implementation path:

Phase 1:
- `recur filter`
- stdin only
- supports newline paths and JSON arrays of paths
- supports `--include-path` and `--exclude-path`
- outputs newline paths by default
- optional `--json`

Phase 2:
- support `--include-name` / `--exclude-name`
- support extension filtering
- preserve recur JSON path arrays more naturally

Phase 3:
- support merge/tree JSON extraction and re-emission
- support separator provenance filtering (`--include-sep`, `--exclude-sep`)

Phase 4:
- optional config defaults via `.recur/config.toml`
- optional reusable named filters


WHY THIS IS WORTH DOING
-----------------------
This is a small feature with disproportionate value.

Why:

1. It fills an obvious hole in recur's pipeline story.
2. It makes recur less dependent on shell-specific glue.
3. It strengthens `merge`, which is already one of recur's most interesting commands.
4. It helps keep repo-root usage practical even in noisy repos.
5. It improves cross-platform composability for both PowerShell and Unix users.


OPEN QUESTIONS
--------------
1. Should `recur filter` preserve input shape exactly, or normalize to paths first?
2. Should separator provenance filtering be MVP, or phase 3?
3. Should the command accept only stdin, or also `<FILE>` JSON input files?
4. Should tree JSON be filtered structurally or flattened to paths then rebuilt later?
5. Should there eventually be a companion `recur unmerge`, or is `filter + merge`
   already enough for most of that story?


RECOMMENDED FIRST MILESTONE
---------------------------
Implement:

```bash
recur filter --exclude-path ".tmp/**" --exclude-path "target2/**" --json
```

with support for:
- stdin JSON path arrays
- stdin newline paths
- plain path output by default
- JSON path-array output with `--json`

That alone would immediately improve real workflows in this repo.


ONE SENTENCE VERSION
--------------------
Improvement 20 adds the missing middle stage in recur's pipeline model:

discover -> filter -> merge/render
