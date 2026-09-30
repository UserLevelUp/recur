using Random
include("main.holdem.jl")
const H = HoldemLab

# triggers: demo.holdem.call Hello World class entry point
print(H.run_hello())
println("Texas Hold'em: deterministic two-player demonstration, seed 28")
state = H.start_hand(order=shuffle(MersenneTwister(28),H.deck().cards))
println(H.render(H.public_view(state)).text)
# triggers: demo.holdem.play actions drive the state machine; renderer only reads
actions = [(1,:call,0),(2,:check,0),(2,:raise,4),(1,:call,0),
           (2,:check,0),(1,:check,0),(2,:check,0),(1,:check,0)]
for command in actions
    println("player $(command[1]): $(command[2])",command[3] == 0 ? "" : " to $(command[3])")
    global state = H.act(state,command...)
    println(H.render(H.public_view(state)).text)
end
println("Showdown hole cards: ",state.deal.holes)
println("Independent symbolic-graph implementation: ",
        H.coordinate((holes=state.deal.holes,board=state.deal.board,pot=12)))
