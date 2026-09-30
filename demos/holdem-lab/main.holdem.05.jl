mutable struct HandState
    deal::NamedTuple
    stacks::Vector{Int}
    paid::Vector{Int}
    pot::Int
    street::Symbol
    actor::Int
    pending::Set{Int}
    min_raise::Int
    settled::Bool
    winners::Vector{Int}
end
# consumes: demo.holdem.deal fixed heads-up table rules
# produces: demo.holdem.play.initial blinds and first actor
function start_hand(; order=deck().cards, stack::Int=100)
    2 <= stack <= typemax(Int)÷2 || throw(ArgumentError("stack outside supported range"))
    HandState(deal(deck(order)), [stack-1,stack-2], [1,2], 3,
              :preflop, 1, Set([1,2]), 2, false, Int[])
end

function pay!(s, player, amount)
    s.stacks[player] -= amount
    s.paid[player] += amount
    s.pot += amount
end

# consumes: demo.holdem.rank both evaluations, never a renderer callback
# produces: demo.holdem.settle final chip ownership
function settle!(s; folded=nothing)
    if isnothing(folded)
        left = evaluate((cards=vcat(s.deal.holes[1],s.deal.board),)).rank
        right = evaluate((cards=vcat(s.deal.holes[2],s.deal.board),)).rank
        s.winners = winners((;left,right)).winners
        s.street = :showdown
    else
        s.winners = [3-folded]
    end
    share, extra = divrem(s.pot,length(s.winners))
    for p in s.winners
        s.stacks[p] += share
    end
    extra > 0 && (s.stacks[2] += extra)
    s.pot = 0
    s.paid .= 0
    s.settled = true
    s.actor = 0
    empty!(s.pending)
    s
end

function advance!(s)
    # With equal starting stacks, equal cumulative contributions mean both
    # players go all-in together once a call matches the outstanding bet.
    if s.stacks == [0,0] || s.street == :river
        return settle!(s)
    end
    s.street = s.street == :preflop ? :flop : s.street == :flop ? :turn : :river
    s.paid .= 0
    s.actor = 2
    s.pending = Set([1,2])
    s.min_raise = 2
    s
end

# consumes: demo.holdem.play command application is the only state transition
# produces: demo.holdem.play a fresh state, including when returning settlement
function act(state::HandState, player::Int, action::Symbol, amount::Int=0)
    !state.settled && player == state.actor || throw(ArgumentError("not this player's turn"))
    action in (:check,:call,:raise,:fold) || throw(ArgumentError("unknown action"))
    action == :raise || amount == 0 || throw(ArgumentError("only raise accepts an amount"))
    debt = maximum(state.paid)-state.paid[player]
    action == :check && debt != 0 && throw(ArgumentError("cannot check facing a bet"))
    action == :call && debt == 0 && throw(ArgumentError("nothing to call"))
    if action == :raise
        current = maximum(state.paid)
        amount > current && amount-current >= state.min_raise || throw(ArgumentError("raise too small"))
        amount <= minimum(state.stacks+state.paid) || throw(ArgumentError("raise exceeds effective stack"))
    end
    s = deepcopy(state)
    action == :fold && return settle!(s; folded=player)
    if action == :raise
        s.min_raise = amount-maximum(s.paid)
        pay!(s,player,amount-s.paid[player])
        s.pending = Set([3-player])
    else
        action == :call && pay!(s,player,debt)
        delete!(s.pending,player)
    end
    if s.stacks == [0,0] || isempty(s.pending)
        advance!(s)
    else
        s.actor = 3-player
        s
    end
end

# consumes: demo.holdem.play public boundary deliberately omits private state
# produces: demo.holdem.public no opponent cards, burns or future board
function public_view(s::HandState)
    n = s.street == :preflop ? 0 : s.street == :flop ? 3 : s.street == :turn ? 4 : 5
    (street=s.street,board=copy(s.deal.board[1:n]),stacks=copy(s.stacks),
     pot=s.pot,actor=s.actor,settled=s.settled,winners=copy(s.winners))
end
render(v) = (text="$(v.street) | board=$(v.board) | stacks=$(v.stacks) | pot=$(v.pot) | actor=$(v.actor) | winners=$(v.winners)",)

# triggers: demo.holdem.play deterministic scripted hierarchical command driver
function play(actions; order=deck().cards, stack=100)
    state = start_hand(;order,stack)
    for action in actions
        state = act(state,action...)
    end
    state.settled || throw(ArgumentError("script ended before hand settled"))
    state
end
