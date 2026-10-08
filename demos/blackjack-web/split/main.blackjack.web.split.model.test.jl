module BlackjackSplitAnalysis
Base.Experimental.@compiler_options compile=min optimize=0 infer=false
using Test, JSON3
include("../main.blackjack.web.engine.jl")
const CORE=get(ENV,"RECUR_BIN","recur")
function query(root,args...)
    output=IOBuffer()
    proc=run(pipeline(ignorestatus(Cmd([CORE,"lang",args...,"-d",root,"--json"])),stdout=output))
    (code=proc.exitcode,data=JSON3.read(String(take!(output)),Dict{String,Any}))
end
@testset "Split model and runtime correspondence" begin
    for name in ("contract","dependencies")
        q=query(@__DIR__,"check","main.blackjack.web.split.$name.recur")
        @test q.code==0
        @test isempty(q.data["footer"]["findings"])
        @test q.data["footer"]["execution"]=="not-run"
    end
    q=query(@__DIR__,"show","main.blackjack.web.split.contract.recur","--scope","view.f")
    contract=only(filter(c->c["canonical_identity"]=="view.o(b)",q.data["contracts"]))
    @test Set(["hands","active_hand_id","escrow","schema"]) ⊆ Set(f["name"] for f in contract["fields"])
    @test Set(f["name"] for f in contract["fields"])==Set(string.(propertynames(BlackjackWeb.public_state(BlackjackWeb.newgame()))))
    @test only(q.data["header"])["binding"]=="BlackjackWeb.public_state"
    @test length(query(@__DIR__,"report","main.blackjack.web.split.contract.recur").data["header"])==6
    mktempdir() do root
        source=read(joinpath(@__DIR__,"main.blackjack.web.split.dependencies.recur"),String)
        source=replace(source,"join(review.plan.o(a), rules.o(b))"=>"join(review.plan.o(a), rules.o(b), settlement.o(b))")
        write(joinpath(root,"main.split.cycle.recur"),source)
        for scope in (String[],["--scope","view"])
            q=query(root,"check","main.split.cycle.recur",scope...)
            @test q.code==1
            cycles=filter(f->f["code"]=="SGR001",q.data["footer"]["findings"])
            @test !isempty(cycles)
            println("Mapped split dependency fault: ",JSON3.write(cycles))
        end
    end
end

if "--runtime-gap" in ARGS
    @testset "Former runtime gap: now implemented" begin
        order=vcat([8,9,21,7,2,3],setdiff(1:52,[8,9,21,7,2,3]))
        game=BlackjackWeb.transition(BlackjackWeb.newgame(100),Dict("version"=>2,"action"=>"deal","revision"=>0,"bet"=>10);order=order)
        @test "split" in BlackjackWeb.allowed(game)
        @test hasproperty(BlackjackWeb.public_state(game),:hands)
    end
end
end
