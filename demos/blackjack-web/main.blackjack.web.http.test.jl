using HTTP, Sockets
include("main.blackjack.web.server.jl")
const S=BlackjackWebServer
decode(r)=JSON3.read(String(r.body))
cookieof(r)=first(split(HTTP.header(r,"Set-Cookie"),';'))
function request(store,method,path;cookie="",body="",origin="http://127.0.0.1:8791",content="application/json")
    headers=["Host"=>"127.0.0.1:8791","Origin"=>origin,"Content-Type"=>content,"Cookie"=>cookie]
    S.handler(HTTP.Request(method,path,headers,body);store=store)
end

# consumer: demo.blackjack.web.split.test.plan protocol, identity and replay
@testset "Split HTTP protocol and replay" begin
    store=S.Store(); initial=request(store,"GET","/api/state"); cookie=cookieof(initial)
    key=split(cookie,'=';limit=2)[2]
    game=S.BlackjackWeb.transition(S.BlackjackWeb.newgame(100),Dict("version"=>2,"action"=>"deal","revision"=>0,"bet"=>10);order=rig([8,9,21,7,2,3,5]))
    store.games[key]=(game,time())
    for version in (nothing,1,true,2.5,"2")
        c=Dict{String,Any}("action"=>"split","revision"=>1,"hand_id"=>1)
        version!==nothing && (c["version"]=version)
        r=request(store,"POST","/api/action";cookie,body=JSON3.write(c))
        @test r.status==426 && occursin("reload",decode(r).error)
        @test store.games[key][1].revision==1
    end
    splitcommand=JSON3.write((version=2,action="split",revision=1,hand_id=1))
    rs=fetch.([@async request(store,"POST","/api/action";cookie,body=splitcommand) for _ in 1:2])
    @test sort([r.status for r in rs])==[200,409]
    @test store.games[key][1].cursor==7 && store.games[key][1].balance==80
    wrong=JSON3.write((version=2,action="stand",revision=2,hand_id=2))
    @test request(store,"POST","/api/action";cookie,body=wrong).status==400
    first=request(store,"POST","/api/action";cookie,body=JSON3.write((version=2,action="stand",revision=2,hand_id=1)))
    @test first.status==200 && decode(first).active_hand_id==2
    @test decode(first).dealer==[9,nothing] && decode(first).dealer_score===nothing
    final=JSON3.write((version=2,action="stand",revision=3,hand_id=2))
    rs=fetch.([@async request(store,"POST","/api/action";cookie,body=final) for _ in 1:2])
    @test sort([r.status for r in rs])==[200,409]
    g=store.games[key][1]
    @test g.balance==80 && g.escrow==0 && g.stats["rounds"]==1 && g.stats["hands"]==2
    @test g.cursor==8 && length(g.history)==1
end

# consumer: demo.blackjack.web.http session, replay and projection contracts
@testset "Blackjack website HTTP" begin
    store=S.Store()
    a=request(store,"GET","/api/state"); b=request(store,"GET","/api/state")
    ca=cookieof(a); cb=cookieof(b)
    @test a.status==200 && b.status==200 && ca!=cb
    @test occursin("HttpOnly",HTTP.header(a,"Set-Cookie"))
    @test occursin("SameSite=Strict",HTTP.header(a,"Set-Cookie"))
    @test decode(a).balance==500
    action=JSON3.write((version=2,action="deal",revision=0,bet=10))
    played=request(store,"POST","/api/action";cookie=ca,body=action)
    @test played.status==200 && decode(played).revision==1
    @test request(store,"POST","/api/action";cookie=ca,body=action).status==409
    @test decode(request(store,"GET","/api/state";cookie=cb)).revision==0
    @test decode(request(store,"GET","/api/state";cookie=ca)).revision==1
    @test request(store,"POST","/api/action";body=action).status==409
    for (body,status) in [("{",400),("null",400),("[]",400),("1",400),
                           (JSON3.write((version=2,action="deal",revision=0,bet=3)),400),
                           (JSON3.write((version=2,action="deal",revision=true,bet=10)),400),
                           (JSON3.write((version=2,action="deal",revision=0,bet=10,deck=[1,2])),400)]
        @test request(store,"POST","/api/action";cookie=cb,body=body).status==status
        @test decode(request(store,"GET","/api/state";cookie=cb)).revision==0
    end
    @test request(store,"POST","/api/action";cookie=cb,body=action,origin="https://outside.invalid").status==403
    @test request(store,"POST","/api/action";cookie=cb,body=action,content="text/plain").status==415
    @test request(store,"POST","/api/action";cookie=cb,body=repeat(" ",4097)).status==413
    @test request(store,"GET","/api/action").status==405
    @test request(store,"POST","/api/state").status==405
    @test request(store,"GET","/../main.blackjack.web.engine.jl").status==404
    @test request(store,"GET","/main.blackjack.web.engine.jl").status==404
    @test request(store,"GET","/.recur/config.toml").status==404
    @test request(store,"GET","/api/state";origin="null").status==403
    @test S.handler(HTTP.Request("GET","/api/state",["Host"=>"outside.invalid"]);store=store).status==403
    # A known private hand lets the HTTP privacy test be independent of randomness.
    key=split(cb,'=';limit=2)[2]
    game=S.BlackjackWeb.transition(S.BlackjackWeb.newgame(),Dict("version"=>2,"action"=>"deal","revision"=>0,"bet"=>10);order=rig([10,9,6,7,13]))
    store.games[key]=(game,time())
    view=decode(request(store,"GET","/api/state";cookie=cb))
    @test view.dealer==[9,nothing] && view.dealer_score===nothing
    @test !haskey(view,:deck) && !haskey(view,:cursor)
    hit=JSON3.write((version=2,action="hit",revision=1,hand_id=1))
    results=fetch.([@async request(store,"POST","/api/action";cookie=cb,body=hit) for _ in 1:2])
    @test sort([r.status for r in results])==[200,409]
    @test decode(request(store,"GET","/api/state";cookie=cb)).stats.hands==1
    store.games[key]=(game,time()-7201)
    @test cookieof(request(store,"GET","/api/state";cookie=cb))!=cb
    full=S.Store()
    for i in 1:128; full.games[string(i)]=(S.BlackjackWeb.newgame(),time()); end
    @test request(full,"GET","/api/state").status==503
    # Exercise real loopback sockets, not just handler calls.
    server=S.start(0)
    try
        port=Int(getsockname(server.listener.server)[2])
        reply=HTTP.get("http://127.0.0.1:$port/api/state")
        @test reply.status==200 && decode(reply).phase=="betting"
        cookie=cookieof(reply)
        reply=HTTP.post("http://127.0.0.1:$port/api/action",["Content-Type"=>"application/json","Cookie"=>cookie],action)
        @test reply.status==200 && decode(reply).revision==1
    finally
        close(server)
    end
end

if get(ENV,"BLACKJACK_WEB_STAGE","all")!="http"
    @testset "Blackjack website assets" begin
        store=S.Store()
        for (path,mime) in [("/","text/html"),("/main.blackjack.web.css","text/css"),("/main.blackjack.web.js","application/javascript")]
            response=request(store,"GET",path)
            @test response.status==200
            @test startswith(HTTP.header(response,"Content-Type"),mime)
            @test length(response.body)>100
            @test occursin("default-src 'self'",HTTP.header(response,"Content-Security-Policy"))
        end
        html=read(joinpath(@__DIR__,"index.html"),String)
        @test occursin("aria-live",html) && occursin("<dialog",html)
        css=read(joinpath(@__DIR__,"main.blackjack.web.css"),String)
        @test occursin("prefers-reduced-motion",css) && occursin(":focus-visible",css)
    end
end
