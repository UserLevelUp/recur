using Test
# consumes: demo.holdem.call class and caller acceptance
@testset "02 Class and caller" begin
    implementation = joinpath(@__DIR__, "main.holdem.02.jl")
    @test isfile(implementation)
    if isfile(implementation)
        include(implementation)
        @test run_hello() == "Hello, World!\n"
        @test run_hello(" Marc ") == "Hello, Marc!\n"
        @test run_hello("Mundo"; prefix="Hola") == "Hola, Mundo!\n"
        @test_throws ArgumentError run_hello("  ")
        @test greet(HelloApp(), "Marc") == (message="Hello, Marc!",)
    end
end
