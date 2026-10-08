module SkillIntegrationTests
Base.Experimental.@compiler_options compile=min optimize=0 infer=false
using Test,HTTP,JSON3,Random
include("../main.blackjack.web.server.jl")
const S=BlackjackWebServer;const P=S.BlackjackReplay;const R=S.BlackjackRivals;const B=S.BlackjackWeb
deck(prefix)=vcat(prefix,[c for c in 1:52 if !(c in prefix)])
cmd(g,action;kwargs...)=merge(Dict{String,Any}("version"=>g isa R.Table ? 3 : 2,"revision"=>g.revision,"action"=>action),action in ("hit","stand","double","split") ? Dict("hand_id"=>g.active_hand_id) : Dict(),Dict(string(k)=>v for (k,v) in kwargs))
function request(store,path="/api/state";cookie="",method="GET",body=nothing)
    headers=["Host"=>"127.0.0.1:8797","Content-Type"=>"application/json"]
    isempty(cookie)||push!(headers,"Cookie"=>cookie)
    S.handler(HTTP.Request(method,path,headers,body===nothing ? "" : JSON3.write(body));store=store)
end
data(r)=JSON3.read(String(r.body),Dict{String,Any})
cookie(r)=first(split(HTTP.header(r,"Set-Cookie"),';'))
function finish_http(store,c,s)
    while s["phase"]=="playing"
        r=request(store,"/api/action";cookie=c,method="POST",body=Dict("version"=>get(s,"protocol_version",2),"revision"=>s["revision"],"action"=>"stand","hand_id"=>s["active_hand_id"]))
        @test r.status==200
        s=data(r)
    end
    s
end
function private_keys(x)
    if x isa AbstractDict
        any(k->String(k) in ("deck","cursor","replay","hole_card","steps"),keys(x)) || any(private_keys,values(x))
    elseif x isa AbstractVector
        any(private_keys,x)
    else
        false
    end
end
# register: demo.blackjack.skill.integration real policy call, private replay and statistics reconciliation
@testset "Integrated frozen modules and engine invariants" begin
    for count in 0:2, round in 1:20
        engine=count==0 ? B : R
        policies=count==0 ? nothing : [i==1 ? "stand17-v1" : "dealer-aware-v1" for i in 1:count]
        rec=P.new_recording(engine;money=500,rival_count=count,policies=policies,initial_revision=7)
        P.record!(rec,cmd(rec.game,"deal";bet=10);order=shuffle(MersenneTwister(500+round+count),collect(1:52)))
        while rec.game.phase=="playing"
            s=engine.public_state(rec.game)
            @test s.dealer[2]===nothing
            @test !private_keys(JSON3.read(JSON3.write(s),Dict{String,Any}))
            # Exercise human doubling when legal; split semantics retain existing suites.
            action="double" in s.allowed ? "double" : "stand"
            P.record!(rec,cmd(rec.game,action))
        end
        g=rec.game
        for seat in (count==0 ? (g,) : (g,g.rivals...))
            @test seat.stats["net_chips"]==seat.balance+seat.escrow-seat.starting
            @test seat.stats["rounds"]==seat.stats["settled_rounds"]==1
            @test seat.stats["resolved_wager"]==sum(h.stake for h in seat.hands)
            @test seat.stats["wins"]+seat.stats["losses"]+seat.stats["pushes"]==seat.stats["hands"]
        end
        @test P.state_hash(P.playback(engine,P.export_replay(rec)))==P.state_hash(g)
    end
    rec=P.new_recording(B;money=100)
    P.record!(rec,cmd(rec.game,"deal";bet=10);order=deck([8,9,21,7,2,3,4,5]))
    P.record!(rec,cmd(rec.game,"split"))
    while rec.game.phase=="playing";P.record!(rec,cmd(rec.game,"stand"));end
    @test rec.game.stats["hands"]==2 && rec.game.stats["rounds"]==1
    @test rec.game.stats["resolved_wager"]==20
    @test P.state_hash(P.playback(B,P.export_replay(rec)))==P.state_hash(rec.game)
    natural=P.new_recording(B;money=100)
    P.record!(natural,cmd(natural.game,"deal";bet=10);order=deck([1,14,13,26]))
    @test natural.game.stats["naturals"]==1 && natural.game.stats["pushes"]==1
    g=R.newgame(100,2;policies=["stand17-v1","dealer-aware-v1"])
    g=R.transition(g,cmd(g,"deal";bet=10);order=deck([10,2,3,6,9,11,12,13]))
    input=R.policy_input(g,g.rivals[2].hands[1],g.rivals[2].balance)
    changed=deepcopy(g);changed.dealer[2]=52;reverse!(changed.deck);changed.cursor=52
    @test R.policy_input(changed,changed.rivals[2].hands[1],changed.rivals[2].balance)==input
    @test P.state_hash(changed)!=P.state_hash(g)
    @test R.public_state(g).rivals[2].policy_id=="dealer-aware-v1"
    @test_throws ArgumentError R.newgame(100,2;policies=["unknown","stand17-v1"])
    @test_throws ArgumentError R.newgame(100,1;policies=["stand17-v1","dealer-aware-v1"])
    near=P.new_recording(B;initial_revision=typemax(Int)-40)
    before=P.state_hash(near.game)
    @test_throws ArgumentError P.record!(near,cmd(near.game,"deal";bet=10))
    @test P.state_hash(near.game)==before && isempty(near.steps)
    # Seed private count state solely to probe admission at the finite limit.
    bounded=P.new_recording(B)
    P.record!(bounded,cmd(bounded.game,"deal";bet=10);order=deck([1,9,13,7]))
    bounded.steps=[deepcopy(bounded.steps[1]) for _ in 1:1949]
    for step in bounded.steps;step["command"]["action"]="stand";end
    before=P.state_hash(bounded.game)
    @test_throws ArgumentError P.record!(bounded,cmd(bounded.game,"deal";bet=10))
    @test P.state_hash(bounded.game)==before && length(bounded.steps)==1949
    detached=P.new_recording(B);detached.game.balance-=1
    @test_throws ArgumentError P.record!(detached,cmd(detached.game,"deal";bet=10))
end

# register: demo.blackjack.skill.http phase guards, session isolation and nonmutating playback
@testset "HTTP session reports and private replay" begin
    store=S.Store();first=request(store);c=cookie(first);s=data(first)
    @test request(store,"/api/replay").status==409
    @test request(store,"/api/session-report";cookie=c).status==409
    @test request(store,"/api/replay";cookie=c).status==409
    other=request(store);othercookie=cookie(other);otherstate=data(other)
    for count in (0,1,2,0)
        prior=s["revision"]
        policy=count==2 ? ["stand17-v1","dealer-aware-v1"] : count==1 ? ["dealer-aware-v1"] : String[]
        reset=Dict("version"=>get(s,"protocol_version",2),"revision"=>s["revision"],"action"=>"reset","money"=>500,"rival_count"=>count,"rival_policies"=>policy)
        reset_response=request(store,"/api/action";cookie=c,method="POST",body=reset)
        @test reset_response.status==200
        s=data(reset_response);@test s["revision"]==prior+1
        @test s["schema"]==(count==0 ? "blackjack-web-state-v2" : "blackjack-web-state-v3")
        @test s["session_statistics"]["roi"]===nothing
        expected=reset["version"]==get(s,"protocol_version",2) ? 409 : 426
        @test request(store,"/api/action";cookie=c,method="POST",body=reset).status==expected
        stale=merge(reset,Dict("version"=>get(s,"protocol_version",2)))
        @test request(store,"/api/action";cookie=c,method="POST",body=stale).status==409
        deal=Dict("version"=>get(s,"protocol_version",2),"revision"=>s["revision"],"action"=>"deal","bet"=>10)
        dealt=request(store,"/api/action";cookie=c,method="POST",body=deal)
        @test dealt.status==200;s=data(dealt)
        if s["phase"]=="playing"
            @test request(store,"/api/replay";cookie=c).status==409
            @test request(store,"/api/session-report";cookie=c).status==409
        end
        s=finish_http(store,c,s)
        exported=request(store,"/api/replay";cookie=c);@test exported.status==200
        envelope=data(exported);@test envelope["initial"]["revision"]==prior+1
        report=request(store,"/api/session-report";cookie=c);@test report.status==200
        totals=data(report)
        @test totals["schema"]=="blackjack-session-report-v1" && length(totals["seats"])==1+count
        @test !private_keys(totals)
        @test !haskey(totals,"dealer") && !haskey(totals,"history")
        @test all(seat->seat["net"]==seat["statistics"]["net_chips"],totals["seats"])
        played=request(store,"/api/replay";cookie=c,method="POST",body=Dict("replay"=>envelope))
        @test played.status==200 && data(played)["schema"]=="blackjack-replay-result-v1"
        @test !private_keys(data(played))
        @test data(request(store;cookie=c))==s
        @test data(request(store;cookie=othercookie))==otherstate
        @test request(store,"/api/replay";cookie=othercookie).status==409
        tampered=deepcopy(envelope);tampered["final_hash"]=repeat("0",64)
        @test request(store,"/api/replay";cookie=c,method="POST",body=Dict("replay"=>tampered)).status==400
        @test data(request(store;cookie=c))==s
    end
    @test request(store,"/api/replay";cookie=c,method="POST",body=Dict("replay"=>Dict(),"extra"=>1)).status==400
    # Bounds reject before JSON parsing and never replace a live game.
    big=HTTP.Request("POST","/api/replay",["Host"=>"127.0.0.1:8797","Content-Type"=>"application/json","Cookie"=>c],repeat("x",1024*1024+1))
    @test S.handler(big;store=store).status==413
    @test data(request(store;cookie=c))==s
end
end
