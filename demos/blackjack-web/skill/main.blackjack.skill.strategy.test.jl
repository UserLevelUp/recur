module StrategyContractTests
using Test
include("main.blackjack.skill.strategy.jl")
const S=BlackjackStrategy
input(total;soft=false,natural=false,bust=false,dealer=6,legal=["hit","stand"],stake=10)=Dict{String,Any}("total"=>total,"soft"=>soft,"natural"=>natural,"bust"=>bust,"dealer_upcard"=>dealer,"legal_actions"=>legal,"affordable_stake"=>stake,"rules_id"=>"s17-3to2-one-split-v1")
# register: demo.blackjack.skill.strategy golden public-information policy tests
@testset "Frozen rival policy golden cases" begin
    ids=[p.id for p in S.policies()]
    @test ids==["stand17-v1","dealer-aware-v1"]
    for (total,soft,dealer,expected) in [(12,false,4,"stand"),(12,false,3,"hit"),(16,false,6,"stand"),(16,false,10,"hit"),(17,false,1,"stand"),(18,true,6,"stand"),(18,true,9,"hit"),(18,true,1,"hit"),(19,true,1,"stand"),(11,false,6,"hit")]
        @test S.decision("dealer-aware-v1",input(total;soft=soft,dealer=dealer))==expected
    end
    @test S.decision("stand17-v1",input(16))=="hit"
    @test S.decision("stand17-v1",input(17;soft=true))=="stand"
    @test S.decision("dealer-aware-v1",input(21;natural=true))=="stand"
    @test S.decision("dealer-aware-v1",input(25;bust=true))=="stand"
    @test S.decision("dealer-aware-v1",input(16;legal=["stand"]))=="stand"
    @test S.decision("stand17-v1",input(19;legal=["hit"]))=="hit"
    @test_throws ArgumentError S.decision("unknown",input(16))
    for (key,value) in [("deck",collect(1:52)),("hole_card",13),("total",true),("soft",1),("dealer_upcard",0),("legal_actions",["double"]),("legal_actions",["hit","hit"]),("rules_id","other"),("affordable_stake",-1)]
        bad=merge(input(16),Dict(key=>value)); before=deepcopy(bad)
        @test_throws ArgumentError S.decision("dealer-aware-v1",bad)
        @test bad==before
    end
    bad=input(16);delete!(bad,"dealer_upcard")
    @test_throws ArgumentError S.decision("dealer-aware-v1",bad)
    for total in 4:31, soft in (false,true), dealer in 1:52
        @test S.decision("dealer-aware-v1",input(total;soft=soft,dealer=dealer)) in ("hit","stand")
    end
end
end
