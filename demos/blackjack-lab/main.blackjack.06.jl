# produces: demo.blackjack.coordination explicitly scheduled batch scoring and audit
player_receipt(order) = (producer="players",ids=copy(order.active),values=score.(order.hands))
house_receipt(order) = (producer="house",ids=[0],values=[score(only(order.hands))])
function coordinate(prepared;player_worker=player_receipt,house_worker=house_receipt)
    orders=Dict("players"=>(hands=deepcopy(prepared.hands[prepared.active]),active=copy(prepared.active),context=deepcopy(prepared)),
                "house"=>(hands=[copy(prepared.dealer_hand)],active=[0],context=deepcopy(prepared)))
    expected_players=player_receipt(orders["players"])
    expected_house=house_receipt(orders["house"])
    players=@async player_worker(orders["players"])
    house=@async house_worker(orders["house"])
    # Always join both producers, including when one fails.
    waitall=Task[players,house]
    for task in waitall; try wait(task) catch; end; end
    p=fetch(players); h=fetch(house)
    for (receipt,expected) in [(p,expected_players),(h,expected_house)]
        receipt==expected || throw(ArgumentError("missing, forged or mislabeled score receipt"))
    end
    result=allocate(prepared,p.values,only(h.values))
    conserved=sum(result.balances)+result.bank==sum(prepared.balances)+prepared.bank
    conserved || throw(ArgumentError("audit rejected nonconserving settlement"))
    (conserved=conserved,result=result)
end
