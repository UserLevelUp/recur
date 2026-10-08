module StatisticsContractTests
using Test
include("main.blackjack.skill.statistics.jl")
const S=BlackjackStatistics
hand(result,stake,profit;natural=false,bust=false)=(result=result,stake=stake,profit=profit,natural=natural,bust=bust)
# register: demo.blackjack.skill.statistics split-hand denominators and zero-wager ROI
@testset "Frozen session statistics" begin
    s=S.empty_stats();z=S.summary(s)
    @test z.roi===nothing && z.sample_hands==0 && z.sample_rounds==0
    S.start_round!(s);S.settle_round!(s,[hand("win",10,10),hand("loss",20,-20;bust=true)])
    @test s["started_rounds"]==1 && s["settled_rounds"]==1 && s["hands"]==2
    @test s["resolved_wager"]==30 && s["net_chips"]==-10 && s["busts"]==1
    @test S.summary(s).roi≈-1/3
    S.start_round!(s;active=false);S.settle_round!(s,[])
    @test s["started_rounds"]==2 && s["sitting_out_rounds"]==1 && s["rounds"]==1
    S.start_round!(s);S.settle_round!(s,[hand("blackjack",10,15;natural=true)])
    @test s["wins"]==2 && s["naturals"]==1 && s["net_chips"]==5
    S.start_round!(s);S.settle_round!(s,[hand("push",10,0)])
    @test s["pushes"]==1 && s["resolved_wager"]==50 && S.summary(s).roi≈0.1
    for bad in [hand("bogus",10,0),hand("win",true,10),hand("loss",-10,-10),hand("push",10,0;natural=1)]
        before=copy(s)
        @test_throws ArgumentError S.settle_round!(s,[hand("win",10,10),bad])
        @test s==before
    end
    detached=S.summary(s);s["hands"]+=1
    @test detached.sample_hands==4
    for bad in [hand("win",big(typemax(Int))+1,10),hand("win",10,big(typemax(Int))+1)]
        before=copy(s)
        @test_throws ArgumentError S.settle_round!(s,[hand("win",10,10),bad])
        @test s==before
    end
    for key in ("resolved_wager","net_chips","hands")
        huge=S.empty_stats();huge[key]=typemax(Int);before=copy(huge)
        @test_throws ArgumentError S.settle_round!(huge,[hand("win",10,10)])
        @test huge==before
    end
    huge=S.empty_stats();huge["active_rounds"]=typemax(Int);before=copy(huge)
    @test_throws ArgumentError S.start_round!(huge)
    @test huge==before
end
end
