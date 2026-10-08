module RecurLangPlanTests
using Test, JSON3
const ROOT=normpath(joinpath(@__DIR__,".."))
const CORE=get(ENV,"RECUR_BIN","recur")
const ACTOR=get(ENV,"RECUR_LANG_BIN",dirname(CORE)=="." ? "recur-lang" : joinpath(dirname(CORE),Sys.iswindows() ? "recur-lang.exe" : "recur-lang"))
function call(root,args...;json=true)
    out,err=IOBuffer(),IOBuffer()
    p=run(pipeline(ignorestatus(Cmd([ACTOR,"plan",args...,"-d",root,(json ? ["--json"] : String[])...])),stdout=out,stderr=err))
    text=String(take!(out)); data=try JSON3.read(text,Dict{String,Any}) catch; nothing end
    (code=p.exitcode,data=data,text=text)
end
inventory(root)=Dict(relpath(joinpath(d,f),root)=>read(joinpath(d,f)) for (d,_,fs) in walkdir(root) for f in fs)
@testset "Companion implementation plan v1" begin
    mktempdir() do root
        wir=read(joinpath(ROOT,"demos/blackjack-lab/main.blackjack.05.session.recur"),String)
        cir=read(joinpath(ROOT,"demos/blackjack-lab/main.blackjack.06.coordination.recur"),String)
        write(joinpath(root,"session.recur"),wir); write(joinpath(root,"coordination.recur"),cir)
        before=inventory(root); initial=call(root,"session.recur")
        @test initial.code==0
        @test initial.data!==nothing
        if initial.data!==nothing && initial.code==0
            p=initial.data
            @test p["schema"]=="recur-lang-implementation-plan-v1"
            @test p["state"]=="needs-target"
            @test p["execution"]=="not-run"
            @test p["policy"]["settings"]["target"]=="unspecified"
            @test p["phases"]==["specification","tests","implementation"]
            @test length(p["work_items"])==2
            @test length(p["test_plan"])>=3
            @test any(t->t["kind"]=="dependency-conformance",p["test_plan"])
            @test p["packet"]["coverage"]["whole_source_validated"]==false
            @test p==call(root,"session.recur").data
            @test inventory(root)==before
            @test !ispath(joinpath(root,".recur"))
            write(joinpath(root,"session.recur"),wir*"\n# changed source\n")
            @test call(root,"session.recur").data["source_hash"]!=p["source_hash"]
            mkpath(joinpath(root,".recur"))
            config="[recur-lang]\ntarget='julia'\n[recur-lang.planning]\nspecification_first=false\ninclude_test_plan=false\nprioritize_graph_findings=false\n"
            write(joinpath(root,".recur/config.toml"),config)
            before=inventory(root); custom=call(root,"session.recur","--scope","render.r").data
            @test custom["state"]=="planned"
            @test custom["phases"]==["implementation","tests"]
            @test isempty(custom["test_plan"])
            @test length(custom["work_items"])==1
            @test inventory(root)==before
            @test custom["policy"]["config_hash"]!==nothing
            @test occursin("planned",call(root,"session.recur";json=false).text)
            @test occursin("Run seeded rounds",call(root,"session.recur";json=false).text)
            @test occursin("implementation -> tests",call(root,"session.recur";json=false).text)
            write(joinpath(root,".recur/config.toml"),config*"# config changed\n")
            @test call(root,"session.recur").data["policy"]["config_hash"]!=custom["policy"]["config_hash"]
            valid=call(root,"coordination.recur")
            @test valid.code==0 && isempty(valid.data["findings"])
            broken=replace(cir,"i(a) := project table.plan.o(a).orders[\"players\"]"=>"i(a) := join(table.plan.o(a), audit.o(b))";count=1)
            @test broken!=cir
            write(joinpath(root,"coordination.recur"),broken)
            bad=call(root,"coordination.recur","--scope","house")
            @test bad.code==1 && bad.data["state"]=="blocked"
            @test any(f->f["code"]=="SGR001",bad.data["findings"])
            @test last(bad.data["work_items"])["kind"]=="repair-static-findings"
            write(joinpath(root,".recur/config.toml"),"[recur-lang]\ntarget='julia'\n")
            @test first(call(root,"coordination.recur").data["work_items"])["kind"]=="repair-static-findings"
            before=inventory(root)
            @test call(root,"missing.recur").code==2
            @test call(root,"session.recur","--scope","unknown").code==2
            @test call(root,"../outside.recur").code==2
            @test inventory(root)==before
            # Ancestor policy applies without initializing a nested project.
            child=joinpath(root,"nested"); mkpath(child)
            write(joinpath(child,"child.recur"),wir)
            inherited=call(child,"child.recur")
            @test inherited.code==0 && inherited.data["policy"]["settings"]["target"]=="julia"
            @test !ispath(joinpath(child,".recur"))
            @test !isempty(inherited.data["test_plan"]) # absent preference uses default true
            write(joinpath(root,".recur/config.toml"),"[recur-lang]\ntarget=3\n")
            before=inventory(root); invalid=call(root,"session.recur")
            @test invalid.code==2
            @test occursin("LINIT002",invalid.text)
            @test inventory(root)==before
        end
    end
end
end
