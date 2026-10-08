module BlackjackSplitTests
Base.Experimental.@compiler_options compile=min optimize=0 infer=false
using Test, Random, JSON3
include("../main.blackjack.web.engine.jl")
const B=BlackjackWeb
rig(cards)=vcat(cards,setdiff(1:52,cards))
function cmd(g,action;kwargs...)
    c=Dict{String,Any}("version"=>2,"action"=>action,"revision"=>g.revision)
    action in ("hit","stand","double","split") && (c["hand_id"]=g.active_hand_id)
    merge!(c,Dict(string(k)=>v for (k,v) in kwargs))
end
deal(cards;money=100,bet=10)=B.transition(B.newgame(money),cmd(B.newgame(money),"deal";bet);order=rig(cards))
act(g,a;kw...)=B.transition(g,cmd(g,a;kw...))
function rejected(g,a;kw...)
    before=JSON3.write(g)
    @test_throws ArgumentError act(g,a;kw...)
    @test JSON3.write(g)==before
end
# consumer: demo.blackjack.web.split.contract acceptance against actual Julia
# publish: demo.blackjack.web.split.runtime observed behavior, not model execution
@testset "Split runtime acceptance" begin
    supported=hasproperty(B.newgame(),:hands)
    @test supported
    if supported
        g=deal([8,9,21,7,2,3,5])
        @test "split" in B.allowed(g)
        @test B.public_state(g).split.extra_stake==10
        for cards in ([10,9,26,7],[8,9,5,7,2])
            n=deal(cards); length(cards)==5 && (n=act(n,"hit"))
            rejected(n,"split")
        end
        n=deal([8,9,21,7];money=20,bet=20); rejected(n,"split")
        n=deepcopy(g); n.bank=19; rejected(n,"split")
        s=act(g,"split")
        @test [h.cards for h in s.hands]==[[8,2],[21,3]]
        @test [h.id for h in s.hands]==[1,2]
        @test (s.balance,s.escrow,s.bank,s.cursor,s.revision)==(80,20,10000,7,2)
        @test s.dealer==[9,7] && s.active_hand_id==1
        @test g.hands[1].cards==[8,21] && g.balance==90
        rejected(s,"split"); rejected(s,"stand";hand_id=2)
        rejected(s,"stand";hand_id=true); rejected(s,"stand";hand_id=1.0)
        firstdone=act(s,"stand")
        @test firstdone.active_hand_id==2 && firstdone.cursor==s.cursor
        @test firstdone.balance==80 && firstdone.escrow==20
        v=B.public_state(firstdone)
        @test v.dealer==[9,nothing] && v.dealer_score===nothing
        @test !hasproperty(v,:deck) && !hasproperty(v,:cursor)
        v.hands[1].cards[1]=52
        @test firstdone.hands[1].cards[1]==8
        done=act(firstdone,"stand")
        @test done.dealer==[9,7,5] && done.cursor==8 && done.active_hand_id===nothing
        @test (done.balance,done.escrow,done.bank)==(80,0,10020)
        @test done.stats==Dict("rounds"=>1,"hands"=>2,"wins"=>0,"losses"=>2,"pushes"=>0)
        @test length(done.history)==1 && length(done.history[1].hands)==2
        rejected(done,"stand")
        pair=act(deal([8,9,21,7,34,47]),"split"); rejected(pair,"split")
        # Both bust: no dealer draw, even after first bust advances the turn.
        bust=act(deal([8,9,21,7,10,11,12,13]),"split")
        bust=act(bust,"hit")
        @test bust.active_hand_id==2 && bust.phase=="playing" && bust.dealer==[9,7]
        bust=act(bust,"hit")
        @test bust.dealer==[9,7] && bust.cursor==9 && bust.balance==80
        # Split aces auto-stand with one extra card. 21 pays 1:1, never 3:2.
        aces=act(deal([1,10,14,23,13,9]),"split")
        @test aces.phase=="settled" && aces.balance==110
        @test [h.profit for h in aces.hands]==[10,0]
        @test all(h->length(h.cards)==2,aces.hands)
        @test !B.public_state(aces).hands[1].score.natural
        rejected(aces,"hit"); rejected(aces,"double")
        # Non-ace split 21 skips completed hand and preserves the remaining turn.
        skip=act(deal([10,9,23,7,1,2,5]),"split")
        @test skip.active_hand_id==2 && skip.hands[1].status=="stood"
        # Double is independent; aggregate reserve and exact mixed settlement.
        d=act(deal([8,10,21,9,3,2,13]),"split")
        low=deepcopy(d);low.bank=29;rejected(low,"double")
        poor=deepcopy(d);poor.balance=9;rejected(poor,"double")
        d=act(d,"double")
        @test [h.stake for h in d.hands]==[20,10] && d.escrow==30 && d.balance==70
        @test d.active_hand_id==2 && length(d.hands[1].cards)==3
        d=act(d,"stand")
        @test [h.profit for h in d.hands]==[20,-10]
        @test (d.balance,d.bank,d.profit,d.escrow)==(110,9990,10,0)
        tie=act(deal([10,9,23,22,13,8]),"split")
        tie=act(act(tie,"stand"),"stand")
        @test [h.profit for h in tie.hands]==[10,0] && tie.balance==110
        # No split before original dealer natural is resolved.
        natural=deal([8,1,21,13]); @test natural.phase=="settled"
        rejected(natural,"split")
        @test B.public_state(B.newgame()).schema=="blackjack-web-state-v2"
        rejected(g,"stand";version=1); rejected(g,"stand";version=true)
        rng=MersenneTwister(20261003); splitcount=0
        for session in 1:30
            p=B.newgame(500)
            for round in 1:30
                p.balance<10 && break
                p=B.transition(p,cmd(p,"deal";bet=10);order=shuffle(rng,collect(1:52)))
                while p.phase=="playing"
                    actions=B.allowed(p)
                    a="split" in actions ? "split" : rand(rng,actions)
                    a=="split" && (splitcount+=1)
                    p=act(p,a)
                    cards=vcat([h.cards for h in p.hands]...,p.dealer)
                    @test length(unique(cards))==length(cards)
                    @test p.balance+p.escrow+p.bank==10500
                    @test p.phase!="playing" || p.escrow==sum(h.stake for h in p.hands)
                    @test 1<=length(p.hands)<=2 && p.balance>=0 && p.bank>=0
                end
                @test p.stats["rounds"]==round && length(p.history)==min(round,20)
                @test sum(p.stats[k] for k in ("wins","losses","pushes"))==p.stats["hands"]
            end
        end
        @test splitcount>20
    end
end
end
