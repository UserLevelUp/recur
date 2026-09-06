"""Documentation acceptance tests, developed red-first. No runtime/doc mutation.
Run: julia julia-tests/main.command.warp.docs-reconciliation.test.jl
Included in runtests.jl after reconciliation; also runnable standalone.
"""
module WarpDocsReconciliationTests
using Test, JSON3

const ROOT = normpath(joinpath(@__DIR__, ".."))
const CLAIMS = "docs/main.command.warp.docs-reconciliation.claims.json"
const REQUIRED = Set(["scoring-config", "status-explain-next", "maps-rings",
    "complete-receipt", "evolve-collapse", "evidence-freshness", "reveal",
    "discovery", "milestones-temporal", "methodology"])
const STATUSES = Set(["implemented", "partially-implemented", "proposed", "superseded", "unresolved"])

function evidence_valid(ref, root)
    ref isa AbstractDict || return false
    path = get(ref,"path","")
    needle = get(ref,"contains","")
    path isa AbstractString && needle isa AbstractString || return false
    (isempty(path) || isempty(strip(needle))) && return false
    isabspath(path) && return false
    # Evidence is repository-local, not an external URL or traversal escape.
    any(==(".."),split(replace(path,'\\'=>'/'),'/')) && return false
    file = joinpath(root,path)
    return isfile(file) && occursin(needle,read(file,String))
end

function claim_errors(matrix, root)
    errors = String[]
    get(matrix,"schema","") == "warp-doc-claims-v1" || push!(errors,"missing schema")
    claims = get(matrix,"claims",[])
    ids = [get(c,"id","") for c in claims]
    Set(ids) == REQUIRED || push!(errors,"claim families missing or unknown")
    length(ids) == length(unique(ids)) || push!(errors,"duplicate claim IDs")
    for claim in claims
        id = get(claim,"id","?")
        status = get(claim,"status","")
        status in STATUSES || push!(errors,"$id: invalid classification")
        for field in ("summary","limitations","publication")
            value = get(claim,field,"")
            value isa AbstractString && !isempty(strip(value)) || push!(errors,"$id: missing $field")
        end
        docs = get(claim,"docs",[])
        !isempty(docs) && all(r->evidence_valid(r,root),docs) || push!(errors,"$id: invalid document evidence")
        if status in ("implemented","partially-implemented")
            for field in ("source","tests")
                refs = get(claim,field,[])
                !isempty(refs) && all(r->evidence_valid(r,root),refs) || push!(errors,"$id: invalid $field evidence")
            end
        end
    end
    return errors
end

@testset "Warp documentation reconciliation (red-first)" begin
    @testset "Validator rejects false evidence (synthetic, not project claims)" begin
        ref = Dict("path"=>"README.CORE.IMPROVEMENT27.md","contains"=>"RECUR IMPROVEMENT 27")
        matrix = Dict("schema"=>"warp-doc-claims-v1","claims"=>[
            Dict("id"=>id,"status"=>"proposed","summary"=>"Synthetic test only",
                 "limitations"=>"Not implementation evidence", "publication"=>"Not published",
                 "docs"=>[ref]) for id in sort(collect(REQUIRED))])
        @test isempty(claim_errors(matrix,ROOT))
        missing = deepcopy(matrix); pop!(missing["claims"])
        @test !isempty(claim_errors(missing,ROOT))
        duplicate = deepcopy(matrix); push!(duplicate["claims"],first(duplicate["claims"]))
        @test !isempty(claim_errors(duplicate,ROOT))
        promoted = deepcopy(matrix); promoted["claims"][1]["status"]="implemented"
        @test !isempty(claim_errors(promoted,ROOT))
        invalid = deepcopy(matrix); invalid["claims"][1]["status"]="done-ish"
        @test !isempty(claim_errors(invalid,ROOT))
        @test !evidence_valid(Dict("path"=>"../outside.md","contains"=>"x"),ROOT)
        @test !evidence_valid(Dict("path"=>"README.CORE.IMPROVEMENT27.md","contains"=>"nonexistent-evidence-marker-918273"),ROOT)
    end

    @testset "Implemented companion commands are not advertised as future-only" begin
        profile = get(ENV,"RECUR_PROFILE","release-safe")
        exe = get(ENV,"RECUR_WARP_BIN",joinpath(ROOT,"target",profile,"recur-warp"*(Sys.iswindows() ? ".exe" : "")))
        @test isfile(exe)
        if isfile(exe)
            println("Read-only companion help: ",exe)
            help = read(`$exe --help`,String)
            @test occursin(r"(?m)^\s+evolve\s+",help)
            @test occursin(r"(?m)^\s+collapse\s+",help)
        end
        text = read(joinpath(ROOT,"README.CORE.IMPROVEMENT27.Appendum.md"),String)
        # Specific known present-tense claims, not a blanket ban on future proposals.
        stale_summary = occursin(r"Warp evolution remains a\s+later write-side slice",text)
        stale_command = occursin(r"(?m)^recur-warp evolve \.\.\.\s+future confirmed Warp supersession record",text)
        stale_ring = occursin(r"Recursive domains,\s+nested projections, and subscription edges require a separately frozen schema",text)
        @test !stale_summary
        @test !stale_command
        @test !stale_ring
    end

    @testset "Claim-by-claim evidence matrix" begin
        path = joinpath(ROOT,CLAIMS)
        @test isfile(path)
        if isfile(path)
            matrix = JSON3.read(read(path,String),Dict{String,Any})
            errors = claim_errors(matrix,ROOT)
            foreach(println,errors)
            @test isempty(errors)
        end
    end

    @testset "Recovery links and historical proposal boundaries" begin
        documents = ["README.CORE.IMPROVEMENT27.md", "README.CORE.IMPROVEMENT27.Appendum.md", ".recur-warp",
            "docs/main.command.warp.docs-reconciliation.current-state.md", "docs/main.command.warp.readme.md"]
        for stem in ("todo.future-plan", "recur-ready.todo.future-plan", "command-boundary.todo.future-plan",
                     "contract.warp-status-v1.todo.future-plan", "epic.milestone.todo.future-plan")
            path = "docs/main.improvement.27.$stem.md"
            push!(documents,path)
            text = read(joinpath(ROOT,path),String)
            @test occursin("## Current split — 2026-09-06",text)
            @test occursin("## Historical proposal snapshot",text)
        end
        for path in documents
            text = read(joinpath(ROOT,path),String)
            for link in eachmatch(r"\]\(([^)]+)\)",text)
                target = first(split(link.captures[1],'#'))
                (isempty(target) || occursin(r"^[a-z]+://",target)) && continue
                @test isfile(normpath(joinpath(dirname(joinpath(ROOT,path)),target)))
            end
        end
        @test !isfile(joinpath(ROOT,"docs/main.improvement.27.differential-execution.todo.current.md"))
        @test isfile(joinpath(ROOT,"docs/main.improvement.27.differential-execution.historical.md"))
        current = read(joinpath(ROOT,"docs/main.command.warp.docs-reconciliation.current-state.md"),String)
        @test occursin("collapse_suffix_policy",current)
        @test occursin("Configuration does not grant itself authority",current)
    end
end
end # module
