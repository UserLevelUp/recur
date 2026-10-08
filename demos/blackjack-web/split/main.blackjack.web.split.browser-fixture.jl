# register: demo.blackjack.web.split.browser.test private fixture server, never production
# Run separately on localhost:8792. No debug endpoint is added to the real app.
using HTTP, Sockets
include("../main.blackjack.web.server.jl")
const S=BlackjackWebServer
const STORE=S.Store()
function fixture(req)
    reply=S.handler(req;store=STORE)
    if req.method=="GET" && HTTP.URI(req.target).path=="/api/state" && reply.status==200
        key=split(first(split(HTTP.header(reply,"Set-Cookie"),';')),'=';limit=2)[2]
        lock(STORE.mutex) do
            g=STORE.games[key][1]
            if g.phase=="betting" && g.revision==0
                cards=[8,10,21,9,3,2,13]
                order=vcat(cards,setdiff(1:52,cards))
                g=S.BlackjackWeb.transition(S.BlackjackWeb.newgame(100),Dict("version"=>2,"action"=>"deal","revision"=>0,"bet"=>10);order)
                STORE.games[key]=(g,time())
            end
            reply=S.response(200,S.BlackjackWeb.public_state(g);cookie=key)
        end
    elseif req.method=="GET" && req.target=="/" && reply.status==200
        body=replace(String(reply.body),"SINGLE PLAYER · BLACKJACK"=>"TEST FIXTURE · SPLIT PAIR")
        reply=S.response(200,body;mime="text/html")
    end
    reply
end
if abspath(PROGRAM_FILE)==(@__FILE__)
    server=HTTP.serve!(fixture,ip"127.0.0.1",8792;verbose=false)
    println("SPLIT_FIXTURE_READY http://localhost:8792/");flush(stdout)
    try wait(server) finally close(server) end
end
