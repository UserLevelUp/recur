# consumes: demo.blackjack.hello optional-name contract
@testset "01 hello bundle" begin
    @test B.hello() == "Hello, World!"
    @test B.hello("  Ada  ") == "Hello, Ada!"
    @test B.hello("José") == "Hello, José!"
    @test_throws ArgumentError B.hello("  ")
end
