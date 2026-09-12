# Recur Lang query contract v1

Target: 0.2.8. Core queries are pure. All opinionated behavior belongs to
`recur-lang`, including read-only recommendations. Recur Lang is the tightest
formalism inside a chosen boundary; it imposes no schema on surrounding Recur.

## Commands and read boundary

```text
recur lang list [-d ROOT] [--json]
recur lang show SOURCE --scope SYMBOL [--expand] [-d ROOT] [--json]
recur lang report SOURCE [--scope SYMBOL] [--eventness SUFFIX] [--expand] [-d ROOT] [--json]
recur lang check SOURCE [--scope SYMBOL] [-d ROOT] [--json]
```

`-d` defaults to the current directory. Relative SOURCE resolves there; absolute
SOURCE must remain inside its canonical root. Symlink escapes fail. Discovery
and recorded-file inventories never follow symlinks and omit `.git`, `target`,
`target2`, `build`, `dist`, `node_modules`, `.venv`. Explicit sources within those
directories remain readable. The root's own `.recur/config.toml` supplies status
suffixes; parent/sibling configurations are not consulted. Discovery includes
invalid/unsupported `.recur` files as diagnostic entries, sorted by relative path.
An empty root produces an empty `sources` array. Queries do not create files,
receipts, config, processes or background workers. Bindings are descriptive data.

Language scopes are exact authored identities, independent of the global filename
separator option. WIR1 accepts a scope (`gcd`), function (`gcd.f`), or unique local
letter. CIR1 accepts a lane, lane function or flow; multiple flows require an exact
flow selection. An ambiguous local letter fails, never chooses a preferred scope.

`list`, `show` and `report` return 0 for a produced result even when it contains
static findings. `check` returns 0 for soundness within the stated fragment and 1
for static findings. Input errors return 2; invalid CLI arguments use Clap's exit
2/help output. None of these outcomes certifies runtime or whole-document success.

## Versioned results and diagnostics

| Schema | Meaning |
| --- | --- |
| `recur-lang-list-v1` | `sources` with relative identities, status, supported symbols or diagnostics; discovery coverage |
| `recur-lang-query-v1` | `source`, FNV1a `source_hash`, `ir_schema`, selection, `coverage`, `header`, `contracts`, `body`, `footer` |
| `recur-lang-error-v1` | `diagnostics` with stable code, message and original IR diagnostic detail |
| `recur-lang-static-graph-report-v1` | SGR1's exact nodes, message edges, ordered waits, entries/reachability, findings, spans and soundness |

Header holds familiar meaning, local function identity, exact input/output
references, span and available binding. Contracts declare complete field bundles
once by canonical identity. WIR1 aliases retain both local and canonical identity;
equal field shape never creates an alias. CIR1 compact inputs retain each message
identity, projection and contract; `--expand` adds each sub-input's producer and
fields. Expansion changes detail, not edge identities or validation.

Body includes flows, selected edges and boundary edges. Footer carries requested
events/transitions or receipts, recorded files, static findings and `execution:
not-run`. CIR1 embeds the unfiltered SGR1 report so outside cycle paths and missing
joins cannot disappear. Text presents the same selected relationships/findings
with compact references; JSON includes full structural data and spans.

| Code | Meaning |
| --- | --- |
| LANG001 | Missing/unreadable/non-UTF-8 source or invalid root |
| LANG002 | Source or config escapes the read root |
| LANG003 | Unknown language scope/symbol |
| LANG004 | Ambiguous symbol or repeated/multiple unselected scope/flow |
| LANG005 | Unsupported language version/declaration kind |
| LANG006 | Malformed supported input (original RLIR/RCIR diagnostic retained) |
| LANG007 | Unknown Eventness suffix |
| LANG008 | Invalid root configuration |
| LANG009 | Model/source/version/schema mismatch |
| LANG101 | Capability notice: excluded content remains unvalidated |
| SGR001 / SGR002 | Dependency cycle / wait cycle, with deterministic closed path |
| SGR003 | Lane unreachable from fork/await activation |
| SGR004 | Unsatisfied join, inconsistent fork/await ports or unsupported mixed await |
| SGR005 | Unknown/duplicate node, invalid typed dependency or unsupported IR schema |

## Supported fragments and Eventness

WIR1 accepts `recur 0.1 class`: existing compact algorithm scopes, exact contracts
and aliases, one function/binding, matching flow, event block and Warp declaration.
CIR1 accepts `recur 0.2 coordination`: named contracts, coordinator output ports,
lane inputs/outputs/policies, projected orders, joins, forks and ordered awaits.
Both reuse their existing parsers. Query defers CIR1 topology validation to SGR1
so syntactically valid graph failures can be reported together and exit 1.

Always inspect `coverage.whole_source_validated` (false) and `coverage.excluded`.
Unknown statements, arbitrary expansions/worker semantics, whole-document grammar,
runtime/watch/grid/feedback/import syntax and coordinator input signatures are
outside this baseline. WIR1 and CIR1 are not a unified AST. Newer versions fail
visibly; a supported fragment cannot confer acceptance on excluded source text.

WIR1's event and E0/Ef identities are declarations. The inventory matches their
exact file stems within the read root and exposes only recorded evidence, never
acceptance. Default configured suffixes are `todo.current`, `todo`, `complete`;
`[status] current_suffix`, `todo_suffix`, `complete_suffix` override them. Filters
accept those exact suffixes (leading dots in config are normalized). A desired
`complete` transition with no matching file does not pass a `complete` filter.
CIR1 has no lifecycle model: state filtering yields no lane detail and explicitly
retains this limitation plus the full static analysis. Receipt contents are not
read/accepted by these commands.

## Additive checked evidence query

`recur lang evidence SOURCE --scope FUNCTION --contract POLICY [--receipt ATTEMPT]
[--status STATUS] [--expand] -d ROOT --json` returns the separately versioned
`recur-lang-evidence-report-v1`. FUNCTION is qualified WIR1, with an explicit
bounded policy and one attempt. This query assesses external results and actual
input fingerprints without execution or recursive inventory. Its embedded
query-v1 packet retains original contracts and fragment coverage; historical
transition status is separate from current evidence. Original list/show/report/
check behavior above is unchanged. Read the
[checked transition contract](main.command.lang.checked-transition.readme.md)
for path/byte limits, CE001–CE007 outcomes and the companion's opt-in writes.

## Deferred work

No arbitrary worker execution, policy advice, repair, scheduling, runtime lane
Eventness, unified IR, interactive grid, new grammar or import resolution is added.
The existing `recur-lang warp` confirmed receipt contract remains intact.

defines: recur.lang.query.v1 source-bound pure header body footer and static checks
consumes: recur.lang.warp.ir.v1 exact compact algorithms and declared transitions
consumes: recur.lang.concurrent.ir.v1 exact concurrent messages and policies
consumes: recur.lang.static.graph.report.v1 deterministic static findings
