# produces: demo.blackjack.session repeated state transitions without module cycles
function tournament(config::TableConfig;orders=nothing)
    rng=MersenneTwister(config.seed); balances=fill(config.money,length(config.players)); bank=config.bank
    history=NamedTuple[]; reason="round_limit"
    for number in 1:config.rounds
        active=count(>=(config.bet),balances)
        if active==0; reason="players_unfunded"; break; end
        if bank<active*(3*config.bet÷2); reason="house_reserve"; break; end
        order = if orders===nothing
            shuffle(rng,collect(1:52))
        else
            number<=length(orders) || throw(ArgumentError("missing scripted round deck"))
            orders[number]
        end
        played=prepare_round(config,balances,bank,order)
        result=settle_round(played)
        push!(history,(number=number,hands=deepcopy(played.hands),dealer_hand=copy(played.dealer_hand),
            remaining=copy(played.remaining),result=result))
        balances=copy(result.balances); bank=result.bank
    end
    (config=config,rounds=history,balances=balances,bank=bank,
     leaders=findall(==(maximum(balances)),balances),profit=balances.-config.money,stop_reason=reason)
end
function render(session)
    io=IOBuffer(); t=session.config
    println(io,describe(t))
    for round in session.rounds
        println(io,"Round $(round.number): $(t.dealer) hand $(round.dealer_hand), score $(score(round.dealer_hand).total)")
        for id in eachindex(t.players)
            println(io,"  $(t.players[id]): $(round.result.outcomes[id]), delta $(round.result.deltas[id]), balance $(round.result.balances[id])")
        end
        println(io,"  Player winners: $(round.result.winners); house wins against: $(round.result.dealer_wins); pushes: $(round.result.pushes); house bank: $(round.result.bank)")
    end
    println(io,"Final leaders: $(join(t.players[session.leaders], ", ")); balances $(session.balances); net profits $(session.profit)")
    println(io,"$(t.dealer) final bank: $(session.bank); stop: $(session.stop_reason)")
    String(take!(io))
end
