module ReplayContractTests
Base.Experimental.@compiler_options compile=min optimize=0 infer=false
using Test,JSON3,SHA
include("../main.blackjack.rivals.engine.jl")
include("main.blackjack.skill.replay.jl")
const E=BlackjackRivals;const P=BlackjackReplay
deck(prefix)=vcat(prefix,[c for c in 1:52 if !(c in prefix)])
cmd(g,action;kwargs...)=merge(Dict{String,Any}("version"=>3,"revision"=>g.revision,"action"=>action),Dict(string(k)=>v for (k,v) in kwargs))
# register: demo.blackjack.skill.replay complete-deck authority, hashes and nonmutating rejection
@testset "Frozen private replay" begin
    rec=P.new_recording(E;money=100,rival_count=1)
    @test_throws ArgumentError P.export_replay(rec)
    P.record!(rec,cmd(rec.game,"deal";bet=10);order=deck([10,2,9,8,11,7]))
    @test rec.game.phase=="playing"
    @test_throws ArgumentError P.export_replay(rec)
    before=P.state_hash(rec.game)
    @test_throws ArgumentError P.record!(rec,cmd(rec.game,"hit";hand_id=2))
    @test P.state_hash(rec.game)==before
    P.record!(rec,cmd(rec.game,"stand";hand_id=1))
    env=P.export_replay(rec)
    @test env["schema"]=="blackjack-private-replay-v1"
    result=P.playback(E,JSON3.read(JSON3.write(env),Dict{String,Any}))
    @test P.state_hash(result)==P.state_hash(rec.game)
    @test E.public_state(result)==E.public_state(rec.game)
    @test P.state_hash(Dict("b"=>1,"a"=>2))==P.state_hash(Dict("a"=>2,"b"=>1))
    @test P.state_hash(Dict("b"=>1,"a"=>2))==bytes2hex(sha256("{\"a\":2,\"b\":1}"))
    @test P.state_hash(Dict("z"=>Any[true,nothing,2]))==bytes2hex(sha256("{\"z\":[true,null,2]}"))
    for mutate in [e->(e["schema"]="unknown"),e->(e["engine_id"]="unknown"),e->(e["steps"][1]["deck"][2]=e["steps"][1]["deck"][1]),e->(e["steps"][1]["deck"][1]=true),e->pop!(e["steps"][1]["deck"]),e->(e["steps"][1]["state_hash"]="tampered"),e->(e["steps"][2]["command"]["revision"]=999),e->(e["initial"]["money"]=true),e->(e["unexpected"]=1)]
        bad=deepcopy(env);mutate(bad);snapshot=deepcopy(bad)
        @test_throws ArgumentError P.playback(E,bad)
        @test bad==snapshot && P.state_hash(rec.game)==P.state_hash(result)
    end
    @test_throws ArgumentError P.record!(rec,cmd(rec.game,"reset"))
    # Deterministic private deck authority is never present in public snapshots.
    @test !hasproperty(E.public_state(rec.game),:deck)
    for count in 0:2
        r=P.new_recording(E;money=100,rival_count=count,initial_revision=10)
        @test r.game.revision==10
        P.record!(r,cmd(r.game,"deal";bet=10);order=collect(1:52))
        while r.game.phase=="playing"
            P.record!(r,cmd(r.game,"stand";hand_id=r.game.active_hand_id))
        end
        @test P.state_hash(P.playback(E,P.export_replay(r)))==P.state_hash(r.game)
    end
    @test_throws ArgumentError P.new_recording(E;initial_revision=true)
end
end
