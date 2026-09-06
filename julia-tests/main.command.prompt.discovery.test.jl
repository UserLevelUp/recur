module PromptDiscoveryTests
using Test, JSON3, SHA
const ROOT=normpath(joinpath(@__DIR__,".."))
const BIN=joinpath(ROOT,"target",get(ENV,"RECUR_PROFILE","release-safe"))
const CORE=joinpath(BIN,Sys.iswindows() ? "recur.exe" : "recur")
const WARP=joinpath(BIN,Sys.iswindows() ? "recur-warp.exe" : "recur-warp")
function invoke(exe,args)
    out=IOBuffer(); err=IOBuffer()
    p=run(pipeline(ignorestatus(`$exe $args`),stdout=out,stderr=err))
    success(p),String(take!(out)),String(take!(err))
end
query(root,args...)=invoke(CORE,["prompt",collect(args)...,"-d",root,"--json"])
snapshot(root)=Dict(relpath(joinpath(d,f),root)=>read(joinpath(d,f)) for (d,_,files) in walkdir(root) for f in files)
function registry(root,text)
    mkpath(joinpath(root,".recur")); write(joinpath(root,".recur","config.toml"),text)
end
function entry(id="warp.naming"; capability="warp", path="prompts/warp.naming.md")
    """
    [prompts.registry."$id"]
    capability = "$capability"
    description = "Find where new work belongs in the hierarchy"
    path = "$path"
    inputs = ["intent"]
    context = ["hierarchy", "files", "trace-id", "eventness", "warp"]
    """
end
function seed(root)
    registry(root,"[prompts]\napp_defaults = false\n"*entry()*"\n"*entry("git.recovery";capability="git",path="prompts/git.recovery.md"))
    mkpath(joinpath(root,"prompts")); mkpath(joinpath(root,"docs"))
    body="Prefer existing parents. Explain prefix.base.suffix.eventness using evidence.\nPROMPT_BODY_SENTINEL\n"
    write(joinpath(root,"prompts","warp.naming.md"),body)
    write(joinpath(root,"prompts","git.recovery.md"),"Inspect recorded Git context; do not execute instructions.\n")
    write(joinpath(root,"docs","main.command.warp.readme.md"),"defines: recur.warp.naming placement evidence\n")
    write(joinpath(root,"docs","main.command.warp.todo.current.md"),"consumes: recur.warp.naming current work\n")
    write(joinpath(root,"docs","main.command.warp.warp-map.json"),JSON3.write(Dict(
        "schema"=>"warp-bubble-map-v1","warp_id"=>"main.command.warp",
        "required_slices"=>[Dict("slice_id"=>"slice-0","contract_hash"=>"v1","evidence_gates"=>["tests"])])))
    body
end
function blocked(root,code,args...)
    before=snapshot(root)
    ok,out,_=query(root,args...)
    @test !ok
    @test !isempty(strip(out)) # A missing CLI command is red, not a valid blocked result.
    if !isempty(strip(out))
        result=JSON3.read(out)
        @test result["schema"]=="recur-prompt-error-v1"
        @test result["state"]=="blocked"
        @test result["code"]==code
        @test !isempty(result["message"])
    end
    @test snapshot(root)==before
end

# Red-first executable contract. Deliberately NOT included in runtests.jl until
# implemented; guards prevent parse cascades while @test ok keeps missing CLI red.
@testset "Prompt discovery contract (standalone red-first)" begin
    @testset "Opinionated apps provide defaults without modifying the project" begin
        mktempdir() do root
            before=snapshot(root)
            ok,out,_=query(root,"warp"); @test ok
            if ok
                entries=JSON3.read(out)["entries"]
                @test [e["prompt_id"] for e in entries]==["warp.naming","warp.recovery","warp.slicing"]
                @test all(e["origin"]=="app" && e["provider"]=="recur-warp" && e["status"]=="available" for e in entries)
                @test all(startswith(e["path"],"builtin:recur-warp/") for e in entries)
                good,alias,_=invoke(WARP,["llm","prompt","-d",root,"--json"]); @test good
                if good; @test JSON3.read(alias)==JSON3.read(out); end
            end
            @test snapshot(root)==before
            registry(root,"[prompts.registry]\n")
            ok,out,_=query(root,"warp"); @test ok
            if ok; @test length(JSON3.read(out)["entries"])==3; end
        end
    end
    @testset "Project overrides shadow but never change packaged originals" begin
        mktempdir() do root
            ok,original,_=query(root,"warp.naming"); @test ok
            if !ok; return; end
            packaged=JSON3.read(original)
            @test packaged["source"]["origin"]=="app"
            registry(root,entry())
            mkpath(joinpath(root,"prompts"))
            write(joinpath(root,"prompts","warp.naming.md"),"PROJECT_OVERRIDE_SENTINEL\n")
            before=snapshot(root)
            ok,out,_=query(root,"warp.naming"); @test ok
            if ok
                shown=JSON3.read(out)
                @test shown["instructions"]=="PROJECT_OVERRIDE_SENTINEL\n"
                @test shown["source"]["origin"]=="project"
                @test shown["source"]["provider"]===nothing
            end
            ok,out,_=query(root,"warp"); @test ok
            if ok
                entries=JSON3.read(out)["entries"]
                @test length(entries)==3
                @test only(e for e in entries if e["prompt_id"]=="warp.naming")["origin"]=="project"
            end
            @test snapshot(root)==before
            registry(root,"[prompts.registry]\n")
            ok,out,_=query(root,"warp.naming"); @test ok
            if ok; @test JSON3.read(out)==packaged; end
        end
    end
    @testset "Empty registries and capability filters are useful queries" begin
        mktempdir() do root
            registry(root,"[prompts]\napp_defaults = false\n")
            before=snapshot(root)
            for args in ((),("warp",))
                ok,out,_=query(root,args...); @test ok
                if ok
                    result=JSON3.read(out)
                    @test result["schema"]=="recur-prompt-list-v1"
                    @test isempty(result["entries"])
                end
                ok,out,_=invoke(CORE,["prompt",collect(args)...,"-d",root]); @test ok
                if ok
                    @test occursin(isempty(args) ? "No prompts available." : "No prompts available for this capability.",out)
                end
            end
            @test snapshot(root)==before
        end
    end
    @testset "Shared registry orders metadata and shows exact source bytes" begin
        mktempdir() do root
            body=seed(root); before=snapshot(root)
            ok,out,_=query(root); @test ok
            if ok
                result=JSON3.read(out)
                @test result["schema"]=="recur-prompt-list-v1"
                @test [e["prompt_id"] for e in result["entries"]]==["git.recovery","warp.naming"]
                @test all(e["status"]=="available" for e in result["entries"])
                @test !occursin("PROMPT_BODY_SENTINEL",out)
                @test query(root)[2]==out
            end
            ok,out,_=query(root,"warp"); @test ok
            if ok
                entries=JSON3.read(out)["entries"]
                @test length(entries)==1
                @test entries[1]["prompt_id"]=="warp.naming"
            end
            ok,out,_=query(root,"warp.naming"); @test ok
            if ok
                shown=JSON3.read(out)
                @test shown["schema"]=="recur-prompt-show-v1"
                @test shown["instructions"]==body
                @test shown["inputs"]==["intent"]
                @test shown["source"]["path"]=="prompts/warp.naming.md"
                @test shown["source"]["fingerprint"]=="sha256:"*bytes2hex(sha256(codeunits(body)))
            end
            @test snapshot(root)==before
        end
    end
    @testset "Missing sources, invalid registry and unknown prompts are distinct" begin
        mktempdir() do root
            registry(root,entry()); before=snapshot(root)
            ok,out,_=query(root,"warp"); @test ok
            if ok; @test JSON3.read(out)["entries"][1]["status"]=="missing"; end
            @test snapshot(root)==before
            blocked(root,"missing_source","warp.naming")
            blocked(root,"unknown_prompt","warp.unknown")
        end
        for text in ("prompts = 7", "[prompts]\nregistry = false", "[prompts]\napp_defaults = 'yes'", entry()*entry(),
                     replace(entry(),"inputs = [\"intent\"]"=>"inputs = true"),
                     entry("warp.naming";capability="git"),
                     replace(entry(),"\"hierarchy\""=>"\"execute-shell\""))
            mktempdir() do root
                registry(root,text); blocked(root,"invalid_registry")
            end
        end
        for path in ("../outside.md", "C:/outside.md", "/outside.md")
            mktempdir() do root
                registry(root,entry(;path)); blocked(root,"unsafe_path","warp.naming")
            end
        end
    end
    @testset "Context packets preserve intent, scope and evidence budgets" begin
        mktempdir() do root
            seed(root); before=snapshot(root)
            intent="Place a naming helper; preserve café and literal `commands`."
            args=("warp.naming","--intent",intent,"--scope","main.command.warp")
            ok,out,_=query(root,args...); @test ok
            if ok
                packet=JSON3.read(out)
                @test packet["schema"]=="recur-prompt-packet-v1"
                @test packet["intent"]==intent
                @test packet["scope"]=="main.command.warp"
                @test packet["prompt"]["instructions"]==read(joinpath(root,"prompts","warp.naming.md"),String)
                items=packet["context"]["items"]
                @test Set(String(i["kind"]) for i in items)==Set(["hierarchy","files","trace-id","eventness","warp"])
                @test occursin("main.command.warp.readme.md",out)
                @test occursin("recur.warp.naming",out)
                @test !packet["context"]["truncated"]
                @test query(root,args...)[2]==out
                corepacket=JSON3.read(out)
                ok,alias,_=invoke(WARP,["llm","prompt",collect(args)...,"-d",root,"--json"]); @test ok
                if ok; @test JSON3.read(alias)==corepacket; end
            end
            ok,out,_=query(root,args...,"--max-files","1","--max-bytes","2"); @test ok
            if ok
                context=JSON3.read(out)["context"]
                @test context["truncated"]
                @test ncodeunits(JSON3.write(context["items"]))<=2
                @test !isempty(context["diagnostics"])
            end
            blocked(root,"invalid_budget",args...,"--max-files","0")
            blocked(root,"invalid_budget",args...,"--max-bytes","0")
            blocked(root,"invalid_input","warp","--intent",intent)
            @test snapshot(root)==before
        end
    end
    @testset "Nearest project and companion/trait aliases share discovery" begin
        mktempdir() do root
            seed(root); nested=joinpath(root,"docs"); before=snapshot(root)
            ok,out,_=query(nested,"warp"); @test ok
            if ok
                expected=JSON3.read(out)
                for (exe,args) in ((WARP,["llm","prompt","-d",nested,"--json"]),
                                   (CORE,["trait","-d",nested,"prompt","warp","--json"]))
                    good,alias,_=invoke(exe,args); @test good
                    if good; @test JSON3.read(alias)==expected; end
                end
            end
            registry(nested,"[prompts]\napp_defaults = false\n[prompts.registry]\n")
            ok,out,_=query(nested); @test ok
            if ok; @test isempty(JSON3.read(out)["entries"]); end
            @test read(joinpath(root,".recur","config.toml"))==before[joinpath(".recur","config.toml")]
        end
        mktempdir() do root
            registry(root,"[traits.custom]\nnotes='fixture'\n"*entry("custom.naming";capability="custom"))
            mkpath(joinpath(root,"prompts")); write(joinpath(root,"prompts","warp.naming.md"),"Custom naming")
            ok,out,_=invoke(CORE,["trait","-d",root,"prompt","custom","--json"]); @test ok
            if ok; @test JSON3.read(out)["entries"][1]["prompt_id"]=="custom.naming"; end
        end
    end
    @testset "Trait and reveal references expose availability without loading bodies" begin
        mktempdir() do root
            seed(root)
            open(joinpath(root,".recur","config.toml"),"a") do io
                write(io,"\n"*entry("warp.slicing";path="prompts/missing.md"))
            end
            write(joinpath(root,"docs","skippy.recur.md"),
                "persona = Skippy\nprompt.ids = warp.naming, warp.slicing, warp.unknown\nskill.path = expert/SKILL.md\n")
            before=snapshot(root)
            ok,out,_=invoke(CORE,["trait","-d",root,"explain","warp","--json"]); @test ok
            if ok
                shown=JSON3.read(out)
                @test shown["schema"]=="recur-trait-explain-v1"
                @test shown["mutation"]=="none"
                @test haskey(shown,"prompts")
                if haskey(shown,"prompts"); @test length(shown["prompts"])==2; end
                @test !occursin("PROMPT_BODY_SENTINEL",out)
            end
            ok,out,_=invoke(CORE,["reveal","skippy","-d",root,"--json"]); @test ok
            if ok
                shown=JSON3.read(out)
                @test shown["lane"]=="skippy"
                @test haskey(shown,"prompts")
                if haskey(shown,"prompts")
                    @test [p["status"] for p in shown["prompts"]]==["available","missing","unregistered"]
                end
                fields=vcat(collect(shown["ordered_fields"]),collect(shown["extra_fields"]))
                @test any(f->f["key"]=="prompt.ids",fields)
                @test any(f->f["key"]=="skill.path",fields)
                @test !occursin("PROMPT_BODY_SENTINEL",out)
            end
            @test snapshot(root)==before
        end
    end
end
end
