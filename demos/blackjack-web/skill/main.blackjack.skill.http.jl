# Included inside BlackjackWebServer after the engines, replay module and Store.
# publish: demo.blackjack.skill.http session-owned private recording and detached exports
const REPORT_COMPARISON="Practice results only: rivals hit/stand; the human may split/double. Shared deck, seat order, stakes and bankroll affect results; no skill rating."
engine_for(game)=game isa BlackjackRivals.Table ? BlackjackRivals : BlackjackWeb

function session_record(money,count;policies=nothing,revision=0)
    count=BlackjackWeb.integer(count,0,2,"Rival count")
    if count==0
        policies===nothing || (policies isa AbstractVector && isempty(policies)) || throw(ArgumentError("Solo tables have no rival policies"))
        return BlackjackReplay.new_recording(BlackjackWeb;money=money,initial_revision=revision)
    end
    BlackjackReplay.new_recording(BlackjackRivals;money=money,rival_count=count,policies=policies,initial_revision=revision)
end

function public_session_report(game)
    game.phase=="settled" || throw(ArgumentError("Finish a round before exporting this session"))
    rival=game isa BlackjackRivals.Table
    seats=Any[(id="human",balance=game.balance,starting=game.starting,
        net=game.balance+game.escrow-game.starting,capabilities=["hit","stand","double","split"],
        statistics=BlackjackWeb.BlackjackStatistics.summary(game.stats))]
    if rival
        for seat in game.rivals
            push!(seats,(id=seat.id,balance=seat.balance,starting=seat.starting,
                net=seat.balance+seat.escrow-seat.starting,capabilities=["hit","stand"],
                statistics=BlackjackRivals.BlackjackStatistics.summary(seat.stats)))
        end
    end
    (schema="blackjack-session-report-v1",
     provenance=(engine=rival ? "blackjack-rivals-v3" : "blackjack-web-v2",rules="s17-3to2-one-split-v1",
        strategies=rival ? [r.policy_id for r in game.rivals] : String[]),
     comparison_label=rival ? REPORT_COMPARISON : "Solo practice; profit is descriptive, not a skill rating.",
     settled_session_rounds=game.stats["settled_rounds"],seats=seats)
end

function current_recorder!(store,key,game)
    if haskey(store.recorders,key)
        # Tests and local tools may explicitly replace a Store fixture. Such a
        # replacement cannot borrow another game's recording or claim replay.
        rec=store.recorders[key]
        if BlackjackReplay.state_hash(rec.game)!=BlackjackReplay.state_hash(game)
            delete!(store.recorders,key)
        end
    end
    get(store.recorders,key,nothing)
end

function recorded_transition!(store,key,game,command)
    if get(command,"action",nothing)=="reset"
        "reset" in table_view(game).allowed || throw(ArgumentError("Finish this round before changing seats"))
        all(k->k in ("version","revision","action","money","rival_count","rival_policies"),keys(command)) || throw(ArgumentError("Unknown command field"))
        current=game isa BlackjackRivals.Table ? length(game.rivals) : 0
        rec=session_record(get(command,"money",500),get(command,"rival_count",current);
            policies=get(command,"rival_policies",nothing),revision=game.revision+1)
        store.recorders[key]=rec
        return deepcopy(rec.game)
    end
    rec=current_recorder!(store,key,game)
    rec===nothing && return table_transition(game,command)
    # Modules are immutable identities and Julia deliberately cannot deepcopy
    # them. Detach recorder-owned data while keeping the engine identity.
    candidate=BlackjackReplay.Recorder(rec.engine,deepcopy(rec.game),deepcopy(rec.initial),
        copy(rec.strategy_ids),rec.initial_hash,deepcopy(rec.steps))
    next=BlackjackReplay.record!(candidate,command)
    store.recorders[key]=candidate
    next
end

function replay_request(request,store,key,game)
    if request.method=="GET"
        game.phase=="settled" || return failure(409,"Finish the round before exporting private replay")
        rec=current_recorder!(store,key,game)
        rec===nothing && return failure(409,"No recording for this table; start a new session")
        return response(200,BlackjackReplay.export_replay(rec);cookie=key)
    end
    parsed=try JSON3.read(String(request.body),Dict{String,Any}) catch
        return failure(400,"Invalid replay JSON")
    end
    parsed isa AbstractDict && Set(keys(parsed))==Set(["replay"]) || return failure(400,"Expected exactly one replay envelope")
    envelope=parsed["replay"]
    envelope isa AbstractDict || return failure(400,"Replay envelope must be an object")
    id=get(envelope,"engine_id",nothing)
    engine=id=="blackjack-web-v2" ? BlackjackWeb : id=="blackjack-rivals-v3" ? BlackjackRivals : nothing
    engine===nothing && return failure(400,"Unsupported replay engine")
    replay=try BlackjackReplay.playback(engine,envelope) catch error
        error isa ArgumentError || rethrow()
        return failure(400,sprint(showerror,error))
    end
    # Playback is an isolated result. Neither session nor its private recorder changes.
    response(200,(schema="blackjack-replay-result-v1",state=table_view(replay));cookie=key)
end
