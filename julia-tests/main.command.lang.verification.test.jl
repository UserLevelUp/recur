# Tests-first demonstrations, also exercised by the main regression runner.
# defines: recur.lang.verification.tests executable baseline fault demonstrations
module LangVerificationTests
using Test, JSON3, SHA, Dates

const REPO = normpath(joinpath(@__DIR__, ".."))
const EXT = Sys.iswindows() ? ".exe" : ""
const CORE = get(ENV, "RECUR_BIN", joinpath(REPO, "target", "release-safe", "recur" * EXT))
const ACTOR = get(ENV, "RECUR_LANG_BIN", joinpath(dirname(CORE), "recur-lang" * EXT))
const HELLO = read(joinpath(REPO, "demos/holdem-lab/main.holdem.01.hello.recur"), String)
const CIR = read(joinpath(REPO, "demos/holdem-lab/main.holdem.06.coordination.recur"), String)
const CASES = Pair{String,Function}[]
case(f::Function, id) = push!(CASES, id => f)
jsonput(root, name, value) = write(joinpath(root, name), JSON3.write(value))
jsonget(root, name) = JSON3.read(read(joinpath(root, name), String), Dict{String,Any})
function fingerprint(bytes)
    h = UInt64(0xcbf29ce484222325)
    for b in bytes; h = (h ⊻ UInt64(b)) * UInt64(0x100000001b3); end
    "fnv1a64:" * string(h; base=16, pad=16)
end
function invoke(exe, args...)
    out, err = IOBuffer(), IOBuffer()
    p = run(pipeline(ignorestatus(Cmd([exe, args...])), stdout=out, stderr=err); wait=false)
    timedwait(() -> process_exited(p), 30.0) == :ok || begin
        kill(p); wait(p); error("CLI exceeded 30 seconds: $exe $(join(args, ' '))")
    end
    wait(p)
    stdout, stderr = String(take!(out)), String(take!(err))
    packet = try JSON3.read(stdout, Dict{String,Any}) catch; nothing end
    (code=p.exitcode, packet=packet, stdout=stdout, stderr=stderr)
end
query(root, args...) = invoke(CORE, "lang", args..., "-d", root, "--json")
actor(root, args...) = invoke(ACTOR, "warp", "spec.recur", "hello.h",
    "--checked-contract", "policy.json", "--receipt", "attempt.json",
    "--eventness", "demo.holdem.hello.todo.current.md", "-d", root, "--json", args...)
evidence(root, args...) = query(root, "evidence", "spec.recur", "--scope", "hello.h",
    "--contract", "policy.json", "--receipt", "attempt.json", args...)
function packet(r, code=0)
    @test r.code == code
    @test r.packet !== nothing
    isnothing(r.packet) && error("Expected JSON, got $(r.stdout) / $(r.stderr)")
    r.packet
end
function inventory(root)
    result = Dict{String,Any}()
    for (dir, dirs, files) in walkdir(root; follow_symlinks=false)
        for name in vcat(dirs, files)
            p = joinpath(dir, name)
            result[replace(relpath(p, root), '\\'=>'/')] =
                islink(p) ? ("link", readlink(p)) : isdir(p) ? ("directory",) : ("file", read(p))
        end
    end
    result
end
function mutate(source, old, new; count=1)
    @assert occursin(old, source) "Mutation did not match source: $old"
    result = replace(source, old=>new; count=count)
    @assert source != result "Mutation did not change fixture"
    result
end
const E0 = "demo.holdem.hello.todo.current.md"
const EF = "demo.holdem.hello.complete.md"
const STATUS = ".recur/lang/checked/attempt-1/accepted.json"
const PREPARED = ".recur/lang/checked/attempt-1/prepared.json"
function checked_fixture(root)
    write(joinpath(root, "spec.recur"), HELLO)
    inputs = Dict("specification"=>["spec.recur"], "implementation"=>["implementation.jl"],
        "tests"=>["cases.jl"], "configuration"=>["config.toml"], "runner"=>["runner.jl"], "behavior"=>["behavior.md"])
    for name in ["implementation.jl", "cases.jl", "config.toml", "runner.jl", "behavior.md"]
        write(joinpath(root, name), "fixture " * name)
    end
    write(joinpath(root, E0), "Exact E0 bytes\r\nJosé\n")
    policy = Dict("schema"=>"recur-lang-checked-contract-v1", "contract_id"=>"verification.v1",
        "source"=>"spec.recur", "source_hash"=>fingerprint(codeunits(HELLO)), "scope"=>"hello.h",
        "aliases"=>Dict("hello.i(a)"=>"hello.i(a)", "hello.o(b)"=>"hello.o(b)"),
        "transition"=>Dict("current"=>"demo.holdem.hello.todo.current", "slice"=>"hello.h", "desired"=>"demo.holdem.hello.complete"),
        "inputs"=>inputs, "requirements"=>[Dict("id"=>"greeting", "cases"=>["case.hello"])])
    jsonput(root, "policy.json", policy)
    jsonput(root, "result.json", Dict("schema"=>"warp-external-result-v1", "kind"=>"test", "outcome"=>"passed", "exit_code"=>0,
        "tests"=>Dict("discovered"=>1, "executed"=>1, "passed"=>1, "failed"=>0, "skipped"=>0)))
    files = vcat(collect(Iterators.flatten(values(inputs))), ["policy.json"])
    jsonput(root, "evidence.json", Dict("schema"=>"warp-external-evidence-v1", "kind"=>"test", "producer"=>"verification fixture",
        "project"=>"isolated fixture", "configuration"=>"test", "platform"=>"local", "executed_at_unix"=>1,
        "result_artifact"=>"result.json", "result_fingerprint"=>fingerprint(read(joinpath(root, "result.json"))),
        "source"=>Dict("dirty"=>true, "revision"=>nothing, "files"=>Dict(p=>fingerprint(read(joinpath(root,p))) for p in files))))
    jsonput(root, "attempt.json", Dict("schema"=>"recur-lang-checked-receipt-v1", "attempt_id"=>"attempt-1",
        "contract_hash"=>fingerprint(read(joinpath(root, "policy.json"))), "source_hash"=>fingerprint(codeunits(HELLO)),
        "scope"=>"hello.h", "phase"=>"green", "producer"=>"verification fixture", "runtime"=>"Julia test fixture",
        "evidence"=>"evidence:evidence.json", "cases"=>[Dict("id"=>"case.hello", "outcome"=>"passed")]))
end
function accept_fixture(root)
    checked_fixture(root)
    pre = packet(evidence(root))
    @test pre["assessment"]["status"] == "checked"
    accepted = packet(actor(root, "--confirm"))
    @test accepted["action"]["ack"] == true
    @assert isfile(joinpath(root, STATUS)) "Positive control did not publish accepted fixture"
    accepted
end

# Queries: default controls, custom suffixes, isolation and CIR's frozen [] contract.
for suffix in ["complete", "todo.complete", ".complete.md", ".todo.complete.md"]
    case("query.suffix." * replace(suffix, "."=>"_")) do
        mktempdir() do parent
            root = joinpath(parent, "project"); mkpath(joinpath(root, ".recur"))
            write(joinpath(root, ".recur/config.toml"), "[status]\ncomplete_suffix = '$suffix'\n")
            state = lstrip(suffix, '.')
            declared = endswith(state,".md") ? state[1:end-3] : state
            write(joinpath(root, "spec.recur"), replace(HELLO,"demo.holdem.hello.complete"=>"demo.holdem.hello." * declared))
            filename = "demo.holdem.hello." * state * (endswith(state, ".md") ? "" : ".md")
            before = packet(query(root, "report", "spec.recur", "--eventness", state))
            @test isempty(before["header"])
            write(joinpath(parent, filename), "sibling must not leak")
            @test isempty(packet(query(root, "report", "spec.recur", "--eventness", state))["header"])
            write(joinpath(root, filename), "recorded, not accepted")
            snapshot = inventory(parent)
            @test length(packet(query(root, "report", "spec.recur", "--eventness", state))["header"]) == 1
            @test inventory(parent) == snapshot
        end
    end
end
case("query.cir.empty.eventness") do
    mktempdir() do root
        write(joinpath(root, "coordination.recur"), CIR)
        p = packet(query(root, "list"))
        @test length(p["sources"]) == 1
        @test p["sources"][1]["recorded_eventness"] == []
    end
end
case("query.simultaneous.states") do
    mktempdir() do root
        write(joinpath(root, "spec.recur"), HELLO)
        write(joinpath(root, E0), "current"); write(joinpath(root, EF), "complete")
        before = inventory(root)
        p = packet(query(root, "report", "spec.recur"))
        @test Set(r["state"] for r in p["footer"]["events"][1]["recorded"]) == Set(["todo.current", "complete"])
        @test p["footer"]["execution"] == "not-run"
        @test inventory(root) == before
    end
end
case("query.invalid.inputs") do
    mktempdir() do root
        write(joinpath(root, "spec.recur"), HELLO)
        write(joinpath(root, "invalid.recur"), UInt8[0xff, 0xfe, 0x00])
        for args in [("check", "missing.recur"), ("check", "invalid.recur"), ("show", "spec.recur", "--scope", "missing")]
            p = packet(query(root, args...), 2)
            @test !isempty(p["diagnostics"])
        end
    end
end
case("query.escape.and.executable.binding") do
    mktempdir() do parent
        root = joinpath(parent,"project"); mkdir(root)
        sentinel = joinpath(root, "binding-ran")
        # A resolvable executable binding would create a marker if erroneously run.
        script = joinpath(root, Sys.iswindows() ? "forbidden.cmd" : "forbidden.sh")
        write(script, Sys.iswindows() ? "@echo invoked>\"$sentinel\"\r\n" : "#!/bin/sh\nprintf invoked > '$sentinel'\n")
        Sys.iswindows() || chmod(script, 0o755)
        write(joinpath(root,"spec.recur"), mutate(HELLO,"HoldemLab.hello",basename(script)))
        write(joinpath(parent,"outside.recur"), HELLO)
        before = inventory(parent)
        for command in ["list", "report", "check"]
            p = command == "list" ? query(root,command) : query(root,command,"spec.recur")
            packet(p)
        end
        @test packet(query(root,"check","../outside.recur"),2)["diagnostics"][1]["code"] == "LANG002"
        @test !isfile(sentinel)
        @test inventory(parent) == before
    end
end
case("query.directory.link.boundary") do
    mktempdir() do parent
        root=joinpath(parent,"root"); outside=joinpath(parent,"outside")
        mkdir(root); mkdir(outside); mkdir(joinpath(root,"inside"))
        write(joinpath(outside,"spec.recur"),HELLO)
        write(joinpath(root,"inside","spec.recur"),HELLO)
        symlink(outside,joinpath(root,"escape");dir_target=true)
        symlink(joinpath(root,"inside"),joinpath(root,"alias");dir_target=true)
        before=inventory(outside)
        @test packet(query(root,"check","escape/spec.recur"),2)["diagnostics"][1]["code"] == "LANG002"
        packet(query(root,"check","alias/spec.recur"))
        @test inventory(outside) == before
    end
end
for (name,extra) in [("confirm.without.receipt",["--confirm"]),
    ("recover.without.checked",["--recover"]),
    ("checked.with.id",["--checked-contract","policy.json","--receipt","attempt.json","--id","manual"]) ]
    case("actor.arguments." * name) do
        mktempdir() do root
            checked_fixture(root); before=inventory(root)
            r=invoke(ACTOR,"warp","spec.recur","hello.h","-d",root,"--eventness",E0,"--json",extra...)
            @test r.code == 2
            @test inventory(root) == before
        end
    end
end

case("delivery.seven.binary.smoke") do
    for name in ["recur","recur-lang","recur-warp","recur-watch","recur-reveal","recur-git","recur-version"]
        exe=joinpath(dirname(CORE),name * EXT)
        @test isfile(exe)
        if isfile(exe)
            @test invoke(exe,"--help").code == 0
            version=invoke(exe,"--version")
            @test version.code == 0
            @test !isempty(strip(version.stdout))
        end
    end
end

# Independent authored graph oracle and deliberate circular relationships.
const EDGES = Set([("table.plan","left"), ("table.plan","right"), ("table.plan","settle"),
    ("left","settle"), ("right","settle"), ("table.plan","audit"), ("settle","audit"), ("audit","table.finish")])
case("graph.exact.oracle") do
    mktempdir() do root
        write(joinpath(root,"coordination.recur"),CIR)
        p = packet(query(root,"report","coordination.recur"))
        g = p["footer"]["graph"]
        @test Set((e["producer"],e["consumer"]) for e in g["edges"]) == EDGES
        @test Set((n["identity"],n["kind"]) for n in g["nodes"]) == Set([
            ("left","lane"),("right","lane"),("settle","lane"),("audit","lane"),("table.plan","coordinator"),("table.finish","coordinator")])
        @test Set(g["entries"]) == Set(["left","right"])
        @test Set(g["reachable"]) == Set(["left","right","settle","audit","table.finish"])
        @test [(w["consumer"],Set(x["identity"] for x in w["required"])) for w in g["waits"]] == [
            ("settle",Set(["left.o(b)","right.o(b)"])),("audit",Set(["settle.o(b)"])),("table.finish",Set(["audit.o(b)"]))]
        @test g["orchestration_sound"] && isempty(g["findings"])
        @test g["source_hash"] == fingerprint(codeunits(CIR))
        @test packet(query(root,"report","coordination.recur")) == p
    end
end
for (name, old, new, code) in [
    ("self", "i(a) := table.plan.o(a)", "i(a) := left.o(b)", "SGR001"),
    ("dependency", "i(a) := table.plan.o(a)", "i(a) := audit.o(b)", "SGR001"),
    ("wait", "await [left.o(b), right.o(b)]", "await [left.o(b), right.o(b), audit.o(b)]", "SGR002"),
    ("missing.join", "await [left.o(b), right.o(b)]", "await [left.o(b)]", "SGR004")]
    case("graph.fault." * name) do
        mktempdir() do root
            write(joinpath(root,"coordination.recur"), mutate(CIR,old,new))
            p = packet(query(root,"check","coordination.recur","--scope","right"),1)
            @test any(f->f["code"] == code,p["footer"]["findings"])
            @test !p["footer"]["graph"]["orchestration_sound"]
            @test length(p["header"]) == 1
            if code == "SGR001"
                edges = Set((e["producer"],e["consumer"]) for e in p["footer"]["graph"]["edges"])
                for f in filter(f->f["code"] == code,p["footer"]["findings"])
                    path=f["path"]
                    @test length(path) >= 2 && first(path) == last(path)
                    @test all((a,b) in edges for (a,b) in zip(path[1:end-1],path[2:end]))
                end
            end
        end
    end
end
case("parser.comments.are.inert") do
    mktempdir() do root
        # Comment contents must not invent a second scope or function.
        fake = "# scope phantom {\n# i(a) := (name: Text)\n# o(b) := (message: Text)\n# f : i(a) -> o(b) ~ \"fake\" by forbidden.runner\n# }\n"
        write(joinpath(root,"spec.recur"), HELLO * fake)
        p = packet(query(root,"report","spec.recur"))
        @test [h["identity"] for h in get(p,"header",[])] == ["hello.h"]
    end
end
for (name, source) in [
    ("duplicate.scope", mutate(HELLO,"body {","header {\n  scope hello {\n    i(a) := (x: Text)\n    o(b) := (y: Text)\n    h : i(a) -> o(b) ~ \"duplicate\" by forbidden.runner\n  }\n}\nbody {")),
    ("missing.function", mutate(HELLO,"h : i(a) -> o(b) ~ \"Return Hello, World! by default or Hello, NAME!\" by HoldemLab.hello","# absent")),
    ("wrong.flow.port", mutate(HELLO,"hello sync : i(a) -> h(a) -> o(b)","hello sync : i(a) -> h(a) -> o(z)")),
    ("undeclared.state", mutate(HELLO,"state demo.holdem.hello.complete","state demo.holdem.hello.other")),
    ("unknown.producer", mutate(CIR,"i(a) := table.plan.o(a)","i(a) := ghost.o(b)")),
    ("missing.policy", mutate(CIR,"allow tools [\"julia\"]","# absent"))]
    case("parser.reject." * name) do
        mktempdir() do root
            write(joinpath(root,"spec.recur"),source)
            p=packet(query(root,"check","spec.recur"),2)
            @test p["diagnostics"][1]["code"] == (name == "duplicate.scope" ? "LANG004" : "LANG006")
        end
    end
end
for ending in ["LF","CRLF"]
    case("parser.unicode.spans." * ending) do
        mktempdir() do root
            source = "# José 日本語\n" * replace(HELLO,"\r\n"=>"\n")
            ending == "CRLF" && (source=replace(source,"\n"=>"\r\n"))
            write(joinpath(root,"spec.recur"),source)
            p=packet(query(root,"show","spec.recur","--scope","hello.h"))
            s=p["header"][1]["span"]; bytes=codeunits(source)
            @test 0 <= s["start_byte"] < s["end_byte"] <= length(bytes)
            excerpt=String(bytes[s["start_byte"]+1:s["end_byte"]])
            @test startswith(strip(excerpt),"h : i(a)")
            @test p["source_hash"] == fingerprint(bytes)
            @test s["start_line"] == 7
        end
    end
end

# Corruption probes start from an independently qualified, actually accepted fixture.
case("checked.control.preview.accept.replay") do
    mktempdir() do root
        checked_fixture(root); before=inventory(root)
        packet(actor(root)); @test before == inventory(root)
        p=packet(actor(root,"--confirm")); @test p["action"]["ack"] == true
        accepted=inventory(root)
        @test packet(evidence(root,"--status",STATUS))["transition_status"]["current_accepted"]
        @test packet(actor(root,"--confirm"))["action"]["ack"] == true
        @test inventory(root) == accepted
    end
end
for field in ["source_hash","before","after","checked_inputs"]
    case("checked.status." * field) do
        mktempdir() do root
            accept_fixture(root); record=jsonget(root,STATUS)
            if field == "source_hash"
                record[field]="fnv1a64:0000000000000000"
            elseif field == "before"
                record[field]="unrelated.absent.md"
                write(joinpath(root,E0),read(joinpath(root,EF)))
            elseif field == "after"
                mv(joinpath(root,EF),joinpath(root,"unrelated.complete.md"))
                record[field]="unrelated.complete.md"
            else
                delete!(record[field],"implementation.jl")
            end
            jsonput(root,STATUS,record); before=inventory(root)
            r=evidence(root,"--status",STATUS)
            @test r.code != 0
            p=r.packet; @test p !== nothing
            @test p["transition_status"]["current_accepted"] == false
            @test inventory(root) == before
            @test actor(root,"--confirm").code != 0
            @test inventory(root) == before
        end
    end
end
for file in [PREPARED,STATUS], mode in ["scope","json"]
    id = file == PREPARED ? "prepared" : "accepted"
    case("checked.conflict." * id * "." * mode) do
        mktempdir() do root
            accept_fixture(root)
            if mode == "json"
                write(joinpath(root,file),"{torn")
            else
                record=jsonget(root,file); record["scope"]="other.f"; jsonput(root,file,record)
            end
            before=inventory(root)
            for flags in [("--confirm",),("--recover","--confirm")]
                r=actor(root,flags...)
                @test r.code != 0
                @test isnothing(r.packet) || get(get(r.packet,"action",Dict()),"ack",false) != true
                @test inventory(root) == before
            end
        end
    end
end
for path in ["spec.recur","implementation.jl","cases.jl","config.toml","runner.jl","behavior.md","policy.json","result.json","evidence.json","attempt.json"]
    case("checked.drift." * replace(path,"."=>"_")) do
        mktempdir() do root
            accept_fixture(root)
            open(joinpath(root,path),"a") do io; write(io," "); end
            before=inventory(root); r=evidence(root,"--status",STATUS)
            @test r.code != 0
            @test r.packet["transition_status"]["current_accepted"] == false
            @test actor(root,"--confirm").code != 0
            @test inventory(root) == before
        end
    end
end
for id in [repeat("a",80),repeat("a",81),".","", "José"]
    label = isempty(id) ? "empty" : id == "." ? "dot" : id == "José" ? "unicode" : string(length(id))
    case("checked.attempt.id." * label) do
        mktempdir() do root
            checked_fixture(root); a=jsonget(root,"attempt.json"); a["attempt_id"]=id; jsonput(root,"attempt.json",a)
            before=inventory(root); r=evidence(root)
            @test (r.code == 0) == (length(id) == 80)
            @test inventory(root) == before
        end
    end
end

# Shared with the actual pathing suite; no corrected duplicate oracle in tests.
include(joinpath(REPO,"demos/pathing/path_validation.jl"))
function path_fixture(tiles; edge=true)
    a=Dict("x"=>0,"y"=>0); b=Dict("x"=>1,"y"=>0)
    Dict("map"=>Dict("graph"=>Dict("corridors"=>edge ? [Dict("from"=>a,"to"=>b)] : []),"paths"=>[Dict("tiles"=>tiles)]))
end
for (name,tiles,edge,expected) in [
    ("adjacent",[Dict("x"=>0,"y"=>0),Dict("x"=>1,"y"=>0)],true,true),
    ("reverse",[Dict("x"=>1,"y"=>0),Dict("x"=>0,"y"=>0)],true,true),
    ("disconnected",[Dict("x"=>0,"y"=>0),Dict("x"=>1,"y"=>0)],false,false),
    ("empty",[],true,false), ("singleton",[Dict("x"=>0,"y"=>0)],true,false),
    ("malformed",[Dict("x"=>0),Dict("x"=>1,"y"=>0)],true,false)]
    case("path.oracle." * name) do
        @test PathValidation.every_path_is_contiguous(path_fixture(tiles;edge)) == expected
    end
end

case("catalog.every.todo.has.test.and.warp.mapping") do
    ledger=jsonget(joinpath(REPO,"demos/lang-verification"),"main.lang.verification.ledger.json")
    text=read(joinpath(REPO,ledger["todo"]),String)
    ids=[m.captures[1] for m in eachmatch(r"<!-- verification: (V\d+) -->",text)]
    checkboxes=length(collect(eachmatch(r"(?m)^- \[[ x]\] ",text)))
    items=ledger["items"]
    @test length(ids) == checkboxes == length(items)
    @test length(unique(ids)) == length(ids)
    @test Set(ids) == Set(i["id"] for i in items)
    map=jsonget(joinpath(REPO,"warps"),"main.lang.verification.warp-map.json")
    slices=Set(s["slice_id"] for s in map["required_slices"])
    covered=Set{String}()
    for item in items
        @test item["slice"] in slices || item["slice"] == "contract-first"
        @test !isempty(item["remaining_test_work"])
        @test haskey(item,"accepted_evidence")
        for path in item["existing_artifacts"]; @test isfile(joinpath(REPO,path)); end
        for prefix in item["case_prefixes"]
            matches=filter(p->startswith(first(p),prefix),CASES)
            @test !isempty(matches)
            union!(covered,first.(matches))
        end
    end
    @test Set(first.(CASES)) == covered
end

function main(args=ARGS)
    prefix=""; output=nothing; listing=false
    i=1
    while i <= length(args)
        if args[i] == "--list"; listing=true
        elseif args[i] in ["--case","--json-output"]
            i < length(args) || error("$(args[i]) requires a value")
            flag=args[i]; i+=1
            flag == "--case" ? (prefix=args[i]) : (output=args[i])
        else; error("Unknown option: $(args[i])")
        end
        i+=1
    end
    selected=filter(p->startswith(first(p),prefix),CASES)
    isempty(selected) && error("Zero eligible cases for prefix '$prefix'")
    length(unique(first.(CASES))) == length(CASES) || error("Duplicate case IDs")
    output !== nothing && ispath(output) && error("Observation already exists: $output")
    if listing
        foreach(p->println(first(p)),selected); return 0
    end
    isfile(CORE) && isfile(ACTOR) || error("Set RECUR_BIN and RECUR_LANG_BIN to the selected matching executables")
    results=Any[]
    for (id,f) in selected
        println("\nDEMO $id")
        result=Dict{String,Any}("id"=>id,"passed"=>0,"failed"=>0,"errors"=>0,"broken"=>0)
        try
            ts=@testset "$id" begin; f(); end
            c=Test.get_test_counts(ts)
            result["passed"]=c.passes; result["failed"]=c.fails; result["errors"]=c.errors; result["broken"]=c.broken
        catch e
            if e isa Test.TestSetException
                result["passed"]=e.pass; result["failed"]=e.fail; result["errors"]=e.error; result["broken"]=e.broken
            else
                result["errors"]=1; result["exception"]=sprint(showerror,e); showerror(stderr,e); println(stderr)
            end
        end
        result["status"] = result["errors"] > 0 ? "error" : result["failed"] > 0 ? "failed" : result["broken"] > 0 ? "broken" : result["passed"] == 0 ? "empty" : "passed"
        push!(results,result)
    end
    counts=Dict(s=>count(r->r["status"]==s,results) for s in ["passed","failed","error","broken","empty"])
    code=counts["passed"] == length(results) ? 0 : 1
    report=Dict("schema"=>"lang-verification-observation-v1","qualification"=>"Observed test run; not a Warp acceptance receipt",
        "recorded_at_utc"=>string(now(UTC)),"julia_version"=>string(VERSION),"platform"=>string(Sys.KERNEL),
        "selected_prefix"=>prefix,"exit_code"=>code,"case_counts"=>counts,"cases"=>results,
        "binaries"=>Dict("core"=>Dict("path"=>CORE,"sha256"=>bytes2hex(sha256(read(CORE)))),
                         "companion"=>Dict("path"=>ACTOR,"sha256"=>bytes2hex(sha256(read(ACTOR))))),
        "inputs"=>Dict(p=>bytes2hex(sha256(read(joinpath(REPO,p)))) for p in [
            "julia-tests/main.command.lang.verification.test.jl", "julia-tests/main.lang.pathing.test.jl",
            "demos/pathing/path_validation.jl",
            "demos/holdem-lab/main.holdem.01.hello.recur", "demos/holdem-lab/main.holdem.06.coordination.recur",
            "demos/lang-verification/main.lang.verification.ledger.json",
            "warps/main.lang.verification.warp-map.json",
            "README.CORE.IMPROVEMENT30.recur-lang.verification.todo.md"]))
    if output !== nothing
        # Immutable observations: caller must choose a fresh path for each attempt.
        ispath(output) && error("Observation already exists: $output")
        mkpath(dirname(abspath(output))); write(output,JSON3.write(report))
    end
    println("\nCase results: ",JSON3.write(counts),"; exit=",code)
    code
end
end
if abspath(PROGRAM_FILE) == @__FILE__
    exit(LangVerificationTests.main())
end
