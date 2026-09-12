"Read-only WIR1 associations; external result semantics belong to Recur's checker."
module LangEvidence
using JSON3

const ORDER = ["malformed","ambiguous","mismatched","failed","stale","declared","absent","checked"]
struct Problem <: Exception
    status::String
    reason::String
end
Base.showerror(io::IO, e::Problem) = print(io,e.reason)
require(ok, status, reason) = ok || throw(Problem(status,reason))
text(x) = x isa AbstractString && !isempty(strip(x))
struct Reads
    root::String
    sizes::Dict{String,Int}
end
function path_in(reads, relative)
    require(text(relative),"malformed","empty/non-string path")
    require(!occursin(r"[\\:\x00]",relative) && !isabspath(relative) &&
        all(p -> !isempty(p) && !(p in (".","..")),split(relative,'/')),
        "malformed","unsafe relative path: $relative")
    path=joinpath(reads.root,relative)
    require(isfile(path),"malformed","missing file: $relative")
    resolved=realpath(path); rel=relpath(resolved,reads.root)
    require(!isabspath(rel) && first(splitpath(rel))!="..","malformed","canonical escape: $relative")
    resolved
end
function bounded(reads, relative; json=false)
    path=path_in(reads,relative)
    n=filesize(path); limit=json ? 1048576 : 64*1024*1024
    require(n<=limit,"malformed","file byte limit: $relative")
    reads.sizes[path]=n
    require(length(reads.sizes)<=128 && sum(values(reads.sizes))<=128*1024*1024,
        "malformed","total file/byte limit")
    bytes=read(path,limit+1)
    require(length(bytes)==n && length(bytes)<=limit,"malformed","file changed during bounded read: $relative")
    json ? JSON3.read(String(bytes),Dict{String,Any}) : bytes
end
function capture(args)
    out=IOBuffer(); err=IOBuffer()
    process=run(pipeline(ignorestatus(Cmd(args)),stdout=out,stderr=err))
    (code=process.exitcode,stdout=String(take!(out)),stderr=String(take!(err)))
end
function query(args,adapter)
    result=adapter(args)
    require(ncodeunits(result.stdout)<=1048576,"malformed","query output byte limit")
    require(result.code==0,"failed","query failed: $(result.stderr) $(result.stdout)")
    JSON3.read(result.stdout,Dict{String,Any})
end
function stringlist(x)
    x isa AbstractVector && !isempty(x) && all(text,x) && length(unique(x))==length(x)
end
function association_shape(a)
    require(a isa AbstractDict,"malformed","association must be an object")
    for key in ["id","source","source_hash","scope","warp_id","slice_id","contract_hash","map"]
        require(text(get(a,key,nothing)),"malformed","missing association $key")
    end
    for key in ["aliases","requirements","transition"]
        require(get(a,key,nothing) isa AbstractDict && !isempty(a[key]),"malformed","invalid $key")
    end
    require(all(text,keys(a["requirements"])) && all(stringlist,values(a["requirements"])),"malformed","invalid requirement cases")
    require(all(text,keys(a["aliases"])) && all(text,values(a["aliases"])),"malformed","invalid aliases")
    require(Set(keys(a["transition"]))==Set(["E0","dE","Ef"]) && all(text,values(a["transition"])),"malformed","invalid transition")
    for key in ["required_inputs","gates"]
        require(stringlist(get(a,key,nothing)),"malformed","invalid $key")
    end
    require(haskey(a,"current_attempt") && (isnothing(a["current_attempt"]) || text(a["current_attempt"])),"malformed","invalid current attempt")
    require(get(a,"attempts",nothing) isa AbstractVector && length(a["attempts"])<=16,"malformed","attempt limit/type")
    ids=String[]
    for attempt in a["attempts"]
        require(attempt isa AbstractDict,"malformed","invalid attempt")
        for key in ["id","producer","runtime","source_hash"]
            require(text(get(attempt,key,nothing)),"malformed","invalid attempt $key")
        end
        require(get(attempt,"phase",nothing) in ["red","green"],"malformed","invalid attempt phase")
        require(get(attempt,"cases",nothing) isa AbstractDict && length(attempt["cases"])<=128 &&
                all(text,keys(attempt["cases"])) && all(v->v in ["passed","failed"],values(attempt["cases"])),"malformed","invalid/too many case results")
        require(get(attempt,"gate_refs",nothing) isa AbstractDict && all(text,keys(attempt["gate_refs"])) &&
                all(text,values(attempt["gate_refs"])),"malformed","invalid gate refs")
        push!(ids,attempt["id"])
    end
    require(length(unique(ids))==length(ids),"ambiguous","duplicate attempt identity")
end
function one(reads,a,header,packet,binary,adapter)
    association_shape(a)
    require(a["source"]==packet["source"] && a["scope"]==header["identity"],"mismatched","source/scope mismatch")
    aliases=Dict(header[k]["local_identity"]=>header[k]["canonical_identity"] for k in ["input","output"])
    require(a["aliases"]==aliases,"mismatched","canonical alias mismatch")
    events=filter(e->get(e,"scope",nothing)==first(split(header["identity"],'.')),packet["footer"]["events"])
    require(length(events)==1,"mismatched","transition scope mismatch")
    t=events[1]["requested_transition"]
    require(a["transition"]==Dict("E0"=>t["current"],"dE"=>t["slice"],"Ef"=>t["desired"]),"mismatched","transition mismatch")
    map=bounded(reads,a["map"];json=true)
    require(get(map,"schema",nothing)=="warp-bubble-map-v1","malformed","invalid map schema")
    require(map["warp_id"]==a["warp_id"],"mismatched","Warp identity mismatch")
    slices=filter(s->s["slice_id"]==a["slice_id"],map["required_slices"])
    require(length(slices)==1,"mismatched","slice identity mismatch")
    slice=only(slices)
    require(slice["contract_hash"]==a["contract_hash"] && Set(slice["evidence_gates"])==Set(a["gates"]),"mismatched","contract/gates mismatch")
    require(dirname(a["map"])=="" || dirname(a["map"])==".","mismatched","map must share fixed evidence root")
    for input in a["required_inputs"]; bounded(reads,input); end
    require(a["source"] in a["required_inputs"],"mismatched","live specification missing from required inputs")
    answer=deepcopy(a)
    answer["assessment"]=Dict("status"=>"declared","reasons"=>String[],"gates"=>Any[])
    answer["acceptance"]=Dict("accepted"=>false,"status"=>"not-assessed")
    current=a["current_attempt"]
    isnothing(current) && return answer
    found=filter(x->x["id"]==current,a["attempts"])
    require(length(found)==1,"malformed","current attempt not found")
    attempt=only(found)
    require(Set(keys(attempt["gate_refs"]))==Set(a["gates"]),"mismatched","attempt gates mismatch")
    required=unique(vcat(collect(values(a["requirements"]))...))
    require(all(c->haskey(attempt["cases"],c),required),"mismatched","required case missing")
    statuses=String[]; reasons=String[]; gates=Any[]
    if attempt["phase"]!="green" || any(c->attempt["cases"][c]!="passed",required)
        push!(statuses,"failed"); push!(reasons,"current attempt has failing cases or is red")
    end
    if a["source_hash"]!=packet["source_hash"] || attempt["source_hash"]!=packet["source_hash"]
        push!(statuses,"stale"); push!(reasons,"selected source hash changed")
    end
    for gate in a["gates"]
        reference=attempt["gate_refs"][gate]
        if !startswith(reference,"evidence:")
            push!(statuses,"declared"); push!(reasons,"$gate is manual/native evidence, not checked tests")
            continue
        end
        path=reference[10:end]; e=bounded(reads,path;json=true)
        require(get(e,"schema",nothing)=="warp-external-evidence-v1","malformed","invalid external evidence schema")
        require(e["producer"]==attempt["producer"],"mismatched","producer mismatch")
        require(e["kind"]=="test","mismatched","test gate requires test evidence")
        inputs=e["source"]["files"]
        require(inputs isa AbstractDict,"malformed","invalid checked inputs")
        require(all(p->haskey(inputs,p),a["required_inputs"]),"mismatched","required live input missing from checked scope")
        bounded(reads,e["result_artifact"];json=true)
        for input in keys(inputs); bounded(reads,input); end
        verdict=query([binary,"warp","evidence",path,"-d",reads.root,"--json"],adapter)
        require(get(verdict,"status",nothing) in ["checked","stale","failed","declared"],"malformed","invalid checker verdict")
        push!(statuses,verdict["status"]); append!(reasons,verdict["reasons"])
        push!(gates,Dict("gate"=>gate,"assessment"=>verdict,"checked_inputs"=>inputs))
    end
    status=combined(statuses)
    answer["assessment"]=Dict("status"=>status,"reasons"=>reasons,"gates"=>gates)
    # Acceptance is never derived from the test outcome or an Eventness marker.
    projection=query([binary,"warp","show",a["warp_id"],"-d",reads.root,"--json"],adapter)
    accepted= status=="checked" && a["slice_id"] in get(projection,"completed_slices",[]) &&
        any(s->s["slice_id"]==a["slice_id"] && s["contract_hash"]==a["contract_hash"],get(projection,"slices",[])) &&
        all(g->any(p->p["slice_id"]==a["slice_id"] && p["gate"]==g && p["status"]=="checked" &&
            any(v->v["reference"]==attempt["gate_refs"][g],p["evidence"]),get(projection,"gate_evidence",[])),a["gates"])
    answer["acceptance"]=Dict("accepted"=>accepted,"status"=>projection["state"],"slice_id"=>a["slice_id"],
        "contract_hash"=>a["contract_hash"],"evidence_status"=>projection["evidence_status"])
    answer
end
combined(states) = isempty(states) ? "absent" : first(s for s in ORDER if s in states)
function assess(root,registry,packet;binary,adapter=capture)
    report=Dict{String,Any}("schema"=>"lang-evidence-report-v1","status"=>"absent","reasons"=>String[],
        "associations"=>Any[],"execution"=>"not-run",
        "qualification"=>"producer not rerun; fingerprints detect changes, not authenticity or dependency closure")
    try
        reads=Reads(realpath(root),Dict{String,Int}())
        # Validate even the optional registry path before deciding it is absent.
        require(text(registry) && !isabspath(registry) && !occursin(r"[\\:\x00]",registry) &&
            all(p->!isempty(p) && !(p in (".","..")),split(registry,'/')),"malformed","unsafe registry path")
        !ispath(joinpath(root,registry)) && return report
        data=bounded(reads,registry;json=true)
        require(get(data,"schema",nothing)=="lang-evidence-associations-v1" && get(data,"associations",nothing) isa AbstractVector,
            "malformed","invalid registry schema")
        associations=data["associations"]
        require(length(associations)<=16,"malformed","association count limit")
        isempty(associations) && return report
        foreach(association_shape,associations)
        statuses=String[]
        for header in packet["header"]
            matches=filter(a->a["scope"]==header["identity"] && a["source"]==packet["source"],associations)
            require(length(matches)<=1,"ambiguous","duplicate source/scope association")
            require(length(matches)==1,"mismatched","no exact source/scope association")
            a=only(matches)
            try
                item=one(reads,a,header,packet,binary,adapter)
                push!(report["associations"],item); push!(statuses,item["assessment"]["status"])
                append!(report["reasons"],item["assessment"]["reasons"])
            catch e
                status=e isa Problem ? e.status : "malformed"
                item=deepcopy(a); item["assessment"]=Dict("status"=>status,"reasons"=>[sprint(showerror,e)])
                item["acceptance"]=Dict("accepted"=>false,"status"=>"not-assessed")
                push!(report["associations"],item); push!(statuses,status); push!(report["reasons"],sprint(showerror,e))
            end
        end
        report["status"]=combined(statuses)
    catch e
        report["status"]=e isa Problem ? e.status : "malformed"
        push!(report["reasons"],sprint(showerror,e))
    end
    report
end
end
