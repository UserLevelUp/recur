"""Executable policy parity and no-mutation contracts. Temporary roots only."""
module CompanionPolicyTests
using Test, JSON3
const ROOT=normpath(joinpath(@__DIR__,".."))
const PROFILE=get(ENV,"RECUR_PROFILE","release-safe")
const CORE=get(ENV,"RECUR_BIN",joinpath(ROOT,"target",PROFILE,"recur"*(Sys.iswindows() ? ".exe" : "")))
const ACTOR=get(ENV,"RECUR_WARP_BIN",joinpath(ROOT,"target",PROFILE,"recur-warp"*(Sys.iswindows() ? ".exe" : "")))
function invoke(exe,args)
    out=IOBuffer(); err=IOBuffer()
    process=run(pipeline(ignorestatus(`$exe $args`),stdout=out,stderr=err))
    return success(process),String(take!(out)),String(take!(err))
end
function snapshot(root)
    Dict(relpath(joinpath(dir,file),root)=>read(joinpath(dir,file))
        for (dir,_,files) in walkdir(root) for file in files)
end
function config(root,text)
    mkpath(joinpath(root,".recur")); write(joinpath(root,".recur","config.toml"),text)
end
const POLICY="[warp.suffixes]\ncomplete=['test.accepted']\ninteresting=['needs.review']\nblocked=['approval.wait']\nactive=['work.open']\n"
@testset "Companion suffix policy safety" begin
    @testset "Inherited compound policy, preview parity, bounded execution" begin
        mktempdir() do root
            config(root,POLICY); lane=joinpath(root,"docs"); mkpath(lane)
            write(joinpath(lane,"demo.lane.a.test.accepted.md"),"verified payload\n")
            write(joinpath(lane,"demo.lane.b.needs.review.md"),"preserve reasoning\n")
            write(joinpath(lane,"demo.lane2.a.test.accepted.md"),"outside exact lane\n")
            before=snapshot(root)
            ok,q,_=invoke(CORE,["warp","collapse-plan","demo.lane","-d",lane,"--json"])
            @test ok
            ok,a,_=invoke(ACTOR,["collapse","demo.lane","-d",lane,"--json"])
            @test ok
            if ok
                query=JSON3.read(q); actor=JSON3.read(a)
                for bucket in ("collapse_known","preserve_interesting","blockers","ambiguous")
                    @test sort([String(f["path"]) for f in query[bucket]]) == sort(String.(actor[bucket]))
                end
            end
            @test snapshot(root)==before
            ok,_,_=invoke(ACTOR,["collapse","demo.lane","-d",lane,"--json","--confirm"])
            @test ok
            archived=joinpath(lane,".recur","warp","archive","demo.lane","demo.lane.a.test.accepted.md")
            @test isfile(archived)
            if isfile(archived); @test read(archived,String)=="verified payload\n"; end
            @test !isfile(joinpath(lane,"demo.lane.a.test.accepted.md"))
            @test read(joinpath(lane,"demo.lane.b.needs.review.md"),String)=="preserve reasoning\n"
            @test read(joinpath(lane,"demo.lane2.a.test.accepted.md"),String)=="outside exact lane\n"
            @test isfile(joinpath(lane,".recur","warp","recur-warp.demo.lane.collapse.ack.json"))
        end
    end
    @testset "Nearest config replaces ancestor; normalization and longest suffix" begin
        mktempdir() do root
            config(root,POLICY); lane=joinpath(root,"docs"); mkpath(lane)
            config(lane,"[warp.suffixes]\ncomplete=[' ACCEPTED ']\ninteresting=['test.accepted']\n")
            write(joinpath(lane,"demo.lane.a.ACCEPTED.md"),"done")
            write(joinpath(lane,"demo.lane.b.test.accepted.md"),"retain")
            before=snapshot(root)
            ok,out,_=invoke(ACTOR,["collapse","demo.lane","-d",lane,"--json"])
            @test ok
            if ok
                @test String.(JSON3.read(out)["collapse_known"])==["demo.lane.a.ACCEPTED.md"]
                @test String.(JSON3.read(out)["preserve_interesting"])==["demo.lane.b.test.accepted.md"]
            end
            @test snapshot(root)==before
        end
    end
    @testset "Invalid policy fails before any writes" begin
        for bad in ("complete='accepted'","complete=[7]","complete=['complete','complete']",
                    "complete=['complete']\nblocked=['complete']","complete=['../escape']",
                    "complete=['bad..suffix']", "complete=[")
            mktempdir() do root
                config(root,"[warp.suffixes]\n"*bad*"\n")
                write(joinpath(root,"demo.lane.a.complete.md"),"must remain")
                before=snapshot(root)
                ok,_,_=invoke(CORE,["warp","collapse-plan","demo.lane","-d",root,"--json"])
                @test !ok
                for flags in (String[],["--confirm"])
                    ok,_,_=invoke(ACTOR,vcat(["collapse","demo.lane","-d",root,"--json"],flags))
                    @test !ok
                    @test snapshot(root)==before
                end
            end
        end
    end
    @testset "Every policy bucket agrees; unknown binary evidence fails closed" begin
        mktempdir() do root
            config(root,POLICY)
            for suffix in ("test.accepted","needs.review","approval.wait","work.open")
                write(joinpath(root,"demo.lane.item.$suffix.md"),"evidence")
            end
            before=snapshot(root)
            qok,q,_=invoke(CORE,["warp","collapse-plan","demo.lane","-d",root,"--json"])
            aok,a,_=invoke(ACTOR,["collapse","demo.lane","-d",root,"--json"])
            @test qok && aok
            if qok && aok
                for bucket in ("collapse_known","preserve_interesting","blockers","ambiguous")
                    @test sort([String(f["path"]) for f in JSON3.read(q)[bucket]]) == sort(String.(JSON3.read(a)[bucket]))
                end
            end
            @test snapshot(root)==before
            ok,_,_=invoke(ACTOR,["collapse","demo.lane","-d",root,"--confirm"])
            @test !ok
            @test snapshot(root)==before
        end
        mktempdir() do root
            config(root,"")
            write(joinpath(root,"demo.lane.a.complete.md"),UInt8[0xff,0xfe,0xfd])
            before=snapshot(root)
            for flags in (String[],["--confirm"])
                ok,_,_=invoke(ACTOR,vcat(["collapse","demo.lane","-d",root],flags))
                @test !ok
                @test snapshot(root)==before
            end
        end
    end
    @testset "Fresh policy, blockers, unknowns and collisions refuse safely" begin
        for reason in ("changed-policy","blocker","unknown","collision")
            mktempdir() do root
                config(root,"")
                write(joinpath(root,"demo.lane.a.complete.md"),"retain on refusal")
                args=["collapse","demo.lane","-d",root,"--json"]
                before=snapshot(root); ok,out,_=invoke(ACTOR,args)
                @test ok
                @test snapshot(root)==before
                if reason=="changed-policy"
                    config(root,"[warp.suffixes]\ncomplete=['accepted']\n")
                elseif reason=="blocker"
                    write(joinpath(root,"demo.lane.b.blocked.md"),"waiting")
                elseif reason=="unknown"
                    write(joinpath(root,"demo.lane.b.unknown.md"),"unclassified")
                else
                    archive=joinpath(root,".recur","warp","archive","demo.lane")
                    mkpath(archive); write(joinpath(archive,"demo.lane.a.complete.md"),"existing")
                end
                before=snapshot(root); ok,_,_=invoke(ACTOR,vcat(args,["--confirm"]))
                @test !ok
                @test snapshot(root)==before
            end
        end
    end
end
end
