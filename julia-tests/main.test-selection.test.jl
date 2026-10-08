using Test
isdefined(@__MODULE__, :TestSelection) || include("runtests.selection.jl")

@testset "Demo test selection" begin
    default = TestSelection.parse_selection(String[])
    @test default.core
    @test isempty(default.files)
    for (name, files) in TestSelection.DEMOS
        selected = TestSelection.parse_selection(["--demo", name])
        @test !selected.core
        @test selected.files == files
        @test all(file -> isfile(joinpath(@__DIR__, file)), selected.files)
    end
    chosen = TestSelection.parse_selection(["--demo", "blackjack-web",
        "--demo", "blackjack-web", "--demo", "sudoku", "--with-core"])
    @test chosen.core
    @test chosen.demos == ["blackjack-web", "sudoku"]
    @test length(chosen.files) == 6
    @test TestSelection.parse_selection(["--list-demos"]).list
    @test TestSelection.parse_selection(["--demo", "blackjack-web", "--dry-run"]).dry_run
    @test_throws ArgumentError TestSelection.parse_selection(["--demo"])
    @test_throws ArgumentError TestSelection.parse_selection(["--demo", "unknown"])
    @test_throws ArgumentError TestSelection.parse_selection(["--typo"])
end
