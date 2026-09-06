module SudokuTeachingTests
using Test, JSON3, Random
include(joinpath(@__DIR__,"..","demos","sudoku","julia","Generator.jl"))

module TeachingTraceStub
trace_id(id;kwargs...) = Dict("define"=>[],"produce"=>[],"consume"=>[],"trigger"=>[])
end

module TeachingFailedTrace
trace_id(id;kwargs...) = nothing
end

@testset "Sudoku playable generation" begin
    Random.seed!(5092026)
    solution = Generator.generate_solution()
    @test Generator.valid_board(solution)
    @test Generator.count_solutions(solution) == 1
    @test Generator.count_solutions(zeros(Int,9,9)) == 2
    invalid = copy(solution); invalid[1,1] = invalid[1,2]
    @test !Generator.valid_board(invalid)
    @test Generator.count_solutions(invalid) == 0
    @test_throws ErrorException Generator.count_solutions(zeros(Int,9,9);node_budget=0)
    @test_throws ErrorException Generator.make_playable(solution,"unknown")
    @test_throws ErrorException Generator.make_playable(solution,"easy";removal_budget=0)
    @test_throws ErrorException Generator.make_playable(invalid,"easy")
    for preset in ("easy","medium","hard")
        result = Generator.make_playable(solution,preset)
        grid = reduce(vcat,permutedims.(result["givens"]))
        @test Generator.count_solutions(grid) == 1
        @test count(==(0),grid) == result["gaps"]
        @test all(grid[r,c] == 0 || grid[r,c] == solution[r,c] for r in 1:9, c in 1:9)
        for step in result["grade"]["steps"]
            m = match(r"sudoku.r(\d).c(\d)",step["cell"])
            r,c = parse.(Int,m.captures)
            @test Generator.candidates(grid,r,c) == [step["value"]]
            grid[r,c] = step["value"]
        end
        @test count(==(0),grid) == result["grade"]["remaining"]
        @test (result["grade"]["label"] == "naked-single-solvable") == all(!=(0),grid)
    end
    @test Generator.grade_playable(zeros(Int,9,9))["label"] == "ungraded"
    mktempdir() do dir
        legacy = joinpath(dir,"sudoku.solution.txt"); write(legacy,"preserve me")
        package = Generator.publish_playable(dir,TeachingTraceStub;solution)
        target = joinpath(dir,"sudoku.playable.json")
        previous = read(target,String)
        @test JSON3.read(previous)["puzzle_id"] == package["puzzle_id"]
        @test length(package["cascades"]) == 81
        @test read(legacy,String) == "preserve me"
        @test_throws ErrorException Generator.publish_playable(dir,TeachingFailedTrace;solution)
        @test read(target,String) == previous
        @test_throws ErrorException Generator.publish_playable(dir,TeachingTraceStub;solution,removal_budget=0)
        @test read(target,String) == previous
        newer = Generator.publish_playable(dir,TeachingTraceStub;solution)
        @test JSON3.read(read(target,String))["puzzle_id"] == newer["puzzle_id"]
        @test !any(startswith(".teaching-stage-"),readdir(dir))
    end
end

end # SudokuTeachingTests
