@testset "Prompt context policy and evidence boundaries" begin
    mktempdir() do root
        ok,_,err=invoke(CORE,["prompt","warp.\e[31m","-d",root])
        @test !ok
        @test !occursin('\e',err)
        @test occursin("\\u{1b}",err)
    end
    mktempdir() do root
        docs=joinpath(root,"docs"); mkpath(docs)
        registry(root,"[docs]\ndir='docs/'\nsep='_'\n[warp.suffixes]\nactive=['doing']\ncomplete=['verified']\n")
        write(joinpath(docs,"product_checkout_retry.todo.doing.md"),
            "defines: product.checkout.retry\nconsumes: product.checkout\nUUID: 01990000-0000-7000-8000-000000000001\n")
        args=("warp.naming","--intent","Place retry","--scope","product_checkout")
        ok,out,_=query(docs,args...); @test ok
        if ok
            packet=JSON3.read(out); items=packet["context"]["items"]
            event=only(i for i in items if i["kind"]=="eventness")
            @test event["data"]["separator"]=="_"
            @test event["data"]["policy"]["active"]==["doing"]
            @test event["data"]["state"]=="doing"
            roles=only(i for i in items if i["kind"]=="trace-id")["data"]["roles"]
            @test [r["role"] for r in roles]==["define","consume"]
            @test all(!occursin("01990000",r["identifier"]) for r in roles)
            for (exe,aliasargs) in ((WARP,["llm","prompt",collect(args)...,"-d",docs,"--json"]),
                                    (CORE,["trait","prompt",collect(args)...,"-d",docs,"--json"]))
                good,alias,_=invoke(exe,aliasargs); @test good
                if good; @test JSON3.read(alias)==packet; end
            end
        end
        # Explicit query separators also pass through all adapters.
        write(joinpath(docs,"alt.root.readme.md"),"EXPLICIT_SEPARATOR_EVIDENCE")
        args=("warp.naming","--intent","Place work","--scope","alt.root","--sep",".")
        ok,out,_=query(docs,args...); @test ok
        if ok
            @test occursin("EXPLICIT_SEPARATOR_EVIDENCE",out)
            good,alias,_=invoke(WARP,["llm","prompt",collect(args)...,"-d",docs,"--json"]); @test good
            if good; @test JSON3.read(alias)==JSON3.read(out); end
            good,alias,_=invoke(CORE,["trait","prompt",collect(args)...,"-d",docs,"--json"]); @test good
            if good; @test JSON3.read(alias)==JSON3.read(out); end
        end
    end
    mktempdir() do root
        for name in ("main.a.readme.md","main.b.readme.md","main.c.readme.md")
            write(joinpath(root,name),"defines: main.evidence\n"*name)
        end
        mkpath(joinpath(root,".hidden")); write(joinpath(root,".hidden","main.secret.md"),"HIDDEN_SENTINEL")
        args=("warp.naming","--intent","Inspect","--scope","main","--max-files","1")
        before=snapshot(root)
        ok,out,_=query(root,args...); @test ok
        if ok
            context=JSON3.read(out)["context"]
            @test context["truncated"]
            @test length(unique(i["path"] for i in context["items"]))==1
            @test !occursin("HIDDEN_SENTINEL",out)
            @test query(root,args...)[2]==out
        end
        @test snapshot(root)==before
        ok,out,_=query(joinpath(root,".hidden"),"warp.naming","--intent","Inspect","--scope","main"); @test ok
        if ok; @test occursin("HIDDEN_SENTINEL",out); end
    end
    @testset "Recovery projection reflects accepted, stale and conflicting receipts" begin
        mktempdir() do root
            case=only(c for c in DEFAULT_PROMPT_CASES if c.id=="stale-attention")
            for (path,body) in case.files; write(joinpath(root,path),body); end
            args=(case.prompt,"--intent",case.intent,"--scope",case.scope)
            receipt=joinpath(root,"main.command.resume.slice-0.accepted.warp-layer.json")
            original=read(receipt,String)
            function projection()
                ok,out,_=query(root,args...); @test ok
                ok || return nothing
                items=JSON3.read(out)["context"]["items"]
                only(i for i in items if i["kind"]=="warp" && haskey(i["data"],"map"))["data"]["projection"]
            end
            p=projection()
            if p!==nothing
                @test p["covered"]==["slice-0"]
                @test p["pending"]==["slice-1"]
            end
            write(receipt,replace(original,"[\"baseline passed\"]"=>"[]"))
            p=projection()
            if p!==nothing
                @test p["state"]=="blocked"
                @test !isempty(p["blocked"])
            end
            write(receipt,replace(original,"baseline-v1"=>"old-contract"))
            p=projection()
            if p!==nothing
                @test p["state"]=="exploded"
                @test isempty(p["covered"])
            end
            write(receipt,original)
            write(joinpath(root,"main.command.resume.slice-0.conflict.warp-layer.json"),
                replace(original,"\"attempt_id\":\"baseline\""=>"\"attempt_id\":\"other\"","baseline-result"=>"other-result"))
            p=projection()
            if p!==nothing
                @test p["state"]=="exploded"
                @test !isempty(p["conflicts"])
            end
        end
    end
    @testset "Checked evidence is disclosed without reading transitive sources" begin
        mktempdir() do root
            case=only(c for c in DEFAULT_PROMPT_CASES if c.id=="stale-attention")
            for (path,body) in case.files
                write(joinpath(root,path),replace(body,"baseline passed"=>"evidence:external.json"))
            end
            write(joinpath(root,"external.json"),"TRANSITIVE_SOURCE_SENTINEL")
            before=snapshot(root)
            ok,out,_=query(root,case.prompt,"--intent",case.intent,"--scope",case.scope); @test ok
            if ok
                context=JSON3.read(out)["context"]
                map=only(i for i in context["items"] if i["kind"]=="warp" && haskey(i["data"],"map"))
                @test map["data"]["projection"]===nothing
                @test any(occursin("additional bounded sources",d) for d in context["diagnostics"])
                @test !occursin("TRANSITIVE_SOURCE_SENTINEL",out)
            end
            @test snapshot(root)==before
            blocked(root,"invalid_budget",case.prompt,"--intent",case.intent,"--max-bytes","1")
        end
    end
end
