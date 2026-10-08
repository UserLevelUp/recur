# consumer: demo.blackjack.web.dependencies reference-to-source correspondence
const CORE=get(ENV,"RECUR_BIN","recur")
function query(root,args...)
    output=IOBuffer(); errors=IOBuffer()
    proc=run(pipeline(ignorestatus(Cmd([CORE,"lang",args...,"-d",root,"--json"])),stdout=output,stderr=errors))
    text=String(take!(output))
    (code=proc.exitcode,data=JSON3.read(text,Dict{String,Any}))
end
@testset "Blackjack website Lang reference" begin
    root=@__DIR__
    specs=sort(filter(x->endswith(x,".recur"),readdir(root)))
    @test length(specs)==5
    for spec in specs
        q=query(root,"check",spec)
        @test q.code==0
        @test isempty(q.data["footer"]["findings"])
        @test q.data["footer"]["execution"]=="not-run"
    end
    # Compare actual public output with the symbolic bundle, including optional fields.
    q=query(root,"show","main.blackjack.web.01.table.recur","--scope","view.f")
    contract=only(filter(c->c["canonical_identity"]=="view.o(b)",q.data["contracts"]))
    @test Set(f["name"] for f in contract["fields"])==Set(string.(propertynames(B.public_state(B.newgame()))))
    @test q.data["header"][1]["binding"]=="BlackjackWeb.public_state"
    q=query(root,"show","main.blackjack.web.03.http.recur","--scope","route.f")
    @test q.data["header"][1]["binding"]=="BlackjackWebServer.handler"
    graph=query(root,"check","main.blackjack.web.05.dependencies.recur").data["footer"]["graph"]
    names=Set(["rules","engine","http","view","controls"])
    edges=Set((e["producer"],e["consumer"]) for e in graph["edges"] if e["producer"] in names && e["consumer"] in names)
    @test edges==Set([("rules","engine"),("engine","http"),("http","view"),("view","controls")])
    trace=JSON3.read(read(Cmd([CORE,"trace-id","demo.blackjack.web.play","--scope","main.blackjack.web.**","-d",root,"--json"]),String),Dict{String,Any})
    @test any(s->endswith(s["path"],"main.blackjack.web.02.play.recur"),trace["define"])
    @test any(s->endswith(s["path"],"main.blackjack.web.engine.jl"),trace["produce"])
    @test any(s->endswith(s["path"],"main.blackjack.web.test.jl"),trace["consume"])
    mktempdir() do tmp
        source=read(joinpath(root,"main.blackjack.web.05.dependencies.recur"),String)
        source=replace(source,"project review.plan.o(a).orders[\"rules\"]"=>"join(review.plan.o(a), engine.o(b))")
        write(joinpath(tmp,"main.cycle.recur"),source)
        cycle=query(tmp,"check","main.cycle.recur")
        @test cycle.code==1
        @test any(f->f["code"]=="SGR001",cycle.data["footer"]["findings"])
        scoped=query(tmp,"check","main.cycle.recur","--scope","controls")
        @test scoped.code==1 # scope cannot conceal a rules/engine cycle
    end
    include("split/main.blackjack.web.split.model.test.jl")
    # Hash freshness verifies the recorded author review, not inferred call closure.
    review=JSON3.read(read(joinpath(root,"rivals/main.blackjack.rivals.review.json"),String),Dict{String,Any})
    @test review["reviewer"]=="Codex author self-review"
    @test review["whole_program_proof"]==false
    for (path,hash) in review["inputs_sha256"]
        @test bytes2hex(sha256(read(joinpath(root,path))))==hash
    end
end
