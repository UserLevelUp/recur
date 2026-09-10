module RevealPersonaSkillsTests
using Test, TOML, JSON3
const ROOT=normpath(joinpath(@__DIR__,".."))
const BIN=joinpath(ROOT,"target",get(ENV,"RECUR_PROFILE","release-safe"))
const EXT=Sys.iswindows() ? ".exe" : ""
const CORE=get(ENV,"RECUR_BIN",joinpath(BIN,"recur"*EXT))
const COMPANION=get(ENV,"RECUR_REVEAL_BIN",joinpath(dirname(CORE),"recur-reveal"*EXT))
function invoke(exe,args)
    out=IOBuffer(); err=IOBuffer()
    p=run(pipeline(ignorestatus(`$exe $args`),stdout=out,stderr=err))
    success(p),String(take!(out)),String(take!(err))
end
function put(root, name, body)
    path=joinpath(root,name)
    mkpath(dirname(path))
    write(path,body)
end
snapshot(root)=Dict(relpath(joinpath(d,f),root)=>read(joinpath(d,f)) for (d,_,fs) in walkdir(root) for f in fs)
function reveal(root,args...)
    ok,out,err=invoke(CORE,["reveal","-d",root,"--json",args...])
    @test ok
    ok || error("reveal failed: $err $out")
    JSON3.read(out)
end

# The v2 discovery baseline deliberately uses the same short name for all three
# types, a standalone skill and a legacy untyped capsule. No global fixtures.
@testset "Reveal v2 independent agent persona skill discovery" begin
    mktempdir() do root
        for kind in ("agent","persona","skill")
            put(root,"$kind.shared.recur.md","artifact.type = $kind\n")
        end
        put(root,"skill.standalone.recur.md","artifact.type = skill\nskill.path = bodies/guide.md\n")
        put(root,"bodies/guide.md","BODY_MUST_NOT_BE_LOADED\n")
        put(root,"legacy.recur.md","agent = shared\npersona = shared\nskill.path = missing/SKILL.md\n")
        before=snapshot(root)
        for kind in ("agent","persona","skill")
            packet=reveal(root,"--type",kind)
            expected=kind=="skill" ? ["skill.shared","skill.standalone"] : ["$kind.shared"]
            @test [e["lane"] for e in packet["entries"]]==expected
            @test all(e["artifact"]["type"]==kind for e in packet["entries"])
            selected=reveal(root,"shared","--type",kind)
            @test selected["lane"]=="$kind.shared"
            @test selected["artifact"]["type"]==kind
        end
        @test reveal(root,"shared")["status"]=="ambiguous"
        @test reveal(root,"agent.shared","--type","skill")["status"]=="type-mismatch"
        standalone=reveal(root,"skill.standalone")
        @test standalone["artifact"]["type"]=="skill"
        @test !occursin("BODY_MUST_NOT_BE_LOADED",JSON3.write(standalone))
        @test reveal(root,"legacy")["artifact"]["status"]=="untyped"
        put(root,"agent.second.shared.recur.md","artifact.type = agent\n")
        @test reveal(root,"shared","--type","agent")["status"]=="ambiguous"
        @test reveal(root,"agent.shared","--type","agent")["lane"]=="agent.shared"
        # Remove only our added fixture from the comparison, not from disk.
        after=snapshot(root)
        delete!(after,"agent.second.shared.recur.md")
        @test after==before
        nested=joinpath(root,"nested")
        put(nested,"agent.inside.recur.md","artifact.type = agent\n")
        @test [e["lane"] for e in reveal(nested,"--type","agent")["entries"]]==["agent.inside"]
    end
end

# Red-first compatibility contract; integrate only after the implementation passes.
# Retain the v1 compatibility expectations. They do not establish v2 association
# or packet acceptance; see the coverage audit before interpreting this result.
@testset "Reveal persona skill defaults and companion" begin
    mktempdir() do root
        ok,_,_=invoke(CORE,["init","-d",root])
        @test ok
        config=TOML.parsefile(joinpath(root,".recur","config.toml"))
        @test haskey(config["reveal"],"agents")
        @test haskey(config["reveal"],"skills")
        @test haskey(config["reveal"],"personas")
        if haskey(config["reveal"],"personas")
            skippy=config["reveal"]["personas"]["skippy"]
            @test skippy["skills"]==["recur-expert","recur-warp"]
            @test skippy["guidance_level"]=="advanced"
        end
        companion=COMPANION
        @test isfile(companion)
        if isfile(companion)
            # Explicit fixture prevents dependence on user-global skills/personas.
            write(joinpath(root,".recur","config.toml"),"""
            [reveal.personas.skippy]
            skills = ["missing-warp-skill"]
            guidance_level = "advanced"
            [reveal.skills.missing-warp-skill]
            path = "skills/missing/SKILL.md"
            """)
            before=Dict(relpath(joinpath(d,f),root)=>read(joinpath(d,f)) for (d,_,fs) in walkdir(root) for f in fs)
            ok,out,_=invoke(companion,["next","skippy","-d",root,"--json"])
            @test !ok
            packet=JSON3.read(out)
            @test packet["schema"]=="recur-reveal-packet-v1"
            @test packet["state"]=="blocked"
            @test packet["skills"][1]["status"]=="missing"
            after=Dict(relpath(joinpath(d,f),root)=>read(joinpath(d,f)) for (d,_,fs) in walkdir(root) for f in fs)
            @test after==before
        end
    end
end

@testset "Reveal v2 companion and pure association queries" begin
    @test isfile(COMPANION)
    if isfile(COMPANION)
        mktempdir() do root
            put(root,".recur/config.toml","""
            # Preserve this project policy
            [reveal.agents.worker]
            capsule = "agent.worker"
            persona = "voice"
            skills = ["craft"]
            [reveal.personas.voice]
            skills = ["craft"]
            [reveal.skills.craft]
            path = "bodies/craft.md"
            """)
            put(root,"agent.worker.recur.md","artifact.type = agent\nverify = never-execute-this\n")
            put(root,"bodies/craft.md","BODY_ONLY_IN_EXPLICIT_PACKET\n")
            before=snapshot(root)
            shown=reveal(root,"agent.worker")
            @test length(shown["associations"])==3 # persona, skill, explicit capsule binding
            @test all(e["status"]=="available" for e in shown["associations"])
            @test !occursin("BODY_ONLY_IN_EXPLICIT_PACKET",JSON3.write(shown))
            @test any(e["lane"]=="voice" for e in reveal(root,"--type","persona")["entries"])
            skill=reveal(root,"craft","--type","skill")
            @test any(f["key"]=="skill.path" && f["value"]=="bodies/craft.md" for f in skill["extra_fields"])
            args=["next","worker","--type","agent","-d",root,"--json"]
            ok,out,_=invoke(COMPANION,args)
            @test ok
            if ok
                p=JSON3.read(out)
                @test p["state"]=="ready"
                @test p["persona"]=="voice"
                @test length(p["skills"])==1
                @test p["execution"]=="not-run"
                @test occursin("BODY_ONLY_IN_EXPLICIT_PACKET",out)
                @test all(startswith(s["hash"],"sha256:") for s in p["sources"])
                _,out2,_=invoke(COMPANION,args)
                @test out==out2
            end
            @test snapshot(root)==before
            ok,out,_=invoke(COMPANION,vcat(args,["--max-files","0"]))
            @test !ok
            @test JSON3.read(out)["context"]["truncated"]
            @test JSON3.read(out)["state"]=="blocked"
            @test snapshot(root)==before
            ok,out,_=invoke(COMPANION,["init","-d",root,"--dry-run","--json"])
            @test ok
            @test JSON3.read(out)["mutation"]=="none"
            @test snapshot(root)==before
            ok,_,_=invoke(COMPANION,["init","-d",root,"--json"])
            @test ok
            @test snapshot(root)==before # all tables already exist; exact no-op
        end
    end
end
end
