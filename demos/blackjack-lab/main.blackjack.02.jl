# produces: demo.blackjack.table validated configuration; no behavior in config
bounded_int(x,lo,hi,label) = x isa Integer && !(x isa Bool) && lo<=x<=hi ? Int(x) :
    throw(ArgumentError("$label must be an integer in $lo:$hi"))
struct TableConfig
    players::Vector{String}
    money::Int
    bet::Int
    rounds::Int
    dealer::String
    bank::Int
    seed::Int
    function TableConfig(;players=3,money=100,bet=10,rounds=5,dealer="Dealer",bank=1000,seed=42)
        n=bounded_int(players,1,6,"players")
        stake=bounded_int(bet,2,10^9,"bet")
        iseven(stake) || throw(ArgumentError("bet must be even for integral 3:2 payouts"))
        funds=bounded_int(money,stake,10^9,"money")
        count=bounded_int(rounds,1,1000,"rounds")
        house=bounded_int(bank,0,10^9,"bank")
        rng=bounded_int(seed,0,10^9,"seed")
        dealer isa AbstractString && !isempty(strip(dealer)) || throw(ArgumentError("dealer must be nonblank text"))
        new(["Player $i" for i in 1:n],funds,stake,count,strip(dealer),house,rng)
    end
end
describe(t::TableConfig) = "$(hello(t.dealer)) $(length(t.players)) players, $(t.money) chips each, bet $(t.bet), $(t.rounds) rounds."
