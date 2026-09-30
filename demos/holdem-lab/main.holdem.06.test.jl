using Test
isdefined(@__MODULE__, :Card) || include("main.holdem.03.jl")
isdefined(@__MODULE__, :evaluate) || include("main.holdem.04.jl")
# consumes: demo.holdem.settle explicit producer identity and join acceptance
@testset "06 Showdown coordination runtime" begin
    implementation = joinpath(@__DIR__, "main.holdem.06.jl")
    @test isfile(implementation)
    if isfile(implementation)
        include(implementation)
        d = deal(deck())
        request = (holes=d.holes,board=d.board,pot=11)
        result = coordinate(request)
        @test result.conserved && sum(result.settlement.payouts) == 11
        royal = [Card(r,4) for r in 10:14]
        tied = (holes=([Card(2,1),Card(3,1)],[Card(4,1),Card(5,1)]),board=royal,pot=11)
        @test coordinate(tied).settlement == (winners=[1,2],payouts=[5,6])
        @test_throws ArgumentError coordinate(merge(request,(pot=-1,)))
        @test_throws ArgumentError coordinate(merge(request,(holes=(d.holes[1],d.holes[1]),)))
        @test_throws ArgumentError coordinate(merge(request,(board=d.board[1:4],)))
        @test_throws ArgumentError coordinate(merge(request,(holes=([d.holes[1][1]],d.holes[2]),)))
        bad_label = (req,p) -> (player=3-p,rank=(0,2,0,0,0,0))
        @test_throws ArgumentError coordinate(request; ranker=bad_label)
        failing = (req,p) -> error("producer failed")
        @test_throws TaskFailedException coordinate(request; ranker=failing)
        # Hand-written asymmetric ranks are independent of the evaluator.
        fixed = (req,p) -> (player=p,rank=p == 1 ? (7,14,2,0,0,0) : (1,14,13,12,11,0))
        @test coordinate(request; ranker=fixed).settlement.payouts == [11,0]
    end
end
