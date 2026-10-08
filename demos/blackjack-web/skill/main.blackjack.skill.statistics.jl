module BlackjackStatistics

const COUNTERS = ("rounds", "hands", "wins", "losses", "pushes", "started_rounds",
    "settled_rounds", "active_rounds", "sitting_out_rounds", "naturals", "busts",
    "resolved_wager", "net_chips")
const RESULTS = ("win", "loss", "push", "blackjack")
const HAND_FIELDS = (:result, :stake, :profit, :natural, :bust)
const ROI_LABEL = "Net chips / resolved wager"

empty_stats() = Dict{String,Int}(k => 0 for k in COUNTERS)

function check_stats(stats)
    stats isa AbstractDict || throw(ArgumentError("stats must be a dictionary"))
    for k in COUNTERS
        v = get(stats, k, nothing)
        v isa Int || throw(ArgumentError("invalid statistics counter $k"))
    end
    return stats
end

function start_round!(stats; active::Bool=true)
    check_stats(stats)
    candidate=copy(stats)
    add!(candidate,"started_rounds",1)
    add!(candidate,active ? "active_rounds" : "sitting_out_rounds",1)
    merge!(stats,candidate)
    return stats
end

isint(x) = x isa Integer && !(x isa Bool)

function validate_hand(h)
    (h isa NamedTuple && Set(keys(h)) == Set(HAND_FIELDS)) ||
        throw(ArgumentError("hand must be a NamedTuple with exactly result/stake/profit/natural/bust"))
    (h.result isa AbstractString && String(h.result) in RESULTS) ||
        throw(ArgumentError("invalid hand result"))
    (isint(h.stake) && 0 < h.stake <= typemax(Int)) || throw(ArgumentError("stake must be a positive machine integer"))
    (isint(h.profit) && typemin(Int) <= h.profit <= typemax(Int)) || throw(ArgumentError("profit must fit a machine integer"))
    (h.natural isa Bool && h.bust isa Bool) || throw(ArgumentError("natural and bust must be Bool"))
    result = String(h.result)
    result == "blackjack" && !h.natural && throw(ArgumentError("blackjack requires natural"))
    h.natural && h.bust && throw(ArgumentError("hand cannot be natural and bust"))
    h.bust && result != "loss" && throw(ArgumentError("bust must be a loss"))
    return nothing
end

function add!(stats,key,value)
    stats[key]=try Base.Checked.checked_add(stats[key],Int(value)) catch e
        e isa OverflowError || rethrow()
        throw(ArgumentError("statistics counter overflow: $key"))
    end
end

function settle_round!(stats, hands)
    check_stats(stats)
    hands isa AbstractVector || throw(ArgumentError("hands must be a list"))
    foreach(validate_hand, hands)
    isempty(hands) && return stats
    candidate=copy(stats)
    add!(candidate,"rounds",1)
    add!(candidate,"settled_rounds",1)
    for h in hands
        r = String(h.result)
        add!(candidate,"hands",1)
        if r == "win" || r == "blackjack"
            add!(candidate,"wins",1)
        elseif r == "loss"
            add!(candidate,"losses",1)
        else
            add!(candidate,"pushes",1)
        end
        h.natural && add!(candidate,"naturals",1)
        h.bust && add!(candidate,"busts",1)
        add!(candidate,"resolved_wager",h.stake)
        add!(candidate,"net_chips",h.profit)
    end
    merge!(stats,candidate)
    return stats
end

function summary(stats)
    check_stats(stats)
    c = Dict(Symbol(k) => Int(stats[k]) for k in COUNTERS)
    wager = c[:resolved_wager]
    roi = wager == 0 ? nothing : c[:net_chips] / wager
    base = (; (Symbol(k) => c[Symbol(k)] for k in COUNTERS)...)
    return merge(base, (roi=roi, roi_label=ROI_LABEL,
        sample_hands=c[:hands], sample_rounds=c[:settled_rounds]))
end

end
