using Test
const TESTS = normpath(joinpath(@__DIR__, "..", "..", "julia-tests"))
@testset "Watch affected core compatibility" begin
    include(joinpath(TESTS, "main.command.watch.test.jl"))
    include(joinpath(TESTS, "main.command.watch.eventness.test.jl"))
    include(joinpath(TESTS, "main.command.warp.query-compatibility.test.jl"))
    include(joinpath(TESTS, "main.command.trait.capabilities.test.jl"))
end
