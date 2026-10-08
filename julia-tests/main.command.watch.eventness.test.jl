# Core Eventness topic/query integration. No demo selection or Lang prerequisite.
include("runtests.setup.jl")

@testset "Native Eventness Watch core integration" begin
    watch_bin = get(ENV, "RECUR_WATCH_BIN", joinpath(dirname(RECUR_BIN), "recur-watch" * (Sys.iswindows() ? ".exe" : "")))
    function invoke_topic(root, args)
        output, errors = IOBuffer(), IOBuffer()
        process = run(pipeline(ignorestatus(Cmd(vcat([watch_bin], args, ["-d", root, "--json"]))), stdout=output, stderr=errors))
        return success(process), String(take!(output)), String(take!(errors))
    end
    function topic_snapshot(root)
        files = Dict{String,Vector{UInt8}}()
        for (dir, _, names) in walkdir(root), name in names
            file = joinpath(dir, name)
            files[relpath(file, root)] = read(file)
        end
        files
    end
    mktempdir() do root
        mkpath(joinpath(root, "eventness"))
        original = topic_snapshot(root)
        ok, output, _ = run_recur(["watch", "topics", "-d", root, "--json"])
        @test ok
        @test isempty(JSON3.read(output))
        @test topic_snapshot(root) == original

        args = ["topic", "create", "project.results", "--filter", "task.**", "--eventness-dir", "eventness"]
        ok, preview, _ = invoke_topic(root, args)
        @test ok
        @test JSON3.read(preview).confirmed == false
        @test topic_snapshot(root) == original
        ok, _, _ = invoke_topic(root, vcat(args, ["--confirm"]))
        @test ok
        registered = topic_snapshot(root)
        ok, topics, _ = run_recur(["watch", "topics", "-d", root, "--json"])
        @test ok
        binding = only(JSON3.read(topics))
        @test binding.topic == "project.results"
        @test binding.warp === nothing
        @test binding.filter == "task.**"
        @test topic_snapshot(root) == registered

        subscribe = ["topic", "subscribe", "project.results", "--id", "coordinator", "--confirm"]
        ok, _, _ = invoke_topic(root, subscribe)
        @test ok
        producer = joinpath(root, "eventness", "task.one.complete.md")
        intelligence = "publish: task.one.ready\nUseful instructions remain in this file.\n"
        write(producer, intelligence)
        before = topic_snapshot(root)
        drain = ["topic", "drain", "project.results", "--id", "coordinator", "--max-events", "1"]
        ok, _, _ = invoke_topic(root, drain)
        @test ok
        @test topic_snapshot(root) == before
        ok, output, _ = invoke_topic(root, vcat(drain, ["--confirm"]))
        @test ok
        notification = only(JSON3.read(output))
        @test collect(keys(notification)) == [:trace_ids]
        @test notification.trace_ids == ["task.one.ready"]
        @test read(producer, String) == intelligence
        ok, _, _ = invoke_topic(root, subscribe)
        @test ok
        ok, output, _ = invoke_topic(root, vcat(drain, ["--confirm"]))
        @test ok
        @test isempty(JSON3.read(output))
        before = topic_snapshot(root)
        ok, output, _ = invoke_topic(root, ["topic", "replay", "project.results", "--id", "coordinator", "--sequence", "1"])
        @test ok
        @test only(JSON3.read(output)).trace_ids == ["task.one.ready"]
        @test topic_snapshot(root) == before
    end
end
