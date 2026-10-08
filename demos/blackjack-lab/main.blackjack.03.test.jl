# consumes: demo.blackjack.cards ace and natural rules
@testset "03 cards and scores" begin
    order=collect(1:52); d=B.deck(order); d[1]=52
    @test order==collect(1:52)
    @test B.score([1,13]) == (total=21,soft=true,natural=true,bust=false)
    @test B.score([1,14,9]).total==21
    @test B.score([1,14,9]).soft
    @test !B.score([1,14,9]).natural
    @test B.score([1,9,22]).total==19
    @test !B.score([1,9,22]).soft
    @test B.score([10,23,5]).bust
    @test B.score([1,6]).total==17 && B.score([1,6]).soft
    for bad in [Int[],[1,1],[0,1],[53,1],Any[true,2]]
        @test_throws ArgumentError B.score(bad)
    end
    @test_throws ArgumentError B.deck([1,2])
    @test_throws ArgumentError B.deck(fill(1,52))
    # Independent exhaustive two-card oracle (1326 physical hands).
    for a in 1:51,b in (a+1):52
        ra=mod(a-1,13)+1; rb=mod(b-1,13)+1
        va=ra==1 ? 11 : min(ra,10); vb=rb==1 ? 11 : min(rb,10)
        expected=va+vb; expected>21 && (expected-=10)
        s=B.score([a,b])
        @test s.total==expected
        @test s.natural == (expected==21)
    end
end
