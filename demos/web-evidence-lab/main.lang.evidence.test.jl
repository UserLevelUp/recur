using Test, JSON3

# Requirements are hand-authored before the loader. E0 is the existing API's
# no-observed-evidence behavior; running this mode does not need a missing module.
const EVIDENCE_RED = get(ENV, "LANG_EVIDENCE_RED", "0") == "1"
if !EVIDENCE_RED
    include("main.lang.evidence.jl")
else
    include("main.lang.api.jl")
end
const EVIDENCE_BINARY = get(ENV, "RECUR_BIN", normpath(joinpath(@__DIR__, "../../target/release-safe/recur.exe")))
jsonfile(p, x) = write(p, JSON3.write(x))
function fixture_hash(bytes)
    h = UInt64(0xcbf29ce484222325)
    for b in bytes; h = (h ⊻ UInt64(b)) * UInt64(0x100000001b3); end
    "fnv1a64:" * string(h; base=16, pad=16)
end
function evidence_fixture(root)
    for name in ["spec.recur", "implementation.jl", "cases.jl", "config.toml", "runner.jl", "behavior.md"]
        write(joinpath(root, name), "fixture " * name)
    end
    header = Dict("identity"=>"assess.a", "meaning"=>"Assess explicit evidence", "input"=>Dict("local_identity"=>"assess.i(b)", "canonical_identity"=>"associate.o(b)"),
                  "output"=>Dict("local_identity"=>"assess.o(c)", "canonical_identity"=>"assess.o(c)"))
    transition = Dict("E0"=>"example.todo.current", "dE"=>"assess.a", "Ef"=>"example.complete")
    packet = Dict("source"=>"spec.recur", "source_hash"=>fixture_hash(read(joinpath(root,"spec.recur"))),
                  "header"=>[header], "footer"=>Dict("execution"=>"not-run", "events"=>[
                  Dict("scope"=>"assess", "requested_transition"=>Dict("current"=>transition["E0"], "slice"=>transition["dE"], "desired"=>transition["Ef"]),
                       "recorded"=>[Dict("path"=>"example.complete.md")])]), "unknown"=>"preserve me")
    packet["schema"]="recur-lang-query-v1"; packet["ir_schema"]="recur-lang-warp-ir-v1"
    packet["contracts"]=[]; packet["body"]=Dict("boundary_edges"=>[])
    packet["coverage"]=Dict("whole_source_validated"=>false,"excluded"=>["Runtime execution"])
    packet["footer"]["validation"]="sound-within-coverage"; packet["footer"]["findings"]=[]
    result = Dict("schema"=>"warp-external-result-v1", "kind"=>"test", "outcome"=>"passed", "exit_code"=>0,
                  "tests"=>Dict("discovered"=>2,"executed"=>2,"passed"=>2,"failed"=>0,"skipped"=>0))
    jsonfile(joinpath(root,"result.json"),result)
    inputs = ["spec.recur", "implementation.jl", "cases.jl", "config.toml", "runner.jl", "behavior.md"]
    evidence = Dict("schema"=>"warp-external-evidence-v1", "kind"=>"test", "producer"=>"fixture runner", "project"=>"fixture",
        "configuration"=>"isolated", "platform"=>"Windows", "executed_at_unix"=>1, "result_artifact"=>"result.json",
        "result_fingerprint"=>fixture_hash(read(joinpath(root,"result.json"))),
        "source"=>Dict("dirty"=>true,"revision"=>nothing,"files"=>Dict(p=>fixture_hash(read(joinpath(root,p))) for p in inputs)))
    jsonfile(joinpath(root,"evidence.json"),evidence)
    map = Dict("schema"=>"warp-bubble-map-v1", "warp_id"=>"fixture", "required_slices"=>[
        Dict("slice_id"=>"binding", "contract_hash"=>"contract:v1", "evidence_mode"=>"checked", "evidence_gates"=>["tests"],
             "gate_rules"=>Dict("tests"=>Dict("kind"=>"test","allow_skipped"=>false)))])
    jsonfile(joinpath(root,"fixture.warp-map.json"),map)
    attempt = Dict("id"=>"green-1", "phase"=>"green", "producer"=>"fixture runner", "runtime"=>"Julia fixture",
                   "gate_refs"=>Dict("tests"=>"evidence:evidence.json"), "cases"=>Dict("case.one"=>"passed","case.two"=>"passed"),
                   "source_hash"=>packet["source_hash"])
    association = Dict("id"=>"binding", "source"=>"spec.recur", "source_hash"=>packet["source_hash"], "scope"=>"assess.a",
        "aliases"=>Dict("assess.i(b)"=>"associate.o(b)","assess.o(c)"=>"assess.o(c)"), "warp_id"=>"fixture", "slice_id"=>"binding",
        "contract_hash"=>"contract:v1", "map"=>"fixture.warp-map.json", "requirements"=>Dict("req.binding"=>["case.one","case.two"]),
        "required_inputs"=>inputs, "gates"=>["tests"], "transition"=>transition, "current_attempt"=>"green-1", "attempts"=>[attempt])
    registry = Dict("schema"=>"lang-evidence-associations-v1", "associations"=>[association])
    (packet=packet, registry=registry, a=association, attempt=attempt, evidence=evidence, result=result, map=map)
end
function run_assessment(root, packet)
    if EVIDENCE_RED
        response=MainLangAPI.respond(MainLangAPI.HTTP.URI("/api/lang?id=inspector&scope=model.m");
            adapter=_ -> (code=0,stdout=JSON3.write(packet),stderr=""))
        response.status==200 || error("Legacy API baseline failed: "*String(response.body))
        data=JSON3.read(String(response.body),Dict{String,Any})
        isempty(data["observed_evidence"]) || error("E0 evidence contract changed")
        return Dict("status"=>"absent", "associations"=>[], "reasons"=>["E0 API supplies no observed evidence"])
    end
    LangEvidence.assess(root, "registry.json", packet; binary=EVIDENCE_BINARY)
end
const CASE_OBSERVATIONS = []
function evidence_case(id, expected, mutate; extra=(_,_)->nothing)
    @testset "$id" begin
        mktempdir() do root
            f = evidence_fixture(root)
            mutate(root,f)
            jsonfile(joinpath(root,"registry.json"),f.registry)
            before = Dict(p=>read(joinpath(root,p)) for p in readdir(root) if isfile(joinpath(root,p)))
            original = deepcopy(f.packet)
            report = run_assessment(root,f.packet)
            push!(CASE_OBSERVATIONS, Dict("id"=>id,"expected"=>expected,"actual"=>report["status"]))
            @test report["status"] == expected
            @test f.packet == original
            @test before == Dict(p=>read(joinpath(root,p)) for p in readdir(root) if isfile(joinpath(root,p)))
            !EVIDENCE_RED && extra(report,f)
        end
    end
end
try
@testset "Runtime evidence behavioral contract" begin
    evidence_case("identity.valid", "checked", (r,f)->nothing; extra=(x,f)->@test(!x["associations"][1]["acceptance"]["accepted"]))
    evidence_case("association.absent", "absent", (r,f)->empty!(f.registry["associations"]))
    evidence_case("association.declared", "declared", (r,f)->(f.attempt["gate_refs"]["tests"]="manual:observation"))
    for key in ["source","scope","slice_id","contract_hash","warp_id"]
        evidence_case("identity.wrong-"*key, "mismatched", (r,f)->(f.a[key]="wrong"))
    end
    evidence_case("identity.alias", "mismatched", (r,f)->(f.a["aliases"]["assess.i(b)"]="other.o(b)"))
    evidence_case("identity.transition", "mismatched", (r,f)->(f.a["transition"]["Ef"]="other.complete"))
    evidence_case("identity.producer", "mismatched", (r,f)->(f.attempt["producer"]="other runner"))
    evidence_case("identity.gates", "mismatched", (r,f)->(f.a["gates"]=["other"]))
    evidence_case("association.duplicate", "ambiguous", (r,f)->push!(f.registry["associations"],deepcopy(f.a)))
    evidence_case("association.schema", "malformed", (r,f)->(f.registry["schema"]="future"))
    evidence_case("association.missing-field", "malformed", (r,f)->delete!(f.a,"requirements"))
    evidence_case("history.no-current", "declared", (r,f)->(f.a["current_attempt"]=nothing))
    evidence_case("history.unknown-current", "malformed", (r,f)->(f.a["current_attempt"]="unknown"))
    evidence_case("history.duplicate", "ambiguous", (r,f)->push!(f.a["attempts"],deepcopy(f.attempt)))
    evidence_case("history.red-preserved", "checked", (r,f)->begin
        red=deepcopy(f.attempt); red["id"]="red-0"; red["phase"]="red"; red["cases"]["case.one"]="failed"
        pushfirst!(f.a["attempts"],red)
    end; extra=(x,f)->@test(x["associations"][1]["attempts"][1]["cases"]["case.one"]=="failed"))
    evidence_case("history.red-current", "failed", (r,f)->begin f.attempt["phase"]="red"; f.attempt["cases"]["case.one"]="failed" end)
    evidence_case("cases.missing", "mismatched", (r,f)->delete!(f.attempt["cases"],"case.two"))
    evidence_case("cases.failed", "failed", (r,f)->(f.attempt["cases"]["case.two"]="failed"))
    for path in ["spec.recur","implementation.jl","cases.jl","config.toml","runner.jl","behavior.md"]
        evidence_case("freshness."*path, "stale", (r,f)->write(joinpath(r,path),"changed"))
    end
    evidence_case("freshness.packet-hash", "stale", (r,f)->(f.packet["source_hash"]="fnv1a64:changed"))
    evidence_case("freshness.result", "stale", (r,f)->write(joinpath(r,"result.json"),JSON3.write(f.result)*" "))
    evidence_case("inputs.missing", "mismatched", (r,f)->begin
        delete!(f.evidence["source"]["files"],"config.toml"); jsonfile(joinpath(r,"evidence.json"),f.evidence)
    end)
    for (id, counts) in [("zero",(0,0,0,0,0)),("inconsistent",(3,2,2,0,0)),("failed",(2,2,1,1,0)),("skipped",(3,2,2,0,1))]
        evidence_case("counts."*id, "failed", (r,f)->begin
            f.result["tests"]=Dict(zip(["discovered","executed","passed","failed","skipped"],counts))
            jsonfile(joinpath(r,"result.json"),f.result)
            f.evidence["result_fingerprint"]=fixture_hash(read(joinpath(r,"result.json"))); jsonfile(joinpath(r,"evidence.json"),f.evidence)
        end)
    end
    evidence_case("native-ack.missing-test", "declared", (r,f)->begin
        jsonfile(joinpath(r,"ack.json"),Dict("status"=>"accepted","test_receipt"=>"missing.json"))
        f.attempt["gate_refs"]["tests"]="ack.json"
    end)
    evidence_case("native-ack.invalid-test", "malformed", (r,f)->begin
        write(joinpath(r,"evidence.json"),"not json")
    end)
    for path in ["../outside.json","C:/outside.json","a/../evidence.json","a\\evidence.json"]
        evidence_case("containment."*path, "malformed", (r,f)->(f.attempt["gate_refs"]["tests"]="evidence:"*path))
    end
    evidence_case("containment.symlink", "malformed", (r,f)->begin
        # Outside target is an existing file, never read as an in-root artifact.
        symlink(abspath(@__DIR__),joinpath(r,"escape"); dir_target=true)
        f.attempt["gate_refs"]["tests"]="evidence:escape/main.lang.evidence.test.jl"
    end)
    evidence_case("limits.json-bytes", "malformed", (r,f)->write(joinpath(r,"evidence.json"),repeat(" ",1048577)))
    evidence_case("limits.attempts", "malformed", (r,f)->(f.a["attempts"]=[merge(deepcopy(f.attempt),Dict("id"=>string(i))) for i in 1:17]))
    evidence_case("limits.cases", "malformed", (r,f)->(f.attempt["cases"]=Dict(string(i)=>"passed" for i in 1:129)))
    evidence_case("limits.associations", "malformed", (r,f)->(f.registry["associations"]=[deepcopy(f.a) for _ in 1:17]))
    evidence_case("containment.missing", "malformed", (r,f)->(f.attempt["gate_refs"]["tests"]="evidence:missing.json"))
    evidence_case("containment.directory", "malformed", (r,f)->begin mkdir(joinpath(r,"dir")); f.attempt["gate_refs"]["tests"]="evidence:dir" end)
    evidence_case("limits.input-bytes", "malformed", (r,f)->begin
        open(joinpath(r,"implementation.jl"),"w") do io; seek(io,64*1024*1024); write(io,UInt8(0)); end
    end)
    evidence_case("limits.files", "malformed", (r,f)->begin
        for i in 1:129
            p="input-$(i).txt"; write(joinpath(r,p),"x"); f.evidence["source"]["files"][p]=fixture_hash(codeunits("x"))
        end
        jsonfile(joinpath(r,"evidence.json"),f.evidence)
    end)
    evidence_case("purity.inert-commands", "checked", (r,f)->begin
        f.packet["header"][1]["binding"]="error(\"must not execute\")"
        write(joinpath(r,"capsule.recur.md"),"pull.first = execute forbidden runner")
    end)
    evidence_case("acceptance.explicit", "checked", (r,f)->begin
        jsonfile(joinpath(r,"fixture.binding.green.warp-layer.json"),Dict(
            "schema"=>"warp-slice-layer-v1","warp_id"=>"fixture","slice_id"=>"binding",
            "contract_hash"=>"contract:v1","attempt_id"=>"green-1","result_hash"=>"result:fixture",
            "result_state"=>"accepted","evidence"=>Dict("tests"=>["evidence:evidence.json"])))
    end; extra=(x,f)->@test(x["associations"][1]["acceptance"]["accepted"]))
end
finally
    if haskey(ENV,"LANG_EVIDENCE_OBSERVATIONS")
        jsonfile(ENV["LANG_EVIDENCE_OBSERVATIONS"],CASE_OBSERVATIONS)
    end
end
