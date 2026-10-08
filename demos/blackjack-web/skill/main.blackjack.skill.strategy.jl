module BlackjackStrategy

export policies, decision

const INPUT_KEYS = ("total", "soft", "natural", "bust", "dealer_upcard",
                    "legal_actions", "affordable_stake", "rules_id")
const RULES_ID = "s17-3to2-one-split-v1"

"""Return detached identities for the deterministic hit/stand benchmarks."""
policies() = [
    (id="stand17-v1", name="Stand on 17 benchmark", version=1,
     capabilities=["hit", "stand"]),
    (id="dealer-aware-v1", name="Dealer-aware benchmark", version=1,
     capabilities=["hit", "stand"]),
]

is_integer(value) = value isa Integer && !(value isa Bool)

function validate_input(input::AbstractDict)
    length(input) == length(INPUT_KEYS) &&
        all(key -> key isa AbstractString && key in INPUT_KEYS, keys(input)) &&
        all(key -> haskey(input, key), INPUT_KEYS) ||
        throw(ArgumentError("decision requires exactly the public input keys"))

    total = input["total"]
    is_integer(total) && 4 <= total <= 31 ||
        throw(ArgumentError("total must be an integer in 4:31"))
    for key in ("soft", "natural", "bust")
        input[key] isa Bool || throw(ArgumentError("$key must be Bool"))
    end
    card = input["dealer_upcard"]
    is_integer(card) && 1 <= card <= 52 ||
        throw(ArgumentError("dealer_upcard must be an integer in 1:52"))
    stake = input["affordable_stake"]
    is_integer(stake) && stake >= 0 ||
        throw(ArgumentError("affordable_stake must be a nonnegative integer"))
    rules = input["rules_id"]
    rules isa AbstractString && rules == RULES_ID ||
        throw(ArgumentError("unsupported rules_id"))

    legal = input["legal_actions"]
    legal isa Union{AbstractVector,Tuple,AbstractSet} && 1 <= length(legal) <= 2 &&
        all(action -> action isa AbstractString && action in ("hit", "stand"), legal) &&
        length(unique(collect(legal))) == length(legal) ||
        throw(ArgumentError("legal_actions must be a nonempty unique subset of hit/stand"))
    return nothing
end

function dealer_aware_preference(total, soft, card)
    # Card IDs contain four consecutive suits of ranks A, 2, ..., K.
    rank = mod(card - 1, 13) + 1
    dealer = rank == 1 ? 11 : min(rank, 10)
    if soft
        return total >= 19 || (total == 18 && dealer <= 8) ? "stand" : "hit"
    end
    return total >= 17 || (13 <= total <= 16 && 2 <= dealer <= 6) ||
           (total == 12 && 4 <= dealer <= 6) ? "stand" : "hit"
end

"""Choose a legal action using only the validated public-information input."""
function decision(policy_id, input::AbstractDict)
    policy_id isa AbstractString && policy_id in ("stand17-v1", "dealer-aware-v1") ||
        throw(ArgumentError("unknown policy_id"))
    validate_input(input)
    preferred = if input["natural"] || input["bust"]
        "stand"
    elseif policy_id == "stand17-v1"
        input["total"] >= 17 ? "stand" : "hit"
    else
        dealer_aware_preference(input["total"], input["soft"], input["dealer_upcard"])
    end
    legal = input["legal_actions"]
    return preferred in legal ? preferred : ("stand" in legal ? "stand" : "hit")
end

decision(policy_id, input) = throw(ArgumentError("input must be an AbstractDict"))

end
