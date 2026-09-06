module WarpQueryCompatibilityTests
using Test, JSON3
const ROOT=normpath(joinpath(@__DIR__,".."))
const CORE=joinpath(ROOT,"target",get(ENV,"RECUR_PROFILE","release-safe"),Sys.iswindows() ? "recur.exe" : "recur")
function query(args)
    out=IOBuffer(); err=IOBuffer()
    p=run(pipeline(ignorestatus(`$CORE $args`),stdout=out,stderr=err))
    success(p),String(take!(out)),String(take!(err))
end
snapshot(root)=Dict(relpath(joinpath(d,f),root)=>read(joinpath(d,f)) for (d,_,files) in walkdir(root) for f in files)
@testset "Warp remains ordinary searchable hierarchy" begin
    for location in ("warps",joinpath(".recur","warps"))
        mktempdir() do root
            folder=joinpath(root,location); mkpath(folder)
            # UUID metadata must not replace the readable filename hierarchy.
            map=Dict("schema"=>"warp-bubble-map-v1","warp_id"=>"demo.pool",
                "bubble_uuid"=>"01991a00-0000-7000-8000-000000000001",
                "current_slice"=>"slice-0",
                "required_slices"=>[Dict("slice_id"=>"slice-$i","contract_hash"=>"v1-$i",
                    "depends_on"=>i==0 ? String[] : ["slice-$(i-1)"],
                    "evidence_gates"=>["tests"]) for i in 0:10])
            write(joinpath(folder,"demo.pool.warp-map.json"),JSON3.write(map))
            write(joinpath(folder,"demo.pool.slice-0.todo.current.md"),
                "# Baseline\ndefines: demo.pool.acceptance swimming safety\n")
            before=snapshot(root)
            # Explicit -d works for hidden placement without changing generic traversal rules.
            for args in (["tree","demo.pool","-d",folder,"--json"],
                         ["files","demo.pool.**","-d",folder,"--json"],
                         ["trace-id","demo.pool.acceptance","--scope","demo.pool.**","--ext","md","-d",folder,"--json"])
                ok,out,_=query(args)
                @test ok
                @test !isempty(out)
                if ok
                    @test JSON3.read(out)!==nothing
                    @test occursin("pool",out)
                end
            end
            ok,out,_=query(["warp","-d",root,"--json"])
            @test ok
            if ok
                inventory=JSON3.read(out)
                @test inventory["listed"]==1
                @test inventory["errors"]==0
                @test inventory["entries"][1]["warp_id"]=="demo.pool"
            end
            for verb in ("show","slices")
                ok,out,_=query(["warp",verb,"demo.pool","-d",root,"--json"])
                @test ok
                if ok
                    progress=JSON3.read(out)
                    @test progress["counts"]["required"]==11
                    @test progress["counts"]["covered"]==0
                    @test progress["current_slice"]=="slice-0"
                    @test progress["ready_slices"]==["slice-0"]
                end
            end
            @test snapshot(root)==before
        end
    end
end
end
