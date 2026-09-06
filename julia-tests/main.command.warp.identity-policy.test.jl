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

# Deliberately red, standalone acceptance contract. Not included in runtests.jl
# until implementation lands; no @test_broken hides missing functionality.
@testset "Warp identity and optional preservation policy (next Warp)" begin
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
end
