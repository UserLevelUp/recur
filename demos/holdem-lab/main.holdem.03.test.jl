using Test, Random
# consumes: demo.holdem.deal independent position and uniqueness oracle
@testset "03 Cards and deal" begin
    implementation = joinpath(@__DIR__, "main.holdem.03.jl")
    @test isfile(implementation)
    if isfile(implementation)
        include(implementation)
        cards = deck().cards
        @test length(unique(cards)) == 52
        @test Set(c.rank for c in cards) == Set(2:14)
        @test Set(c.suit for c in cards) == Set(1:4)
        @test_throws ArgumentError Card(1, 1)
        @test_throws ArgumentError Card(14, 5)
        @test_throws ArgumentError deck(fill(Card(2, 1), 52))
        @test_throws ArgumentError deck(cards[1:51])
        shuffled = shuffle(MersenneTwister(42), cards)
        before = copy(shuffled)
        result = deal(deck(shuffled))
        @test shuffled == before
        @test result.holes == (shuffled[[2,4]], shuffled[[1,3]])
        @test result.board == shuffled[[6,7,8,10,12]]
        @test result.burns == shuffled[[5,9,11]]
        @test length(unique(vcat(result.holes..., result.board, result.burns, result.remaining))) == 52
        @test deal(deck(shuffle(MersenneTwister(42), cards))) == result
    end
end
