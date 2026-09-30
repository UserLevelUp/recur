#!/usr/bin/env julia
# defines: recur.lang.verification.demo bounded CLI demonstrations and todo catalogue
include(joinpath(@__DIR__, "../../julia-tests/main.command.lang.verification.test.jl"))
using .LangVerificationTests, JSON3

function demo_main(args)
    isempty(args) && (args=["catalog"])
    if args[1] == "catalog"
        ledger=JSON3.read(read(joinpath(@__DIR__,"main.lang.verification.ledger.json"),String))
        println(ledger.qualification)
        for item in ledger.items
            length(args)>1 && item.id != args[2] && continue
            println("\n",item.id,"  ",item.title," -> ",item.slice)
            println("  New demos: ",isempty(item.case_prefixes) ? "none; existing coverage / test design below" : join(item.case_prefixes,", "))
            println("  Existing: ",join(item.existing_artifacts,", "))
            println("  Remaining: ",item.remaining_test_work)
        end
        length(args)>1 && !any(x->x.id==args[2],ledger.items) && error("Unknown todo ID: $(args[2])")
        return 0
    elseif args[1] == "list"
        return LangVerificationTests.main(["--list"])
    elseif args[1] == "demo"
        rest=args[2:end]
        isempty(rest) && return LangVerificationTests.main()
        startswith(rest[1],"--") && return LangVerificationTests.main(rest)
        return LangVerificationTests.main(vcat(["--case",rest[1]],rest[2:end]))
    elseif args[1] == "suite"
        suites=Dict(
            "init"=>"julia-tests/main.command.recur-lang.init.test.jl",
            "baseline"=>"julia-tests/main.command.lang.baseline.test.jl",
            "language"=>"julia-tests/main.lang.test.jl",
            "inspector"=>"julia-tests/main.demo.lang-inspector.test.jl",
            "dogfood"=>"julia-tests/main.demo.lang-dogfood.test.jl",
            "protomap"=>"julia-tests/main.lang.pathing.proto-map.test.jl",
            "loader"=>"demos/web-evidence-lab/main.lang.evidence.test.jl",
            "api"=>"demos/web-evidence-lab/main.lang.api.test.jl",
            "holdem"=>"demos/holdem-lab/main.holdem.test.jl",
            "holdem-exhaustive"=>"demos/holdem-lab/main.holdem.exhaustive.jl")
        length(args)==2 && haskey(suites,args[2]) || error("Choose suite: " * join(sort(collect(keys(suites))),", "))
        repo=LangVerificationTests.REPO
        script=joinpath(repo,suites[args[2]])
        cmd=`$(Base.julia_cmd()) --startup-file=no --project=$(joinpath(repo,"demos/web-evidence-lab")) $script`
        # Run existing suites in separate processes; preserve their actual exit code.
        return run(ignorestatus(Cmd(cmd;dir=repo))).exitcode
    end
    error("Use catalog [Vnn], list, demo [CASE_PREFIX] [--json-output NEW_PATH], or suite NAME")
end
exit(demo_main(ARGS))
