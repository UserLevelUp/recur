using Test, Random
isdefined(@__MODULE__, :Card) || include("main.holdem.03.jl")
isdefined(@__MODULE__, :evaluate) || include("main.holdem.04.jl")
# consumes: demo.holdem.play legal transitions and conservation
# consumes: demo.holdem.settle fold/showdown/tie outcomes
@testset "05 Heads-up Holdem app" begin
    implementation = joinpath(@__DIR__, "main.holdem.05.jl")
    @test isfile(implementation)
    if isfile(implementation)
        include(implementation)
        s = start_hand()
        @test s.stacks == [99,98] && s.paid == [1,2] && s.pot == 3 && s.actor == 1
        @test isempty(public_view(s).board)
        @test !hasproperty(public_view(s), :deal)
        @test !hasproperty(public_view(s), :holes)
        before = deepcopy(s)
        for args in [(2,:call,0), (1,:check,0), (1,:raise,3), (1,:raise,101), (1,:dance,0), (1,:call,7)]
            @test_throws ArgumentError act(s,args...)
            @test public_view(s) == public_view(before)
            @test s.paid == before.paid
        end
        c = act(s,1,:call)
        @test c.actor == 2 && c.street == :preflop && c.pot == 4
        @test s.pot == 3 # input not changed
        @test_throws ArgumentError act(c,2,:call)
        f = act(c,2,:check)
        @test f.street == :flop && f.actor == 2 && length(public_view(f).board) == 3
        r = act(f,2,:raise,6)
        @test r.actor == 1 && r.min_raise == 6
        @test_throws ArgumentError act(r,1,:raise,10)
        t = act(r,1,:call)
        @test t.street == :turn && length(public_view(t).board) == 4
        rv = act(act(t,2,:check),1,:check)
        @test rv.street == :river && length(public_view(rv).board) == 5
        done = act(act(rv,2,:check),1,:check)
        @test done.settled && done.pot == 0 && sum(done.stacks) == 200
        @test_throws ArgumentError act(done,1,:check)
        folded = act(s,1,:fold)
        @test folded.settled && folded.winners == [2] && folded.stacks == [99,101]
        @test isempty(public_view(folded).board) # folding doesn't expose future cards
        allin = act(act(s,1,:raise,100),2,:call)
        @test allin.settled && sum(allin.stacks) == 200 && allin.street == :showdown
        tiny = act(start_hand(stack=2),1,:call)
        @test tiny.settled && sum(tiny.stacks) == 4
        @test_throws ArgumentError start_hand(stack=1)
        # Construct a board royal flush, making both players use zero hole cards.
        allcards = deck().cards
        board = [Card(r,4) for r in 10:14]
        others = filter(c -> !(c in board),allcards)
        order = Vector{Card}(undef,52)
        order[[6,7,8,10,12]] = board
        order[setdiff(1:52,[6,7,8,10,12])] = others
        script = [(1,:call,0),(2,:check,0),(2,:check,0),(1,:check,0),
                  (2,:check,0),(1,:check,0),(2,:check,0),(1,:check,0)]
        tie = play(script; order)
        @test tie.winners == [1,2] && tie.stacks == [100,100]
        @test public_view(play(script; order)) == public_view(tie)
        @test occursin("showdown",render(public_view(tie)).text)
        @test_throws ArgumentError play(script[1:2]; order)
        # Legal random action sequences: input immutability, finite completion,
        # no negative chips, conservation at every transition across 30 decks.
        rng = MersenneTwister(281)
        for trial in 1:30
            state = start_hand(order=shuffle(rng,deck().cards), stack=20)
            steps = 0
            while !state.settled && steps < 100
                steps += 1
                debt = maximum(state.paid)-state.paid[state.actor]
                target = maximum(state.paid)+state.min_raise
                limit = minimum(state.stacks+state.paid)
                action = rand(rng) < 0.15 ? :fold : target <= limit && rand(rng) < 0.35 ? :raise : debt > 0 ? :call : :check
                state = act(state,state.actor,action,action == :raise ? target : 0)
                @test all(>=(0),state.stacks)
                @test sum(state.stacks)+state.pot == 40
            end
            @test state.settled && steps < 100
        end
    end
end
