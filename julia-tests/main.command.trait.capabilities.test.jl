# Capability traits are metadata, not command permissions or Rust interfaces.
include("runtests.setup.jl")

function ct_run(root,args...)
    out,err=IOBuffer(),IOBuffer()
    command=args[1]
    rest=collect(args[2:end])
    p=run(pipeline(ignorestatus(`$RECUR_BIN $command -d $root $rest`),stdout=out,stderr=err))
    return success(p),String(take!(out)),String(take!(err))
end

@testset "Capability traits" begin
    mktempdir() do root
        ok,text,_=ct_run(root,"trait","list","--json")
        @test ok
        listed=JSON3.read(text)
        @test all(k->haskey(listed,k),["warp","watch","merge","unmerge","git"])
        @test listed.unmerge.status=="proposed"
        @test isempty(listed.unmerge.commands)
        @test listed.warp.preference=="unspecified"
        @test !ispath(joinpath(root,".recur"))
        @test !ct_run(root,"trait","set","warp.preference","preferred")[1]
        @test ct_run(root,"trait","get","warp.preference","--json")[1]
        for name in ("warp","watch","merge","unmerge","git")
            ok,text,_=ct_run(root,"trait","explain",name,"--json")
            @test ok
            info=JSON3.read(text)
            @test info.name==name
            @test info.source=="built-in-defaults"
            @test info.mutation=="none"
            @test occursin("descriptive-only",info.catalog.effect)
        end
        @test !ct_run(root,"trait","explain","unknown")[1]
        @test ct_run(root,"init")[1]
        path=joinpath(root,".recur","config.toml")
        config=read(path,String)
        @test all(k->occursin("[traits."*k*"]",config),["warp","watch","merge","unmerge","git"])
        @test ct_run(root,"trait","set","warp.preference","preferred")[1]
        @test ct_run(root,"trait","set","warp.notes","Use slices for releases")[1]
        @test JSON3.read(ct_run(root,"trait","get","warp.preference","--json")[2]).value=="preferred"
        @test JSON3.read(ct_run(root,"trait","explain","warp","--json")[2]).source=="project-with-defaults"
        @test occursin("descriptive-only",ct_run(root,"trait","explain","warp")[2])
        @test ct_run(root,"trait","set","watch.preference","discouraged")[1]
        before=read(path)
        for (key,value) in (("warp.enabled","false"),("warp.status","proposed"),
                            ("warp.preference","yes"),("warp.notes","false"),("warp.preference.nested","true"))
            @test !ct_run(root,"trait","set",key,value)[1]
            @test read(path)==before
        end
        @test ct_run(root,"warp","--json")[1] # preference is not a gate
        @test ct_run(root,"trait","set","custom.enabled","true")[1]
        @test JSON3.read(ct_run(root,"trait","explain","custom","--json")[2]).kind=="configured"
        child=joinpath(root,"nested")
        mkpath(child)
        @test JSON3.read(ct_run(child,"trait","get","warp.preference","--json")[2]).value=="preferred"
        @test ct_run(child,"trait","set","git.preference","preferred")[1]
        @test !ispath(joinpath(child,".recur"))
        # Legacy config: virtual defaults never rewrite or erase existing settings.
        write(path,"[traits.custom]\nanswer=42\n[warp.discovery]\nroots=[\".\"]\n")
        before=read(path)
        old=JSON3.read(ct_run(root,"trait","list","--json")[2])
        @test old.custom.answer==42
        @test old.warp.preference=="unspecified"
        @test read(path)==before
        @test ct_run(root,"trait","set","git.notes","Local workflow")[1]
        @test occursin("roots",read(path,String))
        write(path,"[traits.warp]\nenabled=false\n")
        @test !ct_run(root,"trait","list","--json")[1]
        @test occursin("not supported",ct_run(root,"trait","list","--json")[3])
    end
end
