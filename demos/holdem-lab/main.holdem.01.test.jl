using Test
# consumes: demo.holdem.hello acceptance examples independent of implementation
@testset "01 Hello World" begin
    implementation = joinpath(@__DIR__, "main.holdem.01.jl")
    @test isfile(implementation)
    if isfile(implementation)
        include(implementation)
        @test hello() == "Hello, World!"
        @test hello("Marc") == "Hello, Marc!"
    end
end
