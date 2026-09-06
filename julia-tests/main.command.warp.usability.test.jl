module WarpUsabilityTests
using Test, JSON3
const ROOT=normpath(joinpath(@__DIR__,".."))
const PROFILE=get(ENV,"RECUR_PROFILE","release-safe")
const CORE=joinpath(ROOT,"target",PROFILE,"recur"*(Sys.iswindows() ? ".exe" : ""))
const ACTOR=joinpath(ROOT,"target",PROFILE,"recur-warp"*(Sys.iswindows() ? ".exe" : ""))
function invoke(exe,args)
    out=IOBuffer(); err=IOBuffer()
    p=run(pipeline(ignorestatus(`$exe $args`),stdout=out,stderr=err))
    return success(p),String(take!(out)),String(take!(err))
end
snapshot(root)=Dict(relpath(joinpath(d,f),root)=>read(joinpath(d,f)) for (d,_,files) in walkdir(root) for f in files)
function cfg(root,text)
    mkpath(joinpath(root,".recur")); write(joinpath(root,".recur","config.toml"),text)
end
@testset "Warp beginner commands" begin
    @testset "Create, inspect, accept, inspect again" begin
        mktempdir() do root
            cfg(root,"[warp.creation]\ndirectory='.recur/warps'\n")
            args=["create","demo.pool","--goal","Safe swimming, not reactor cooling","-d",root,"--json"]
            before=snapshot(root); ok,out,_=invoke(ACTOR,args)
            @test ok
            @test snapshot(root)==before
            if !ok; return; end
            @test JSON3.read(out)["state"]=="planned"
            @test length(JSON3.read(out)["map"]["required_slices"])==2
            ok,out,_=invoke(ACTOR,vcat(args,["--confirm"]))
            @test ok
            folder=joinpath(root,".recur","warps")
            path=joinpath(folder,"demo.pool.warp-map.json")
            @test isfile(path)
            if !isfile(path); return; end
            before=snapshot(root)
            ok,_,_=invoke(ACTOR,vcat(args,["--confirm"]))
            @test !ok
            @test snapshot(root)==before
            for verb in ("show","slices")
                ok,out,_=invoke(CORE,["warp",verb,"demo.pool","-d",root,"--json"])
                @test ok
                if ok
                    summary=JSON3.read(out)
                    @test summary["state"]=="incomplete"
                    @test summary["ready_slices"]==["slice-0"]
                    @test summary["current_slice"]=="slice-0"
                    @test length(summary["slices"])==2
                end
            end
            @test snapshot(root)==before
            ok,out,_=invoke(CORE,["warp","show","demo.pool","-d",root])
            @test ok
            @test occursin("slice-0",out) && occursin("slice-final",out)
            map=JSON3.read(read(path,String))
            for slice in map["required_slices"]
                args=["complete","demo.pool",String(slice["slice_id"]),"--attempt-id","verified-1",
                    "--result-hash","result-1","-d",folder,"--confirm","--json"]
                for gate in slice["evidence_gates"]; append!(args,["--evidence",String(gate)*"=reviewed-receipt.md"]); end
                ok,_,_=invoke(ACTOR,args); @test ok
                if slice["slice_id"]=="slice-0"
                    ok,out,_=invoke(CORE,["warp","show","demo.pool","-d",root,"--json"])
                    @test ok
                    if ok
                        progress=JSON3.read(out)
                        @test progress["ready_slices"]==["slice-final"]
                        @test progress["current_slice"]===nothing
                        @test length(progress["warnings"])==1
                        @test progress["completed_slices"]==["slice-0"]
                    end
                end
            end
            ok,out,_=invoke(CORE,["warp","show","demo.pool","-d",root,"--json"])
            @test ok
            if ok
                summary=JSON3.read(out)
                @test summary["state"]=="complete"
                @test summary["current_slice"]===nothing
                @test isempty(summary["ready_slices"])
                @test length(summary["completed_slices"])==2
                @test summary["evidence_status"]=="declared"
            end
        end
    end
    @testset "Bad names and escaping output do not write" begin
        for (folder,id) in (("complete","demo.release"),("partial","demo.partial"),("exploded","demo.explosion"))
            root=joinpath(@__DIR__,"fixtures","warp-bubble-v1",folder)
            before=snapshot(root)
            ok,out,_=invoke(CORE,["warp","show",id,"-d",root,"--json"])
            @test ok
            good,merged,_=invoke(CORE,["warp","merge",id,"-d",root,"--json"])
            @test good
            if ok && good
                progress=JSON3.read(out); projection=JSON3.read(merged)
                @test progress["state"]==projection["state"]
                @test progress["counts"]==projection["counts"]
                @test progress["completed_slices"]==projection["covered"]
                if folder=="exploded"; @test isempty(progress["ready_slices"]); end
            end
            @test snapshot(root)==before
        end
        for name in ("../outside","bad/name","bad..id", "CON")
            mktempdir() do root
                before=snapshot(root)
                ok,_,_=invoke(ACTOR,["create",name,"--goal","goal","-d",root,"--confirm"])
                @test !ok; @test snapshot(root)==before
            end
        end
        mktempdir() do root
            cfg(root,"[warp.creation]\ndirectory='../escape'\n")
            before=snapshot(root)
            ok,_,_=invoke(ACTOR,["create","demo.pool","--goal","goal","-d",root,"--confirm"])
            @test !ok; @test snapshot(root)==before
        end
    end
    @testset "Defaults, templates, ambiguity and bounded scope" begin
        mktempdir() do root
            goal="Swimming \"only\"\n{warp} stays literal in goal"
            ok,out,_=invoke(ACTOR,["create","demo.default","--goal",goal,"-d",root,"--confirm","--json"])
            @test ok
            if !ok; return; end
            path=joinpath(root,"warps","demo.default.warp-map.json")
            @test isfile(path)
            @test JSON3.read(read(path,String))["goal"]==goal
            template=replace(read(path,String),"demo.default"=>"{warp}")
            write(joinpath(root,"template.json"),template)
            cfg(root,"[warp.creation]\ndirectory='custom'\ntemplate='template.json'\n")
            ok,_,_=invoke(ACTOR,["create","demo.custom","--goal","Custom","-d",root,"--confirm"])
            @test ok
            @test isfile(joinpath(root,"custom","demo.custom.warp-map.json"))
            mkpath(joinpath(root,"duplicate")); cp(path,joinpath(root,"duplicate",basename(path)))
            before=snapshot(root)
            ok,_,err=invoke(CORE,["warp","show","demo.default","-d",root,"--json"])
            @test !ok
            @test occursin("ambiguous",err)
            ok,_,_=invoke(CORE,["warp","show","demo.default","-d",joinpath(root,"warps"),"--json"])
            @test ok
            @test snapshot(root)==before
            mkpath(joinpath(root,"nested"))
            ok,_,_=invoke(ACTOR,["create","demo.escape","--goal","goal","-d",joinpath(root,"nested"),"--confirm"])
            @test !ok
            @test snapshot(root)==before
            ok,_,_=invoke(ACTOR,["create","demo.blank","--goal","  ","-d",root,"--confirm"])
            @test !ok
            @test snapshot(root)==before
        end
    end
    @testset "Template validation and ambiguity" begin
        for bad in (
            "\"required_slices\":[{\"slice_id\":\"s\",\"contract_hash\":\"v1\",\"evidence_gates\":[]}]",
            "\"required_slices\":[{\"slice_id\":\"s\",\"contract_hash\":\"v1\",\"depends_on\":[\"s\"],\"evidence_gates\":[\"test\"]}]",
            "\"current_slice\":\"missing\",\"required_slices\":[{\"slice_id\":\"s\",\"contract_hash\":\"v1\",\"evidence_gates\":[\"test\"]}]"
        )
            mktempdir() do root
                cfg(root,"[warp.creation]\ntemplate='template.json'\n")
                write(joinpath(root,"template.json"),"{\"schema\":\"warp-bubble-map-v1\",\"warp_id\":\"{warp}\","*bad*"}")
                before=snapshot(root)
                ok,_,_=invoke(ACTOR,["create","demo.invalid","--goal","goal","-d",root,"--confirm"])
                @test !ok
                @test snapshot(root)==before
            end
        end
        mktempdir() do root
            cfg(root,"[warp.creation]\ndirectory='warps'\ntemplate='template.json'\n")
            write(joinpath(root,"template.json"),"{}")
            before=snapshot(root)
            ok,_,_=invoke(ACTOR,["create","demo.pool","--goal","goal","-d",root,"--confirm"])
            @test !ok; @test snapshot(root)==before
            ok,_,_=invoke(CORE,["warp","show","missing","-d",root,"--json"])
            @test !ok
        end
    end
end
end
