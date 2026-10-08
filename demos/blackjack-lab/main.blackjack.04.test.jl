# consumes: demo.blackjack.round payoffs, rejection and conservation
ordered(prefix) = vcat(prefix,setdiff(collect(1:52),prefix))
@testset "04 round outcomes and chip conservation" begin
    t=B.TableConfig(players=1)
    cases=[([1,9,10,7],:blackjack,15),([9,1,7,10],:loss,-10),
           ([1,14,10,23],:push,0),([10,23,7,20],:push,0),
           ([10,9,6,8,23],:loss,-10),([10,9,7,20,23],:win,10)]
    for (prefix,outcome,delta) in cases
        balances=[100]; order=ordered(prefix); original=copy(order)
        prepared=B.prepare_round(t,balances,1000,order)
        result=B.settle_round(prepared)
        @test result.outcomes==[outcome]
        @test result.deltas==[delta]
        @test result.balances==[100+delta]
        @test result.bank==1000-delta
        @test sum(result.balances)+result.bank==1100
        @test balances==[100] && order==original
        @test result.winners == (delta>0 ? [1] : Int[])
        @test result.dealer_wins == (delta<0 ? [1] : Int[])
    end
    t2=B.TableConfig(players=2)
    result=B.settle_round(B.prepare_round(t2,[5,100],1000,ordered([1,9,10,7])))
    @test result.outcomes==[:sitting_out,:blackjack]
    @test result.balances==[5,115] && result.winners==[2]
    for (balances,bank) in [([100],14),([0],1000),([-1],1000),([100,100],1000),([100],-1)]
        before=copy(balances)
        @test_throws ArgumentError B.prepare_round(t,balances,bank,B.deck())
        @test before==balances
    end
    @test_throws ArgumentError B.prepare_round(t,[100],1000,fill(1,52))
    # A natural outranks a three-card 21; player bust loses against dealer bust.
    @test B.compare_hands([7,4,10],[1,23])==:loss
    @test B.compare_hands([10,23,5],[9,22,7])==:loss
    @test B.compare_hands([1,10],[7,4,23])==:blackjack
end
