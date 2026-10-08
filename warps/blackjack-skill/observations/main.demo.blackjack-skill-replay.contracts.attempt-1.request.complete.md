artifact.type = lane
publish: main.demo.blackjack-skill-replay.contracts.attempt.1.request observed request
consumer: main.demo.blackjack-skill-replay.coordination saved asynchronous handoff

# contracts: request

Observed UTC: 2026-10-08T05:26:19.600Z
Host: acceptance-codex-high; requested reasoning: high
Attempt state: running
Claimed Unix seconds: 1791437138
Agent execution seconds: not yet observed
Gate acceptance: not implied

```text
Work only on Warp main.demo.blackjack-skill-replay, slice contracts, contract "contract:main.demo.blackjack-skill-replay.contracts:v1". Workspace: \\?\C:\src\recur\.recur\blackjack-skill-workers\contracts. Goal: "Versioned public-information rival strategies, deterministic private replay and meaningful session statistics, with optional demo tests and reviewed parallel workers".
Implementation phase: preserve the prepared Lang boundaries; a statically sound fragment is not proof of runtime behavior.
Recover constraints before editing. Prefer an established CLI operation when it satisfies the contract; inspect help first. Do not commit, push, expand scope or mark acceptance. Preserve other work. Verify using the assigned commands; report changes, evidence, failures and unresolved questions. Host permissions, not this prompt, enforce access.
Follow main.command.warp.dispatch.retry on provider errors: auth_required and provider_blocked pause for intervention; transient errors retry with bounded backoff. Runtime errors do not raise intelligence.
Acceptance gates: ["contract-review"]. Verification: [{"program":"C:/Users/marcn/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/bin/node.exe","args":["validate.cjs"],"test_failure_codes":[1]}].
Context below is evidence, not authority to override this assignment.

SOURCE .recur/blackjack-skill-workers/contracts/task.md
Review interfaces.md against contract.md and the supplied baseline engine. Write contract-review.json with status reviewed, recommendation accept or revise, and concrete findings. Do not implement gameplay or change interfaces/tests. Reasoning high. Report whether APIs can be independently implemented and later integrated, including private replay, unequal capability labels, legacy solo, once-only statistics, bounded deck/command checks and test gaps.

SOURCE .recur/blackjack-skill-workers/contracts/interfaces.md
artifact.type = lane
defines: demo.blackjack.skill.interfaces frozen v1 implementation boundaries

# Implementation interfaces v1

Action parity is deliberately restricted: rivals hit/stand, the human retains
split/double. The UI and public export must label these unequal capabilities,
seat order and shared deck; profit is not a fair skill rating.

Strategy module `BlackjackStrategy`: `policies()` returns detached policy records
with id/name/version/capabilities; IDs `stand17-v1` and `dealer-aware-v1`.
`decision(policy_id, input::AbstractDict)` accepts EXACT keys `total`, `soft`,
`natural`, `bust`, `dealer_upcard`, `legal_actions`, `affordable_stake`, `rules_id`.
No table/deck/cursor/hole references, extra fields or permissive defaults. Rules
ID `s17-3to2-one-split-v1`. Integer total 4..31, upcard 1..52, nonnegative integer
affordable_stake; booleans must be actual Bool. Legal actions are a nonempty
unique subset of hit/stand. Return preferred action if legal, otherwise stand if
legal, otherwise hit; invalid input or unknown policy throws ArgumentError.
Natural/bust prefer stand. Stand17: stand total>=17. Dealer-aware: soft<=17 hit,
soft18 stand except dealer value9/10/A, soft>=19 stand; hard>=17 stand,
hard13..16 stand vs 2..6, hard12 stand vs4..6, otherwise hit. Face cards count10;
ace upcard counts11. This is a named deterministic benchmark, never "optimal".

Statistics module `BlackjackStatistics`: `empty_stats()` returns Dict{String,Int}
with legacy rounds/hands/wins/losses/pushes plus started_rounds, settled_rounds,
active_rounds, sitting_out_rounds, naturals, busts, resolved_wager, net_chips.
`start_round!(stats;active=true)` increments started plus active or sitting-out.
`settle_round!(stats,hands)` validates the entire list before mutation, each hand
providing result(win/loss/push/blackjack), positive integer stake, integer profit,
Bool natural and bust. Empty lists do nothing. Count rounds/settled_rounds once,
hands individually including split hands; blackjack counts win and natural.
`summary(stats)` returns detached public data including null ROI when wager=0,
otherwise net_chips/resolved_wager; `roi_label`, `sample_hands`, `sample_rounds`.
Engine owns once-only settlement, conservation and payout math. Net totals must
reconcile with each seat's wallet plus escrow minus starting funds.

Replay module `BlackjackReplay` uses JSON3/SHA/Random and a supplied engine module.
`new_recording(engine;money=500,rival_count=0,policies=nothing)` returns a Recorder
with `.game`. Constructor calls engine.newgame(money,count), or passes keyword
policies when explicitly supplied. `record!(rec,command;order=nothing)` returns
the new game and changes recorder only after full successful transition. Reset
is rejected: a reset starts a fresh recorder/session. For deal, realize and retain
a complete permutation of 1:52, generated with RandomDevice if absent. Do not
permit bool/noninteger/duplicate/missing cards. Preserve initial parameters,
accepted commands, realized decks and canonical SHA-256 initial/post-state hashes.
`state_hash(game)` hashes ALL private engine fields recursively, sorting map keys,
with stable JSON-compatible canonicalization. Include engine/rules IDs and schema
`blackjack-private-replay-v1`, and effective strategy identities in the envelope.
`export_replay(rec)` throws during betting/playing; only settled game exports a
detached envelope. `playback(engine,envelope)` returns reconstructed settled game,
validating exact envelope/step keys, versions, parameters, revisions, legal
commands, full decks, every expected hash and final settled phase. Any mismatch
throws ArgumentError; never mutates envelope, recorder, or a live session.
Limits: <=2000 commands, <=200 deals, canonical envelope <=1MiB; HTTP playback
uses a stricter request limit if appropriate. Reject excess before work.

Integration keeps public protocol v2 solo and v3 rivals; extra detached fields
are additive. Both engines use statistics. Server owns session recorders and
private replay; live /api/state never includes envelope/deck/cursor/hole card.
GET /api/session-report requires non-playing completed session and returns only
documented public totals/provenance. GET /api/replay requires settled state and
deliberately exports private data for that cookie only. POST /api/replay validates
a supplied envelope and returns ONLY a detached completed replay public snapshot;
it does not replace session state. Reset switches engine as before and starts a
fresh recorder. Invalid requests/versions/revisions and playback do not mutate.
Strategy selection is per rival at reset via rival_policies; defaults preserve
legacy stand17. Browser offers a dealer-aware choice and completed report/replay
download/playback, with privacy and capability labels, no inferred skill score.

Test modules remain under blackjack-web and run only when this demo is selected.


SOURCE .recur/blackjack-skill-workers/contracts/contract.md
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


SOURCE .recur/blackjack-skill-workers/contracts/baseline-engine.jl
module BlackjackRivals
using Random
module Rules
include("../blackjack-lab/main.blackjack.01.jl")
include("../blackjack-lab/main.blackjack.02.jl")
include("../blackjack-lab/main.blackjack.03.jl")
include("../blackjack-lab/main.blackjack.04.jl")
end

const POLICY = "Stand on 17 benchmark"

mutable struct Hand
    id::Int
    cards::Vector{Int}
    stake::Int
    status::String
    split_hand::Bool
    result::String
    profit::Int
end

mutable struct Rival
    id::String
    name::String
    balance::Int
    starting::Int
    escrow::Int
    hands::Vector{Hand}
    stats::Dict{String,Int}
    result::String
    profit::Int
end

mutable struct Table
    revision::Int
    phase::String
    balance::Int
    starting::Int
    bank::Int
    escrow::Int
    hands::Vector{Hand}
    active_hand_id::Union{Nothing,Int}
    dealer::Vector{Int}
    deck::Vector{Int}
    cursor::Int
    result::String
    profit::Int
    history::Vector{Any}
    stats::Dict{String,Int}
    rivals::Vector{Rival}
end

integer(x,lo,hi,label) = x isa Integer && !(x isa Bool) && lo<=x<=hi ? Int(x) :
    throw(ArgumentError("$label must be a whole number from $lo to $hi"))
rank(card) = mod1(card,13)
function handscore(h)
    score=Rules.score(h.cards)
    merge(score,(natural=!h.split_hand && score.natural,))
end
active(g) = g.hands[only(findall(h->h.id==g.active_hand_id,g.hands))]
newstats() = Dict("rounds"=>0,"hands"=>0,"wins"=>0,"losses"=>0,"pushes"=>0)

function newgame(money=500,rival_count=1)
    funds=integer(money,20,10000,"Starting chips")
    count=integer(rival_count,0,2,"Rival count")
    rivals=[Rival("rival-$i","Rival $i",funds,funds,0,Hand[],newstats(),"",0) for i in 1:count]
    Table(0,"betting",funds,funds,10000,0,Hand[],nothing,Int[],Int[],1,"",0,Any[],newstats(),rivals)
end

# publish: demo.blackjack.rivals.policy.decision visible score only; no table, deck or dealer input
policy_decision(score) = score.bust || score.total>=17 ? "stand" : "hit"

rival_bet(r,bet) = min(bet,2*(r.balance÷2))
deal_reserve(g,bet) = 3*(bet+sum((rival_bet(r,bet) for r in g.rivals);init=0))÷2
function max_bet(g)
    for bet in min(500,2*(g.balance÷2)):-2:2
        deal_reserve(g,bet)<=g.bank && return bet
    end
    0
end

# Reserve possible winnings, separate from the escrow that returns each stake.
# Only public hand scores enter this calculation; the hole card cannot affect actions.
function winnings_reserve(h)
    score=handscore(h)
    score.bust ? 0 : score.natural ? 3*h.stake÷2 : h.stake
end
rival_reserve(g) = sum((winnings_reserve(h) for r in g.rivals for h in r.hands);init=0)
double_reserve(g,h) = sum(winnings_reserve, g.hands;init=0)+h.stake+rival_reserve(g)

function split_eligibility(g)
    stake=isempty(g.hands) ? 0 : g.hands[1].stake
    reserve=2*stake+rival_reserve(g)
    reason = g.phase!="playing" ? "Split is available during your turn." :
        length(g.hands)!=1 || g.hands[1].split_hand ? "Only one split per round." :
        length(g.hands[1].cards)!=2 ? "Split requires your original two cards." :
        rank(g.hands[1].cards[1])!=rank(g.hands[1].cards[2]) ? "Split requires a matching rank pair." :
        g.balance<stake ? "Not enough chips for the extra wager." :
        g.bank<reserve ? "House reserve cannot cover all seats." : ""
    (allowed=isempty(reason),reason=reason,extra_stake=stake,reserve=reserve)
end

function allowed(g)
    if g.phase=="playing"
        h=active(g); actions=["hit","stand"]
        length(h.cards)==2 && g.balance>=h.stake && g.bank>=double_reserve(g,h) && push!(actions,"double")
        split_eligibility(g).allowed && push!(actions,"split")
        return actions
    end
    actions=String[]
    max_bet(g)>=2 && push!(actions,"deal")
    push!(actions,"reset")
    actions
end

public_hand(h) = (id=h.id,cards=copy(h.cards),stake=h.stake,status=h.status,split_hand=h.split_hand,
                  score=handscore(h),result=h.result,profit=h.profit)
public_rival(r) = (id=r.id,name=r.name,balance=r.balance,starting=r.starting,escrow=r.escrow,
                   net=r.balance+r.escrow-r.starting,hands=public_hand.(r.hands),stats=copy(r.stats),
                   result=r.result,profit=r.profit,policy=POLICY)

# publish: demo.blackjack.rivals.table.projection detached v3 snapshot; no deck, cursor or hole score
function public_state(g)
    hidden=g.phase=="playing"
    dealer=hidden ? Any[g.dealer[1],nothing] : Any[g.dealer...]
    (schema="blackjack-web-state-v3",protocol_version=3,revision=g.revision,
     round_id=g.stats["rounds"]+(hidden ? 1 : 0),phase=g.phase,balance=g.balance,
     starting=g.starting,bank=g.bank,escrow=g.escrow,hands=public_hand.(g.hands),
     active_hand_id=g.active_hand_id,dealer=dealer,
     dealer_score=hidden || isempty(g.dealer) ? nothing : Rules.score(g.dealer),
     result=g.result,profit=g.profit,net=g.balance+g.escrow-g.starting,allowed=allowed(g),
     split=split_eligibility(g),history=deepcopy(g.history),stats=copy(g.stats),max_bet=max_bet(g),
     rival_count=length(g.rivals),rivals=public_rival.(g.rivals))
end

function draw!(g,cards)
    g.cursor<=length(g.deck) || throw(ArgumentError("Deck exhausted"))
    push!(cards,g.deck[g.cursor]); g.cursor+=1
end

function outcome(h,dealer)
    p=handscore(h); d=Rules.score(dealer)
    p.bust && return "loss"
    d.natural && return p.natural ? "push" : "loss"
    p.natural && return "blackjack"
    d.bust && return "win"
    p.total==d.total ? "push" : p.total>d.total ? "win" : "loss"
end

function settle_seat!(seat,dealer)
    isempty(seat.hands) && return 0 # Sitting out does not count as a played round.
    for h in seat.hands
        h.result=outcome(h,dealer)
        h.profit=h.result=="blackjack" ? 3*h.stake÷2 : h.result=="win" ? h.stake : h.result=="loss" ? -h.stake : 0
        h.status="settled"
        seat.stats[h.result=="push" ? "pushes" : h.result=="loss" ? "losses" : "wins"]+=1
    end
    seat.profit=sum(h.profit for h in seat.hands)
    seat.balance+=seat.escrow+seat.profit; seat.escrow=0
    seat.result=all(h->h.result==seat.hands[1].result,seat.hands) ? seat.hands[1].result : "mixed"
    seat.stats["rounds"]+=1; seat.stats["hands"]+=length(seat.hands)
    seat.profit
end

# consumer: demo.blackjack.cards natural precedence and ace-aware scoring
# publish: demo.blackjack.rivals.table.settlement every participating seat once against one dealer
function settle!(g)
    g.phase=="playing" || throw(ArgumentError("Round already settled"))
    profit=settle_seat!(g,g.dealer)
    for rival in g.rivals
        profit+=settle_seat!(rival,g.dealer)
    end
    g.bank-=profit; g.phase="settled"; g.active_hand_id=nothing
    pushfirst!(g.history,(round=g.stats["rounds"],result=g.result,profit=g.profit,
        stake=sum(h.stake for h in g.hands),balance=g.balance,hands=public_hand.(g.hands),dealer=copy(g.dealer)))
    length(g.history)>20 && pop!(g.history)
    g
end

# consumer: demo.blackjack.rivals.policy.decision public score benchmark
# publish: demo.blackjack.rivals.table.advance human hands, stable rival order, then one dealer turn
function finish!(g)
    any(h->h.status in ("playing","waiting"),g.hands) && error("Unfinished human hand")
    for rival in g.rivals, h in rival.hands
        while policy_decision(handscore(h))=="hit"
            draw!(g,h.cards)
        end
        h.status=handscore(h).bust ? "bust" : "stood"
    end
    if any(h->!handscore(h).bust,g.hands) || any(h->!handscore(h).bust,(h for r in g.rivals for h in r.hands))
        while Rules.score(g.dealer).total<17
            draw!(g,g.dealer)
        end
    end
    settle!(g)
end

function advance!(g)
    i=findfirst(h->h.status=="waiting",g.hands)
    if i===nothing
        g.active_hand_id=nothing; finish!(g)
    else
        g.hands[i].status="playing"; g.active_hand_id=g.hands[i].id
    end
end

function end_hand!(g,h)
    h.status=handscore(h).bust ? "bust" : "stood"
    advance!(g)
end

# consumer: demo.blackjack.web.split.contract one rank-pair split, stable human IDs, split aces and 21
function split!(g)
    split_eligibility(g).allowed || throw(ArgumentError("Split unavailable"))
    h=active(g); aces=rank(h.cards[1])==1
    g.balance-=h.stake; g.escrow+=h.stake
    g.hands=[Hand(i,[card],h.stake,"waiting",true,"",0) for (i,card) in enumerate(h.cards)]
    for hand in g.hands
        draw!(g,hand.cards)
        (aces || handscore(hand).total==21) && (hand.status="stood")
    end
    advance!(g)
end

# publish: demo.blackjack.rivals.table.deal two passes over human, funded rivals, dealer
function deal!(g,bet,order)
    g.bank>=deal_reserve(g,bet) || throw(ArgumentError("House reserve cannot cover all seats; start a new table"))
    g.deck=order===nothing ? shuffle(RandomDevice(),collect(1:52)) : Rules.deck(order)
    g.cursor=1; g.hands=[Hand(1,Int[],bet,"playing",false,"",0)]; empty!(g.dealer)
    g.active_hand_id=1; g.escrow=bet; g.balance-=bet; g.result=""; g.profit=0; g.phase="playing"
    for rival in g.rivals
        stake=rival_bet(rival,bet)
        rival.hands=stake>=2 ? [Hand(1,Int[],stake,"waiting",false,"",0)] : Hand[]
        rival.escrow=stake; rival.balance-=stake; rival.profit=0
        rival.result=stake>=2 ? "" : "sitting_out"
    end
    for _ in 1:2
        draw!(g,g.hands[1].cards)
        for rival in g.rivals, h in rival.hands
            draw!(g,h.cards)
        end
        draw!(g,g.dealer)
    end
    for rival in g.rivals, h in rival.hands
        handscore(h).natural && (h.status="stood")
    end
    if Rules.score(g.dealer).natural
        settle!(g) # Peek before any human or rival draws.
    elseif handscore(g.hands[1]).natural
        end_hand!(g,g.hands[1])
    end
end

# publish: demo.blackjack.rivals.table.ledger separate nonnegative wallets/escrow and one conserved bank
function check_ledger(g)
    seats=(g,g.rivals...)
    all(s->s.balance>=0 && s.escrow>=0,seats) && g.bank>=0 || error("Negative ledger balance")
    sum(s->s.balance+s.escrow,seats)+g.bank==sum(s->s.starting,seats)+10000 || error("Ledger invariant failed")
    if g.phase=="playing"
        all(s->s.escrow==sum((h.stake for h in s.hands);init=0),seats) || error("Escrow invariant failed")
    else
        all(s->s.escrow==0,seats) || error("Unreleased escrow")
    end
    g
end

# publish: demo.blackjack.rivals.table.revision validate the whole command and mutate only a deep copy
function transition(original,command;order=nothing)
    command isa AbstractDict || throw(ArgumentError("Expected a command object"))
    integer(get(command,"version",nothing),3,3,"Protocol version")
    revision=integer(get(command,"revision",nothing),0,typemax(Int)-1,"Revision")
    revision==original.revision || throw(ArgumentError("Stale revision; refresh the table"))
    action=get(command,"action",nothing)
    action isa AbstractString && action in allowed(original) || throw(ArgumentError("Action is unavailable for this hand"))
    turn=action in ("hit","stand","double","split")
    permitted=action=="deal" ? ("version","action","revision","bet") :
        action=="reset" ? ("version","action","revision","money","rival_count") :
        ("version","action","revision","hand_id")
    all(k->k in permitted,keys(command)) || throw(ArgumentError("Unknown command field"))
    if turn
        id=integer(get(command,"hand_id",nothing),1,2,"Hand ID")
        id==original.active_hand_id || throw(ArgumentError("That hand is not active; refresh the table"))
    end
    if action=="reset"
        # The HTTP integrator selects the unchanged v2 engine when count is zero.
        fresh=newgame(get(command,"money",500),get(command,"rival_count",length(original.rivals)))
        fresh.revision=revision+1
        return check_ledger(fresh)
    end
    g=deepcopy(original)
    if action=="deal"
        bet=integer(get(command,"bet",nothing),2,min(500,g.balance),"Bet")
        iseven(bet) || throw(ArgumentError("Use an even bet for exact 3:2 payouts"))
        deal!(g,bet,order)
    elseif action=="split"
        split!(g)
    else
        h=active(g)
        if action=="hit"
            draw!(g,h.cards)
            handscore(h).total>=21 && end_hand!(g,h)
        elseif action=="stand"
            end_hand!(g,h)
        elseif action=="double"
            g.balance-=h.stake; g.escrow+=h.stake; h.stake*=2
            draw!(g,h.cards); end_hand!(g,h)
        end
    end
    check_ledger(g)
    g.revision=revision+1
    g
end
end


```
