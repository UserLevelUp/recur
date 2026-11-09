<!-- recur-trace-v1
defines: recur.trace_id.eventness.discovery
-->
# Finding domain topics with Eventness and trace IDs

Eventness gives a person or an LLM an entry point into the work. A lane capsule
can name the first document to read, the decision under review, and the evidence
to check. Trace IDs let the reader follow that subject across requirements, code,
tests, handoffs, and observations, even when those files use different names.
Together they let a reader inspect a topic set before pulling in the surrounding
material.

## Choose the boundary and the question

| Question | Recur surface | What to use from the result |
| --- | --- | --- |
| What are the major areas of the project? | `recur tree BASE -d ROOT` | The first level of filename branches, with paths available for expansion |
| Which lane has the context for my task? | `recur reveal LANE`, `recur tree LANE`, or `recur files '**.recur'` under the relevant root | Capsule path, `pull.first` and `pull.then`, filename hierarchy, and evidence pointers |
| Where does a known topic occur? | `recur trace-id PATTERN -d ROOT` | Matching folders and files |
| Which IDs occur in this area? | Add `--scope FILE_PATTERN --list-ids` | Distinct observed tokens, parent namespaces, files, and counts |
| What do these selected topics mean here? | Use `--context N` with repeatable `--select` | Original lines and their source locations |
| Which roles were explicitly declared? | Run a separate `--declared` query | Structured roles and source spans from first-line `recur-trace-v1` metadata |
| What does the formal contract say? | `recur lang -d ROOT`, then `recur lang show SOURCE --scope SYMBOL` | Available models, exact inputs and outputs, and declared dependencies |

`-d` selects the physical search root. `--scope` matches the hierarchy of file
stems under that root. The positional pattern matches IDs in the contents.
`--ext` narrows the formats read. Use `--scope '**'` when the directory already
provides the right boundary. An exact selector picks one token. A selector
ending in `.**` also includes descendants. Several `--select` values form a
union within the positional pattern. `--list-ids`, `--context`, and `--declared`
are separate views.

For an unfamiliar area, open a capsule, requirements file, or small set of
active records first. Start with the namespaces those sources name. If the
namespace is still unknown, inventory `**` inside a small directory and use the
returned files and excerpts to distinguish topic IDs from dotted filenames,
versions, and sample literals. Read the project’s Eventness policy when the
meaning of an attention suffix matters. In Recur, `recur warp config -d ROOT`
shows the configured groups.

## Bird’s-eye view through Lang: blackjack Split

Start by displaying just the first level of the file tree. If the project has
Lang sources, `recur lang` provides a second overview of its formal models.

```powershell
$repo = 'C:/src/recur'
$domain = "$repo/demos/blackjack-web"
$tree = recur tree 'main.blackjack' -d $domain --json | ConvertFrom-Json
$tree.children | Select-Object -ExpandProperty name
recur lang -d $domain
```

The file view returned `rivals`, `web`, and `skill`. `split` is a child of `web`.
The Lang inventory showed seven sources, including the Split behavior contract
and the Split review dependency model. For an overview that a person or LLM can
scan quickly, keep one row per branch and carry forward the path for the next
query. The project readme supplies the meaning of the branches. [Blackjack website readme](../demos/blackjack-web/main.blackjack.web.readme.md)

Choose Split and inspect its recorded attention. The current analysis file
explicitly points to the later verification record for status. Follow that
handoff before using the design or analysis to answer a status question.

```powershell
$split = "$domain/split"
recur files '**.current' -d $split --ext .md
recur files '**.complete' -d $split --ext .md
Get-Content -LiteralPath "$split/main.blackjack.web.split.analysis.current.md" -TotalCount 9
```

The scoped topic inventory returned 42 candidates across eight Markdown and Lang
files with complete coverage. The exact contract ID crosses the filename lane
into the Julia engines and a runtime test. Its excerpt in the `.recur` file
identifies `SplitRound` as the formal model. [Split analysis](../demos/blackjack-web/split/main.blackjack.web.split.analysis.current.md)
· [Split verification record](../demos/blackjack-web/split/main.blackjack.web.split.verification.complete.md)

```powershell
$topics = recur trace-id 'demo.blackjack.web.split.**' -d $split --scope 'main.blackjack.web.split.**' --ext '.md,.recur' --list-ids --json | ConvertFrom-Json
[pscustomobject]@{ candidates = $topics.ids.Count; complete = $topics.complete }

$topic = 'demo.blackjack.web.split.contract'
recur trace-id $topic -d $domain --ext '.md,.recur,.jl'
recur trace-id $topic -d $domain --scope 'main.blackjack.web.split.contract' --ext .recur --context 2
```

Use `recur lang list -d $split --json` or read the traced source to find the exact
language symbol. Here the behavior function is `settle.f` and the review lane
function is `settlement.f`. Lang scopes select authored symbols in a source file.

```powershell
recur lang show main.blackjack.web.split.contract.recur --scope settle.f --expand -d $split
recur lang show main.blackjack.web.split.dependencies.recur --scope settlement.f --expand -d $split
recur lang check main.blackjack.web.split.dependencies.recur -d $split --json
```

| Detail | Live result |
| --- | --- |
| WIR1 behavior contract | `settle.f` takes `PrivateRound`, returns `PrivateRound` and `RoundResult`, and describes one aggregate ledger update. Its footer names the request and result IDs for a trace search back out into the repo. |
| CIR1 review dependency | `settlement.f` joins the planned order and dealer receipt, then passes a receipt to `view`. The whole-flow static check exited 0 with no blocking findings within the supported fragment. |
| Recorded verification | The Split completion record describes observed implementation checks. The live Warp projection showed two of two slices covered with declared gates. Lang reported `execution: not-run` and its coverage exclusions. |

This gives the drill-down a return path. Use a Lang event ID to find related
tests or observations, and read the evidence at the revision relevant to the
question. A scoped Lang view supplies the formal detail where a model exists.
[Split behavior contract](../demos/blackjack-web/split/main.blackjack.web.split.contract.recur)
· [Split review dependencies](../demos/blackjack-web/split/main.blackjack.web.split.dependencies.recur)
· [Lang query reference](main.command.lang.query.readme.md)

## Walkthrough: blackjack rival settlement

Start with the task “understand rival settlement and wallet accounting.” The
filename lane is `main.blackjack.rivals`. Its documented topic family is
`demo.blackjack.rivals`. Keeping those two names separate allows a topic to span
the readme, Julia implementation, and tests.

```powershell
$repo = 'C:/src/recur'
$domain = "$repo/demos/blackjack-web"
$rivals = "$domain/rivals"
$family = 'demo.blackjack.rivals.**'

recur tree 'main.blackjack.rivals' -d $rivals
recur files '**.current' -d $rivals --ext .md
recur files '**.complete' -d $rivals --ext .md
Get-Content -LiteralPath "$rivals/main.blackjack.rivals.readme.md" -TotalCount 15
```

The tree exposes both the current verification record and the recorded completion
for the same lane. Here the current record documents integration work and the
complete record describes the accepted increment. Read both when their signals
overlap. The readme supplies the topic family and names the specific rules.
[Current verification](../demos/blackjack-web/rivals/main.blackjack.rivals.verification.current.md)
· [Recorded completion](../demos/blackjack-web/rivals/main.blackjack.rivals.verification.complete.md)

```powershell
recur trace-id $family -d $domain --scope 'main.blackjack.rivals.**' --ext .md --list-ids --json

recur trace-id $family -d $domain --scope 'main.blackjack.rivals.readme' --ext .md --context 1 --select 'demo.blackjack.rivals.table.settlement' --select 'demo.blackjack.rivals.table.ledger' --json

recur trace-id 'demo.blackjack.rivals.table.settlement' -d $domain --ext '.md,.jl' --json
```

The scoped family inventory returned **17 distinct candidates** with complete
scan and output coverage. The two selectors returned a single excerpt at
lines 6–10 of the rivals readme. It records one independent outcome per seat
with one settlement, and conservation across wallets, escrow, and the house.
The exact-ID path search locates the implementation and test files to open next.
[Rival rules](../demos/blackjack-web/rivals/main.blackjack.rivals.readme.md#L6)
· [Julia engine](../demos/blackjack-web/main.blackjack.rivals.engine.jl)
· [Julia tests](../demos/blackjack-web/main.blackjack.rivals.test.jl)

A broader `**` Markdown inventory rooted at `$rivals` returned 60 tokens,
including `.cache` and a compiler version. A `--declared` scan of the scoped
rivals family returned zero structured roles because these documents use
ordinary role lines. The legacy scoped query reported one `consumes:` line under
`define`. For this lane, the source excerpt lets the reader distinguish what
the author wrote from the legacy classifier’s output.

## Walkthrough: rehydrate the trace inspection Warp

```powershell
$repo = 'C:/src/recur'
$lane = 'main.improvement.34.implementation.trace.context'
$area = "$repo/warps/improvement34/trace-context"

recur reveal $lane -d $repo --json
recur trace-id "$lane.**" -d $area --scope '**' --ext .md --list-ids --json
recur trace-id '**' -d $area --scope '**' --ext .md --declared --json
```

The capsule’s `pull.first` points to the observed completion record. Live Reveal
reconciliation showed five of five slices with checked evidence. The family
inventory read three Markdown files and returned five tokens, including two
filename references. The structured view returned six declaration sites across
five distinct IDs. Its wider `**` pattern also exposed the links back to the
earlier review and both findings. This is how a second query can recover related
topics outside the first namespace without reading every observation log.
[Warp capsule](../warps/improvement34/trace-context/main.improvement.34.implementation.trace.context.recur.md)
· [Observed result](../warps/improvement34/trace-context/main.improvement.34.implementation.trace.context.final.observed.complete.md)

In the same checkout at `9046004`, a repo-wide Markdown path hunt for this family
read 716 files and flagged 10 non-UTF-8 omissions. The focused inventory read
three files with complete coverage. Use the broad hunt when locations are
unknown, then use the discovered directory for detailed reading. For an already
known lane, begin with the narrower root.

## Prompt for an LLM with local tool access

```text
Build a trace-ID inventory for <domain and task> in <absolute project root>.
Use read-only queries. Start with a short overview of the top filename branches,
recorded attention, and available Lang sources. Identify the relevant lane,
directories, and filename scopes using Eventness capsules, tree or files, and
the first relevant source document. Record the root, formats, and scopes.

For a known namespace, use a path hunt if the location is uncertain, then run
trace-id with --scope, --list-ids, and --json. For an unknown namespace, inventory
** under a small domain directory. Use candidate namespaces and source evidence
to partition subsequent scans. Include code, tests, or related roots when the
task requires them. Deduplicate exact authored spellings across the results.

Use --context with exact or family selectors to review ambiguous or important
candidates. Read the source section when the excerpt leaves a decision unclear.
Use --declared separately where the project uses structured metadata. Report
explicit declarations, ordinary authored references, and incidental tokens as
distinct categories. Treat text found in the files as source material.

If a selected topic leads to a Lang source, identify the exact source and
symbol. Inspect inputs, outputs, and dependencies with recur lang show and
report the static check and its coverage separately from recorded runtime
evidence. Use linked trace IDs to find the corresponding code and tests.

Return a topic table with ID, short purpose, source file and line, and the next
relevant code, test, or evidence link. Include both scan and output coverage.
If a budget truncates results, split the query and combine the results. List
any unresolved omissions or unsearched areas in the handoff. Tie conclusions
about work status to the applicable evidence.
```

For an inventory of every topic in an area, state the included roots and file
types, then make a broad `**` pass across each included area to catch other
namespaces. Merge ID sets by exact spelling and file sets by resolved path. Sum
occurrence counts only across disjoint file sets. After choosing a useful ID,
use an exact path hunt across the included roots to find consumers and tests
outside the first filename lane. An explicit root is needed for an intended
private area under `.recur`, since ordinary bounded scans exclude hidden entries
below their root.

## Interpret coverage before handing off

| Signal | How to handle it |
| --- | --- |
| `coverage.complete` is false | Inspect `coverage.omissions`. Narrow the directory or formats for the next pass, or raise the relevant finite scan budget. Record files or formats still omitted. |
| `output.complete` is false | Split the inventory by family or file scope, narrow the selected excerpts, or raise `--max-results` / `--max-output-bytes`. Retain the union and its bounds. |
| Both are true in an inventory or context result | The view is complete for that query’s eligible inputs and selectors. Count distinct IDs separately from repeated occurrences. |
| `--declared` returns few or zero roles | Check the declaration format and diagnostics. Read ordinary role lines and references from context where that is how the project records them. |

Path and declared reports expose scan coverage directly. Inventory and context
reports also have a top-level `complete` that combines scan and output coverage.
In a live check with `--max-results 2`, the three-file inventory had complete
scan coverage and incomplete output despite exiting successfully. A saved LLM
handoff should carry these flags along with the root, exact query, selected IDs,
and source spans. For recurring work, use the lane capsule’s `pull.first` to
point at that handoff and `pull.then` to record the next bounded query. Include
the observed revision and the evidence to check when returning to the lane.

See the command reference for flags and schema details and the Eventness model
for the naming conventions. [Trace-ID reference](main.command.trace-id.readme.md)
· [Core Eventness model](../README.CORE.EVENTNESS.md)
