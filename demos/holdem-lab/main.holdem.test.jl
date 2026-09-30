module HoldemLabExperiment
using Test
@testset "Hello World to Holdem specification-first lab" begin
    include("main.holdem.01.test.jl")
    include("main.holdem.02.test.jl")
    include("main.holdem.03.test.jl")
    include("main.holdem.04.test.jl")
    include("main.holdem.05.test.jl")
    include("main.holdem.06.test.jl")
    include("main.holdem.graph.test.jl")
end
end
