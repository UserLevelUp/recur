# Evidence integrity regressions; RECUR_EI_BASELINE=1 captures original behavior.
# consumes: recur.warp.evidence.integrity.external external result and source checks
# produces: recur.warp.evidence.integrity.verification tested lifecycle and map behavior
include("runtests.setup.jl")

const EI_BASELINE = get(ENV, "RECUR_EI_BASELINE", "0") == "1"

function ei_writer(args)
    bin = joinpath(dirname(RECUR_BIN), "recur-warp" * (Sys.iswindows() ? ".exe" : ""))
    out, err = IOBuffer(), IOBuffer()
    p = run(pipeline(ignorestatus(`$bin $args`), stdout=out, stderr=err))
    return success(p), String(take!(out)), String(take!(err))
end

function ei_map(root; gates=["tests"], dependencies=String[], mode="declared")
    map = Dict("schema"=>"warp-bubble-map-v1", "warp_id"=>"demo.release",
        "required_slices"=>[Dict("slice_id"=>"alpha", "contract_hash"=>"contract:alpha:v1",
        "depends_on"=>dependencies, "evidence_gates"=>gates,"evidence_mode"=>mode)])
    write(joinpath(root,"demo.release.warp-map.json"), JSON3.write(map))
end

function ei_fingerprint(bytes)
    h = UInt64(0xcbf29ce484222325)
    for b in bytes
        h = (h ⊻ UInt64(b)) * UInt64(0x100000001b3)
    end
    return "fnv1a64:" * string(h, base=16, pad=16)
end

function ei_evidence(root; executed=50, failed=0, skipped=0, exit_code=0, kind="test")
    write(joinpath(root,"source.txt"), "version1")
    result = Dict("schema"=>"warp-external-result-v1","kind"=>kind,"outcome"=>"passed",
        "exit_code"=>exit_code,"tests"=>Dict("discovered"=>executed+skipped,
        "executed"=>executed,"passed"=>executed-failed,"failed"=>failed,"skipped"=>skipped))
    bytes = Vector{UInt8}(JSON3.write(result))
    write(joinpath(root,"result.json"), bytes)
    evidence = Dict("schema"=>"warp-external-evidence-v1","kind"=>kind,
        "producer"=>"synthetic external fixture","project"=>"BasicGameEngine-like",
        "configuration"=>"Debug","platform"=>"x64","executed_at_unix"=>1,
        "result_artifact"=>"result.json","result_fingerprint"=>ei_fingerprint(bytes),
        "source"=>Dict("revision"=>nothing,"dirty"=>true,
            "files"=>Dict("source.txt"=>ei_fingerprint(read(joinpath(root,"source.txt"))))))
    write(joinpath(root,"evidence.json"), JSON3.write(evidence))
end

if !EI_BASELINE
    @testset "Whole-map prerequisites and stale contracts" begin
        mktempdir() do root
            map = Dict("schema"=>"warp-bubble-map-v1","warp_id"=>"demo.release",
                "required_slices"=>[
                    Dict("slice_id"=>"baseline","contract_hash"=>"baseline-v1","depends_on"=>String[],"evidence_gates"=>["audit"]),
                    Dict("slice_id"=>"final","contract_hash"=>"final-v1","depends_on"=>["baseline"],"evidence_gates"=>["tests"])])
            path=joinpath(root,"demo.release.warp-map.json")
            write(path,JSON3.write(map))
            layer=Dict("schema"=>"warp-slice-layer-v1","warp_id"=>"demo.release","slice_id"=>"final",
                "contract_hash"=>"final-v1","attempt_id"=>"r1","result_state"=>"accepted","result_hash"=>"result",
                "evidence"=>Dict("tests"=>["manual:test-run"]))
            write(joinpath(root,"demo.release.final.r1.warp-layer.json"),JSON3.write(layer))
            merge=["warp","merge","demo.release","-d",root,"--json"]
            ok,out,_=run_recur(merge)
            @test ok
            @test JSON3.read(out)["state"]=="blocked"
            @test JSON3.read(out)["pending"]==["baseline"]
            @test occursin("required dependencies are not covered",out)
            layer["slice_id"]="baseline"; layer["contract_hash"]="baseline-v1"; layer["evidence"]=Dict("audit"=>["manual:baseline"])
            write(joinpath(root,"demo.release.baseline.r1.warp-layer.json"),JSON3.write(layer))
            write(joinpath(root,"demo.release.baseline.complete.md"),"Historical baseline retained.\n")
            ok,out,_=run_recur(merge)
            @test ok
            @test JSON3.read(out)["state"]=="complete"
            @test isfile(joinpath(root,"demo.release.baseline.complete.md"))
            map["required_slices"][2]["contract_hash"]="final-v2"
            write(path,JSON3.write(map))
            ok,out,_=run_recur(merge)
            @test ok
            @test JSON3.read(out)["state"]=="exploded"
            @test JSON3.read(out)["stale_contract"][1]["slice_id"]=="final"
            map["required_slices"][1]["depends_on"]=["final"]
            write(path,JSON3.write(map))
            ok,_,err=run_recur(merge)
            @test !ok
            @test occursin("cycle",err)
            ok,_,err=ei_writer(["receipt","demo.release","final","--attempt-id","r2","-d",root])
            @test !ok
            @test occursin("cycle",err)
            map["required_slices"][1]["depends_on"]=["missing"]
            write(path,JSON3.write(map))
            ok,_,err=run_recur(merge)
            @test !ok
            @test occursin("invalid dependency",err)
        end
    end
end

if !EI_BASELINE
    @testset "External evidence and checked map gates" begin
        mktempdir() do root
            ei_map(root;mode="checked")
            ei_evidence(root)
            query = ["warp","evidence","evidence.json","-d",root,"--json"]
            ok, out, _ = run_recur(["warp","fingerprint","source.txt","-d",root,"--json"])
            @test ok
            @test JSON3.read(out)["files"]["source.txt"] == ei_fingerprint(read(joinpath(root,"source.txt")))
            ok, out, _ = run_recur(query)
            @test ok
            @test JSON3.read(out)["status"] == "checked"
            args = ["complete","demo.release","alpha","--attempt-id","r1","--result-hash","result1",
                "--evidence","tests=evidence:evidence.json","-d",root,"--json"]
            ok, _, _ = ei_writer(vcat(args,["--confirm"]))
            @test ok
            merge = ["warp","merge","demo.release","-d",root,"--json"]
            ok, out, _ = run_recur(merge)
            @test ok
            @test JSON3.read(out)["state"] == "complete"
            @test JSON3.read(out)["contract_status"] == "checked-gates-satisfied"
            capsule = "warp.id = demo.release\nwarp.root = .\nobserved.state = verified\nreadiness.slice = alpha\ngoals.now = Desired outcome, preserved\n"
            write(joinpath(root,"demo.recur.md"),capsule)
            ok, out, _ = run_recur(["reveal","demo","-d",root,"--json"])
            @test ok
            @test occursin("readiness.slice=alpha is stale",out)
            @test read(joinpath(root,"demo.recur.md"),String) == capsule
            write(joinpath(root,"demo.release.complete.md"), "Recorded final result.\n")
            for (kw, state) in [((executed=0,),"failed"),((failed=1,),"failed"),
                ((skipped=1,),"failed"),((exit_code=1,),"failed")]
                ei_evidence(root;kw...)
                ok, out, _ = run_recur(query)
                @test ok
                @test JSON3.read(out)["status"] == state
                ok, out, _ = run_recur(["warp","status","demo.release","-d",root,"--json"])
                @test ok
                @test JSON3.read(out)["verdict"] == "blocked"
                @test JSON3.read(out)["evidence_status"] == state
            end
            ei_evidence(root;skipped=1)
            ok, out, _ = run_recur(vcat(query,["--allow-skipped"]))
            @test ok
            @test JSON3.read(out)["status"] == "checked"
            ei_evidence(root)
            write(joinpath(root,"source.txt"), "version2")
            ok, out, _ = run_recur(merge)
            @test ok
            @test JSON3.read(out)["state"] == "blocked"
            @test JSON3.read(out)["evidence_status"] == "stale"
            ok, out, _ = run_recur(["reveal","demo","-d",root,"--json"])
            @test ok
            @test occursin("not supported by checked map gates",out)
            @test read(joinpath(root,"demo.recur.md"),String) == capsule
            ok, _, err = ei_writer(args)
            @test !ok
            @test occursin("stale",err)
            rm(joinpath(root,"result.json"))
            ok, out, _ = run_recur(query)
            @test ok
            @test JSON3.read(out)["status"] == "failed"
            @test occursin("missing artifact",out)
            ei_evidence(root;kind="build")
            ok, out, _ = run_recur(query)
            @test ok
            @test JSON3.read(out)["status"] == "checked"
            ei_evidence(root;kind="build",exit_code=1)
            ok, out, _ = run_recur(query)
            @test ok
            @test JSON3.read(out)["status"] == "failed"
            ei_evidence(root)
            outcome = Dict(JSON3.read(read(joinpath(root,"result.json"),String)))
            outcome[:kind]="scan"; outcome[:matches]=0
            bytes = Vector{UInt8}(JSON3.write(outcome))
            write(joinpath(root,"result.json"),bytes)
            evidence = Dict(JSON3.read(read(joinpath(root,"evidence.json"),String)))
            evidence[:kind]="scan"; evidence[:result_fingerprint]=ei_fingerprint(bytes)
            write(joinpath(root,"evidence.json"),JSON3.write(evidence))
            ok, out, _ = run_recur(query)
            @test ok
            @test JSON3.read(out)["status"] == "checked"
            outcome[:matches]=1; bytes=Vector{UInt8}(JSON3.write(outcome))
            write(joinpath(root,"result.json"),bytes)
            evidence[:result_fingerprint]=ei_fingerprint(bytes)
            write(joinpath(root,"evidence.json"),JSON3.write(evidence))
            ok, out, _ = run_recur(query)
            @test ok
            @test JSON3.read(out)["status"] == "failed"
        end
        mktempdir() do root
            ei_map(root;mode="checked")
            ok, _, err = ei_writer(["complete","demo.release","alpha","--attempt-id","r1",
                "--result-hash","r1","--evidence","tests=manual:50-passed","-d",root])
            @test !ok
            @test occursin("declared",err)
        end
    end
end

if !EI_BASELINE
    @testset "Policy-aware lifecycle receipt" begin
        mktempdir() do root
            ei_map(root)
            mkpath(joinpath(root,".recur"))
            write(joinpath(root,".recur/config.toml"), "[warp.suffixes]\ncomplete=['test.accepted']\n")
            args = ["receipt","demo.release","alpha","--attempt-id","run1","-d",root,"--json"]
            ok, out, _ = ei_writer(args)
            @test ok
            p = JSON3.read(out)
            @test p["state"] == "planned"
            @test endswith(p["path"], ".test.accepted.md")
            @test !isfile(joinpath(root,p["path"]))
            @test p["receipt"]["evidence_status"] == "declared"
            @test p["receipt"]["evidence_gates"] == ["tests"]
            ok, out, _ = ei_writer(vcat(args,["--confirm"]))
            @test ok
            @test isfile(joinpath(root,p["path"]))
            ok, out, _ = run_recur(["warp","status","demo.release.alpha","-d",root,"--json"])
            @test ok
            @test JSON3.read(out)["state_groups"]["complete"] == 1
            ok, _, err = ei_writer(vcat(args,["--confirm"]))
            @test !ok
            @test occursin("conflict",err)
        end
    end
end

if !EI_BASELINE
    @testset "Warp policy in reveal" begin
        mktempdir() do root
            mkpath(joinpath(root, ".recur"))
            write(joinpath(root, ".recur", "config.toml"), "[warp.suffixes]\ncomplete=['test.accepted']\nactive=['working']\n")
            write(joinpath(root, "demo.recur.md"), "persona = Test\n")
            ok, output, _ = run_recur(["reveal", "demo", "-d", root, "--json"])
            @test ok
            p = JSON3.read(output)["eventness_policy"]
            @test p["complete"] == ["test.accepted"]
            @test p["active"] == ["working"]
            @test endswith(p["source"], "config.toml")
            @test p["field_sources"]["blocked"] == "defaults"
            mkpath(joinpath(root, "docs"))
            ok, output, _ = run_recur(["warp", "config", "-d", joinpath(root,"docs"), "--json"])
            @test ok
            @test JSON3.read(output)["complete_suffixes"] == p["complete"]
            ok, output, _ = run_recur(["reveal", "demo", "-d", root])
            @test ok
            @test occursin("test.accepted", output)
            mkpath(joinpath(root,"docs/.recur"))
            write(joinpath(root,"docs/.recur/config.toml"), "[warp.suffixes]\ncomplete=['reviewed']\n")
            write(joinpath(root,"demo.recur.md"), "persona = Test\nwarp.root = docs\n")
            ok, output, _ = run_recur(["reveal","demo","-d",root,"--json"])
            @test ok
            @test JSON3.read(output)["eventness_policy"]["complete"] == ["reviewed"]
        end
    end
end

@testset "Warp evidence integrity: suffix diagnosis" begin
    mktempdir() do root
        file = joinpath(root, "demo.receipt.verified.md")
        write(file, "External build and tests reported passing.\n")
        ok, output, err = run_recur(["warp", "status", "demo.receipt", "-d", root, "--json"])
        @test !ok
        @test occursin("no eventness files found", err)
        if !EI_BASELINE
            @test occursin("demo.receipt.verified.md", err)
            @test occursin("complete", err)
            @test occursin("unsupported", err)
        end
        @test read(file, String) == "External build and tests reported passing.\n"
        @test readdir(root) == ["demo.receipt.verified.md"]
        ok, _, err = run_recur(["warp", "status", "demo.other", "-d", root, "--json"])
        @test !ok
        if !EI_BASELINE
            @test occursin("no matching lane artifacts", err)
            @test !occursin("demo.receipt.verified.md", err)
        end
        write(joinpath(root, "demo.receipt.complete.md"), "Recorded completion only.\n")
        ok, output, _ = run_recur(["warp", "status", "demo.receipt", "-d", root, "--json"])
        @test ok
        @test JSON3.read(output)["verdict"] == "optimum"
        if !EI_BASELINE
            @test JSON3.read(output)["recorded_state"] == "completion-present"
            @test JSON3.read(output)["evidence_status"] == "absent"
            @test JSON3.read(output)["contract_status"] == "not-declared"
        end
    end
end
