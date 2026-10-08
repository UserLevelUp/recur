# consumes: demo.blackjack.coordination joined producers and audit
@testset "06 explicit coordination" begin
    for n in 1:6,seed in 1:8
        cfg=B.TableConfig(players=n,seed=seed)
        played=B.prepare_round(cfg,fill(100,n),1000,shuffle(B.MersenneTwister(seed),collect(1:52)))
        @test B.coordinate(played).result==B.settle_round(played)
        @test B.coordinate(played).conserved
    end
    p=B.prepare_round(B.TableConfig(players=1),[100],1000,ordered([1,9,10,7]))
    @test_throws ArgumentError B.coordinate(p;player_worker=x->(producer="house",values=[]))
    @test_throws ArgumentError B.coordinate(p;house_worker=x->(producer="house",values=[]))
    @test_throws TaskFailedException B.coordinate(p;player_worker=x->error("producer failed"))
    before=deepcopy(p.hands)
    @test_throws ArgumentError B.coordinate(p;player_worker=x->begin x.hands[1]=[7,8]; B.player_receipt(x) end)
    @test p.hands==before
end
