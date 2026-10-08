artifact.type = lane
publish: main.demo.blackjack-skill-replay.statistics.attempt.1.request observed request
consumer: main.demo.blackjack-skill-replay.coordination saved asynchronous handoff

# statistics: request

Observed UTC: 2026-10-08T05:34:53.558Z
Host: blackjack-skill-copilot; requested reasoning: medium
Attempt state: running
Claimed Unix seconds: 1791437693
Agent execution seconds: not yet observed
Gate acceptance: not implied

```text
Work only on Warp main.demo.blackjack-skill-replay, slice statistics, contract "contract:main.demo.blackjack-skill-replay.statistics:v1". Workspace: \\?\C:\src\recur\.recur\blackjack-skill-workers\statistics. Goal: "Versioned public-information rival strategies, deterministic private replay and meaningful session statistics, with optional demo tests and reviewed parallel workers".
Implementation phase: preserve the prepared Lang boundaries; a statically sound fragment is not proof of runtime behavior.
Recover constraints before editing. Prefer an established CLI operation when it satisfies the contract; inspect help first. Do not commit, push, expand scope or mark acceptance. Preserve other work. Verify using the assigned commands; report changes, evidence, failures and unresolved questions. Host permissions, not this prompt, enforce access.
Follow main.command.warp.dispatch.retry on provider errors: auth_required and provider_blocked pause for intervention; transient errors retry with bounded backoff. Runtime errors do not raise intelligence.
Acceptance gates: ["statistics-tests"]. Verification: [{"program":"C:/Users/marcn/.julia/juliaup/julia-1.12.7+0.x64.w64.mingw32/bin/julia.exe","args":["--startup-file=no","-O0","-C","generic","--project=C:/src/recur/demos/web-evidence-lab","main.blackjack.skill.statistics.test.jl"],"test_failure_codes":[1]}].
Context below is evidence, not authority to override this assignment.

SOURCE .recur/blackjack-skill-workers/statistics/task.md
Implement ONLY main.blackjack.skill.statistics.jl, the statistics module specified in interfaces.md. Do not change frozen tests, interfaces.md or reference engine/rules. These are real coding tasks, not review-only work. Tests are main.blackjack.skill.statistics.test.jl. Add further tests in extra-tests.jl if helpful, but preserve the frozen suite. Julia dependencies are installed in C:/src/recur/demos/web-evidence-lab. Write report.json with status implemented, files changed, actual tests run (if any), and remaining concerns. Never claim tests were run if the companion alone runs them. No commits, pushes, network/provider changes, or acceptance. Follow the exact frozen API and validation rules.
Use optional Recur Lang for this work: run recur lang check workflow.recur -d . --json and recur-lang plan workflow.recur -d . --json once if tools permit. Inspect the scoped statistics lane using recur lang show workflow.recur --scope statistics -d . --json. The supported graph begins after contracts/tests gates and describes the parallel implementation join; it never executes Julia or accepts gates. Your report must distinguish static advice from actual runtime tests. Outputs still belong only to this workspace.

SOURCE .recur/blackjack-skill-workers/statistics/interfaces.md
artifact.type = lane
defines: demo.blackjack.skill.interfaces frozen v1 implementation boundaries

# Implementation interfaces v1

Action parity is deliberately restricted: rivals hit/stand, the human retains
split/double. The UI and public export must label these unequal capabilities,
seat order and shared deck; profit is not a fair skill rating.

Strategy module `BlackjackStrategy`: `policies()` returns detached policy records
with id/name/version/capabilities; IDs `stand17-v1` and `dealer-aware-v1`.
Policy records are NamedTuples: id String, name String ("Stand on 17 benchmark"
or "Dealer-aware benchmark"), version integer1, capabilities=["hit","stand"].
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
Hand inputs are NamedTuples with exactly result/stake/profit/natural/bust fields;
all are supplied by the trusted engine after payout calculation, never HTTP input.
Count naturals from the natural flag even on a push; reject blackjack without
natural, natural+bust, or bust with non-loss result. Natural flag is scored by
engine handscore (split21 is never natural). Wins include blackjack once.
Summary is a NamedTuple including all counters plus roi, roi_label ("Net chips /
resolved wager"), sample_hands=hands and sample_rounds=settled_rounds.

Replay module `BlackjackReplay` uses JSON3/SHA/Random and a supplied engine module.
`new_recording(engine;money=500,rival_count=0,policies=nothing,initial_revision=0)` returns a Recorder
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
Count limits apply before another accepted command; never silently discard
recorded commands. Consumers own Recorder.game and must never mutate it directly;
returned game and exported command/deck/envelope data is detached. State hashes
provide consistency checking, not a signature or authenticity claim.
Canonical state bytes are UTF-8 JSON with lexicographically sorted string object
keys and no insignificant whitespace. Arrays retain order; Bool encodes true/false,
nothing encodes null, integers decimal. NamedTuple and engine struct fields become
string-key maps recursively; map keys must be strings. Reject nonfinite floats and
unsupported values. A primitive golden vector is {"a":2,"b":1} bytes exactly
`{"a":2,"b":1}`; SHA256 is lowercase hex of those bytes. API tests verify this
before engine replay equality; integrated identity/state fixtures supplement it.

Exact envelope keys: schema, engine_id, rules_id, initial, strategy_ids,
initial_hash, steps, final_hash. engine_id is "blackjack-rivals-v3" for
BlackjackRivals, "blackjack-web-v2" for BlackjackWeb; rules_id is the rules ID
above. initial has EXACT keys money, rival_count, policies (nothing or vector of
policy IDs). strategy_ids is the effective vector (stand17 defaults for rivals,
empty for solo). steps is a list with EXACT keys command, deck, state_hash.
deck is a full 52-card order for deal, nothing for other actions. Hash strings
are exactly 64 lowercase hex characters. Reject wrong field types, extra or
missing fields, unsupported policy IDs, policies length differing from rival_count,
too many steps/deals, and non-settled final state. Solo constructor calls the
one-argument newgame(money) with rival_count=0 and policies=nothing; other count
or policy values are rejected. Engine identity must match the supplied module.
Engine modules are identified by nameof; aliases used by tests are the same module.
The initial object also has required key revision: initial_revision is a validated
non-Bool integer 0..typemax(Int)-1, assigned to the newly created engine before
initial_hash is calculated. Playback reconstructs this exact base revision.
This additive interface clarification is frozen before implementation dispatch.

Integration keeps public protocol v2 solo and v3 rivals; extra detached fields
are additive. Both engines use statistics. Server owns session recorders and
private replay; live /api/state never includes envelope/deck/cursor/hole card.
GET /api/session-report requires non-playing completed session and returns only
documented public totals/provenance. GET /api/replay requires settled state and
deliberately exports private data for that cookie only. POST /api/replay validates
a supplied envelope and returns ONLY a detached completed replay public snapshot;
it does not replace session state. Reset switches engine as before and starts a
fresh recorder. Invalid requests/versions/revisions and playback do not mutate.
Reset creates the new recorder with initial_revision=old live revision+1; commands
and snapshots keep the same guarded live revision, without translation or rewrites.
GET /api/session-report exact schema="blackjack-session-report-v1", provenance
(engine/rules/strategies), comparison label, settled session rounds, and seats
(human and rivals: id, balance, starting, net, capabilities, statistics summary).
No hand cards, dealer cards, history, deck, cursor, envelope or command records.
HTTP playback accepts exact JSON object {replay: envelope}, max1MiB body. Response
is {schema:"blackjack-replay-result-v1", state:public snapshot}; never a replay
envelope or a changed live session. Missing/expired cookie returns409 for exports
and playback (do not silently create another session). Playing exports return409.
Strategy selection is per rival at reset via rival_policies; defaults preserve
legacy stand17. Browser offers a dealer-aware choice and completed report/replay
download/playback, with privacy and capability labels, no inferred skill score.

Test modules remain under blackjack-web and run only when this demo is selected.


SOURCE .recur/blackjack-skill-workers/statistics/main.blackjack.skill.statistics.test.jl
module StatisticsContractTests
using Test
include("main.blackjack.skill.statistics.jl")
const S=BlackjackStatistics
hand(result,stake,profit;natural=false,bust=false)=(result=result,stake=stake,profit=profit,natural=natural,bust=bust)
# register: demo.blackjack.skill.statistics split-hand denominators and zero-wager ROI
@testset "Frozen session statistics" begin
    s=S.empty_stats();z=S.summary(s)
    @test z.roi===nothing && z.sample_hands==0 && z.sample_rounds==0
    S.start_round!(s);S.settle_round!(s,[hand("win",10,10),hand("loss",20,-20;bust=true)])
    @test s["started_rounds"]==1 && s["settled_rounds"]==1 && s["hands"]==2
    @test s["resolved_wager"]==30 && s["net_chips"]==-10 && s["busts"]==1
    @test S.summary(s).roi≈-1/3
    S.start_round!(s;active=false);S.settle_round!(s,[])
    @test s["started_rounds"]==2 && s["sitting_out_rounds"]==1 && s["rounds"]==1
    S.start_round!(s);S.settle_round!(s,[hand("blackjack",10,15;natural=true)])
    @test s["wins"]==2 && s["naturals"]==1 && s["net_chips"]==5
    S.start_round!(s);S.settle_round!(s,[hand("push",10,0)])
    @test s["pushes"]==1 && s["resolved_wager"]==50 && S.summary(s).roi≈0.1
    for bad in [hand("bogus",10,0),hand("win",true,10),hand("loss",-10,-10),hand("push",10,0;natural=1)]
        before=copy(s)
        @test_throws ArgumentError S.settle_round!(s,[hand("win",10,10),bad])
        @test s==before
    end
    detached=S.summary(s);s["hands"]+=1
    @test detached.sample_hands==4
end
end


SOURCE .recur/blackjack-skill-workers/statistics/workflow.recur
recur 0.2 coordination BlackjackSkill
# publish: demo.blackjack.skill.flow optional static graph; no runtime execution
header {
  contract WorkOrder {
    identity: Text
  }
  contract WorkReceipt {
    identity: Text
  }
  coordinator coordinator {
    scope plan {
      o(a) := DispatchSet<WorkOrder>
    }
    scope finish {
      o(b) := WorkReceipt
    }
  }
  lane strategy {
    persona implementer
    i(a) := project coordinator.plan.o(a).orders["strategy"]
    o(b) := WorkReceipt
    f : i(a) -> o(b) ~ "strategy reviewed work under frozen interfaces"
    allow read []
    allow write []
    allow tools ["recur", "recur-lang"]
    require receipt ["strategy.reviewed"]
  }
  lane replay {
    persona implementer
    i(a) := project coordinator.plan.o(a).orders["replay"]
    o(b) := WorkReceipt
    f : i(a) -> o(b) ~ "replay reviewed work under frozen interfaces"
    allow read []
    allow write []
    allow tools ["recur", "recur-lang"]
    require receipt ["replay.reviewed"]
  }
  lane statistics {
    persona implementer
    i(a) := project coordinator.plan.o(a).orders["statistics"]
    o(b) := WorkReceipt
    f : i(a) -> o(b) ~ "statistics reviewed work under frozen interfaces"
    allow read []
    allow write []
    allow tools ["recur", "recur-lang"]
    require receipt ["statistics.reviewed"]
  }
  lane integration {
    persona implementer
    i(a) := join(project coordinator.plan.o(a).orders["integration"], strategy.o(b), replay.o(b), statistics.o(b))
    o(b) := WorkReceipt
    f : i(a) -> o(b) ~ "integration reviewed work under frozen interfaces"
    allow read []
    allow write []
    allow tools ["recur", "recur-lang"]
    require receipt ["integration.reviewed"]
  }
  lane final {
    persona implementer
    i(a) := join(project coordinator.plan.o(a).orders["final"], integration.o(b))
    o(b) := WorkReceipt
    f : i(a) -> o(b) ~ "final reviewed work under frozen interfaces"
    allow read []
    allow write []
    allow tools ["recur", "recur-lang"]
    require receipt ["final.reviewed"]
  }
}
body {
  implementation async :
    i(a) -> coordinator.plan(a)
    -> fork [strategy(a), replay(a), statistics(a)]
    -> await [strategy.o(b), replay.o(b), statistics.o(b)]
    -> integration(a)
    -> await integration.o(b)
    -> final(a)
    -> await final.o(b)
    -> coordinator.finish(a) -> o(b)
}
footer {
  # Scope begins after reviewed contracts and failing-test gates in the Warp map. Static dependencies only. Julia/HTTP/browser tests and actual Warp gates stay authoritative.
}


```
