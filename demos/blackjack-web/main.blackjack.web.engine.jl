module BlackjackWeb
using Random
include("skill/main.blackjack.skill.statistics.jl")
module Rules
include("../blackjack-lab/main.blackjack.01.jl")
include("../blackjack-lab/main.blackjack.02.jl")
include("../blackjack-lab/main.blackjack.03.jl")
include("../blackjack-lab/main.blackjack.04.jl")
end

mutable struct Hand
    id::Int
    cards::Vector{Int}
    stake::Int
    status::String
    split_hand::Bool
    result::String
    profit::Int
end
mutable struct Game
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
end
integer(x,lo,hi,label) = x isa Integer && !(x isa Bool) && lo<=x<=hi ? Int(x) : throw(ArgumentError("$label must be a whole number from $lo to $hi"))
rank(card)=mod1(card,13)
handscore(h)=merge(Rules.score(h.cards),(natural=!h.split_hand && Rules.score(h.cards).natural,))
active(g)=g.hands[only(findall(h->h.id==g.active_hand_id,g.hands))]
# publish: demo.blackjack.web.table isolated initial ledger
function newgame(money=500)
    funds=integer(money,20,10000,"Starting chips")
    Game(0,"betting",funds,funds,10000,0,Hand[],nothing,Int[],Int[],1,"",0,Any[],
         BlackjackStatistics.empty_stats())
end
# publish: demo.blackjack.web.split.eligibility.result pair, wallet and aggregate reserve
function split_eligibility(g)
    stake=isempty(g.hands) ? 0 : g.hands[1].stake
    reserve=sum((h.stake for h in g.hands);init=0)+stake
    reason = g.phase!="playing" ? "Split is available during your turn." :
        length(g.hands)!=1 || g.hands[1].split_hand ? "Only one split per round." :
        length(g.hands[1].cards)!=2 ? "Split requires your original two cards." :
        rank(g.hands[1].cards[1])!=rank(g.hands[1].cards[2]) ? "Split requires a matching rank pair." :
        g.balance<stake ? "Not enough chips for the extra wager." :
        g.bank<reserve ? "House reserve cannot cover both hands." : ""
    (allowed=isempty(reason),reason=reason,extra_stake=stake,reserve=reserve)
end
function allowed(g)
    if g.phase=="playing"
        h=active(g); actions=["hit","stand"]
        length(h.cards)==2 && g.balance>=h.stake && g.bank>=g.escrow+h.stake && push!(actions,"double")
        split_eligibility(g).allowed && push!(actions,"split")
        return actions
    end
    actions=String[]
    g.balance>=2 && g.bank>=3 && push!(actions,"deal")
    push!(actions,"reset")
    actions
end
public_hand(h)=(id=h.id,cards=copy(h.cards),stake=h.stake,status=h.status,split_hand=h.split_hand,
                score=handscore(h),result=h.result,profit=h.profit)
# publish: demo.blackjack.web.view public snapshot, never private deck
# publish: demo.blackjack.web.split.view.result same projection for HTTP and model correspondence
function public_state(g)
    hidden=g.phase=="playing"
    dealer=hidden ? Any[g.dealer[1],nothing] : Any[g.dealer...]
    (schema="blackjack-web-state-v2",revision=g.revision,round_id=g.stats["rounds"]+(hidden ? 1 : 0),
     phase=g.phase,balance=g.balance,starting=g.starting,bank=g.bank,escrow=g.escrow,
     hands=public_hand.(g.hands),active_hand_id=g.active_hand_id,dealer=dealer,
     dealer_score=hidden || isempty(g.dealer) ? nothing : Rules.score(g.dealer),
     result=g.result,profit=g.profit,net=g.balance+g.escrow-g.starting,allowed=allowed(g),
     split=split_eligibility(g),history=deepcopy(g.history),stats=copy(g.stats),
     max_bet=min(500,2*(g.balance÷2),2*(g.bank÷3)),session_statistics=BlackjackStatistics.summary(g.stats),rules_id="s17-3to2-one-split-v1",engine_id="blackjack-web-v2",comparison_label="Solo practice; session profit is descriptive, not a skill rating.")
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
# consumer: demo.blackjack.cards ace-aware scoring and natural precedence
# publish: demo.blackjack.web.settle conserved one-time payout
# publish: demo.blackjack.web.split.settle.result one aggregate settlement, per-hand outcomes
function settle!(g)
    g.phase=="playing" || throw(ArgumentError("Round already settled"))
    for h in g.hands
        h.result=outcome(h,g.dealer)
        h.profit=h.result=="blackjack" ? 3*h.stake÷2 : h.result=="win" ? h.stake : h.result=="loss" ? -h.stake : 0
        h.status="settled"
    end
    g.profit=sum(h.profit for h in g.hands)
    g.balance+=g.escrow+g.profit; g.bank-=g.profit; g.escrow=0
    g.result=all(h->h.result==g.hands[1].result,g.hands) ? g.hands[1].result : "mixed"
    g.phase="settled"; g.active_hand_id=nothing
    BlackjackStatistics.settle_round!(g.stats,[(result=h.result,stake=h.stake,profit=h.profit,natural=handscore(h).natural,bust=handscore(h).bust) for h in g.hands])
    pushfirst!(g.history,(round=g.stats["rounds"],result=g.result,profit=g.profit,
        stake=sum(h.stake for h in g.hands),balance=g.balance,hands=public_hand.(g.hands),dealer=copy(g.dealer)))
    length(g.history)>20 && pop!(g.history)
    g
end
# publish: demo.blackjack.web.split.dealer.result dealer plays once after all hands finish
function finish!(g)
    any(h->h.status in ("playing","waiting"),g.hands) && error("Unfinished player hand")
    if any(h->!handscore(h).bust,g.hands)
        while Rules.score(g.dealer).total<17; draw!(g,g.dealer); end
    end
    settle!(g)
end
# publish: demo.blackjack.web.split.advance.result next unfinished hand or dealer phase
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
# publish: demo.blackjack.web.split.split.result stable IDs and one replacement each
function split!(g)
    split_eligibility(g).allowed || throw(ArgumentError("Split unavailable"))
    h=active(g); aces=rank(h.cards[1])==1
    g.balance-=h.stake;g.escrow+=h.stake
    g.hands=[Hand(i,[card],h.stake,"waiting",true,"",0) for (i,card) in enumerate(h.cards)]
    for hand in g.hands
        draw!(g,hand.cards)
        (aces || handscore(hand).total==21) && (hand.status="stood")
    end
    advance!(g)
end
# consumer: demo.blackjack.web.play request bundle and expected revision
# consumer: demo.blackjack.web.split.contract revision AND active identity
# publish: demo.blackjack.web.play atomic copied state, or error without mutation
function transition(original,command;order=nothing)
    command isa AbstractDict || throw(ArgumentError("Expected a command object"))
    integer(get(command,"version",nothing),2,2,"Protocol version")
    revision=integer(get(command,"revision",nothing),0,typemax(Int),"Revision")
    revision==original.revision || throw(ArgumentError("Stale revision; refresh the table"))
    action=get(command,"action",nothing)
    action isa AbstractString && action in allowed(original) || throw(ArgumentError("Action is unavailable for this hand"))
    turn=action in ("hit","stand","double","split")
    permitted=action=="deal" ? ("version","action","revision","bet") : action=="reset" ? ("version","action","revision","money") : ("version","action","revision","hand_id")
    all(k->k in permitted,keys(command)) || throw(ArgumentError("Unknown command field"))
    if turn
        id=integer(get(command,"hand_id",nothing),1,2,"Hand ID")
        id==original.active_hand_id || throw(ArgumentError("That hand is not active; refresh the table"))
    end
    if action=="reset"
        fresh=newgame(get(command,"money",500));fresh.revision=revision+1;return fresh
    end
    g=deepcopy(original)
    if action=="deal"
        bet=integer(get(command,"bet",nothing),2,min(500,g.balance),"Bet")
        iseven(bet) || throw(ArgumentError("Use an even bet for exact 3:2 payouts"))
        g.bank>=3*bet÷2 || throw(ArgumentError("House reserve is too low; start a new table"))
        g.deck=order===nothing ? shuffle(RandomDevice(),collect(1:52)) : Rules.deck(order)
        g.cursor=1;g.hands=[Hand(1,Int[],bet,"playing",false,"",0)];empty!(g.dealer)
        g.active_hand_id=1;g.escrow=bet;g.balance-=bet;g.result="";g.profit=0;g.phase="playing"
        BlackjackStatistics.start_round!(g.stats)
        for _ in 1:2;draw!(g,g.hands[1].cards);draw!(g,g.dealer);end
        if handscore(g.hands[1]).natural || Rules.score(g.dealer).natural;settle!(g);end
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
            g.balance-=h.stake;g.escrow+=h.stake;h.stake*=2
            draw!(g,h.cards);end_hand!(g,h)
        end
    end
    g.balance>=0 && g.bank>=0 && g.balance+g.escrow+g.bank==g.starting+10000 || error("Ledger invariant failed")
    g.phase!="playing" || g.escrow==sum(h.stake for h in g.hands) || error("Escrow invariant failed")
    g.revision=revision+1
    g
end
end
