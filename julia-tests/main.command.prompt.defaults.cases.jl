# Included by the standalone discovery contract; uses its CLI and snapshot helpers.
include(joinpath(@__DIR__, "fixtures", "prompt-defaults", "cases.jl"))

@testset "Initial app prompts carry decision evidence" begin
    for case in DEFAULT_PROMPT_CASES
        @testset "$(case.id)" begin
            mktempdir() do root
                docs=joinpath(root,"docs"); mkpath(docs)
                for (path,body) in case.files
                    write(joinpath(docs,path),body)
                end
                write(joinpath(docs,"unrelated.private.readme.md"),"OUTSIDE_SCOPE_SENTINEL")
                before=snapshot(root)
                if case.id=="stale-attention"
                    valid,inventory,_=invoke(CORE,["warp","--json","-d",docs])
                    @test valid
                    if valid
                        entries=JSON3.read(inventory)["entries"]
                        @test length(entries)==1
                        if length(entries)==1
                            @test entries[1]["state"]=="incomplete"
                            @test entries[1]["counts"]["pending"]==1
                            @test entries[1]["counts"]["covered"]==1
                        end
                    end
                end
                args=(case.prompt,"--intent",case.intent,"--scope",case.scope)
                ok,out,_=query(root,args...)
                @test ok # Missing prompt implementation remains an explicit failure.
                if ok
                    packet=JSON3.read(out)
                    @test packet["schema"]=="recur-prompt-packet-v1"
                    @test packet["intent"]==case.intent
                    @test packet["scope"]==case.scope
                    prompt=packet["prompt"]
                    @test prompt["prompt_id"]==case.prompt
                    @test !isempty(strip(prompt["instructions"]))
                    @test prompt["source"]["origin"]=="app"
                    @test prompt["source"]["provider"]=="recur-warp"
                    @test prompt["source"]["fingerprint"]=="sha256:"*bytes2hex(sha256(String(prompt["instructions"])))
                    context=packet["context"]
                    @test !context["truncated"]
                    evidence=JSON3.write(context["items"])
                    for marker in case.evidence
                        @test occursin(marker,evidence)
                    end
                    @test !occursin("OUTSIDE_SCOPE_SENTINEL",evidence)
                    @test all(haskey(item,"kind") && haskey(item,"path") && haskey(item,"fingerprint") && haskey(item,"data") for item in context["items"])
                    again,repeated,_=query(root,args...)
                    @test again
                    @test repeated==out

                    # A tiny valid budget must expose incompleteness explicitly.
                    bounded,small,_=query(root,args...,"--max-bytes","2")
                    @test bounded
                    if bounded
                        limited=JSON3.read(small)["context"]
                        @test limited["truncated"]
                        @test isempty(limited["items"])
                        @test !isempty(limited["diagnostics"])
                        @test ncodeunits(JSON3.write(limited["items"]))<=2
                    end
                end
                @test snapshot(root)==before
            end
        end
    end
end

@testset "Default naming evidence stays fresh and beneath the requested directory" begin
    mktempdir() do root
        registry(root,"[prompts.registry]\n") # Ancestor config supplies app defaults.
        nested=joinpath(root,"nested"); mkpath(nested)
        path=joinpath(nested,"product.checkout.readme.md")
        write(path,"defines: product.checkout\nFRESH_OLD café 日本語\n")
        write(joinpath(root,"product.checkout.secret.readme.md"),"ANCESTOR_SECRET_SENTINEL")
        args=("warp.naming","--intent","Place checkout work","--scope","product.checkout")
        before=snapshot(root)
        ok,out,_=query(nested,args...); @test ok
        @test snapshot(root)==before
        if ok
            old=JSON3.read(out)
            @test occursin("FRESH_OLD",JSON3.write(old["context"]["items"]))
            @test !occursin("ANCESTOR_SECRET_SENTINEL",out)
            write(path,"defines: product.checkout\nFRESH_NEW café 日本語\n")
            changed=snapshot(root)
            good,newout,_=query(nested,args...); @test good
            if good
                fresh=JSON3.read(newout)
                @test fresh["prompt"]==old["prompt"]
                @test fresh["context"]["items"]!=old["context"]["items"]
                @test occursin("FRESH_NEW",JSON3.write(fresh["context"]["items"]))
                @test !occursin("FRESH_OLD",newout)
                @test !occursin("ANCESTOR_SECRET_SENTINEL",newout)
            end
            good,limited,_=query(nested,args...,"--max-bytes","128"); @test good
            if good
                context=JSON3.read(limited)["context"]
                @test ncodeunits(JSON3.write(context["items"]))<=128
                @test context["truncated"]
                @test !isempty(context["diagnostics"])
            end
            @test snapshot(root)==changed
        end
    end
end
