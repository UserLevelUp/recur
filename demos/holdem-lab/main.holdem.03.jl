struct Card
    rank::Int
    suit::Int
    function Card(rank::Integer, suit::Integer)
        2 <= rank <= 14 && 1 <= suit <= 4 || throw(ArgumentError("invalid card"))
        new(rank, suit)
    end
end
Base.show(io::IO, c::Card) = print(io, "23456789TJQKA"[c.rank-1], "cdhs"[c.suit])
# produces: demo.holdem.deck validated copied permutation
function deck(order=[Card(r,s) for s in 1:4 for r in 2:14])
    length(order) == 52 && length(unique(order)) == 52 || throw(ArgumentError("expected 52 unique cards"))
    all(c -> c isa Card, order) || throw(ArgumentError("expected cards"))
    (cards=copy(order),)
end
# consumes: demo.holdem.deck order is an input, not a hidden shuffle
# produces: demo.holdem.deal full private deal; public view is another boundary
function deal(bundle)
    cards = deck(bundle.cards).cards
    (holes=(cards[[2,4]], cards[[1,3]]), board=cards[[6,7,8,10,12]],
     burns=cards[[5,9,11]], remaining=cards[13:end])
end
