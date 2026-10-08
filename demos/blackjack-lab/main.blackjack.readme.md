# Hello World to configurable blackjack, Lang first

Six numbered Lang contracts grow from a greeting to a multi-round game and a
coordinated settlement. Each was authored before its Julia implementation.
The demo is a bounded teaching variant with simulated chips; its
[requirements](main.blackjack.requirements.md) define exact rules and exclusions.

## Run

From the repository root, using a verified Julia 1.12 executable:

```powershell
julia --startup-file=no -O0 -C generic demos/blackjack-lab/main.blackjack.app.jl --players 4 --money 100 --bet 10 --rounds 8 --dealer Ada --bank 1000 --seed 7
julia --startup-file=no -O0 -C generic --project=demos/web-evidence-lab demos/blackjack-lab/main.blackjack.test.jl
recur tree main.blackjack -d demos/blackjack-lab
recur lang -d demos/blackjack-lab
recur lang show main.blackjack.04.round.recur --scope settle.s --expand -d demos/blackjack-lab
recur lang show main.blackjack.06.coordination.recur --scope settle.s --expand -d demos/blackjack-lab
recur-lang plan main.blackjack.05.session.recur -d demos/blackjack-lab --json
recur trace-id demo.blackjack.session --scope 'main.blackjack.**' -d demos/blackjack-lab --format full
```

On the development host use Juliaup's executable at
`C:/Users/marcn/AppData/Local/Microsoft/WindowsApps/julia.exe`; the older Julia
shim has previously faulted. Tests use the web-evidence-lab JSON3 environment.
The app itself uses only Julia standard libraries. Set `RECUR_BIN` and
`RECUR_LANG_BIN` to matching current binaries, including a companion with `plan`.
For a partial runtime iteration set `BLACKJACK_STAGE` to 1–5; remove that variable
for the complete six-stage runtime plus CLI/evidence tests. The full suite also
runs from `julia-tests/runtests.jl`. No Python is used.

The demo's assertion harness disables specialization/inference and uses minimal
compilation to avoid a reproduced Julia 1.12 Windows compiler fault in large
test closures. The game module retains ordinary compilation. Failed compiler
attempts are preserved separately; they are not successful validation.

The app prints per-player outcomes, payout deltas, balances, dealer results and
bank, then all tied final leaders and the stopping reason. There may be multiple
winning players or none in a round. Final leaders are determined by ending money;
the dealer is a separately accounted house, not an entrant in that ranking.

## Feature matrix

| Stage / probe | Exercised behavior | What remains outside it |
| --- | --- | --- |
| 01 hello | Optional name bundle, function symbol, binding, sync flow, event and E0/dE/Ef | Lang does not enforce the prose default |
| 02 table | Table object, calling function, seven-parameter bundle, input alias/share | Opaque types do not constrain 1–6 players or even wagers |
| 03 cards | Separate deck/score contracts; all 1,326 two-card scores tested | Domain rules are Julia assertions |
| 04 round | Large result bundle passed as one symbol, per-player payout and conservation | No split, double, insurance, surrender or interactive decisions |
| 05 session | Seeded repeated rounds, eliminated players, house reserve, final ties and CLI | Sequential feedback is a controller loop, not a module cycle |
| 06 coordination | Named contracts, DispatchSet projections, lane policies, fork, join, ordered awaits and audit | Variable players are a batch; no dynamic per-player Lang fan-out |
| 07 query | list/show/report/check, expansion, scope boundaries, recorded Eventness, trace IDs | A recorded complete file is not acceptance |
| 07 faults | Self/dependency cycles, wait cycle, missing join, ambiguous symbol, unsupported version | A hidden Julia cycle still passes static Lang checking |
| 07 exclusions | Added unknown syntax remains outside declared coverage | Static success is never whole-document validation |
| 08 init | Preview, installation and byte-identical repeat in a temporary project | Initialization stores preferences; it does not run a planner |
| 08 warp | Legacy preview; real observed-case checked receipt, preview/confirm, accepted recovery replay, stale-input rejection | Interrupted publication faults remain in native Cargo tests; legacy confirmation remains in its existing tests |
| Companion plan | Both IRs, graph blockers under scope filtering, policy choices/opt-outs and input hashes | Advice only; no generated implementation, runner or claimed acceptance |
| 09 reference review | 19 helper contracts, reviewed call registry, source freshness, mapping an implementation-only call into a detectable cycle | Author self-review of default bindings; custom callbacks remain unknown |

The CIR body passes three messages to settlement without repeating each hand,
player identifier or balance. Player and dealer scoring run independently only
after hands are fully played; sharing the live deck across those producers would
make dealing order nondeterministic. The Julia coordinator joins both tasks and
checks complete producer identities and scores before allocation and audit.
Each projected order also carries round context, so settlement's joined plan
retains the configuration and money ledger it needs. Five explicit tests compare
structured Julia result fields against the authored WIR output contracts. This
caught three mismatches while building the demo. Scalar greeting/render results
and deck vectors use conventional single-field packing; no automatic binding
dispatcher or general structural/type checker is claimed.

Tests run all destructive/fault experiments in temporary roots. The checked
receipt is adapted from two actually evaluated session cases, with the source,
implementation, tests, configuration, runner and behavior files named explicitly.
It demonstrates the workflow; it does not certify every language feature or
authenticate arbitrary producers. Existing Cargo/Julia boundary tests remain
part of the overall evidence.

See [observations](main.blackjack.observations.md) for actual runs and
[follow-up work](main.blackjack.improvements.todo.md) for grounded additions.

For the reference-first review workflow, read
[dependency review](main.blackjack.dependencies.review.todo.current.md) and its
linked Lang model. Use it as an LLM/human comparison standard for the Julia code.
The initial observation is intentionally not a blanket completion marker.
