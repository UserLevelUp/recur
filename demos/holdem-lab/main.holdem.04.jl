# Internal five-card scorer; the public boundary validates seven unique cards.
function rank5(hand)
    ranks = sort([c.rank for c in hand]; rev=true)
    unique_ranks = unique(ranks)
    straight = length(unique_ranks) == 5 && first(ranks)-last(ranks) == 4 ? first(ranks) :
               unique_ranks == [14,5,4,3,2] ? 5 : 0
    flush = all(c -> c.suit == hand[1].suit, hand)
    counts = Dict(r => count(==(r), ranks) for r in unique_ranks)
    groups = sort(collect(keys(counts)); by=r -> (counts[r],r), rev=true)
    sizes = [counts[r] for r in groups]
    category, kickers = if flush && straight > 0
        (8, [straight])
    elseif sizes == [4,1]
        (7, groups)
    elseif sizes == [3,2]
        (6, groups)
    elseif flush
        (5, ranks)
    elseif straight > 0
        (4, [straight])
    elseif sizes == [3,1,1]
        (3, groups)
    elseif sizes == [2,2,1]
        (2, groups)
    elseif sizes == [2,1,1,1]
        (1, groups)
    else
        (0, ranks)
    end
    Tuple(vcat(category, kickers, zeros(Int,5-length(kickers))))
end
# consumes: demo.holdem.deal exactly seven unique cards
# produces: demo.holdem.rank lexicographic category and complete kickers
function evaluate(bundle)
    hand = bundle.cards
    length(hand) == 7 && length(unique(hand)) == 7 || throw(ArgumentError("expected seven unique cards"))
    maximum(rank5(hand[[a,b,c,d,e]]) for a in 1:3 for b in a+1:4 for c in b+1:5 for d in c+1:6 for e in d+1:7) |> r -> (rank=r,)
end
winners(bundle) = (winners=bundle.left == bundle.right ? [1,2] : bundle.left > bundle.right ? [1] : [2],)
