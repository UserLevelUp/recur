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
# Human output contract; JSON inventory remains unchanged.
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

@testset "List rings, duplicate scopes, escaping and qualified counts" begin
    mktempdir() do root
        cp(joinpath(@__DIR__,"fixtures","warp-ring-v1","complete"),joinpath(root,"ring"))
        before=snapshot(root)
        ok,text,_=query(root,"list","--all"); @test ok
        @test occursin("kind = \"ring\"",text)
        ring=split(split(text,"[warps.\"coordinator.release\"]")[2],"[warps.")[1]
        @test occursin("completed = \"unknown\"",ring)
        @test occursin("required = \"unknown\"",ring)
        @test occursin("pending = \"unknown\"",ring)
        @test occursin("ring.\"domains\" = 3",ring)
        @test occursin("ring.\"ready\" = 3",ring)
        @test snapshot(root)==before
    end
    mktempdir() do root
        for scope in ("one","two")
            mkpath(joinpath(root,scope))
            write(joinpath(root,scope,"demo.same.warp-map.json"),JSON3.write(Dict(
                "schema"=>"warp-bubble-map-v1","warp_id"=>"demo.same","required_slices"=>[
                Dict("slice_id"=>"a","contract_hash"=>"v2","evidence_gates"=>["tests"])])))
        end
        write(joinpath(root,"one","demo.same.a.old.warp-layer.json"),JSON3.write(Dict(
            "schema"=>"warp-slice-layer-v1","warp_id"=>"demo.same","slice_id"=>"a",
            "contract_hash"=>"v1","attempt_id"=>"old","result_state"=>"accepted",
            "result_hash"=>"old","evidence"=>Dict("tests"=>["old-receipt"]))))
        before=snapshot(root)
        ok,text,_=query(root,"list"); @test ok
        @test length(findall("[warps.\"demo.same\"]",text))==2
        @test occursin("stale_contract = 1",text)
        @test occursin("stale_contract = 0",text)
        @test occursin("one/demo.same.warp-map.json",text)
        @test occursin("two/demo.same.warp-map.json",text)
        @test occursin("2 listed / 2 discovered; 0 errors",text)
        @test query(root)[2]==text
        @test snapshot(root)==before
        write(joinpath(root,"demo.escape.warp-map.json"),JSON3.write(Dict(
            "schema"=>"warp-bubble-map-v1","warp_id"=>"bad\n\"name\"\t",
            "required_slices"=>[Dict("slice_id"=>"a","contract_hash"=>"v1")])) )
        before=snapshot(root)
        ok,text,_=query(root,"list"); @test ok
        @test !occursin("bad\n\"name\"\t",text)
        @test occursin("bad\\n\\\"name\\\"\\t",text)
        errorentry=split(split(text,"[warps.\"demo.escape\"]")[2],"[warps.")[1]
        @test occursin("completed = \"unknown\"",errorentry)
        @test occursin("error = ",errorentry)
        @test snapshot(root)==before
    end
end
end
