# Blackjack strategy, replay and statistics v1 (planned)

artifact.type = lane
defines: demo.blackjack.skill.contract bounded public-information practice
consumes: demo.blackjack.rivals.contract optional zero, one or two competing seats
consumes: main.command.warp.dispatch reviewed asynchronous slice assignments

This is the next implementation contract. No new gameplay is claimed by writing
this plan. The accepted computer-player Warp remains the baseline. Julia owns
rules and settlement; browser panels consume detached public snapshots. Lang is
optional for graph/design exploration, never a requirement for implementation.

Freeze the strategy/rules/replay/statistics API and fixture expectations first.
Then implement three disjoint worker slices in parallel; review their observations
before the integration coordinator accepts them. Codex coordinator uses high;
smaller bounded reviews can use Copilot. Enable other providers only after actual
local invocation and provider access are verified. Cross talk must retain Warp,
slice, attempt, host, requested reasoning, claim/finish time and review disposition.

## Rules and strategies

Keep solo play and the existing zero/one/two-rival selection. Retain the named,
versioned Stand on 17 benchmark. Add a distinct public-information strategy whose
decision input contains only visible hand state, dealer upcard, legal actions,
public rules and affordable stake. No table/deck/cursor/hole-card references.
It must return a legal action or an explicit contract error, with deterministic
tie-breaking and a versioned policy. Freeze the decision table and golden cases
before implementation; do not label it optimal without appropriate evidence.

Choose and declare action parity explicitly. The current rivals cannot split or
double while the human can; raw seat profit cannot be presented as a fair skill
rating. If parity is added, preserve house reserve, escrow, one split, split-ace,
natural, double and payout rules across seats. Otherwise retain capability labels
and restrict comparisons to matched eligible decisions. Shared-table outcomes
also depend on seat order, stake, bankroll and card consumption. A future matched
scenario practice mode must specify its separate deck protocol before claiming
fair counterfactual comparisons. Natural termination and deck exhaustion remain
bounded; failed actions mutate no seat; all chips including escrow are conserved.

## Deterministic private replay

Version a private replay envelope containing engine/rules/strategy identities,
initial funds and seats, a validated complete deck order, accepted commands and
revisions, and expected state hashes. A seed alone is insufficient to survive RNG
or implementation changes: record the realized deck order as the replay authority.
Replaying accepted commands from initial state must reproduce canonical states,
settlement, stats and profit. Reject malformed/duplicate cards, unsupported versions,
illegal commands, stale revisions and tampered expectations without mutation.

Live public snapshots and public exports must hide hole cards, deck and cursor
throughout play. Private replay records stay server-owned; release completed-round
replay deliberately, never through the live state endpoint. Session isolation and
HTTP replay rejection remain mandatory. Playback must not alter a live game.

## Meaningful session statistics

Track separately started/settled rounds, active/sitting-out seats, settled hands,
wins/losses/pushes, naturals/busts, resolved wager and net chips. Define split-hand
versus round denominators. ROI is net chips / resolved wager; zero denominator is
null with a clear label. Show sample size and strategy/rules version. Wallet changes
and aggregate stats must reconcile to recorded settlements. Do not turn small-sample
profit into a skill rating or compare unequal actions/stakes without a warning label.

Default sessions remain in memory. Reset creates a new session; an explicitly
exported completed-session report can be retained independently. Do not imply
durable storage until a separate persistence contract and isolation tests exist.
Export only documented public completed-session fields; include provenance and
denominators, and exclude private deck/hole data while play remains active.

## Slice gates and optional demo tests

1. contracts: freeze schema, strategy action parity and golden fixtures; review.
2. tests: execute failing tests for the new contracts against current baseline.
3. strategy, replay, statistics: disjoint workers implement the frozen interfaces
   with scoped tests; can run in parallel after contracts/tests gates are accepted.
4. integration: Julia HTTP and browser adapters, independent sessions, original
   solo/split/rival regressions, stale requests and privacy; then browser verification.
5. final: coordinator reviews actual current evidence and accepts the integrated
   result. Produced worker observations alone never accept gates.

Test matrix includes hard/soft totals and naturals, two rivals with differing
policies, insufficient funds and sit-outs, split/double parity if selected, chip
conservation, deck exhaustion, deterministic replay equality, bad versions/decks,
hidden-information sentinel checks, reset/session isolation, statistics denominators,
export privacy, zero/one/two-rival browser play and a narrow viewport.

All demo test additions route through the existing blackjack-web selection:
`julia julia-tests/runtests.jl --demo blackjack-web`. Default runner selects core
only. Select blackjack-lab separately when its rules are changed; add --with-core
only for a change that also affects core. Do not run unrelated demo suites.

Assign actual commands/inputs only after worker scaffolds and failing fixtures
exist. Until then this Warp is planned, with no claims of enabled dispatch or
accepted implementation gates. Use root-owned integration and reviewed handoffs.
