# Run from any directory; Julia standard libraries only.
include("main.blackjack.jl")
function main(args)
    if "--help" in args
        println("Blackjack demo: --players 1..6 --money 100 --bet 10 --rounds 5 --dealer Dealer --bank 1000 --seed 42")
        return 0
    end
    options=Dict{Symbol,Any}(); i=1
    while i<=length(args)
        flag=args[i]
        flag in ["--players","--money","--bet","--rounds","--dealer","--bank","--seed"] || throw(ArgumentError("unknown option: $flag"))
        i<length(args) || throw(ArgumentError("missing value for $flag"))
        key=Symbol(flag[3:end]); haskey(options,key) && throw(ArgumentError("duplicate $flag"))
        options[key]=flag=="--dealer" ? args[i+1] : parse(Int,args[i+1]); i+=2
    end
    println(BlackjackLab.render(BlackjackLab.tournament(BlackjackLab.TableConfig(;options...))))
    0
end
if abspath(PROGRAM_FILE)==@__FILE__
    try exit(main(ARGS)) catch e; println(stderr,sprint(showerror,e)); exit(2); end
end
