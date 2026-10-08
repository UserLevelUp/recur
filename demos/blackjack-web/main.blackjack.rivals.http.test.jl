module BlackjackRivalsHttpTests
Base.Experimental.@compiler_options compile=min optimize=0 infer=false
using Test, HTTP, JSON3
include("main.blackjack.web.server.jl")
const S=BlackjackWebServer
# consumer: demo.blackjack.rivals.http.revision replay, isolation and seat validation
# consumer: demo.blackjack.rivals.table.projection hidden-information boundary
function req(store,action=nothing;cookie="",body=nothing,path=nothing)
    headers=["Host"=>"127.0.0.1:8798","Content-Type"=>"application/json"]
    !isempty(cookie) && push!(headers,"Cookie"=>cookie)
    target=path===nothing ? (action===nothing ? "/api/state" : "/api/action") : path
    r=S.handler(HTTP.Request(action===nothing ? "GET" : "POST",target,headers,body===nothing ? "" : JSON3.write(body));store=store)
    r
end
data(r)=JSON3.read(String(r.body),Dict{String,Any})
cookie(r)=first(split(HTTP.header(r,"Set-Cookie"),';'))
bundle(s,action;kwargs...)=merge(Dict("version"=>get(s,"protocol_version",2),"revision"=>s["revision"],"action"=>action),Dict(string(k)=>v for (k,v) in kwargs))
@testset "Rival HTTP seats and guarded mode changes" begin
    store=S.Store(); initial=req(store); c=cookie(initial); s=data(initial)
    @test s["schema"]=="blackjack-web-state-v2"
    for count in (1,2,0,2,1)
        r=req(store,"reset";cookie=c,body=bundle(s,"reset";money=500,rival_count=count))
        @test r.status==200
        s=data(r)
        @test s["schema"]==(count==0 ? "blackjack-web-state-v2" : "blackjack-web-state-v3")
        @test get(s,"rival_count",0)==count
        @test length(get(s,"rivals",[]))==count
    end
    before=deepcopy(s)
    for count in (-1,3,true,"2",nothing)
        r=req(store,"reset";cookie=c,body=bundle(s,"reset";rival_count=count))
        @test r.status==400
        @test data(req(store;cookie=c))==before
    end
    bad=bundle(s,"reset";rival_count=2);bad["revision"]-=1
    @test req(store,"reset";cookie=c,body=bad).status==409
    bad=bundle(s,"reset";rival_count=2);bad["version"]=2
    @test req(store,"reset";cookie=c,body=bad).status==426
    other=req(store)
    @test cookie(other)!=c && data(other)["schema"]=="blackjack-web-state-v2"
    key=last(split(c,'=';limit=2))
    g=S.BlackjackRivals.newgame(500,1)
    prefix=[10,2,4,9,11,13]
    order=vcat(prefix,[card for card in 1:52 if !(card in prefix)])
    g=S.BlackjackRivals.transition(g,Dict("version"=>3,"revision"=>0,"action"=>"deal","bet"=>10);order=order)
    store.games[key]=(g,time());s=data(req(store;cookie=c))
    @test s["phase"]=="playing" && s["dealer"][2]===nothing
    @test !haskey(s,"deck") && !haskey(s,"cursor")
    @test req(store,"reset";cookie=c,body=bundle(s,"reset";rival_count=2)).status==400
    @test data(req(store;cookie=c))==s
    command=bundle(s,"stand";hand_id=s["active_hand_id"])
    r=req(store,"stand";cookie=c,body=command)
    @test r.status==200 && data(r)["phase"]=="settled"
    settled=data(r)
    @test req(store,"stand";cookie=c,body=command).status==409
    @test data(req(store;cookie=c))==settled
    @test settled["escrow"]==0 && all(r->r["escrow"]==0,settled["rivals"])
    @test req(store;path="/main.blackjack.rivals.js").status==200
end
end
