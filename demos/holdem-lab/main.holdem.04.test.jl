using Test, Random
isdefined(@__MODULE__, :Card) || include("main.holdem.03.jl")
cards(text) = [Card(findfirst(==(t[1]), "23456789TJQKA")+1, findfirst(==(t[2]), "cdhs")) for t in split(text)]
# consumes: demo.holdem.rank golden hands and metamorphic invariants
@testset "04 Best five of seven" begin
    implementation = joinpath(@__DIR__, "main.holdem.04.jl")
    @test isfile(implementation)
    if isfile(implementation)
        include(implementation)
        examples = [
            ("As Ks Qs Js Ts 2c 3d", (8,14,0,0,0,0)),
            ("As Ah Ad Ac Ks 2c 3d", (7,14,13,0,0,0)),
            ("As Ah Ad Ks Kh Kd 2c", (6,14,13,0,0,0)),
            ("As Js 8s 5s 2s Kh Qd", (5,14,11,8,5,2)),
            ("As 2h 3d 4c 5s Kh Qd", (4,5,0,0,0,0)),
            ("As Ah Ad Ks Qh 2d 3c", (3,14,13,12,0,0)),
            ("As Ah Ks Kh Qh Qd 2c", (2,14,13,12,0,0)),
            ("As Ah Ks Qh Jd 9c 2c", (1,14,13,12,11,0)),
            ("As Kh Qd Jc 9s 4h 2d", (0,14,13,12,11,9)),
        ]
        for (text, expected) in examples
            hand = cards(text)
            @test evaluate((cards=hand,)).rank == expected
            @test evaluate((cards=reverse(hand),)).rank == expected
            renamed = [Card(c.rank, mod1(c.suit+1,4)) for c in hand]
            @test evaluate((cards=renamed,)).rank == expected
        end
        @test evaluate((cards=cards("6s 2h 3d 4c 5s Kh Qd"),)).rank > examples[5][2]
        @test evaluate((cards=cards("As Ah Ks Qh Td 9c 2c"),)).rank < examples[8][2]
        @test winners((left=examples[1][2], right=examples[2][2])).winners == [1]
        @test winners((left=examples[2][2], right=examples[1][2])).winners == [2]
        @test winners((left=examples[1][2], right=examples[1][2])).winners == [1,2]
        board = cards("Ts Js Qs Ks As")
        @test evaluate((cards=vcat(board,cards("2h 3c")),)) == evaluate((cards=vcat(board,cards("9h 9c")),))
        @test_throws ArgumentError evaluate((cards=cards("As As 2c 3d 4h 5s 6c"),))
        @test_throws ArgumentError evaluate((cards=board,))
        # Exhaustive five-card category histogram is an optional independent
        # combinatorial oracle, run by main.holdem.exhaustive.jl.
    end
end
