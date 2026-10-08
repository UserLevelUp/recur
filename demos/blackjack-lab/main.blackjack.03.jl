# produces: demo.blackjack.cards physical cards and ace-aware scoring
function cards_checked(cards; full=false)
    cards isa AbstractVector || throw(ArgumentError("cards must be a vector"))
    c=[bounded_int(x,1,52,"card") for x in cards]
    !isempty(c) && length(unique(c))==length(c) || throw(ArgumentError("cards must be nonempty and unique"))
    full && length(c)!=52 && throw(ArgumentError("deck must contain all 52 cards"))
    c
end
deck(order=collect(1:52)) = cards_checked(order;full=true)
function score(cards)
    c=cards_checked(cards)
    ranks=mod.(c.-1,13).+1
    aces=count(==(1),ranks)
    total=sum(r==1 ? 11 : min(r,10) for r in ranks)
    while total>21 && aces>0; total-=10; aces-=1; end
    (total=total,soft=aces>0,natural=length(c)==2 && total==21,bust=total>21)
end
