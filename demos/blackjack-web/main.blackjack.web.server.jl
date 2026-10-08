module BlackjackWebServer
using HTTP, JSON3, Sockets, UUIDs
include("main.blackjack.web.engine.jl")
include("main.blackjack.rivals.engine.jl")
include("skill/main.blackjack.skill.replay.jl")

const TableGame = Union{BlackjackWeb.Game,BlackjackRivals.Table}
table_view(game) = game isa BlackjackRivals.Table ? BlackjackRivals.public_state(game) : BlackjackWeb.public_state(game)
table_version(game) = game isa BlackjackRivals.Table ? 3 : 2
# consumer: demo.blackjack.rivals.table.projection public snapshots only
# publish: demo.blackjack.rivals.http.revision version and revision guarded mode switches
function table_transition(game, command)
    if get(command,"action",nothing)=="reset"
        "reset" in table_view(game).allowed || throw(ArgumentError("Finish this round before changing seats"))
        all(k->k in ("version","revision","action","money","rival_count","rival_policies"),keys(command)) || throw(ArgumentError("Unknown command field"))
        current = game isa BlackjackRivals.Table ? table_view(game).rival_count : 0
        count = BlackjackWeb.integer(get(command,"rival_count",current),0,2,"Rival count")
        money = get(command,"money",500)
        fresh = count==0 ? BlackjackWeb.newgame(money) : BlackjackRivals.newgame(money,count;policies=get(command,"rival_policies",nothing))
        fresh.revision=game.revision+1
        return fresh
    end
    game isa BlackjackRivals.Table ? BlackjackRivals.transition(game,command) : BlackjackWeb.transition(game,command)
end

mutable struct Store
    games::Dict{String,Tuple{TableGame,Float64}}
    mutex::ReentrantLock
    recorders::Dict{String,Any}
end
Store()=Store(Dict{String,Tuple{TableGame,Float64}}(),ReentrantLock(),Dict{String,Any}())
include("skill/main.blackjack.skill.http.jl")
const DEFAULT_STORE=Store()
const ASSETS=Dict("/"=>("index.html","text/html"),
    "/main.blackjack.web.css"=>("main.blackjack.web.css","text/css"),
    "/main.blackjack.web.js"=>("main.blackjack.web.js","application/javascript"),
    "/main.blackjack.rivals.js"=>("main.blackjack.rivals.js","application/javascript"))
const POLICY="default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; connect-src 'self'; frame-ancestors 'none'; base-uri 'none'; form-action 'self'"
function response(status,body;mime="application/json",cookie=nothing)
    headers=["Content-Type"=>mime*"; charset=utf-8","Cache-Control"=>"no-store",
        "X-Content-Type-Options"=>"nosniff","Content-Security-Policy"=>POLICY,
        "Referrer-Policy"=>"same-origin"]
    cookie!==nothing && push!(headers,"Set-Cookie"=>"blackjack=$cookie; Path=/; HttpOnly; SameSite=Strict; Max-Age=7200")
    HTTP.Response(status,headers,mime=="application/json" ? JSON3.write(body) : body)
end
failure(code,msg)=response(code,(error=msg,))

function session_key(request)
    for part in split(HTTP.header(request,"Cookie"),';')
        pair=split(strip(part),'=';limit=2)
        length(pair)==2 && pair[1]=="blackjack" && occursin(r"^[a-f0-9-]{36}$",pair[2]) && return pair[2]
    end
    ""
end

# consumer: demo.blackjack.web.http untrusted request and server session
# publish: demo.blackjack.web.http public response; no Julia source or deck
function handler(request::HTTP.Request;store=DEFAULT_STORE)
    host=HTTP.header(request,"Host")
    occursin(r"^(127\.0\.0\.1|localhost)(:[0-9]+)?$",host) || return failure(403,"Localhost only")
    origin=HTTP.header(request,"Origin")
    !isempty(origin) && origin!="http://"*host && return failure(403,"Use this table's own page")
    path=HTTP.URI(request.target).path
    if haskey(ASSETS,path)
        request.method=="GET" || return failure(405,"GET required")
        file,mime=ASSETS[path]
        isfile(joinpath(@__DIR__,file)) || return failure(404,"Asset unavailable")
        return response(200,read(joinpath(@__DIR__,file));mime=mime)
    end
    path in ("/api/state","/api/action","/api/session-report","/api/replay") || return failure(404,"Not found")
    expected=path=="/api/action" ? "POST" : path=="/api/replay" && request.method=="POST" ? "POST" : "GET"
    request.method==expected || return failure(405,"$expected required")
    if expected=="POST"
        first(split(lowercase(HTTP.header(request,"Content-Type")),';'))=="application/json" || return failure(415,"JSON required")
        length(request.body)<=(path=="/api/replay" ? 1024*1024 : 4096) || return failure(413,"Request too large")
    end
    lock(store.mutex) do
        now=time()
        filter!(pair->now-pair.second[2]<7200,store.games)
        filter!(pair->haskey(store.games,pair.first),store.recorders)
        key=session_key(request)
        if !haskey(store.games,key)
            (expected=="POST" || path in ("/api/replay","/api/session-report")) && return failure(409,"Session expired; reload your table")
            length(store.games)<128 || return failure(503,"Table capacity reached; try again later")
            key=string(uuid4())
            rec=session_record(500,0)
            store.recorders[key]=rec
            store.games[key]=(deepcopy(rec.game),now)
        end
        game=store.games[key][1]
        if path=="/api/replay"
            return replay_request(request,store,key,game)
        elseif path=="/api/session-report"
            game.phase=="settled" || return failure(409,"Finish a round before exporting this session")
            return response(200,public_session_report(game);cookie=key)
        end
        if expected=="POST"
            command=try
                JSON3.read(String(request.body),Dict{String,Any})
            catch
                return failure(400,"Invalid JSON command")
            end
            command isa AbstractDict || return failure(400,"Expected a command object")
            version=get(command,"version",nothing)
            version isa Integer && !(version isa Bool) && version==table_version(game) || return failure(426,"Table mode changed; reload this page before playing (protocol v$(table_version(game)) required)")
            revision=get(command,"revision",nothing)
            revision isa Integer && !(revision isa Bool) || return failure(400,"Integer revision required")
            revision!=game.revision && return response(409,(error="Table changed; current hand restored",state=table_view(game));cookie=key)
            game=try
                recorded_transition!(store,key,game,command)
            catch e
                e isa ArgumentError || rethrow()
                return failure(400,sprint(showerror,e))
            end
        end
        store.games[key]=(game,now)
        response(200,table_view(game);cookie=key)
    end
end

function start(port=8791;store=Store())
    HTTP.serve!(r->handler(r;store=store),ip"127.0.0.1",port;verbose=false)
end
function main(args=ARGS)
    length(args) in (0,2) || error("Usage: main.blackjack.web.server.jl [--port PORT]")
    !isempty(args) && args[1]!="--port" && error("Expected --port")
    port=isempty(args) ? 8791 : parse(Int,args[2])
    0<=port<=65535 || error("Invalid port")
    server=start(port)
    println("BLACKJACK_WEB_READY http://127.0.0.1:$(Int(getsockname(server.listener.server)[2]))")
    flush(stdout)
    try wait(server) finally close(server) end
end
end
abspath(PROGRAM_FILE)==(@__FILE__) && BlackjackWebServer.main()
