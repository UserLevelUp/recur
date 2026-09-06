# consumes: recur.warp.discovery.inventory real CLI inventory contract
# consumes: recur.warp.project.discovery.slice.1 project-aware scoped inventory contract
include("runtests.setup.jl")

function wd_run(root, args...)
    out, err = IOBuffer(), IOBuffer()
    p = run(pipeline(ignorestatus(`$RECUR_BIN warp $args -d $root`), stdout=out, stderr=err))
    return success(p), String(take!(out)), String(take!(err))
end

function wd_map(root, id; mode="declared")
    mkpath(root)
    write(joinpath(root, id * ".warp-map.json"), JSON3.write(Dict(
        "schema"=>"warp-bubble-map-v1", "warp_id"=>id,
        "required_slices"=>[Dict("slice_id"=>"one","contract_hash"=>"v1",
            "evidence_mode"=>mode,"evidence_gates"=>["tests"])])))
end

function wd_layer(root, id; contract="v1", attempt="a", result="r1", reference="reviewed")
    write(joinpath(root, id * "." * attempt * ".warp-layer.json"), JSON3.write(Dict(
        "schema"=>"warp-slice-layer-v1","warp_id"=>id,"slice_id"=>"one",
        "contract_hash"=>contract,"attempt_id"=>attempt,"result_state"=>"accepted",
        "result_hash"=>result,"evidence"=>Dict("tests"=>[reference]))))
end

@testset "Warp inventory CLI" begin
    mktempdir() do root
        ok, text, err = wd_run(root, "--json")
        @test ok
        empty = JSON3.read(text)
        @test empty.schema == "warp-list-v1"
        @test isempty(empty.entries)
        @test empty.discovered == 0
        @test occursin("No matching Warp bubbles", wd_run(root)[2])

        wd_map(root, "demo.pending")
        wd_map(root, "demo.complete")
        wd_layer(root, "demo.complete")
        wd_map(root, "demo.stale")
        wd_layer(root, "demo.stale"; contract="old")
        wd_map(root, "demo.conflict")
        wd_layer(root, "demo.conflict")
        wd_layer(root, "demo.conflict"; attempt="b", result="different")
        wd_map(root, "demo.failed"; mode="checked")
        wd_layer(root, "demo.failed"; reference="evidence:missing.json")
        wd_map(joinpath(root, ".recur"), "demo.private")
        wd_map(joinpath(root, "nested"), "demo.nested")
        write(joinpath(root, "demo.marker.todo.current.md"), "no map")
        write(joinpath(root, "demo.invalid.warp-map.json"), "{invalid")
        before = Dict(relpath(joinpath(d,f),root)=>read(joinpath(d,f))
            for (d,_,files) in walkdir(root) for f in files)
        ok, text, err = wd_run(root, "--json")
        @test ok
        @test isempty(err)
        @test text == wd_run(root, "list", "--json")[2]
        @test text == wd_run(root, "--json")[2]
        result = JSON3.read(text)
        ids = String[e.warp_id for e in result.entries]
        @test issorted(ids)
        @test !("demo.complete" in ids)
        @test "demo.private" in ids
        @test !("demo.marker" in ids)
        @test "demo.nested" in ids
        @test result.errors == 1
        @test result.discovered == 8
        entries = Dict(String(e.warp_id)=>e for e in result.entries)
        @test entries["demo.invalid"].state == "error"
        @test occursin("parse", entries["demo.invalid"].error)
        @test entries["demo.stale"].state == "exploded"
        @test entries["demo.conflict"].state == "exploded"
        @test entries["demo.failed"].state == "blocked"
        @test entries["demo.pending"].counts.pending == 1
        @test length(JSON3.read(wd_run(root,"list","--all","--json")[2]).entries) == 8
        @test occursin("demo.invalid", wd_run(root)[2])
        @test occursin("error:", wd_run(root)[2])
        after = Dict(relpath(joinpath(d,f),root)=>read(joinpath(d,f))
            for (d,_,files) in walkdir(root) for f in files)
        @test before == after

        wd_map(joinpath(root,"duplicate"), "demo.pending")
        dup = JSON3.read(wd_run(root,"--json")[2])
        entries = filter(e->e.warp_id=="demo.pending",dup.entries)
        @test length(entries) == 2
        @test all(e->e.state == "incomplete", entries)
        @test length(unique(e.scope for e in entries)) == 2
        @test !wd_run(joinpath(root,"absent"),"--json")[1]
    end
    mktempdir() do root
        wd_map(root,"demo.ring")
        ring=Dict("schema"=>"warp-ring-map-v1","warp_id"=>"demo.ring",
            "coordinator_domain"=>"parent","projection_depth"=>1,
            "domains"=>[Dict("domain_id"=>"parent","relative_root"=>".",
                "role"=>"coordinator","warp_id"=>"demo.ring","required_state"=>"complete")],"subscriptions"=>[])
        write(joinpath(root,"demo.ring.warp-ring.json"),JSON3.write(ring))
        listed=JSON3.read(wd_run(root,"--json")[2])
        ringentry=only(filter(e->e.warp_id=="demo.ring",listed.entries))
        @test ringentry.kind == "ring"
        @test ringentry.error === nothing
        projection=JSON3.read(wd_run(root,"merge","demo.ring","--json")[2])
        @test ringentry.state == projection.state
        @test ringentry.counts == projection.counts
        wd_layer(root,"demo.ring")
        @test isempty(JSON3.read(wd_run(root,"--json")[2]).entries)
        @test length(JSON3.read(wd_run(root,"list","--all","--json")[2]).entries)==1
    end
end


@testset "Project-aware discovery policy and scope" begin
    mktempdir() do root
        wd_map(joinpath(root,"docs"),"demo.real")
        wd_map(joinpath(root,".recur","work"),"demo.hidden")
        wd_map(joinpath(root,"fixtures","one"),"demo.fixture")
        wd_map(joinpath(root,"target"),"demo.build")
        wd_map(joinpath(root,"other"),"demo.other")
        result=JSON3.read(wd_run(root,"--json")[2])
        @test Set(e.warp_id for e in result.entries)==Set(["demo.real","demo.hidden","demo.other"])
        @test result.errors==0
        @test result.discovery_policy.source=="defaults"
        @test length(JSON3.read(wd_run(root,"list","--scan-all","--json")[2]).entries)==5
        config=joinpath(root,".recur","config.toml")
        write(config,"[warp.discovery]\nroots = [\"docs\", \".recur/work\"]\nexclude_dirs = [\"fixtures\", \"target\"]\n")
        configured=JSON3.read(wd_run(root,"--json")[2])
        @test Set(e.warp_id for e in configured.entries)==Set(["demo.real","demo.hidden"])
        @test occursin("config.toml",configured.discovery_policy.source)
        @test length(JSON3.read(wd_run(joinpath(root,"docs"),"--json")[2]).entries)==1
        @test isempty(JSON3.read(wd_run(joinpath(root,"other"),"--json")[2]).entries)
        @test length(JSON3.read(wd_run(root,"list","--scan-all","--json")[2]).entries)==5
        # Child config wins, and configured exclusions replace defaults.
        mkpath(joinpath(root,"other",".recur"))
        write(joinpath(root,"other",".recur","config.toml"),"[warp.discovery]\nroots=[\".\"]\nexclude_dirs=[]\n")
        @test length(JSON3.read(wd_run(joinpath(root,"other"),"--json")[2]).entries)==1
        for bad in ("roots=[\"../escape\"]", "roots=[]", "roots=[123]", "roots=[\"missing\"]",
                    "exclude_dirs=[\"bad/path\"]", "exclude_dirs=3")
            write(config,"[warp.discovery]\n" * bad * "\n")
            @test !wd_run(root,"--json")[1]
        end
        write(config,"[warp.discovery]\nroots=[\".\",\"docs\"]\nexclude_dirs=[\"fixtures\",\"target\"]\n")
        @test length(JSON3.read(wd_run(root,"--json")[2]).entries)==3 # overlapping roots deduplicate
    end
    mktempdir() do root
        # Same identity in nested scopes must not exchange layers or evidence.
        wd_map(root,"demo.shared")
        child=joinpath(root,"child")
        wd_map(child,"demo.shared")
        wd_layer(root,"demo.shared")
        remaining=JSON3.read(wd_run(root,"--json")[2])
        @test remaining.errors==0
        @test length(remaining.entries)==1
        @test occursin("child",remaining.entries[1].scope)
        wd_layer(child,"demo.shared"; contract="old")
        states=JSON3.read(wd_run(root,"list","--all","--json")[2])
        @test Set(e.state for e in states.entries)==Set(["complete","exploded"])
        @test states.errors==0
        # Ring maps of the same identity also remain local to each scope.
        ring=Dict("schema"=>"warp-ring-map-v1","warp_id"=>"demo.shared",
            "coordinator_domain"=>"p","projection_depth"=>1,
            "domains"=>[Dict("domain_id"=>"p","relative_root"=>".","role"=>"coordinator",
            "warp_id"=>"demo.shared","required_state"=>"complete")],"subscriptions"=>[])
        for scope in (root,child)
            write(joinpath(scope,"demo.shared.warp-ring.json"),JSON3.write(ring))
        end
        rings=JSON3.read(wd_run(root,"list","--all","--json")[2])
        @test rings.errors==0
        @test Set(e.state for e in rings.entries)==Set(["complete","exploded"])
        @test all(e->e.kind=="ring",rings.entries)
    end
    mktempdir() do root
        out=IOBuffer()
        p=run(pipeline(ignorestatus(`$RECUR_BIN init -d $root`),stdout=out))
        @test success(p)
        config=read(joinpath(root,".recur","config.toml"),String)
        @test occursin("[warp.discovery]",config)
        @test occursin("exclude_dirs",config)
        wd_map(joinpath(root,".recur"),"demo.direct")
        @test only(JSON3.read(wd_run(root,"--json")[2]).entries).warp_id=="demo.direct"
    end
end

@testset "Discovery evidence scope and real repository root" begin
    mktempdir() do root
        scope=joinpath(root,".recur","work")
        wd_map(scope,"demo.checked"; mode="checked")
        wd_layer(scope,"demo.checked"; reference="evidence:check.json")
        fp(bytes)=begin
            h=UInt64(0xcbf29ce484222325)
            for b in bytes; h=(h ⊻ UInt64(b))*UInt64(0x100000001b3); end
            "fnv1a64:" * string(h,base=16,pad=16)
        end
        write(joinpath(scope,"source.txt"),"local source")
        result=JSON3.write(Dict("schema"=>"warp-external-result-v1","kind"=>"build","outcome"=>"passed","exit_code"=>0))
        write(joinpath(scope,"result.json"),result)
        evidence=Dict("schema"=>"warp-external-evidence-v1","kind"=>"build","producer"=>"fixture",
            "project"=>"scope-fixture","configuration"=>"Debug","platform"=>"test","executed_at_unix"=>1,
            "result_artifact"=>"result.json","result_fingerprint"=>fp(codeunits(result)),
            "source"=>Dict("revision"=>nothing,"dirty"=>true,"files"=>Dict("source.txt"=>fp(read(joinpath(scope,"source.txt"))))))
        write(joinpath(scope,"check.json"),JSON3.write(evidence))
        entry=only(JSON3.read(wd_run(root,"list","--all","--json")[2]).entries)
        @test entry.state=="complete"
        @test entry.evidence_status=="checked"
        @test isempty(JSON3.read(wd_run(root,"--json")[2]).entries)
        write(joinpath(scope,"source.txt"),"drift")
        stale=only(JSON3.read(wd_run(root,"--json")[2]).entries)
        @test stale.state=="blocked"
        @test stale.evidence_status=="stale"
    end
    root=dirname(@__DIR__)
    result=JSON3.read(wd_run(root,"--json")[2])
    @test result.errors==0
    @test all(e->e.state!="complete",result.entries) # no dependency on a particular live project
    @test all(e->all(p->!occursin("fixtures/",p),e.manifests),result.entries)
    # Exercise truly bare invocation with the repo as cwd, not merely -d docs.
    raw=read(Cmd(`$RECUR_BIN warp --json`;dir=root),String)
    @test JSON3.read(raw).entries==result.entries
    allrows=JSON3.read(wd_run(root,"list","--all","--json")[2])
    @test any(e->e.warp_id=="main.command.warp.evidence-integrity" && e.state=="complete",allrows.entries)
end
