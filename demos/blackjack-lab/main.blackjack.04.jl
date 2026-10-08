# produces: demo.blackjack.round atomic play and settlement over copied bundles
function prepare_round(config::TableConfig,balances,bank,order)
    length(balances)==length(config.players) || throw(ArgumentError("one balance per player required"))
    funds=[bounded_int(x,0,10^12,"balance") for x in balances]
    house=bounded_int(bank,0,10^12,"bank")
    active=findall(>=(config.bet),funds)
    isempty(active) && throw(ArgumentError("no funded players"))
    house>=length(active)*(3*config.bet÷2) || throw(ArgumentError("house reserve insufficient"))
    cards=deck(order); cursor=1
    draw!() = (cursor<=52 || throw(ArgumentError("deck exhausted")); c=cards[cursor]; cursor+=1; c)
    hands=[Int[] for _ in funds]; dealer=Int[]
    for _ in 1:2
        for id in active; push!(hands[id],draw!()); end
        push!(dealer,draw!())
    end
    if !score(dealer).natural
        for id in active
            while score(hands[id]).total<17; push!(hands[id],draw!()); end
        end
        if any(id->!score(hands[id]).bust && !score(hands[id]).natural,active)
            while score(dealer).total<17; push!(dealer,draw!()); end
        end
    end
    (config=config,hands=hands,dealer_hand=dealer,active=active,balances=funds,bank=house,remaining=cards[cursor:end])
end
function compare_scores(player,dealer)
    player.bust && return :loss
    dealer.natural && return player.natural ? :push : :loss
    player.natural && return :blackjack
    dealer.bust && return :win
    player.total>dealer.total ? :win : player.total<dealer.total ? :loss : :push
end
compare_hands(player,dealer) = compare_scores(score(player),score(dealer))
function allocate(prepared,player_scores,dealer_score)
    length(player_scores)==length(prepared.active) || throw(ArgumentError("missing player score"))
    outcomes=fill(:sitting_out,length(prepared.balances)); deltas=zeros(Int,length(outcomes))
    for (id,rank) in zip(prepared.active,player_scores)
        outcome=compare_scores(rank,dealer_score); outcomes[id]=outcome
        deltas[id]=outcome==:blackjack ? 3*prepared.config.bet÷2 : outcome==:win ? prepared.config.bet : outcome==:loss ? -prepared.config.bet : 0
    end
    funds=prepared.balances.+deltas; house=prepared.bank-sum(deltas)
    all(>=(0),funds) && house>=0 || throw(ArgumentError("settlement overdraft"))
    (outcomes=outcomes,deltas=deltas,balances=funds,bank=house,winners=findall(>(0),deltas),
     dealer_wins=findall(<(0),deltas),pushes=findall(==(:push),outcomes))
end
settle_round(prepared) = allocate(prepared,[score(prepared.hands[id]) for id in prepared.active],score(prepared.dealer_hand))
