module BlackjackRivals
using Random
include("skill/main.blackjack.skill.strategy.jl")
include("skill/main.blackjack.skill.statistics.jl")
const RULES_ID="s17-3to2-one-split-v1"
const COMPARISON_LABEL="Practice results only: rivals hit/stand; you may split/double. Shared deck, seat order, stakes and bankroll affect results; this is not a skill rating."
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
    policy_id::String
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
newstats() = BlackjackStatistics.empty_stats()

function newgame(money=500,rival_count=1;policies=nothing)
    funds=integer(money,20,10000,"Starting chips")
    count=integer(rival_count,0,2,"Rival count")
    ids=policies===nothing ? fill("stand17-v1",count) : policies
    ids isa AbstractVector && length(ids)==count && all(id->id isa AbstractString && id in ("stand17-v1","dealer-aware-v1"),ids) || throw(ArgumentError("Choose one supported policy per rival"))
    rivals=[Rival(String(ids[i]),"rival-$i","Rival $i",funds,funds,0,Hand[],newstats(),"",0) for i in 1:count]
    Table(0,"betting",funds,funds,10000,0,Hand[],nothing,Int[],Int[],1,"",0,Any[],newstats(),rivals)
end

# publish: demo.blackjack.rivals.policy.decision visible score only; no table, deck or dealer input
policy_decision(score) = score.bust || score.total>=17 ? "stand" : "hit"
# publish: demo.blackjack.skill.strategy.input fresh visible values only
function policy_input(g,h,balance)
    score=handscore(h)
    Dict{String,Any}("total"=>score.total,"soft"=>score.soft,"natural"=>score.natural,"bust"=>score.bust,
        "dealer_upcard"=>g.dealer[1],"legal_actions"=>["hit","stand"],"affordable_stake"=>balance,"rules_id"=>RULES_ID)
end
policy_record(id)=only(filter(p->p.id==id,BlackjackStrategy.policies()))

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
                   result=r.result,profit=r.profit,policy=policy_record(r.policy_id).name,policy_id=r.policy_id,policy_version=1,capabilities=["hit","stand"],session_statistics=BlackjackStatistics.summary(r.stats))

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
     rival_count=length(g.rivals),rivals=public_rival.(g.rivals),session_statistics=BlackjackStatistics.summary(g.stats),rules_id=RULES_ID,engine_id="blackjack-rivals-v3",comparison_label=COMPARISON_LABEL)
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
    end
    seat.profit=sum(h.profit for h in seat.hands)
    seat.balance+=seat.escrow+seat.profit; seat.escrow=0
    seat.result=all(h->h.result==seat.hands[1].result,seat.hands) ? seat.hands[1].result : "mixed"
    BlackjackStatistics.settle_round!(seat.stats,[(result=h.result,stake=h.stake,profit=h.profit,natural=handscore(h).natural,bust=handscore(h).bust) for h in seat.hands])
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
        while BlackjackStrategy.decision(rival.policy_id,policy_input(g,h,rival.balance))=="hit"
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
    BlackjackStatistics.start_round!(g.stats)
    for rival in g.rivals
        stake=rival_bet(rival,bet)
        rival.hands=stake>=2 ? [Hand(1,Int[],stake,"waiting",false,"",0)] : Hand[]
        rival.escrow=stake; rival.balance-=stake; rival.profit=0
        rival.result=stake>=2 ? "" : "sitting_out"
        BlackjackStatistics.start_round!(rival.stats;active=stake>=2)
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
        action=="reset" ? ("version","action","revision","money","rival_count","rival_policies") :
        ("version","action","revision","hand_id")
    all(k->k in permitted,keys(command)) || throw(ArgumentError("Unknown command field"))
    if turn
        id=integer(get(command,"hand_id",nothing),1,2,"Hand ID")
        id==original.active_hand_id || throw(ArgumentError("That hand is not active; refresh the table"))
    end
    if action=="reset"
        # The HTTP integrator selects the unchanged v2 engine when count is zero.
        fresh=newgame(get(command,"money",500),get(command,"rival_count",length(original.rivals));policies=get(command,"rival_policies",nothing))
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
