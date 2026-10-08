# consumes: demo.blackjack.table configuration and object caller
@testset "02 table object and caller" begin
    t=B.TableConfig()
    @test t.players == ["Player 1","Player 2","Player 3"]
    @test (t.money,t.bet,t.rounds,t.dealer,t.bank,t.seed)==(100,10,5,"Dealer",1000,42)
    @test occursin("3 players",B.describe(t))
    @test occursin("Ada",B.describe(B.TableConfig(players=1,dealer=" Ada ")))
    for kw in [(players=0,),(players=7,),(players=true,),(money=0,),(money=9,),
               (bet=3,),(bet=0,),(rounds=0,),(rounds=1001,),(dealer=" ",),
               (bank=-1,),(seed=-1,),(money=1.5,),(money=typemax(Int),)]
        @test_throws ArgumentError B.TableConfig(;kw...)
    end
end
