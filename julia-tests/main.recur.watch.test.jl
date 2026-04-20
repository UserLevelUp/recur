"""
Tests for recur watch command
=============================

Executable specification for the `recur-watch` binary boundary.
The behavioral contract remains identical to the former subcommand.
"""

include("runtests.setup.jl")

const RECUR_WATCH_BIN = get(
    ENV,
    "RECUR_WATCH_BIN",
    joinpath(@__DIR__, "..", "target", RECUR_PROFILE, "recur-watch" * (Sys.iswindows() ? ".exe" : "")),
)
const WATCH_TIMEOUT_SECONDS = 8.0

mutable struct WatchHandle
    process::Base.Process
    stdout_path::String
    stderr_path::String
end

repo_root() = normpath(joinpath(@__DIR__, ".."))

function run_recur_raw(args::Vector{String})
    display_cmd = join(map(arg -> contains(arg, ' ') ? "\"$arg\"" : arg, args), " ")
    println("  -> recur-watch $display_cmd")

    cmd = Cmd(`$RECUR_WATCH_BIN $args`, dir=repo_root())
    out = IOBuffer()
    err = IOBuffer()
    success = true

    try
        run(pipeline(cmd, stdout=out, stderr=err))
    catch e
        if isa(e, ProcessFailedException)
            success = false
        else
            return (false, "", "Error running command: $e")
        end
    end

    return (success, String(take!(out)), String(take!(err)))
end

function spawn_watch(args::Vector{String})
    display_cmd = join(map(arg -> contains(arg, ' ') ? "\"$arg\"" : arg, args), " ")
    println("  -> recur-watch $display_cmd")

    stdout_path = tempname()
    stderr_path = tempname()
    stdout_io = open(stdout_path, "w")
    stderr_io = open(stderr_path, "w")
    cmd = Cmd(`$RECUR_WATCH_BIN $args`, dir=repo_root())

    process = run(pipeline(cmd, stdout=stdout_io, stderr=stderr_io); wait=false)
    close(stdout_io)
    close(stderr_io)

    return WatchHandle(process, stdout_path, stderr_path)
end

function read_watch_streams(handle::WatchHandle)
    stdout_text = isfile(handle.stdout_path) ? read(handle.stdout_path, String) : ""
    stderr_text = isfile(handle.stderr_path) ? read(handle.stderr_path, String) : ""
    return stdout_text, stderr_text
end

function wait_for_watch_start(handle::WatchHandle; timeout_seconds::Float64=2.0)
    deadline = time() + timeout_seconds
    while time() < deadline
        process_running(handle.process) && return :running
        process_exited(handle.process) && return :exited
        sleep(0.05)
    end
    return :timeout
end

function wait_for_watch_ready(handle::WatchHandle; timeout_seconds::Float64=4.0)
    deadline = time() + timeout_seconds
    while time() < deadline
        stderr_text = isfile(handle.stderr_path) ? read(handle.stderr_path, String) : ""
        occursin("recur watch: ready", stderr_text) && return true
        process_exited(handle.process) && return false
        sleep(0.05)
    end
    return false
end

function wait_for_output_line(handle::WatchHandle; timeout_seconds::Float64=WATCH_TIMEOUT_SECONDS)
    deadline = time() + timeout_seconds
    while time() < deadline
        stdout_text, stderr_text = read_watch_streams(handle)
        lines = split(chomp(stdout_text), '\n'; keepempty=false)
        !isempty(lines) && return (lines[1], stderr_text)
        process_exited(handle.process) && return (nothing, stderr_text)
        sleep(0.05)
    end
    return (nothing, nothing)
end

function stop_watch!(handle::WatchHandle)
    try
        process_running(handle.process) && kill(handle.process)
    catch
    end

    try
        wait(handle.process)
    catch
    end

    stdout_text, stderr_text = read_watch_streams(handle)
    rm(handle.stdout_path; force=true)
    rm(handle.stderr_path; force=true)
    return stdout_text, stderr_text
end

function assert_no_timeout(state, message::String)
    state == :timeout && error(message)
end

function assert_line_received(line, message::String)
    isnothing(line) && error(message)
end

@testset "recur watch command" begin
    log_section("Testing: recur watch command")

    @testset "watch help prints usage" begin
        success, output, error_output = run_recur_raw(["--help"])
        help_text = output * error_output

        @test success
        @test contains(lowercase(help_text), "watch")
        @test contains(lowercase(help_text), "filter")
        @test contains(lowercase(help_text), "pattern")
        @test contains(lowercase(help_text), "poll-framing")
    end

    @testset "watch with no args exits with usage error" begin
        success, _, error_output = run_recur_raw(String[])
        error_text = lowercase(error_output)

        @test !success
        @test contains(error_text, "filter") || contains(error_text, "required")
    end

    @testset "watch filter accepts recur pattern language" begin
        root = mktempdir()
        handle = spawn_watch(["--filter", "**.current.md", "--dir", root])

        try
            state = wait_for_watch_start(handle)
            assert_no_timeout(state, "Timed out waiting for recur watch filter validation")
            _, stderr_text = stop_watch!(handle)

            @test state == :running
            @test !contains(lowercase(stderr_text), "error")
        finally
            stop_watch!(handle)
            rm(root; recursive=true, force=true)
        end
    end

    @testset "watch dir honors scope and rejects missing path" begin
        missing_dir = joinpath(mktempdir(), "does-not-exist")
        success, _, error_output = run_recur_raw([
            "--filter", "**.current.md", "--dir", missing_dir
        ])
        error_text = lowercase(error_output)

        @test !success
        @test contains(error_text, "dir") || contains(error_text, "directory") || contains(error_text, "not found")
    end

    @testset "watch json format emits parseable event objects" begin
        root = mktempdir()
        handle = spawn_watch(["--filter", "**.current.md", "--dir", root, "--format", "json"])

        try
            state = wait_for_watch_start(handle)
            assert_no_timeout(state, "Timed out waiting for recur watch JSON mode startup")
            @test state == :running

            wait_for_watch_ready(handle) ||
                error("Timed out waiting for recur watch readiness signal")

            write(joinpath(root, "lane.status.current.md"), "STATE: active\n")
            line, stderr_text = wait_for_output_line(handle)
            assert_line_received(line, "Timed out waiting for recur watch JSON event")

            event = JSON3.read(line)
            @test haskey(event, :path) || haskey(event, "path")
            @test haskey(event, :event_type) || haskey(event, "event_type")
            @test !contains(lowercase(stderr_text), "error")
        finally
            stop_watch!(handle)
            rm(root; recursive=true, force=true)
        end
    end

    @testset "watch poll framing accepts integer seconds and rejects unit suffix" begin
        root = mktempdir()
        handle = spawn_watch([
            "--filter", "**.current.md", "--dir", root, "--poll-framing", "1"
        ])

        try
            state = wait_for_watch_start(handle)
            assert_no_timeout(state, "Timed out waiting for recur watch poll mode startup")
            @test state == :running
        finally
            stop_watch!(handle)
            rm(root; recursive=true, force=true)
        end

        success, _, error_output = run_recur_raw([
            "--filter", "**.current.md", "--poll-framing", "5s"
        ])
        error_text = lowercase(error_output)

        @test !success
        @test contains(error_text, "poll-framing") || contains(error_text, "invalid")
    end

    @testset "watch poll framing rejects zero interval" begin
        success, _, error_output = run_recur_raw([
            "--filter", "**.current.md", "--poll-framing", "0"
        ])
        error_text = lowercase(error_output)

        @test !success
        @test contains(error_text, "poll-framing") || contains(error_text, "zero") || contains(error_text, "positive")
    end

    @testset "watch poll framing rejects negative interval" begin
        success, _, error_output = run_recur_raw([
            "--filter", "**.current.md", "--poll-framing", "-1"
        ])
        error_text = lowercase(error_output)

        @test !success
        @test contains(error_text, "poll-framing") || contains(error_text, "invalid") || contains(error_text, "negative")
    end
end
