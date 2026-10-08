# consumes: demo.blackjack.session rounds, stable identities and final ties
@testset "05 session history and leaders" begin
    t=B.TableConfig(players=2,rounds=3,seed=9)
    result=B.tournament(t)
    @test length(result.rounds)==3
    @test result.stop_reason=="round_limit"
    @test result==B.tournament(t)
    @test result.leaders==findall(==(maximum(result.balances)),result.balances)
    @test result.profit==result.balances.-100
    @test occursin("Final leaders",B.render(result))
    @test occursin("Round 1",B.render(result))
    @test occursin("Dealer",B.render(result))
    empty=B.tournament(B.TableConfig(players=2,bank=0))
    @test isempty(empty.rounds) && empty.leaders==[1,2]
    @test empty.stop_reason=="house_reserve"
    one=B.TableConfig(players=1,money=10,rounds=4)
    lose=B.tournament(one;orders=[ordered([9,1,7,10])])
    @test length(lose.rounds)==1 && lose.balances==[0]
    @test lose.stop_reason=="players_unfunded"
    @test_throws ArgumentError B.tournament(t;orders=[])
    for players in 1:6, seed in 1:30
        cfg=B.TableConfig(players=players,seed=seed,rounds=12)
        run=B.tournament(cfg); previous=fill(cfg.money,players)
        for round in run.rounds
            @test all(>=(0),round.result.balances) && round.result.bank>=0
            @test sum(round.result.balances)+round.result.bank==players*cfg.money+cfg.bank
            @test round.result.balances==previous.+round.result.deltas
            used=vcat(round.hands...,round.dealer_hand,round.remaining)
            @test sort(used)==collect(1:52)
            @test all(i->round.result.outcomes[i]==:sitting_out,findall(<(cfg.bet),previous))
            previous=round.result.balances
        end
    end
end
