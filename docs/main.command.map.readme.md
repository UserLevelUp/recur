# recur Command Map

Status: `readme` (permanent)
Date: 2026-03-08

recur's own command surface is hierarchically navigable using recur's own tools.
The `docs/main.command.*` file naming convention IS the hierarchy.

## Navigate the Command Surface with recur

```bash
# Full command tree (all docs for all commands)
recur tree "main.command" -d docs/ --sep .

# All command-level docs
recur children "main.command" -d docs/ --sep .

# Siblings of trace-id (other traverse commands)
recur related "main.command.traverse" -d docs/ --sep .

# All active command work
recur files "main.command.**.current" -d docs/

# Metrics: how wide is the command surface?
recur stats "main.command" -d docs/

# Find commands that support stdin
recur find "stdin" --scope "main.command.**" -d docs/

# Find commands in the traverse family
recur find "traverse" --scope "main.command.**" -d docs/

# Find pipeline relationships (what publishes JSON?)
recur find "publish" --scope "main.command.**" -d docs/

# trace-id: classify the command map's produce/consume/trigger roles
recur trace-id "recur.pipe.json" --scope "main.command.**" --json -d docs/
```

---

## Command Families

Commands group into four natural families. Each family has an index file
(`main.command.FAMILY.readme.md`) so the family level is navigable:

```bash
recur children "main.command.traverse" -d docs/ --sep .
recur children "main.command.discover" -d docs/ --sep .
recur children "main.command.compose" -d docs/ --sep .
recur children "main.command.config" -d docs/ --sep .
```

### main.command.traverse — follow identifier and call relationships

main.command.traverse.trace = single-hop: find callers + callees of a function
main.command.traverse.trace_id = identifier flow: classify define/produce/consume/trigger roles
main.command.traverse.trace_stats = bulk complexity: direct, transitive, circular, depth, risk
main.command.traverse.callers = upstream: who calls this function?
main.command.traverse.callees = downstream: what does this function call?

### main.command.discover — explore file sets and hierarchy structure

main.command.discover.files = file set: list files matching a hierarchical pattern
main.command.discover.find = content search: find pattern occurrences in scoped files
main.command.discover.tree = hierarchy display: show identifier tree across files
main.command.discover.children = scope narrowing: list direct children of a node
main.command.discover.related = peer discovery: list siblings of a node
main.command.discover.stats = hierarchy metrics: depth, width, node count per level
main.command.discover.id = identifier listing: show all unique identifiers in scope

### main.command.compose — combine and reshape hierarchy views

main.command.compose.merge = multi-separator unification: combine dot/underscore/hyphen views
main.command.compose.flatten = hierarchy collapse: squash nested structure to a flat list

### main.command.config — project setup and trait configuration

main.command.config.trait = trait config: list/get/set trait parameters in .recur/config.toml
main.command.config.init = project scaffold: initialize .recur/ directory with full config
main.command.config.reveal = lane rehydration: open one configured *.recur.md capsule

---

## Pipeline Relationships

Commands that produce `--json` output publish to the merge pipeline.
`recur.pipe.json` is the shared contract between JSON producers and consumers.

main.command.traverse.trace publish recur.pipe.json
main.command.traverse.trace_id publish recur.pipe.json
main.command.traverse.trace_stats publish recur.pipe.json
main.command.traverse.callers publish recur.pipe.json
main.command.traverse.callees publish recur.pipe.json
main.command.discover.files publish recur.pipe.json
main.command.discover.find publish recur.pipe.json
main.command.discover.stats publish recur.pipe.json
main.command.discover.id publish recur.pipe.json

main.command.compose.merge subscribe recur.pipe.json
main.command.compose.flatten subscribe recur.pipe.json

main.command.discover.files trigger recur.pipe.stdin
main.command.traverse.trace_id trigger recur.pipe.stdin

---

## Shared Flags

main.command.flag.scope.scope = --scope: hierarchical glob pattern to restrict search
main.command.flag.scope.sep = --sep: separator character (. _ -)
main.command.flag.scope.ext = --ext: file extension filter (.rs .cs .jl etc.)
main.command.flag.scope.stdin = --stdin: read file list from stdin pipe
main.command.flag.scope.dir = -d: working directory override

main.command.flag.output.json = --json: machine-readable JSON output
main.command.flag.output.format = --format: output style (summary, full, table, csv)

main.command.flag.traversal.depth = --depth: maximum traversal depth cap
main.command.flag.traversal.depth_guard = --depth-guard: guardrail policy (hard-fail, clamp, warn)
main.command.flag.traversal.force = --force: bypass depth cap entirely

main.command.flag.nav.sort_by = --sort-by: sort results (transitive, direct, circular, depth, risk)
main.command.flag.nav.filter = --filter: narrow output (circular-only, high-risk, medium-risk, low-risk)
main.command.flag.nav.top = --top: limit to top N results

---

## References

- `docs/main.command.traverse.readme.md` — traverse family index
- `docs/main.command.discover.readme.md` — discover family index
- `docs/main.command.compose.readme.md` — compose family index
- `docs/main.command.config.readme.md` — config family index
- `docs/main.command.*.readme.md` — per-command reference docs
- `src/main.rs` — Commands enum (source of truth)
