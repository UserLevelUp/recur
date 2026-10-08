# Engine worker report

Warp: `main.demo.blackjack-computer-player`  
Slice: `engine`  
Contract: `contract:demo.blackjack.rivals.engine:v1`  
Workspace: `C:/src/recur/.recur/blackjack-workers/engine`  
Observation date: 2026-10-06

Implemented the standalone `BlackjackRivals` module and extended the supplied test
suite. All 1,193 assertions pass on the installed Julia 1.12.7 runtime. The exact
assigned WindowsApps launcher could not start in this worker environment, so this
is supplemental runtime evidence, not a passing observation of the assigned
command. No acceptance gate, receipt, map, configuration, production source or
other worker workspace was changed. No commit or push was made.

## Recovered constraints

Read the workspace task, root rival contract/map/capsule, existing website engine,
copied Rules, and prepared table/play/Split Lang boundaries before implementation.
No applicable ancestor AGENTS.md was found. Used the recur-expert skill and read
the playbook's Warp/Lang guidance. Inspected CLI help before using the existing
read-only `warp show`, `warp slices`, and `lang check` operations. The live Warp
projection reported engine ready, all gates absent and no completed slices.
No CLI operation found in this guidance implements the required Julia gameplay.

The prepared Lang sources remain unchanged. Their read-only checks reported
`sound-within-coverage`, with bindings **not run**, no receipts validated, and
runtime/worker semantics excluded (LANG101). Those checks are not runtime proof
for this engine. Preserved the projection, copied transition, settlement and
human Split boundaries in the implementation.

## Changes and decisions

- `main.blackjack.rivals.engine.jl`: replaced the placeholder with `Table`,
  `newgame(money=500,rival_count=1)`, `public_state`, and protocol-3 `transition`.
  Includes only the copied `rules/` files and uses Random for production shuffling.
- Human fields and dictionary statistics retain the v2 shapes. Rival IDs are
  strings (`rival-1`, `rival-2`), separate from numeric human hand IDs. Rival hand
  IDs are local to their seat; all hand projections retain the human hand shape.
- Deal uses one validated/shuffled deck, in two passes over human, participating
  rivals in seat order, and dealer. Rivals run after every human hand finishes;
  Ada runs once afterward. Dealer natural peeks and settles immediately. Human
  natural triggers automatic completion without a client action.
- The policy receives only a public hand score and returns hit below 17 or stand
  otherwise. Both soft/hard 17 stand. The displayed policy is exactly
  `Stand on 17 benchmark`; rivals never split or double.
- Each wallet/escrow is independent. Deal reserves aggregate possible 3:2
  winnings; Split/Double reserve all remaining possible seat winnings, including
  rival naturals. Returned stakes remain separately covered by escrow. Public
  max_bet and allowed actions use aggregate reserves and no hidden information.
- Rivals wager the largest affordable even amount up to the human bet. A seat
  below two chips has empty hands, zero escrow/profit and `sitting_out` result;
  its played-round/hand statistics do not increment. Its prior hands are cleared.
- Split preserves rank-pair checks, one split, stable IDs, sequential human
  turns, one replacement for split aces, and 1:1 split-21 payouts. Every successful
  transition checks conserved/nonnegative ledgers and escrow, then advances the
  revision once. Failed commands and late draw errors cannot alter the original.
- Snapshots detach cards, statistics and history; hole card/score stay hidden
  during play. The last 20 human rounds are retained.
- Reset accepts integer count 0..2 only between rounds; omitted count preserves
  the current rival count. This module always projects v3, including an internal
  zero-seat Table. As required by the frozen contract, the integrator must route
  zero-rival website mode to the existing v2 engine to retain the exact solo
  schema. No v2 source was changed or duplicated into this worker.
- `main.blackjack.rivals.test.jl`: retained all initial supplied assertions and
  added deterministic naturals/peek, two-rival execution order, soft 17, Split,
  Double, wallet/reserve thresholds, sitting-out, malformed deck/command, stale
  revision, snapshot isolation, failure atomicity and history tests. All seven
  requested trace branches have producer/consumer comments.

## Verification and failures

Assigned command, attempted before and after implementation from this workspace:

```powershell
& 'C:/Users/marcn/AppData/Local/Microsoft/WindowsApps/julia.exe' --startup-file=no -O0 -C generic --project=C:/src/recur/demos/web-evidence-lab main.blackjack.rivals.test.jl
```

Both attempts failed **before Julia started**: `ResourceUnavailable`, “The file
cannot be accessed by the system.” The command wrapper returned 1; this is a
launch failure, not a Julia test-failure observation. The final error is retained
in `assigned-verification.log`. No permission escalation was requested.

The readable juliaup configuration identifies installed release 1.12.7 and its
runtime binary. Running that existing binary with otherwise identical arguments:

```powershell
& 'C:/Users/marcn/.julia/juliaup/julia-1.12.7+0.x64.w64.mingw32/bin/julia.exe' --startup-file=no -O0 -C generic --project=C:/src/recur/demos/web-evidence-lab main.blackjack.rivals.test.jl
```

Observed baseline: exit 1, `BlackjackRivals` undefined, matching the supplied
placeholder and historical `red.log` (preserved). After implementation, all 904
original assertions passed. Final expanded run: **exit 0; 1,193 pass, zero failures
or errors**, recorded in `runtime-verification.log`:

| Test set | Passed |
| --- | ---: |
| Supplied rival table contract, including 200 seeded rounds | 904 |
| Visible Stand on 17 benchmark | 11 |
| Natural blackjack and peek | 18 |
| Two rivals act in order before one dealer turn | 7 |
| Split aces/21, sequential hands and Double | 49 |
| Aggregate reserves and affordable wagers | 29 |
| Commands, revision, reset and snapshot isolation | 94 |
| Repeated rounds and history | 81 |

Supplemental static checks used `C:/src/recur/target/release-safe/recur.exe lang
check SOURCE -d ROOT` for `main.blackjack.web.01.table.recur` and
`main.blackjack.web.02.play.recur` under `C:/src/recur/demos/blackjack-web`, and
`main.blackjack.web.split.contract.recur` under its `split` directory. All
reported sound within coverage, with the limitations described above.

Final runtime input SHA-256 fingerprints:

| Input | SHA-256 |
| --- | --- |
| main.blackjack.rivals.engine.jl | 760476A5E8392FF8AD3B9E2D6A5F3891A1EB508B03BAD938489C17867CE63F3B |
| main.blackjack.rivals.test.jl | A9AF4BA1E37A25774D200DC38B706EC46F08926CE34413BF25B19191D3CFD0F0 |
| rules/main.blackjack.01.jl | 99794C1E25D2C2F0B347BEB695F5D50830F33A948BE10ADD00D1549FCB9A6C30 |
| rules/main.blackjack.02.jl | F981430BD9A70EF86509133DFA09AD0DE4D9EE63AB716BECA3B43480863C2FDA |
| rules/main.blackjack.03.jl | 766BE0CF1BFA8B4913B15C129337AC1AE5482F5618091A55DF04F08CEAE03C5D |
| rules/main.blackjack.04.jl | 8127B6716C45945995A3C3D095013347BA948111F39CCAB75E6766910683F4DF |

## Limitations and review questions

- The coordinator/reviewer must resolve the assigned launcher's accessibility and
  rerun the exact assigned command before treating that command as verified.
  Supplemental runtime success does not publish or accept `engine-tests`.
- HTTP isolation, v2/v3 mode routing, canonical include-path adaptation, browser
  assets/play, narrow viewport and original website regressions belong to later
  slices and were not run or modified here.
- Reserve-boundary and low-wallet tests redistribute existing funds while
  conserving the ledger to reach those states directly. Seeded random play and
  deterministic scenarios demonstrate tested behavior, not exhaustive proof.
- No unresolved engine-rule question remains under the frozen contract. The
  zero-mode routing and sitting-out statistics decisions above are explicit
  integration review points.
