module RevealPersonaSkillsTests
using Test, TOML, JSON3
const ROOT=normpath(joinpath(@__DIR__,".."))
const BIN=joinpath(ROOT,"target",get(ENV,"RECUR_PROFILE","release-safe"))
const EXT=Sys.iswindows() ? ".exe" : ""
function invoke(exe,args)
    out=IOBuffer(); err=IOBuffer()
    p=run(pipeline(ignorestatus(`$exe $args`),stdout=out,stderr=err))
    success(p),String(take!(out)),String(take!(err))
end
# Deliberately standalone red-first contract. Do not include in runtests.jl yet.
@testset "Reveal persona skill defaults and companion" begin
    mktempdir() do root
        ok,_,_=invoke(joinpath(BIN,"recur"*EXT),["init","-d",root])
        @test ok
        config=TOML.parsefile(joinpath(root,".recur","config.toml"))
        @test haskey(config["reveal"],"personas")
        if haskey(config["reveal"],"personas")
            skippy=config["reveal"]["personas"]["skippy"]
            @test skippy["skills"]==["recur-expert","recur-warp"]
            @test skippy["guidance_level"]=="advanced"
        end
        companion=joinpath(BIN,"recur-reveal"*EXT)
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
end
