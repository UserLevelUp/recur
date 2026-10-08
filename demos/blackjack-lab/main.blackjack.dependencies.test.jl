# consumes: demo.blackjack.dependencies.review reference-based LLM conformance checks
@testset "09 source-bound dependency reference" begin
    root=joinpath(@__DIR__,"dependencies")
    registry=JSON3.read(read(joinpath(root,"main.blackjack.dependencies.registry.json"),String),Dict{String,Any})
    review=JSON3.read(read(joinpath(root,"main.blackjack.dependencies.review.json"),String),Dict{String,Any})
    nodes=registry["nodes"]; ids=Set(n["id"] for n in nodes)
    @test length(ids)==length(nodes)==19
    @test review["scope"]=="default-bindings-only"
    @test review["overall_complete"]==false
    for (file,hash) in review["source_files"]
        @test bytes2hex(sha256(read(joinpath(@__DIR__,file))))==hash
    end
    for (file,hash) in review["reference_files"]
        @test bytes2hex(sha256(read(joinpath(root,file))))==hash
    end
    expected=Set{Tuple{String,String}}()
    for n in nodes
        @test isfile(joinpath(@__DIR__,n["file"]))
        @test !isempty(n["input"]) && !isempty(n["output"])
        @test all(c->c in ids,n["calls"])
        for callee in n["calls"]; push!(expected,(callee,n["id"])); end
    end
    checked=query(root,"check","main.blackjack.dependencies.recur")
    @test checked.code==0
    edges=checked.data["footer"]["graph"]["edges"]
    actual=Set((e["producer"],e["consumer"]) for e in edges if e["producer"] in ids && e["consumer"] in ids)
    @test actual==expected
    # These are independent steps: implementation changes first invalidate the
    # review; the reviewer then maps a real new call into the reference model.
    base=read(joinpath(root,"main.blackjack.dependencies.recur"),String)
    original=read(joinpath(@__DIR__,"main.blackjack.03.jl"),String)
    altered=replace(original,"function score(cards)"=>"function score(cards)\n    tournament(TableConfig()) # forbidden upward call";count=1)
    @test altered!=original
    @test bytes2hex(sha256(altered))!=review["source_files"]["main.blackjack.03.jl"]
    mktempdir() do tmp
        write(joinpath(tmp,"implementation.jl"),altered)
        write(joinpath(tmp,"reference.recur"),base)
        @test query(tmp,"check","reference.recur").code==0 # no automatic Julia inspection
        mapped=replace(base,
            "i(a) := join(project review.plan.o(a).orders[\"score\"], cards.o(b))"=>
            "i(a) := join(project review.plan.o(a).orders[\"score\"], cards.o(b), session.o(b))",
            "await [cards.o(b)] -> score(a)"=>"await [cards.o(b), session.o(b)] -> score(a)")
        @test mapped!=base
        write(joinpath(tmp,"reference.recur"),mapped)
        fault=query(tmp,"check","reference.recur")
        @test fault.code==1
        cycles=filter(f->f["code"]=="SGR001",fault.data["footer"]["graph"]["findings"])
        @test !isempty(cycles)
        @test any(c->"score" in c["path"] && "session" in c["path"],cycles)
        @test all(c->first(c["path"])==last(c["path"]),cycles)
        println("  reviewed hidden call: ",join(first(cycles)["path"]," -> "))
        scoped=query(tmp,"check","reference.recur","--scope","hello")
        @test scoped.code==1
        @test scoped.data["footer"]["graph"]==fault.data["footer"]["graph"]
    end
end
