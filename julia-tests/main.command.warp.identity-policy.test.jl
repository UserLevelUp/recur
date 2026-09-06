module WarpIdentityPolicyTests
using Test, JSON3, TOML
const ROOT = normpath(joinpath(@__DIR__, ".."))
const ACTOR = joinpath(ROOT, "target", get(ENV, "RECUR_PROFILE", "release-safe"), Sys.iswindows() ? "recur-warp.exe" : "recur-warp")
function invoke(args)
    out=IOBuffer(); err=IOBuffer()
    process=run(pipeline(ignorestatus(`$ACTOR $args`), stdout=out, stderr=err))
    (success(process), String(take!(out)), String(take!(err)))
end
snapshot(root)=Dict(relpath(joinpath(d,f),root)=>read(joinpath(d,f)) for (d,_,files) in walkdir(root) for f in files)
uuid7(value)=value isa AbstractString && occursin(r"^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$",value)

@testset "Warp identity and optional preservation policy" begin
    @testset "Init previews, installs editable defaults, and is idempotent" begin
        mktempdir() do root
            before=snapshot(root)
            ok,_,_=invoke(["init","-d",root,"--dry-run","--json"])
            @test ok
            @test snapshot(root)==before
            ok,_,_=invoke(["init","-d",root,"--json"])
            @test ok
            path=joinpath(root,".recur","config.toml")
            @test isfile(path)
            if !isfile(path); return; end
            config=TOML.parsefile(path)
            removal=config["warp"]["removal"]
            @test removal["require_confirmation"]===true
            @test removal["require_committed_snapshot"]===true
            @test removal["require_preservation_ref"]===true
            @test removal["require_pushed_ref"]===false
            creation=config["warp"]["creation"]
            @test creation["directory"]=="warps"
            @test isfile(joinpath(root,creation["template"]))
            @test occursin(".recur/warps",read(path,String))
            before=snapshot(root)
            ok,_,_=invoke(["init","-d",root,"--json"])
            @test ok
            @test snapshot(root)==before
            ok,_,_=invoke(["create","demo.initialized","--goal","Test initialized template","-d",root,"--confirm","--json"])
            @test ok
        end
    end
    @testset "Explicit opt-out and custom settings survive initialization" begin
        mktempdir() do root
            mkpath(joinpath(root,".recur"))
            path=joinpath(root,".recur","config.toml")
            write(path,"""
            # User comment must survive.
            [warp.creation]
            directory = "docs/warps"
            template = "custom.json"
            [warp.removal]
            require_confirmation = true
            require_committed_snapshot = false
            require_preservation_ref = false
            require_pushed_ref = false
            [unrelated]
            keep = "yes"
            """)
            write(joinpath(root,"custom.json"),"{\"user-owned\":true}")
            before=snapshot(root)
            ok,_,_=invoke(["init","-d",root,"--json"])
            @test ok
            @test snapshot(root)==before
        end
    end
    @testset "Malformed configuration is refused without writes" begin
        mktempdir() do root
            mkpath(joinpath(root,".recur"))
            write(joinpath(root,".recur","config.toml"),"[warp.removal]\nrequire_committed_snapshot = 'not-a-boolean'\n")
            before=snapshot(root)
            ok,_,err=invoke(["init","-d",root,"--json"])
            @test !ok
            @test occursin("require_committed_snapshot",err)
            @test snapshot(root)==before
        end
    end
    @testset "Creation assigns persistent distinct bubble and slice UUIDv7 identities" begin
        mktempdir() do root
            ids=String[]
            for name in ("demo.pool","demo.cooling")
                ok,out,_=invoke(["create",name,"--goal","Independent work","-d",root,"--confirm","--json"])
                @test ok
                if !ok; continue; end
                map=JSON3.read(out)["map"]
                @test haskey(map,"bubble_uuid")
                if haskey(map,"bubble_uuid")
                    @test uuid7(map["bubble_uuid"])
                    push!(ids,String(map["bubble_uuid"]))
                end
                for slice in map["required_slices"]
                    @test haskey(slice,"slice_uuid")
                    if haskey(slice,"slice_uuid")
                        @test uuid7(slice["slice_uuid"])
                        push!(ids,String(slice["slice_uuid"]))
                    end
                end
                path=joinpath(root,"warps",name*".warp-map.json")
                @test JSON3.read(read(path,String))==map
                before=snapshot(root)
                ok,_,_=invoke(["create",name,"--goal","Retry","-d",root,"--confirm","--json"])
                @test !ok
                @test snapshot(root)==before
            end
            @test length(ids)==6
            @test length(unique(ids))==length(ids)
        end
    end
end

const CORE = joinpath(dirname(ACTOR), Sys.iswindows() ? "recur.exe" : "recur")
function core(args)
    out=IOBuffer(); err=IOBuffer()
    p=run(pipeline(ignorestatus(`$CORE warp $args`),stdout=out,stderr=err))
    success(p),String(take!(out)),String(take!(err))
end
mutablejson(path)=JSON3.read(read(path,String),Dict{String,Any})
@testset "Identity policy edge cases" begin
    @testset "Evolution retains predecessor and requires a distinct successor identity" begin
        mktempdir() do root
            @test invoke(["create","demo.source","--goal","Source","-d",root,"--confirm"])[1]
            sourcepath=joinpath(root,"warps","demo.source.warp-map.json")
            source=mutablejson(sourcepath); sourcebytes=read(sourcepath)
            for attempt in ("one","two")
                write(joinpath(root,"warps","demo.source.slice-0.$attempt.warp-layer.json"),JSON3.write(Dict(
                    "schema"=>"warp-slice-layer-v1","warp_id"=>"demo.source","slice_id"=>"slice-0",
                    "contract_hash"=>source["required_slices"][1]["contract_hash"],
                    "attempt_id"=>attempt,"result_state"=>"accepted","result_hash"=>attempt,
                    "evidence"=>Dict("baseline"=>["fixture"])) ))
            end
            ok,out,_=invoke(["create","demo.next","--goal","Successor","-d",root,"--json"]); @test ok
            if !ok; return; end
            target=JSON3.read(out,Dict{String,Any})["map"]
            candidate=joinpath(root,"candidate.json")
            for reused in (source["bubble_uuid"],nothing)
                invalid=deepcopy(target)
                if reused===nothing; delete!(invalid,"bubble_uuid"); else; invalid["bubble_uuid"]=reused; end
                write(candidate,JSON3.write(invalid)); before=snapshot(root)
                ok,_,err=invoke(["evolve","demo.source",candidate,"-d",root,"--confirm"])
                @test !ok
                @test occursin("new bubble_uuid",err)
                @test snapshot(root)==before
            end
            write(candidate,JSON3.write(target))
            @test invoke(["evolve","demo.source",candidate,"-d",root,"--confirm"])[1]
            @test read(sourcepath)==sourcebytes
            @test read(joinpath(root,"warps","demo.next.warp-map.json"))==read(candidate)
        end
    end
    @testset "Partial and inline config, nearest scope, explicit false" begin
        for contents in ("# keep me\n[warp.removal]\nrequire_confirmation=false\n",
                         "# keep me\nwarp = { removal = { require_confirmation = false } }\n")
            mktempdir() do root
                mkpath(joinpath(root,".recur")); nested=joinpath(root,"nested"); mkpath(nested)
                path=joinpath(root,".recur","config.toml"); write(path,contents)
                ok,_,err=invoke(["init","-d",nested,"--json"]); @test ok
                @test !isdir(joinpath(nested,".recur"))
                if !ok; @info err; return; end
                cfg=TOML.parsefile(path)
                @test cfg["warp"]["creation"]["directory"]=="warps"
                @test isfile(joinpath(root,cfg["warp"]["creation"]["template"]))
                @test !cfg["warp"]["removal"]["require_confirmation"]
                @test cfg["warp"]["removal"]["require_committed_snapshot"]
                @test occursin("# keep me",read(path,String))
                before=snapshot(root)
                @test invoke(["init","-d",nested,"--json"])[1]
                @test snapshot(root)==before
                ok,out,_=core(["config","-d",nested,"--json"]); @test ok
                if ok
                    policy=JSON3.read(out)
                    @test !policy["removal"]["require_confirmation"]
                    @test !policy["removal_guards_enforced"]
                end
            end
        end
    end
    @testset "Init failures publish no configuration or template" begin
        for value in ("../escape.json", "blocked/template.json", ".recur/config.toml")
            mktempdir() do root
                mkpath(joinpath(root,".recur")); write(joinpath(root,"blocked"),"ordinary file")
                path=joinpath(root,".recur","config.toml")
                write(path,"[warp.creation]\ntemplate = \"$value\"\n")
                before=snapshot(root)
                @test !invoke(["init","-d",root,"--json"])[1]
                @test snapshot(root)==before
            end
        end
        for contents in ("warp = 1", "[warp]\ncreation = false", "[warp]\nremoval = false",
                         "[warp.creation]\ndirectory = 42", "[warp.removal]\nrequire_pushed_ref = 42")
            mktempdir() do root
                mkpath(joinpath(root,".recur")); write(joinpath(root,".recur","config.toml"),contents)
                before=snapshot(root)
                @test !invoke(["init","-d",root,"--json"])[1]
                @test snapshot(root)==before
            end
        end
    end
    @testset "Templates cannot clone identities; completion and rename preserve them" begin
        mktempdir() do root
            @test invoke(["init","-d",root])[1]
            templatepath=joinpath(root,".recur","warp-template.json")
            template=mutablejson(templatepath)
            injected="01991a00-0000-7000-8000-000000000001"
            template["bubble_uuid"]=injected
            for s in template["required_slices"]; s["slice_uuid"]=injected; end
            write(templatepath,JSON3.write(template))
            preview=snapshot(root)
            ok,out,_=invoke(["create","demo.test","--goal","Identity","-d",root,"--json"])
            @test ok
            @test snapshot(root)==preview
            @test invoke(["create","demo.test","--goal","Identity","-d",root,"--confirm"])[1]
            path=joinpath(root,"warps","demo.test.warp-map.json"); original=read(path)
            map=mutablejson(path)
            @test map["bubble_uuid"]!=injected
            @test all(s["slice_uuid"]!=injected for s in map["required_slices"])
            for verb in ("map","show","slices","merge")
                ok,out,_=core([verb,"demo.test","-d",root,"--json"]); @test ok
                if ok
                    @test JSON3.read(out)["bubble_uuid"]==map["bubble_uuid"]
                    if verb in ("show","slices")
                        @test JSON3.read(out)["slices"][1]["slice_uuid"]==map["required_slices"][1]["slice_uuid"]
                    end
                end
            end
            @test invoke(["complete","demo.test","slice-0","--attempt-id","checked",
                "--result-hash","test-result","--evidence","baseline=observed-test",
                "-d",root,"--confirm"])[1]
            @test read(path)==original
            mktempdir() do moved
                renamed=deepcopy(map); renamed["warp_id"]="demo.renamed"
                write(joinpath(moved,"demo.renamed.warp-map.json"),JSON3.write(renamed))
                before=snapshot(moved)
                ok,out,_=core(["show","demo.renamed","-d",moved,"--json"]); @test ok
                if ok
                    @test JSON3.read(out)["bubble_uuid"]==map["bubble_uuid"]
                    @test JSON3.read(out)["slices"][1]["slice_uuid"]==map["required_slices"][1]["slice_uuid"]
                end
                @test snapshot(moved)==before
            end
            for bad in ("bad", "01991a00-0000-4000-8000-000000000001", "", nothing, 42)
                for field in ("bubble_uuid","slice_uuid")
                    invalid=deepcopy(map)
                    (field=="bubble_uuid" ? invalid : invalid["required_slices"][1])[field]=bad
                    write(path,JSON3.write(invalid)); before=snapshot(root)
                    @test !core(["show","demo.test","-d",root,"--json"])[1]
                    @test snapshot(root)==before
                end
            end
            for duplicate in (map["bubble_uuid"],map["required_slices"][2]["slice_uuid"])
                invalid=deepcopy(map); invalid["required_slices"][1]["slice_uuid"]=duplicate
                write(path,JSON3.write(invalid)); before=snapshot(root)
                ok,_,err=core(["show","demo.test","-d",root,"--json"])
                @test !ok
                @test occursin("duplicate UUID",err)
                @test snapshot(root)==before
            end
            delete!(map,"bubble_uuid")
            for s in map["required_slices"]; delete!(s,"slice_uuid"); end
            write(path,JSON3.write(map)); before=snapshot(root)
            ok,out,_=core(["show","demo.test","-d",root,"--json"]); @test ok
            if ok
                @test !haskey(JSON3.read(out),"bubble_uuid")
                @test !haskey(JSON3.read(out)["slices"][1],"slice_uuid")
            end
            @test snapshot(root)==before
        end
    end
end
end
