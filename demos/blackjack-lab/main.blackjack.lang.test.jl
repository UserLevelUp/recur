# consumes: demo.blackjack Lang fragments, adversarial graphs and real CLI transitions
using JSON3, SHA
const CORE=get(ENV,"RECUR_BIN","recur")
const ACTOR=get(ENV,"RECUR_LANG_BIN",dirname(CORE)=="." ? "recur-lang" : joinpath(dirname(CORE),Sys.iswindows() ? "recur-lang.exe" : "recur-lang"))
function command(exe,args...)
    out,err=IOBuffer(),IOBuffer()
    p=run(pipeline(ignorestatus(Cmd([exe,args...])),stdout=out,stderr=err))
    text=String(take!(out)); data=try JSON3.read(text,Dict{String,Any}) catch; nothing end
    (code=p.exitcode,data=data,text=text,stderr=String(take!(err)))
end
query(root,args...)=command(CORE,"lang",args...,"-d",root,"--json")
inventory(root)=Dict(relpath(joinpath(d,f),root)=>read(joinpath(d,f)) for (d,_,fs) in walkdir(root) for f in fs)
function fnv(bytes)
    h=UInt64(0xcbf29ce484222325)
    for b in bytes; h=(h ⊻ UInt64(b))*UInt64(0x100000001b3); end
    "fnv1a64:"*string(h;base=16,pad=16)
end
writejson(root,name,value)=write(joinpath(root,name),JSON3.write(value))
@testset "07 language surface and deliberate faults" begin
    mktempdir() do root
        specs=sort(filter(x->endswith(x,".recur"),readdir(@__DIR__)))
        @test length(specs)==6
        for spec in specs; cp(joinpath(@__DIR__,spec),joinpath(root,spec)); end
        before=inventory(root)
        listed=query(root,"list")
        @test listed.code==0 && length(listed.data["sources"])==6
        for spec in specs
            check=query(root,"check",spec)
            @test check.code==0
            @test check.data["coverage"]["whole_source_validated"]==false
            @test check.data["footer"]["execution"]=="not-run"
            @test query(root,"report",spec).code==0
        end
        alias=query(root,"show",specs[4],"--scope","settle.s")
        @test alias.data["header"][1]["input"]["canonical_identity"]=="play.o(b)"
        c=query(root,"show",specs[6],"--scope","settle.s")
        e=query(root,"show",specs[6],"--scope","settle.s","--expand")
        @test length(c.data["header"][1]["input"]["members"])==3
        @test length(e.data["header"][1]["input"]["messages"])==3
        @test c.data["footer"]["graph"]==e.data["footer"]["graph"]
        projected=query(root,"show",specs[6],"--scope","players.s","--expand")
        @test occursin("orders",JSON3.write(projected.data["header"]))
        @test query(root,"show",specs[6],"--scope","s").code==2 # ambiguous local symbol
        @test query(root,"show",specs[5],"--scope","unknown").code==2
        # Compare authored result fields with real Julia result shapes, rather
        # than treating a valid static signature as implementation agreement.
        table=B.TableConfig(players=1,rounds=1)
        played=B.prepare_round(table,[100],1000,ordered([1,9,10,7]))
        for (spec,symbol,result) in [(specs[2],"configure.c",table),(specs[3],"score.s",B.score([1,10])),
                (specs[4],"play.p",played),(specs[4],"settle.s",B.settle_round(played)),
                (specs[5],"tournament.t",B.tournament(table))]
            packet=query(root,"show",spec,"--scope",symbol).data
            identity=packet["header"][1]["output"]["canonical_identity"]
            contract=only(filter(c->c["canonical_identity"]==identity,packet["contracts"]))
            @test Set(f["name"] for f in contract["fields"])==Set(string.(propertynames(result)))
        end
        @test inventory(root)==before
        write(joinpath(root,"demo.blackjack.tournament.complete.md"),"Recorded only; no test receipt")
        recorded=query(root,"report",specs[5],"--eventness","complete")
        @test recorded.code==0 && !isempty(recorded.data["header"])
        @test recorded.data["footer"]["execution"]=="not-run"
        base=read(joinpath(root,specs[6]),String)
        for (name,altered,code) in [
            ("self",replace(base,"i(a) := project table.plan.o(a).orders[\"players\"]"=>"i(a) := join(table.plan.o(a), players.o(b))";count=1),"SGR001"),
            ("dependency",replace(base,"i(a) := project table.plan.o(a).orders[\"players\"]"=>"i(a) := join(table.plan.o(a), audit.o(b))";count=1),"SGR001"),
            ("wait",replace(base,"-> table.finish(a)"=>"-> settle(a) -> table.finish(a)"),"SGR002"),
            ("join",replace(base,"await [players.o(b), house.o(b)]"=>"await players.o(b)"),"SGR004")]
            @test altered!=base
            write(joinpath(root,"fault.recur"),altered)
            full=query(root,"check","fault.recur"); scoped=query(root,"check","fault.recur","--scope","house")
            @test full.code==scoped.code==1
            @test any(f->f["code"]==code,full.data["footer"]["graph"]["findings"])
            @test full.data["footer"]["graph"]==scoped.data["footer"]["graph"]
            println("  injected $name: $code")
        end
        # Valid static signatures cannot enforce even wagers or inspect bindings.
        original=read(joinpath(root,specs[2]),String)
        write(joinpath(root,"unsupported.recur"),replace(original,"recur 0.1"=>"recur 9.0"))
        @test query(root,"check","unsupported.recur").code==2
        write(joinpath(root,"excluded.recur"),original*"\nretry forever { invented syntax }\n")
        excluded=query(root,"check","excluded.recur")
        @test excluded.code==0 && !excluded.data["coverage"]["whole_source_validated"]
        @test_throws ArgumentError B.TableConfig(bet=3)
        hidden=replace(original,"by BlackjackLab.TableConfig"=>"by Hidden.configure")
        write(joinpath(root,"hidden.recur"),hidden)
        write(joinpath(root,"hidden.jl"),"configure(n=0)=n==4 ? error(\"cycle\") : caller(n+1)\ncaller(n)=configure(n)\n")
        @test query(root,"check","hidden.recur").code==0
        hiddenmod=Module(:BlackjackHidden); Base.include(hiddenmod,joinpath(root,"hidden.jl"))
        @test_throws ErrorException Base.invokelatest(() -> getproperty(hiddenmod,:configure)())
    end
    trace=command(CORE,"trace-id","demo.blackjack.session","--scope","main.blackjack.**","-d",@__DIR__,"--format","full","--json")
    @test trace.code==0
    @test occursin("main.blackjack.requirements.md",trace.text)
    @test occursin("main.blackjack.05.jl",trace.text)
    @test occursin("main.blackjack.05.test.jl",trace.text)
end
@testset "08 companion init and observed checked transition" begin
    mktempdir() do root
        for f in readdir(@__DIR__)
            (endswith(f,".jl") || endswith(f,".recur") || f=="main.blackjack.requirements.md") && cp(joinpath(@__DIR__,f),joinpath(root,f))
        end
        before=inventory(root)
        @test command(ACTOR,"init","--dry-run","--json","-d",root).code==0
        @test inventory(root)==before
        @test command(ACTOR,"init","--json","-d",root).code==0
        installed=inventory(root)
        @test command(ACTOR,"init","--json","-d",root).data["changed"]==false
        @test inventory(root)==installed
        cfg=B.TableConfig(players=1,rounds=2)
        orders=[ordered([1,9,10,7]),ordered([9,1,7,10])]
        session=B.tournament(cfg;orders=orders)
        outcomes=[("session.payoffs",session.balances==[105] && session.bank==995 && session.leaders==[1]),
                  ("session.repeatability",session==B.tournament(cfg;orders=orders))]
        @test all(last,outcomes)
        all(last,outcomes) || error("Refusing to issue evidence for failed observed cases")
        source="main.blackjack.05.session.recur"; scope="tournament.t"
        legacy=command(ACTOR,"warp",source,"tournament","-d",root,"--json")
        @test legacy.code==0 && legacy.data["schema"]=="recur-lang-warp-plan-v1"
        @test inventory(root)==installed
        e0="demo.blackjack.tournament.todo.current.md"; ef="demo.blackjack.tournament.complete.md"
        write(joinpath(root,e0),"Observed bounded blackjack session; not whole-language certification\n")
        writejson(root,"runtime.json",Dict("julia"=>string(VERSION),"core_sha256"=>bytes2hex(sha256(read(Sys.which(CORE)))),"companion_sha256"=>bytes2hex(sha256(read(Sys.which(ACTOR))))))
        roles=Dict("specification"=>[source],"implementation"=>["main.blackjack.$(lpad(i,2,'0')).jl" for i in 1:6],
            "tests"=>vcat(["main.blackjack.$(lpad(i,2,'0')).test.jl" for i in 1:6],["main.blackjack.lang.test.jl"]),
            "configuration"=>[".recur/config.toml","runtime.json"],"runner"=>["main.blackjack.jl","main.blackjack.test.jl"],
            "behavior"=>["main.blackjack.requirements.md"])
        writejson(root,"policy.json",Dict("schema"=>"recur-lang-checked-contract-v1","contract_id"=>"blackjack.session.v1",
            "source"=>source,"source_hash"=>fnv(read(joinpath(root,source))),"scope"=>scope,
            "aliases"=>Dict("tournament.i(a)"=>"tournament.i(a)","tournament.o(b)"=>"tournament.o(b)"),
            "transition"=>Dict("current"=>"demo.blackjack.tournament.todo.current","slice"=>scope,"desired"=>"demo.blackjack.tournament.complete"),
            "inputs"=>roles,"requirements"=>[Dict("id"=>"bounded-session","cases"=>first.(outcomes))]))
        writejson(root,"result.json",Dict("schema"=>"warp-external-result-v1","kind"=>"test","outcome"=>"passed","exit_code"=>0,
            "tests"=>Dict("discovered"=>2,"executed"=>2,"passed"=>2,"failed"=>0,"skipped"=>0)))
        files=vcat(collect(Iterators.flatten(values(roles))),["policy.json"])
        writejson(root,"evidence.json",Dict("schema"=>"warp-external-evidence-v1","kind"=>"test","producer"=>"observed blackjack Julia cases",
            "project"=>"blackjack-lab","configuration"=>"Julia -O0 -C generic","platform"=>string(Sys.KERNEL),"executed_at_unix"=>floor(Int,time()),
            "result_artifact"=>"result.json","result_fingerprint"=>fnv(read(joinpath(root,"result.json"))),
            "source"=>Dict("revision"=>nothing,"dirty"=>true,"files"=>Dict(p=>fnv(read(joinpath(root,p))) for p in files))))
        writejson(root,"attempt.json",Dict("schema"=>"recur-lang-checked-receipt-v1","attempt_id"=>"blackjack-1",
            "contract_hash"=>fnv(read(joinpath(root,"policy.json"))),"source_hash"=>fnv(read(joinpath(root,source))),
            "scope"=>scope,"phase"=>"green","producer"=>"observed blackjack Julia cases","runtime"=>string(VERSION),
            "evidence"=>"evidence:evidence.json","cases"=>[Dict("id"=>id,"outcome"=>"passed") for (id,_) in outcomes]))
        args=["warp",source,scope,"--checked-contract","policy.json","--receipt","attempt.json","--eventness",e0,"-d",root,"--json"]
        before=inventory(root)
        @test command(ACTOR,args...).code==0
        @test inventory(root)==before
        accepted=command(ACTOR,args...,"--confirm")
        @test accepted.code==0 && accepted.data["action"]["ack"]
        @test !isfile(joinpath(root,e0)) && isfile(joinpath(root,ef))
        @test command(ACTOR,args...,"--recover","--confirm").code==0
        status=".recur/lang/checked/blackjack-1/accepted.json"
        evidence_args=["evidence",source,"--scope",scope,"--contract","policy.json","--receipt","attempt.json","--status",status]
        current=query(root,evidence_args...)
        @test current.data["transition_status"]["current_accepted"]
        open(joinpath(root,"main.blackjack.05.jl"),"a") do io; write(io,"\n# changed after observed tests\n"); end
        before=inventory(root)
        stale=query(root,evidence_args...)
        @test stale.code!=0 && !stale.data["transition_status"]["current_accepted"]
        @test command(ACTOR,args...,"--confirm").code!=0
        @test inventory(root)==before
    end
end
