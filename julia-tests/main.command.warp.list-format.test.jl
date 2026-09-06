module WarpListFormatTests
using Test, JSON3
const ROOT=normpath(joinpath(@__DIR__,".."))
const CORE=joinpath(ROOT,"target",get(ENV,"RECUR_PROFILE","release-safe"),Sys.iswindows() ? "recur.exe" : "recur")
function query(root,args...)
    out=IOBuffer(); err=IOBuffer()
    p=run(pipeline(ignorestatus(`$CORE warp $args -d $root`),stdout=out,stderr=err))
    success(p),String(take!(out)),String(take!(err))
end
snapshot(root)=Dict(relpath(joinpath(d,f),root)=>read(joinpath(d,f)) for (d,_,files) in walkdir(root) for f in files)
# Intentionally red standalone contract; integrate into runtests.jl when implemented.
@testset "Warp list trait-style human presentation" begin
    mktempdir() do root
        id="demo.pool"
        write(joinpath(root,id*".warp-map.json"),JSON3.write(Dict(
            "schema"=>"warp-bubble-map-v1","warp_id"=>id,"required_slices"=>[
                Dict("slice_id"=>"baseline","contract_hash"=>"v1","evidence_gates"=>["tests"]) ])))
        before=snapshot(root)
        ok,json_before,_=query(root,"list","--json")
        @test ok
        for args in ((),("list",))
            ok,text,_=query(root,args...)
            @test ok
            @test occursin("[warps.\"demo.pool\"]",text)
            for line in ("state = \"incomplete\"","completed = 0","required = 1",
                         "pending = 1","blocked = 0","evidence = \"absent\"")
                @test occursin(line,text)
            end
            @test !occursin("counts={",text)
            @test occursin("1 listed / 1 discovered; 0 errors",text)
        end
        ok,json_after,_=query(root,"list","--json")
        @test ok
        @test json_before==json_after
        inventory=JSON3.read(json_after)
        @test inventory["schema"]=="warp-list-v1"
        @test inventory["entries"][1]["counts"]["required"]==1
        @test snapshot(root)==before
    end
    mktempdir() do root
        ok,text,_=query(root,"list")
        @test ok
        @test occursin("No matching Warp bubbles.",text)
        @test occursin("0 listed / 0 discovered; 0 errors",text)
        write(joinpath(root,"demo.bad.warp-map.json"),"{}")
        before=snapshot(root)
        ok,text,_=query(root,"list")
        @test ok # Existing inventory API reports per-entry errors in output.
        @test occursin("[warps.\"demo.bad\"]",text)
        @test occursin("state = \"error\"",text)
        @test occursin("error = ",text)
        @test occursin("1 errors",text)
        @test snapshot(root)==before
    end
end
end
