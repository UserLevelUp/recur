# consumes: recur.warp.discovery.inventory real CLI inventory contract
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
        @test !("demo.private" in ids)
        @test !("demo.marker" in ids)
        @test "demo.nested" in ids
        @test result.errors == 1
        @test result.discovered == 7
        entries = Dict(String(e.warp_id)=>e for e in result.entries)
        @test entries["demo.invalid"].state == "error"
        @test occursin("parse", entries["demo.invalid"].error)
        @test entries["demo.stale"].state == "exploded"
        @test entries["demo.conflict"].state == "exploded"
        @test entries["demo.failed"].state == "blocked"
        @test entries["demo.pending"].counts.pending == 1
        @test length(JSON3.read(wd_run(root,"list","--all","--json")[2]).entries) == 7
        @test occursin("demo.invalid", wd_run(root)[2])
        @test occursin("error:", wd_run(root)[2])
        after = Dict(relpath(joinpath(d,f),root)=>read(joinpath(d,f))
            for (d,_,files) in walkdir(root) for f in files)
        @test before == after

        wd_map(joinpath(root,"duplicate"), "demo.pending")
        dup = JSON3.read(wd_run(root,"--json")[2])
        entry = only(filter(e->e.warp_id=="demo.pending",dup.entries))
        @test entry.state == "error"
        @test length(entry.manifests) == 2
        @test occursin("ambiguous", entry.error)
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
