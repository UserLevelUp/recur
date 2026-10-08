module BlackjackReplay
using JSON3, SHA, Random

const SCHEMA = "blackjack-private-replay-v1"
const RULES_ID = "s17-3to2-one-split-v1"
const POLICY_IDS = ("stand17-v1", "dealer-aware-v1")
const MAX_COMMANDS = 2000
const MAX_DEALS = 200
const MAX_BYTES = 1024 * 1024
const ENVELOPE_KEYS = ("schema", "engine_id", "rules_id", "initial", "strategy_ids",
                       "initial_hash", "steps", "final_hash")

fail(message) = throw(ArgumentError(message))
function integer(value, lo, hi, label)
    value isa Integer && !(value isa Bool) && lo <= value <= hi ||
        fail("$label must be an integer in $lo:$hi")
    Int(value)
end

function exact_keys(value, expected, label)
    value isa AbstractDict || fail("$label must be an object")
    length(value) == length(expected) &&
        all(k -> k isa AbstractString && k in expected, keys(value)) ||
        fail("Unexpected or missing $label keys")
    value
end

# Write canonical UTF-8 JSON directly, without relying on dictionary iteration or
# JSON3's struct mapping. The bounded writer stops before allocating a huge envelope.
function canonical(value; limit=typemax(Int), structs=true)
    io = IOBuffer()
    ancestors = IdDict{Any,Nothing}()
    function emit(s)
        ncodeunits(s) <= limit - position(io) || fail("Canonical data exceeds byte limit")
        write(io, s)
    end
    function quoted(s)
        isvalid(s) || fail("Invalid UTF-8 string")
        ncodeunits(s) <= limit - position(io) || fail("Canonical data exceeds byte limit")
        emit(JSON3.write(String(s)))
    end
    function visit(x, depth)
        depth <= 128 || fail("Cyclic or excessively nested canonical data")
        if x === nothing
            emit("null")
        elseif x isa Bool
            emit(x ? "true" : "false")
        elseif x isa Integer
            emit(string(x))
        elseif x isa AbstractFloat
            isfinite(x) || fail("Nonfinite number")
            emit(JSON3.write(x))
        elseif x isa AbstractString
            quoted(x)
        else
            haskey(ancestors, x) && fail("Cyclic canonical data")
            ancestors[x] = nothing
            try
                if x isa AbstractDict || x isa NamedTuple
                    length(x) <= limit - position(io) || fail("Canonical data exceeds byte limit")
                    names = x isa NamedTuple ? string.(collect(keys(x))) : collect(keys(x))
                    all(k -> k isa AbstractString, names) || fail("Map keys must be strings")
                    # Check key sizes before sorting or escaping them.
                    keybytes = 0
                    for k in names
                        ncodeunits(k) <= limit - keybytes || fail("Canonical data exceeds byte limit")
                        keybytes += ncodeunits(k)
                    end
                    sort!(names)
                    emit("{")
                    for (i, k) in enumerate(names)
                        i > 1 && emit(",")
                        quoted(k); emit(":")
                        visit(x isa NamedTuple ? getfield(x, Symbol(k)) : x[k], depth + 1)
                    end
                    emit("}")
                elseif x isa AbstractVector || (structs && x isa Tuple)
                    length(x) <= limit - position(io) || fail("Canonical data exceeds byte limit")
                    emit("[")
                    for (i, item) in enumerate(x)
                        i > 1 && emit(",")
                        visit(item, depth + 1)
                    end
                    emit("]")
                elseif structs && isstructtype(typeof(x)) && !isprimitivetype(typeof(x)) &&
                       !(x isa Union{Function,Module,Type,AbstractArray,Number,AbstractSet,IO,Task}) &&
                       !(parentmodule(typeof(x)) in (Base, Core))
                    names = sort!(collect(fieldnames(typeof(x))); by=string)
                    emit("{")
                    for (i, name) in enumerate(names)
                        i > 1 && emit(",")
                        quoted(string(name)); emit(":")
                        isdefined(x, name) || fail("Undefined state field")
                        visit(getfield(x, name), depth + 1)
                    end
                    emit("}")
                else
                    fail("Unsupported canonical value: $(typeof(x))")
                end
            finally
                delete!(ancestors, x)
            end
        end
    end
    visit(value, 0)
    take!(io)
end

state_hash(game) = bytes2hex(sha256(canonical(game)))
function check_hash(value)
    value isa AbstractString && ncodeunits(value) == 64 &&
        all(c -> c in '0':'9' || c in 'a':'f', value) || fail("Invalid SHA-256 hash")
    value
end

function identity(engine)
    engine isa Module || fail("Expected an engine module")
    nameof(engine) == :BlackjackRivals && return ("blackjack-rivals-v3", 3)
    nameof(engine) == :BlackjackWeb && return ("blackjack-web-v2", 2)
    fail("Unsupported engine")
end

function parameters(engine, money, rival_count, policies, revision)
    _, version = identity(engine)
    funds = integer(money, 20, 10000, "Starting chips")
    count = integer(rival_count, 0, 2, "Rival count")
    base = integer(revision, 0, typemax(Int)-1, "Initial revision")
    version == 2 && (count != 0 || policies !== nothing) && fail("Solo engine cannot use rivals or policies")
    selected = nothing
    if policies !== nothing
        policies isa AbstractVector && length(policies) == count || fail("Policies must match rival count")
        all(p -> p isa AbstractString && p in POLICY_IDS, policies) || fail("Unsupported policy")
        selected = String[p for p in policies]
    end
    initial = Dict{String,Any}("money"=>funds, "rival_count"=>count,
                               "policies"=>selected, "revision"=>base)
    strategies = selected === nothing ? fill("stand17-v1", count) : copy(selected)
    initial, strategies
end

mutable struct Recorder
    engine::Module
    game::Any
    initial::Dict{String,Any}
    strategy_ids::Vector{String}
    initial_hash::String
    steps::Vector{Dict{String,Any}}
end

# Only explicitly selected policies use the optional keyword. In particular, the
# frozen baseline engine has no such keyword and remains usable with defaults.
function new_recording(engine; money=500, rival_count=0, policies=nothing, initial_revision=0)
    initial, strategies = parameters(engine, money, rival_count, policies, initial_revision)
    _, version = identity(engine)
    game = try
        if version == 2
            engine.newgame(initial["money"])
        elseif policies === nothing
            engine.newgame(initial["money"], initial["rival_count"])
        else
            engine.newgame(initial["money"], initial["rival_count"]; policies=copy(initial["policies"]))
        end
    catch err
        err isa InterruptException && rethrow()
        fail("Engine could not create recording: $(sprint(showerror, err))")
    end
    game.revision = initial["revision"]
    Recorder(engine, game, initial, strategies, state_hash(game), Dict{String,Any}[])
end

function checked_command(engine, command, revision)
    command isa AbstractDict || fail("Command must be an object")
    action = get(command, "action", nothing)
    action isa AbstractString && action in ("deal", "hit", "stand", "double", "split") ||
        fail("Unsupported command; reset requires a new recording")
    exact_keys(command, ("version", "revision", "action", action == "deal" ? "bet" : "hand_id"), "command")
    _, version = identity(engine)
    integer(command["version"], version, version, "Protocol version")
    integer(command["revision"], 0, typemax(Int)-1, "Revision") == revision || fail("Stale revision")
    if action == "deal"
        bet = integer(command["bet"], 2, 500, "Bet")
        iseven(bet) || fail("Bet must be even")
    else
        integer(command["hand_id"], 1, 2, "Hand ID")
    end
    Dict{String,Any}(String(k)=>deepcopy(v) for (k,v) in command)
end

function checked_deck(order)
    order isa AbstractVector && length(order) == 52 || fail("Deck must contain 52 cards")
    cards = Int[integer(card, 1, 52, "Card") for card in order]
    length(unique(cards)) == 52 || fail("Deck must be a permutation of 1:52")
    cards
end

function envelope(rec, steps=rec.steps, final_hash=isempty(steps) ? rec.initial_hash : steps[end]["state_hash"])
    Dict{String,Any}("schema"=>SCHEMA, "engine_id"=>identity(rec.engine)[1], "rules_id"=>RULES_ID,
        "initial"=>rec.initial, "strategy_ids"=>rec.strategy_ids, "initial_hash"=>rec.initial_hash,
        "steps"=>steps, "final_hash"=>final_hash)
end

function transition(engine, game, command, deck)
    try
        # Defend recorder ownership even if an engine mutates its input before failing.
        result = engine.transition(deepcopy(game), deepcopy(command); order=deepcopy(deck))
        result.revision == game.revision + 1 || fail("Engine revision did not advance once")
        result
    catch err
        err isa InterruptException && rethrow()
        fail("Replay transition rejected: $(sprint(showerror, err))")
    end
end

# publish: demo.blackjack.skill.replay transactional complete-deck recording
function record!(rec::Recorder, command; order=nothing)
    state_hash(rec.game)==(isempty(rec.steps) ? rec.initial_hash : rec.steps[end]["state_hash"]) || fail("Recorder game was modified directly")
    length(rec.steps) < MAX_COMMANDS || fail("Command limit reached")
    accepted = checked_command(rec.engine, command, rec.game.revision)
    deal = accepted["action"] == "deal"
    # One deck bounds a round's card-taking actions. Reserve room for settlement
    # before admitting a new round; hitting a limit must not strand a live hand.
    if deal
        length(rec.steps) <= MAX_COMMANDS-52 || fail("Start a new table: recording command capacity is nearly full")
        rec.game.revision <= typemax(Int)-53 || fail("Start a new table: revision capacity is nearly full")
        length(canonical(envelope(rec);limit=MAX_BYTES,structs=false)) <= MAX_BYTES-32768 || fail("Start a new table: recording byte capacity is nearly full")
    end
    deal && count(s -> s["command"]["action"] == "deal", rec.steps) >= MAX_DEALS && fail("Deal limit reached")
    !deal && order !== nothing && fail("Only deal may supply a deck")
    cards = deal ? (order === nothing ? shuffle(RandomDevice(), collect(1:52)) : checked_deck(order)) : nothing
    step = Dict{String,Any}("command"=>accepted, "deck"=>cards, "state_hash"=>repeat("0",64))
    steps = [rec.steps; [step]]
    # Hash length is fixed, so the placeholder gives the exact candidate size.
    canonical(envelope(rec, steps); limit=MAX_BYTES, structs=false)
    game = transition(rec.engine, rec.game, accepted, cards)
    step["state_hash"] = state_hash(game)
    returned = deepcopy(game)
    rec.game = game
    rec.steps = steps
    returned
end

function export_replay(rec::Recorder)
    rec.game.phase == "settled" || fail("Only settled games may be exported")
    result = envelope(rec)
    state_hash(rec.game) == result["final_hash"] || fail("Recorder game was modified directly")
    canonical(result; limit=MAX_BYTES, structs=false)
    deepcopy(result)
end

# publish: demo.blackjack.skill.replay bounded private reconstruction with every hash checked
function playback(engine, input)
    engine_id, _ = identity(engine)
    exact_keys(input, ENVELOPE_KEYS, "envelope")
    input["schema"] isa AbstractString && input["schema"] == SCHEMA || fail("Unknown replay schema")
    input["engine_id"] isa AbstractString && input["engine_id"] == engine_id || fail("Engine identity mismatch")
    input["rules_id"] isa AbstractString && input["rules_id"] == RULES_ID || fail("Rules identity mismatch")
    initial = exact_keys(input["initial"], ("money", "rival_count", "policies", "revision"), "initial")
    params, strategies = parameters(engine, initial["money"], initial["rival_count"], initial["policies"], initial["revision"])
    ids = input["strategy_ids"]
    ids isa AbstractVector && length(ids) == length(strategies) &&
        all(p -> p isa AbstractString, ids) && ids == strategies || fail("Strategy identity mismatch")
    check_hash(input["initial_hash"]); check_hash(input["final_hash"])
    steps = input["steps"]
    steps isa AbstractVector && length(steps) <= MAX_COMMANDS || fail("Invalid or excessive steps")
    # Validate all input and limits before constructing or running an engine.
    deals = 0
    revision = params["revision"]
    for step in steps
        exact_keys(step, ("command", "deck", "state_hash"), "step")
        check_hash(step["state_hash"])
        command = checked_command(engine, step["command"], revision)
        revision += 1 # checked_command excluded typemax(Int), so this cannot overflow
        if command["action"] == "deal"
            deals += 1
            deals <= MAX_DEALS || fail("Deal limit exceeded")
            checked_deck(step["deck"])
        else
            step["deck"] === nothing || fail("Only deal may carry a deck")
        end
    end
    canonical(input; limit=MAX_BYTES, structs=false)
    rec = new_recording(engine; money=params["money"], rival_count=params["rival_count"],
                        policies=params["policies"], initial_revision=params["revision"])
    rec.initial_hash == input["initial_hash"] || fail("Initial state hash mismatch")
    game = rec.game
    for step in steps
        game = transition(engine, game, step["command"], step["deck"])
        state_hash(game) == step["state_hash"] || fail("Post-state hash mismatch")
    end
    game.phase == "settled" || fail("Replay must finish settled")
    state_hash(game) == input["final_hash"] || fail("Final state hash mismatch")
    deepcopy(game)
end
end
