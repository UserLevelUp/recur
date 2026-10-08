module BlackjackWebTests
Base.Experimental.@compiler_options compile=min optimize=0 infer=false
using Test, Random, JSON3, SHA
include("main.blackjack.web.engine.jl")
const B = BlackjackWeb
rig(cards) = vcat(cards, setdiff(1:52, cards))
cmd(g, action; kwargs...) = merge(Dict{String,Any}("version"=>2,"action"=>action,"revision"=>g.revision), action in ("hit","stand","double","split") ? Dict("hand_id"=>g.active_hand_id) : Dict(), Dict(string(k)=>v for (k,v) in kwargs))
deal(cards; money=100, bet=10) = B.transition(B.newgame(money), Dict("version"=>2,"action"=>"deal","revision"=>0,"bet"=>bet); order=rig(cards))

# consumer: demo.blackjack.web.play atomic actions, payouts and privacy
@testset "Blackjack website engine" begin
    g=B.newgame(100)
    @test g.balance==100 && g.bank==10000 && g.phase=="betting"
    @test B.public_state(g).allowed==["deal", "reset"]
    for money in (true, 0, 19, 10001, 20.5, "100")
        @test_throws ArgumentError B.newgame(money)
    end
    @test B.newgame(20).balance==20
    for bet in (true, 0, 1, 3, 102, 502, "10", 10.5)
        @test_throws ArgumentError B.transition(g,cmd(g,"deal";bet=bet))
        @test g.revision==0 && g.balance==100
    end
    @test_throws ArgumentError B.transition(g, cmd(g,"hit"))
    @test_throws ArgumentError B.transition(g, Dict("version"=>2,"action"=>"deal","revision"=>1,"bet"=>10))
    @test_throws ArgumentError B.transition(g, Dict("version"=>2,"action"=>"deal","revision"=>true,"bet"=>10))
    @test_throws ArgumentError B.transition(g,cmd(g,"deal";bet=10);order=[1,1])
    # Naturals and pushes; payout must include returning held wager.
    n=deal([1,9,13,7])
    @test (n.phase,n.balance,n.bank,n.result,n.profit)==("settled",115,9985,"blackjack",15)
    @test n.escrow==0 && n.stats["wins"]==1
    n=deal([9,1,7,13]); @test (n.result,n.balance)==("loss",90)
    n=deal([1,14,13,26]); @test (n.result,n.balance)==("push",100)
    n=deal([10,9,8,22]); n=B.transition(n,cmd(n,"stand"))
    @test (n.result,n.balance)==("push",100)
    # Hidden dealer card and shoe never cross the public boundary.
    g=deal([10,9,6,7,13])
    v=B.public_state(g)
    @test v.dealer==[9,nothing] && v.dealer_score===nothing
    @test v.hands[1].cards==[10,6] && v.hands[1].score.total==16
    @test !hasproperty(v,:deck) && !hasproperty(v,:cursor)
    @test v.balance==90 && v.hands[1].stake==10 && v.escrow==10
    @test "double" in v.allowed
    before=deepcopy(g)
    bust=B.transition(g,cmd(g,"hit"))
    @test bust.result=="loss" && bust.balance==90 && bust.hands[1].cards==[10,6,13]
    @test bust.dealer==[9,7] # no dealer draws after bust
    @test g.hands[1].cards==before.hands[1].cards && g.revision==before.revision
    @test B.public_state(bust).dealer==[9,7]
    @test_throws ArgumentError B.transition(bust,cmd(bust,"stand"))
    @test_throws ArgumentError B.transition(g,cmd(g,"reset";money=500))
    # Soft seventeen stands; a hit adjusts aces correctly and removes Double.
    soft=deal([10,1,8,6,5]); soft=B.transition(soft,cmd(soft,"stand"))
    @test soft.result=="win" && soft.dealer==[1,6]
    ace=deal([1,9,5,7,13]); ace=B.transition(ace,cmd(ace,"hit"))
    @test B.public_state(ace).hands[1].score.total==16 && ace.phase=="playing"
    @test !("double" in B.public_state(ace).allowed)
    @test_throws ArgumentError B.transition(ace,cmd(ace,"double"))
    # Hit to 21 automatically completes the hand.
    h=deal([10,9,6,7,5,8]); h=B.transition(h,cmd(h,"hit"))
    @test h.phase=="settled" && h.result=="win"
    # Double takes exactly one player card then resolves house.
    d=deal([5,9,6,7,13,8]); d=B.transition(d,cmd(d,"double"))
    @test (d.hands[1].stake,d.balance,d.bank,d.result)==(20,120,9980,"win")
    @test length(d.hands[1].cards)==3 && d.escrow==0
    poor=deal([5,9,6,7];money=20,bet=20)
    @test !("double" in B.public_state(poor).allowed)
    @test_throws ArgumentError B.transition(poor,cmd(poor,"double"))
    reserve=B.newgame(100); reserve.bank=10
    @test_throws ArgumentError B.transition(reserve,cmd(reserve,"deal";bet=10))
    reset=B.transition(d,cmd(d,"reset";money=500))
    @test reset.balance==500 && isempty(reset.history) && reset.revision==d.revision+1
    @test reset.stats["hands"]==0 && reset.phase=="betting"
    # Independent invariants across legal random actions, including history cap.
    rng=MersenneTwister(20261002)
    for session in 1:40
        state=B.newgame(500)
        for hand in 1:30
            state.balance<10 && break
            state=B.transition(state,cmd(state,"deal";bet=10);order=shuffle(rng,collect(1:52)))
            @test state.balance+state.escrow+state.bank==10500
            while state.phase=="playing"
                choices=filter(x->x in ("hit","stand","double"),B.public_state(state).allowed)
                state=B.transition(state,cmd(state,rand(rng,choices)))
                @test state.balance+state.escrow+state.bank==10500
                @test length(unique(vcat(state.hands[1].cards,state.dealer)))==length(state.hands[1].cards)+length(state.dealer)
            end
            @test state.balance>=0 && state.bank>=0 && state.escrow==0
            @test state.stats["hands"]==hand
            @test state.stats["wins"]+state.stats["losses"]+state.stats["pushes"]==hand
            @test length(state.history)==min(20,hand)
        end
    end
end

if get(ENV,"BLACKJACK_WEB_STAGE","all")!="engine"
    include("split/main.blackjack.web.split.runtime.test.jl")
    include("main.blackjack.web.http.test.jl")
    include("main.blackjack.rivals.test.jl")
    include("main.blackjack.rivals.http.test.jl")
    include("main.blackjack.web.reference.test.jl")
    # Optional demo-only suites: included by the selected blackjack-web wrapper.
    include("skill/main.blackjack.skill.strategy.test.jl")
    include("skill/main.blackjack.skill.statistics.test.jl")
    include("skill/main.blackjack.skill.replay.test.jl")
    include("skill/main.blackjack.skill.integration.test.jl")
end
end
