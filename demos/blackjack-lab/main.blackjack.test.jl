module BlackjackExperiment
# Keep the assertion harness out of Julia 1.12's failing large-closure inference
# path on this Windows host. The game module retains its ordinary compilation.
Base.Experimental.@compiler_options compile=min optimize=0 infer=false
using Test, Random
include("main.blackjack.jl")
const B=BlackjackLab
@testset "Blackjack contract-first stages" begin
    for stage in 1:parse(Int,get(ENV,"BLACKJACK_STAGE","6"))
        include("main.blackjack.$(lpad(stage,2,'0')).test.jl")
    end
    if parse(Int,get(ENV,"BLACKJACK_STAGE","6"))==6
        include("main.blackjack.lang.test.jl")
        include("main.blackjack.dependencies.test.jl")
    end
end
end
