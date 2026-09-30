# Optional slow independent combinatorial oracle for all C(52,5) poker hands.
using Test
include("main.holdem.jl")
function histogram()
    cards = HoldemLab.deck().cards
    counts = zeros(Int,9)
    for a in 1:48, b in a+1:49, c in b+1:50, d in c+1:51, e in d+1:52
        rank = HoldemLab.rank5(cards[[a,b,c,d,e]])
        counts[rank[1]+1] += 1
    end
    counts
end
@testset "All 2,598,960 five-card hands" begin
    observed = histogram()
    # Counts follow rank/suit combinatorics, not another call to our evaluator.
    @test observed == [1302540,1098240,123552,54912,10200,5108,3744,624,40]
    @test sum(observed) == binomial(52,5)
    println("category histogram: ",observed)
end
