# consumes: demo.holdem.deal explicit showdown request
# produces: demo.holdem.rank distinct, labeled producer results
rank_player(request, player) = (player=player,rank=evaluate((cards=vcat(request.holes[player],request.board),)).rank)

# consumes: demo.holdem.graph implemented dependency order; Lang does not run it
# produces: demo.holdem.settle audited settlement from both independent rankings
function coordinate(request; ranker=rank_player)
    length(request.holes) == 2 && all(h -> length(h) == 2, request.holes) &&
        length(request.board) == 5 || throw(ArgumentError("invalid showdown shape"))
    length(unique(vcat(request.holes...,request.board))) == 9 || throw(ArgumentError("duplicate card across players/board"))
    request.pot isa Int && request.pot >= 0 || throw(ArgumentError("invalid pot"))
    # Both tasks start before either is awaited. Pure jobs need no shared writes.
    # Julia tasks are cooperative; this promises dependencies, not CPU speedup.
    tasks = [@async ranker(request,p) for p in 1:2]
    # Wait for both even on failure, so a failing producer cannot leave work behind.
    foreach(t -> wait(t; throw=false), tasks)
    left, right = fetch.(tasks)
    left.player == 1 && right.player == 2 || throw(ArgumentError("wrong producer identity"))
    selected = winners((left=left.rank,right=right.rank)).winners
    payouts = zeros(Int,2)
    amount, extra = divrem(request.pot,length(selected))
    for p in selected
        payouts[p] += amount
    end
    extra > 0 && (payouts[2] += extra)
    (conserved=all(>=(0),payouts) && sum(payouts) == request.pot,
     settlement=(winners=selected,payouts=payouts))
end
