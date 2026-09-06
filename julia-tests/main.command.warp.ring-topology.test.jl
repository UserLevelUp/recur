"""Executable contract for recursive Warp rings and coordinator/worker watches."""

include("runtests.setup.jl")

const WARP_RING_FIXTURE_ROOT = joinpath(@__DIR__, "fixtures", "warp-ring-v1")

mutable struct RingWatchHandle
    process::Base.Process
    stdout_path::String
    stderr_path::String
end

ring_watch_bin() = get(
    ENV,
    "RECUR_WATCH_BIN",
    joinpath(@__DIR__, "..", "target", RECUR_PROFILE, "recur-watch" * (Sys.iswindows() ? ".exe" : "")),
)

function spawn_ring_watch(args::Vector{String})
    stdout_path = tempname()
    stderr_path = tempname()
    stdout_io = open(stdout_path, "w")
    stderr_io = open(stderr_path, "w")
    cmd = Cmd(`$(ring_watch_bin()) $args`, dir=normpath(joinpath(@__DIR__, "..")))
    process = run(pipeline(cmd, stdout=stdout_io, stderr=stderr_io); wait=false)
    close(stdout_io)
    close(stderr_io)
    return RingWatchHandle(process, stdout_path, stderr_path)
end

function wait_for_ring_watch_ready(handle::RingWatchHandle; timeout_seconds::Float64=4.0)
    deadline = time() + timeout_seconds
    while time() < deadline
        stderr_text = isfile(handle.stderr_path) ? read(handle.stderr_path, String) : ""
        occursin("recur watch: ready", stderr_text) && return true
        process_exited(handle.process) && return false
        sleep(0.05)
    end
    return false
end

function wait_for_ring_event(handle::RingWatchHandle; timeout_seconds::Float64=8.0)
    deadline = time() + timeout_seconds
    while time() < deadline
        stdout_text = isfile(handle.stdout_path) ? read(handle.stdout_path, String) : ""
        lines = split(chomp(stdout_text), '\n'; keepempty=false)
        !isempty(lines) && return JSON3.read(lines[1])
        process_exited(handle.process) && return nothing
        sleep(0.05)
    end
    return nothing
end

function stop_ring_watch!(handle::RingWatchHandle)
    try
        process_running(handle.process) && kill(handle.process)
    catch
    end
    try
        wait(handle.process)
    catch
    end
    rm(handle.stdout_path; force=true)
    rm(handle.stderr_path; force=true)
end

@testset "Warp recursive ring topology" begin
    @testset "companion schema freezes coordinator, workers, and subscriptions" begin
        fixture = JSON3.read(read(
            joinpath(WARP_RING_FIXTURE_ROOT, "complete", "coordinator.release.warp-ring.json"),
            String,
        ))

        @test String(fixture["schema"]) == "warp-ring-map-v1"
        @test String(fixture["coordinator_domain"]) == "coordinator"
        @test Int(fixture["projection_depth"]) == 3
        @test String[domain["domain_id"] for domain in fixture["domains"]] == [
            "coordinator", "docs-monkey", "test-bird",
        ]
        @test all(subscription -> Int(subscription["freshness_seconds"]) > 0, fixture["subscriptions"])
        @test Set(String(subscription["direction"]) for subscription in fixture["subscriptions"]) ==
              Set(["parent-to-child", "child-to-parent"])
    end

    @testset "child completion and parent acceptance remain separate" begin
        fixture = JSON3.read(read(
            joinpath(WARP_RING_FIXTURE_ROOT, "missing-acceptance", "coordinator.release.warp-ring.json"),
            String,
        ))
        worker = only(filter(
            domain -> String(domain["domain_id"]) == "docs-monkey",
            fixture["domains"],
        ))

        @test String(worker["required_state"]) == "complete"
        @test !haskey(worker, "parent_acceptance")
    end

    @testset "nested Recur domains exchange task and completion events asynchronously" begin
        root = mktempdir()
        coordinator = joinpath(root, "coordinator")
        docs_worker = joinpath(coordinator, "workers", "docs-monkey")
        test_worker = joinpath(coordinator, "workers", "test-bird")
        mkpath(docs_worker)
        mkpath(test_worker)

        task_watch = nothing
        receipt_watch = nothing
        try
            for domain_root in [coordinator, docs_worker, test_worker]
                success, _, error_output = run_recur(["init", "-d", domain_root])
                @test success
                @test isempty(strip(error_output))
                @test isfile(joinpath(domain_root, ".recur", "config.toml"))
            end

            task_watch = spawn_ring_watch([
                "--id", "coordinator-to-docs",
                "--filter", "task.docs.**",
                "--dir", docs_worker,
                "--format", "json",
            ])
            receipt_watch = spawn_ring_watch([
                "--id", "docs-to-coordinator",
                "--filter", "receipt.docs.**",
                "--dir", coordinator,
                "--format", "json",
            ])

            @test wait_for_ring_watch_ready(task_watch)
            @test wait_for_ring_watch_ready(receipt_watch)

            task_path = joinpath(docs_worker, "task.docs.slice-alpha.todo.current.md")
            write(task_path, "coordinator asks docs worker to complete slice alpha\n")
            task_event = wait_for_ring_event(task_watch)
            @test !isnothing(task_event)
            @test String(task_event["event_type"]) in ["created", "modified"]
            @test endswith(String(task_event["path"]), "task.docs.slice-alpha.todo.current.md")

            receipt_path = joinpath(coordinator, "receipt.docs.slice-alpha.complete.md")
            write(receipt_path, "docs worker reports slice alpha complete\n")
            receipt_event = wait_for_ring_event(receipt_watch)
            @test !isnothing(receipt_event)
            @test String(receipt_event["event_type"]) in ["created", "modified"]
            @test endswith(String(receipt_event["path"]), "receipt.docs.slice-alpha.complete.md")

            success, task_status, _ = run_recur([
                "--json", "watch", "status", "coordinator-to-docs", "-d", docs_worker,
            ])
            @test success
            @test String(JSON3.read(task_status)[1]["ack"]) == "accepted"

            success, receipt_status, _ = run_recur([
                "--json", "watch", "status", "docs-to-coordinator", "-d", coordinator,
            ])
            @test success
            @test String(JSON3.read(receipt_status)[1]["ack"]) == "accepted"
        finally
            !isnothing(task_watch) && stop_ring_watch!(task_watch)
            !isnothing(receipt_watch) && stop_ring_watch!(receipt_watch)
            rm(root; recursive=true, force=true)
        end
    end

    @testset "ring projection composes child completion and parent acceptance" begin
        root = joinpath(WARP_RING_FIXTURE_ROOT, "complete")

        success, output, error_output = run_recur([
            "warp", "map", "coordinator.release", "-d", root, "--json",
        ])
        @test success
        @test isempty(strip(error_output))
        map_view = JSON3.read(output)
        @test String(map_view["schema"]) == "warp-ring-map-view-v1"
        @test length(map_view["domains"]) == 3

        success, output, error_output = run_recur([
            "warp", "merge", "coordinator.release", "-d", root, "--json",
        ])

        @test success
        @test isempty(strip(error_output))
        projection = JSON3.read(output)
        @test String(projection["schema"]) == "warp-ring-projection-v1"
        @test String(projection["state"]) == "complete"
        @test Int(projection["counts"]["ready"]) == 3
        @test all(
            domain -> Bool(domain["child_state_satisfied"]),
            projection["domains"],
        )
        workers = filter(domain -> String(domain["role"]) == "worker", projection["domains"])
        @test all(
            domain -> String(domain["parent_acceptance"]) == "accepted",
            workers,
        )

        success, output, error_output = run_recur([
            "warp", "status", "coordinator.release", "-d", root, "--json",
        ])
        @test success
        @test isempty(strip(error_output))
        status = JSON3.read(output)
        @test String(status["verdict"]) == "optimum"
        @test String(status["ring"]["state"]) == "complete"
    end
end
