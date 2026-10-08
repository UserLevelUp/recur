module RivalWorkerTests
Base.Experimental.@compiler_options compile=min optimize=0 infer=false
using Test, Random
include("main.blackjack.rivals.engine.jl")
const R = BlackjackRivals
# consumer: demo.blackjack.rivals.table.deal stable shared-deck order
# consumer: demo.blackjack.rivals.table.advance finite automatic turns
# consumer: demo.blackjack.rivals.table.settlement per-seat outcomes
# consumer: demo.blackjack.rivals.table.projection public privacy
# consumer: demo.blackjack.rivals.table.ledger conservation
# consumer: demo.blackjack.rivals.table.revision copied transitions
deck(prefix) = vcat(prefix, [c for c in 1:52 if !(c in prefix)])
cmd(g, action; kwargs...) = merge(Dict{String,Any}("version"=>3,"revision"=>g.revision,"action"=>action),Dict(string(k)=>v for (k,v) in kwargs))
ledger(s) = s.balance+s.escrow+s.bank+sum((r.balance+r.escrow for r in s.rivals);init=0)
@testset "Rival table contract" begin
    for count in (1,2)
        g=R.newgame(500,count); s=R.public_state(g)
        @test s.schema=="blackjack-web-state-v3"
        @test s.protocol_version==3 && s.rival_count==count
        @test length(s.rivals)==count
        @test ledger(s)==10000+500*(1+count)
        @test !(:deck in keys(s)) && !(:cursor in keys(s))
        @test_throws ArgumentError R.newgame(500,3)
        @test_throws ArgumentError R.newgame(500,true)
        before=deepcopy(s)
        @test_throws ArgumentError R.transition(g,cmd(g,"deal";bet=3))
        @test R.public_state(g)==before
    end
    g=R.newgame(500,2)
    g=R.transition(g,cmd(g,"deal";bet=10);order=deck([10,2,3,4,9,11,12,13]))
    s=R.public_state(g)
    @test s.phase=="playing" && s.dealer[2]===nothing && s.dealer_score===nothing
    @test s.hands[1].cards==[10,9]
    @test s.rivals[1].hands[1].cards==[2,11]
    @test s.rivals[2].hands[1].cards==[3,12]
    @test_throws ArgumentError R.transition(g,cmd(g,"stand";hand_id=2))
    @test_throws ArgumentError R.transition(g,cmd(g,"stand";hand_id=1,rival_id=1))
    old=cmd(g,"stand";hand_id=1)
    g=R.transition(g,old); s=R.public_state(g)
    @test s.phase=="settled" && s.escrow==0
    @test all(r->r.escrow==0,s.rivals)
    @test ledger(s)==11500
    @test s.stats["rounds"]==1
    @test all(r->r.stats["rounds"]==1,s.rivals)
    @test_throws ArgumentError R.transition(g,old)
    rng=MersenneTwister(27)
    for count in (1,2), round in 1:100
        g=R.newgame(500,count)
        g=R.transition(g,cmd(g,"deal";bet=10);order=shuffle(rng,collect(1:52)))
        for step in 1:40
            s=R.public_state(g)
            @test ledger(s)==10000+500*(1+count)
            s.phase=="settled" && break
            @test s.dealer[2]===nothing && s.dealer_score===nothing
            action=rand(rng,s.allowed)
            g=R.transition(g,cmd(g,action;hand_id=s.active_hand_id))
        end
        @test R.public_state(g).phase=="settled"
    end
end

private_state(g) = (public=R.public_state(g),deck=copy(g.deck),cursor=g.cursor,dealer=copy(g.dealer))
function rejects(g,command;order=nothing)
    before=private_state(g)
    @test_throws ArgumentError R.transition(g,command;order=order)
    @test private_state(g)==before
end
deal(prefix;count=1,bet=10,money=500) = begin
    g=R.newgame(money,count)
    R.transition(g,cmd(g,"deal";bet=bet);order=deck(prefix))
end
stand(g) = R.transition(g,cmd(g,"stand";hand_id=g.active_hand_id))
function house!(g,bank)
    # Redistribute a valid conserved ledger to exercise house exhaustion.
    g.balance+=g.bank-bank; g.bank=bank
    R.check_ledger(g)
end

# consumer: demo.blackjack.rivals.policy.decision no private policy inputs; hard/soft boundary
@testset "Visible Stand on 17 benchmark" begin
    for cards in ([10,6],[1,5],[10,7],[1,6],[1,1+13,5],[10,12,3])
        score=R.Rules.score(cards)
        @test R.policy_decision(score)==(score.total<17 ? "hit" : "stand")
    end
    g=deal([10,14,1,8,19,6])
    @test all(r->r.policy=="Stand on 17 benchmark",R.public_state(g).rivals)
    g=stand(g)
    @test g.dealer==[1,6] # Ada stands on soft 17.
    @test g.rivals[1].hands[1].cards==[14,19] # Rival also stands on soft 17.
    @test g.hands[1].result=="win" && g.rivals[1].result=="push"
    @test g.cursor==7
end

@testset "Natural blackjack and peek" begin
    g=deal([1,2,10,13,3,7,11,6])
    @test g.phase=="settled" && g.active_hand_id===nothing
    @test g.hands[1].result=="blackjack" && g.profit==15
    @test g.rivals[1].hands[1].cards==[2,3,11,6]
    @test g.rivals[1].profit==10 && g.dealer==[10,7]
    @test g.cursor==9 && g.revision==1 && ledger(R.public_state(g))==11000

    g=deal([10,2,1,9,3,13])
    @test g.phase=="settled" && g.cursor==7
    @test g.result=="loss" && g.rivals[1].result=="loss"
    @test g.profit==-10 && g.rivals[1].profit==-10
    @test length(g.rivals[1].hands[1].cards)==2

    g=deal([1,14,27,40,10,11,12,13];count=2)
    @test g.phase=="settled" && g.cursor==9
    @test g.result=="push" && all(r->r.result=="push",g.rivals)
    @test g.balance==500 && all(r->r.balance==500,g.rivals)
    @test ledger(R.public_state(g))==11500

    g=deal([10,1,9,8,13,7,2])
    @test g.phase=="playing" && g.rivals[1].hands[1].status=="stood"
    @test g.rivals[1].stats["rounds"]==0 && g.rivals[1].escrow==10
    g=stand(g)
    @test g.result=="push" && g.rivals[1].result=="blackjack"
    @test g.rivals[1].profit==15 && length(g.rivals[1].hands[1].cards)==2
    @test g.dealer==[9,7,2]
end

@testset "Two rivals act in seat order before one dealer turn" begin
    g=deal([10,2,3,9,8,4,5,6,11,12,7,13,14];count=2)
    @test g.cursor==9 && g.dealer==[9,6]
    g=stand(g)
    @test g.rivals[1].hands[1].cards==[2,4,11,12] # 6 -> 16 -> bust
    @test g.rivals[2].hands[1].cards==[3,5,7,13] # 8 -> 15 -> bust
    @test g.dealer==[9,6,14,1] # Dealer draws only after both rivals: 15 -> 16 -> 17.
    @test g.cursor==15 && g.result=="win" && all(r->r.result=="loss",g.rivals)
    @test g.stats["rounds"]==1 && all(r->r.stats["rounds"]==1,g.rivals)
    @test ledger(R.public_state(g))==11500
end

@testset "Split aces, split 21, sequential human hands and double" begin
    for prefix in ([1,10,9,14,7,8,13,12], [10,9,6,23,8,11,1,14,2])
        g=deal(prefix)
        before=private_state(g)
        g2=R.transition(g,cmd(g,"split";hand_id=1))
        @test private_state(g)==before
        @test g2.phase=="settled" && g2.active_hand_id===nothing
        @test [h.id for h in g2.hands]==[1,2]
        @test all(h->length(h.cards)==2 && h.split_hand,g2.hands)
        @test all(h->R.handscore(h).total==21 && !R.handscore(h).natural,g2.hands)
        @test all(h->h.result=="win" && h.profit==10,g2.hands)
        @test g2.profit==20 && g2.stats["hands"]==2 && g2.stats["wins"]==2
        @test g2.stats["rounds"]==1 && g2.rivals[1].stats["rounds"]==1
        @test ledger(R.public_state(g2))==11000
    end
    g=deal([1,10,9,14,7,8,2,3])
    g=R.transition(g,cmd(g,"split";hand_id=1))
    @test g.phase=="settled" && all(h->length(h.cards)==2,g.hands)
    @test [R.handscore(h).total for h in g.hands]==[13,14]

    g=deal([8,10,9,21,7,6,2,3,13,4])
    g=R.transition(g,cmd(g,"split";hand_id=1))
    @test g.active_hand_id==1 && [h.status for h in g.hands]==["playing","waiting"]
    @test g.escrow==20 && g.cursor==9
    @test g.rivals[1].hands[1].cards==[10,7] && g.rivals[1].stats["rounds"]==0
    rejects(g,cmd(g,"stand";hand_id=2))
    rejects(g,cmd(g,"split";hand_id=1))
    old=cmd(g,"double";hand_id=1)
    g=R.transition(g,old)
    @test g.active_hand_id==2 && g.hands[1].stake==20 && g.escrow==30
    @test g.dealer==[9,6] && g.rivals[1].stats["rounds"]==0
    rejects(g,old)
    rejects(g,cmd(g,"stand";hand_id=1))
    g=stand(g)
    @test [h.result for h in g.hands]==["win","loss"] && g.result=="mixed" && g.profit==10
    @test g.dealer==[9,6,4] && g.cursor==11
    @test all(g.stats[k]==v for (k,v) in Dict("rounds"=>1,"hands"=>2,"wins"=>1,"losses"=>1,"pushes"=>0))
    @test g.stats["settled_rounds"]==1 && g.stats["hands"]==2

    g=deal([5,10,9,6,7,8,13])
    g=R.transition(g,cmd(g,"double";hand_id=1))
    @test g.hands[1].cards==[5,6,13] && g.hands[1].stake==20
    @test g.phase=="settled" && g.profit==20 && g.rivals[1].profit==0
    @test ledger(R.public_state(g))==11000
    g=deal([10,8,9,6,7,11,13,2])
    g=R.transition(g,cmd(g,"double";hand_id=1))
    @test g.profit==-20 && g.rivals[1].profit==-10
    @test g.rivals[1].hands[1].cards==[8,7,2] # Rival still acts after human bust.
    g=deal([10,9,2,6,7,3,13,12])
    g=R.transition(g,cmd(g,"hit";hand_id=1))
    @test g.phase=="settled" && g.dealer==[2,3] && g.cursor==9 # All seats bust: no dealer draws.
    # Wallet, rank-pair and original-two-card restrictions remain the human v2 rules.
    g=deal([8,10,9,21,7,6];money=20,bet=20)
    @test !("split" in R.allowed(g)) && !("double" in R.allowed(g))
    rejects(g,cmd(g,"split";hand_id=1))
    rejects(g,cmd(g,"double";hand_id=1))
    g=deal([12,10,9,13,7,6])
    @test !("split" in R.allowed(g)) # Equal values are insufficient: ranks must match.
    g=deal([2,10,9,15,7,6,3])
    g=R.transition(g,cmd(g,"hit";hand_id=1))
    @test R.allowed(g)==["hit","stand"]
end

@testset "Aggregate reserves and affordable rival wagers" begin
    g=house!(R.newgame(500,2),44)
    @test R.public_state(g).max_bet==8
    rejects(g,cmd(g,"deal";bet=10);order=deck([1,14,27,10,11,12,13,8]))
    house!(g,45)
    @test R.public_state(g).max_bet==10
    g=R.transition(g,cmd(g,"deal";bet=10);order=deck([1,14,27,10,11,12,13,8]))
    @test g.bank==0 && g.profit==15 && all(r->r.profit==15,g.rivals)
    @test ledger(R.public_state(g))==11500
    @test R.public_state(g).allowed==["reset"] && R.public_state(g).max_bet==0
    g=house!(R.newgame(500,2),8)
    @test R.public_state(g).allowed==["reset"] && R.public_state(g).max_bet==0
    house!(g,9)
    @test R.public_state(g).max_bet==2 && "deal" in R.allowed(g)

    g=deal([8,1,9,21,13,7,2,3,4])
    house!(g,34)
    @test R.public_state(g).split.reserve==35 # 20 for split human plus rival natural's 15.
    @test !("split" in R.allowed(g)) && !("double" in R.allowed(g))
    rejects(g,cmd(g,"split";hand_id=1))
    rejects(g,cmd(g,"double";hand_id=1))
    house!(g,35)
    @test "split" in R.allowed(g) && "double" in R.allowed(g)
    split=R.transition(g,cmd(g,"split";hand_id=1))
    @test split.escrow==20 && split.rivals[1].escrow==10
    doubled=R.transition(g,cmd(g,"double";hand_id=1))
    @test doubled.phase=="settled" && doubled.bank>=0
    @test ledger(R.public_state(doubled))==11000

    g=R.newgame(500,2)
    for (r,balance) in zip(g.rivals,[5,1])
        g.bank+=r.balance-balance; r.balance=balance
    end
    g=R.transition(g,cmd(g,"deal";bet=10);order=deck([10,2,9,8,3,7,11,6,4]))
    @test g.hands[1].cards==[10,8] && g.rivals[1].hands[1].cards==[2,3]
    @test g.rivals[1].escrow==4 && g.rivals[1].balance==1
    @test isempty(g.rivals[2].hands) && g.rivals[2].escrow==0 && g.rivals[2].balance==1
    @test g.rivals[2].result=="sitting_out" && g.dealer==[9,7]
    g=stand(g)
    @test g.rivals[1].balance==9 && g.rivals[1].profit==4
    @test all(g.rivals[2].stats[k]==0 for k in ("rounds","hands","wins","losses","pushes","resolved_wager","net_chips")) && g.rivals[2].profit==0
    @test g.rivals[2].stats["started_rounds"]==1 && g.rivals[2].stats["sitting_out_rounds"]==1 && g.rivals[2].stats["active_rounds"]==0
    @test ledger(R.public_state(g))==11500
    @test g.cursor==10 && g.dealer==[9,7,4]
    # A previously active seat's stale hands/result disappear when it sits out.
    g.bank+=g.rivals[1].balance; g.rivals[1].balance=0
    g=R.transition(g,cmd(g,"deal";bet=10);order=deck([10,9,8,7,2]))
    @test all(r->isempty(r.hands) && r.result=="sitting_out",g.rivals)
    @test g.hands[1].cards==[10,8] && g.dealer==[9,7]
end

@testset "Command rejection, revision, reset and snapshot isolation" begin
    g=R.newgame()
    for command in (nothing,[],Dict(),cmd(g,"rival_hit"),cmd(g,"deal";bet=true),
                    cmd(g,"deal";bet=2.0),cmd(g,"deal";bet=0),cmd(g,"deal";bet=502),
                    cmd(g,"deal";bet=10,rival_count=2),cmd(g,"stand";hand_id=1),
                    merge(cmd(g,"deal";bet=10),Dict("version"=>2)),
                    merge(cmd(g,"deal";bet=10),Dict("revision"=>false)))
        rejects(g,command)
    end
    boolean_deck=Any[]
    for card in 1:52; push!(boolean_deck,card); end
    boolean_deck[1]=true
    for order in (collect(1:51),vcat(collect(1:51),51),vcat(collect(1:51),53),
                  boolean_deck,Float64.(1:52),"deck")
        rejects(g,cmd(g,"deal";bet=10);order=order)
    end
    for count in (-1,3,true,1.0,"1",nothing)
        rejects(g,cmd(g,"reset";rival_count=count))
    end
    for money in (19,10001,true,500.0)
        rejects(g,cmd(g,"reset";money=money))
    end
    for count in (0,1,2)
        fresh=R.transition(g,cmd(g,"reset";money=200,rival_count=count))
        @test fresh.revision==1 && length(fresh.rivals)==count && fresh.balance==200
        @test all(r->r.balance==200 && r.starting==200,fresh.rivals)
        @test ledger(R.public_state(fresh))==10000+200*(1+count)
    end
    two=R.newgame(200,2)
    @test length(R.transition(two,cmd(two,"reset")).rivals)==2

    g=deal([10,2,9,8,3,7,11,6,4])
    before=private_state(g)
    for command in (cmd(g,"reset";rival_count=0),cmd(g,"deal";bet=10),
                    cmd(g,"hit"),cmd(g,"hit";hand_id=true),cmd(g,"hit";hand_id="rival-1"),
                    cmd(g,"stand";hand_id=1,seat_id="rival-1"))
        rejects(g,command)
    end
    s=R.public_state(g)
    @test s.rivals[1].id=="rival-1" && s.rivals[1].id!=s.hands[1].id
    @test keys(s.hands[1])==keys(s.rivals[1].hands[1])
    s.hands[1].cards[1]=52; s.rivals[1].hands[1].cards[1]=51
    s.stats["rounds"]=99; s.rivals[1].stats["rounds"]=99; s.dealer[1]=50
    @test private_state(g)==before
    different=deepcopy(g)
    different.dealer[2]=13; reverse!(different.deck)
    @test R.public_state(different)==R.public_state(g) # Hole/deck cannot affect public eligibility.
    settled=stand(g)
    @test private_state(g)==before && settled.revision==g.revision+1
    s=R.public_state(settled); before=private_state(settled)
    s.history[1].hands[1].cards[1]=52; s.history[1].dealer[1]=51
    @test private_state(settled)==before
    rejects(settled,cmd(g,"stand";hand_id=1))
    rejects(settled,cmd(settled,"stand";hand_id=1))
    @test_throws ArgumentError R.settle!(settled)
    @test private_state(settled)==before
    # A late failure, after mutation of the transition copy, must not leak any changes.
    exhausted=deepcopy(g); exhausted.cursor=53
    rejects(exhausted,cmd(exhausted,"hit";hand_id=1))
    rejects(exhausted,cmd(exhausted,"stand";hand_id=1))
end

@testset "Repeated rounds and retained human history" begin
    g=R.newgame()
    for round in 1:25
        g=R.transition(g,cmd(g,"deal";bet=10);order=deck([10,11,12,8,21,34]))
        g=stand(g)
        @test g.result=="push" && g.rivals[1].result=="push"
        @test g.stats["rounds"]==round && g.rivals[1].stats["rounds"]==round
        @test ledger(R.public_state(g))==11000
    end
    @test length(g.history)==20 && first(g.history).round==25 && last(g.history).round==6
    @test g.revision==50 && g.stats["pushes"]==25 && g.rivals[1].stats["pushes"]==25
    @test all(h->keys(h)==(:round,:result,:profit,:stake,:balance,:hands,:dealer),g.history)
    # Exercise the production shuffle path as well as deterministic fixtures.
    fresh=R.newgame(500,2)
    shuffled=R.transition(fresh,cmd(fresh,"deal";bet=10))
    @test sort(shuffled.deck)==collect(1:52)
    @test isempty(fresh.deck) && fresh.revision==0
    while shuffled.phase=="playing"
        shuffled=stand(shuffled)
    end
    @test ledger(R.public_state(shuffled))==11500
end
end
