"""
run_watch_demo.jl - Cross-loop demo for `recur-watch`.

Runs two `recur-watch` subprocesses over a shared table directory:
- server watch subscribes to `move.**.current.md`
- player watch subscribes to `result.**.current.md`

One Julia process hosts both orchestrator halves as async tasks. The
player writes move files, the server reacts by writing result files, and
the loop closes after three moves.
"""

const SCRIPT_DIR = @__DIR__
const DEMO_DIR = normpath(joinpath(SCRIPT_DIR, ".."))
const PROJECT_ROOT = normpath(joinpath(SCRIPT_DIR, "..", "..", ".."))
const TABLE_DIR = joinpath(DEMO_DIR, "table")
const RECUR_WATCH_BIN = normpath(joinpath(PROJECT_ROOT, "target", "release-safe", "recur-watch.exe"))
const READY_TOKEN = "recur watch: ready"
const POLL_INTERVAL_SECONDS = 0.05
const READY_TIMEOUT_SECONDS = 4.0
const LOOP_TIMEOUT_SECONDS = 8.0

const MOVE_PAYLOADS = Dict(
    1 => "{\"row\":1,\"col\":1,\"value\":5}",
    2 => "{\"row\":1,\"col\":2,\"value\":3}",
    3 => "{\"row\":2,\"col\":1,\"value\":6}",
)

mutable struct WatchHandle
    name::String
    process::Base.Process
    stdout_path::String
    stderr_path::String
    stdout_line_count::Int
end

function ensure_binary()
    isfile(RECUR_WATCH_BIN) || error("Missing recur-watch binary at $(RECUR_WATCH_BIN)")
end

function reset_table_dir!()
    isdir(TABLE_DIR) && rm(TABLE_DIR; recursive=true, force=true)
    mkpath(TABLE_DIR)
end

function spawn_watch(name::String, filter::String)
    stdout_path = tempname()
    stderr_path = tempname()
    stdout_io = open(stdout_path, "w")
    stderr_io = open(stderr_path, "w")

    cmd = Cmd(
        Cmd([RECUR_WATCH_BIN, "--filter", filter, "--dir", TABLE_DIR, "--format", "json"]);
        dir=PROJECT_ROOT,
    )
    process = run(pipeline(cmd, stdout=stdout_io, stderr=stderr_io); wait=false)

    close(stdout_io)
    close(stderr_io)

    return WatchHandle(name, process, stdout_path, stderr_path, 0)
end

function read_file_or_empty(path::String)
    return isfile(path) ? read(path, String) : ""
end

function read_new_stdout_lines!(handle::WatchHandle)
    text = read_file_or_empty(handle.stdout_path)
    lines = split(text, '\n'; keepempty=false)

    if handle.stdout_line_count >= length(lines)
        return String[]
    end

    new_lines = lines[(handle.stdout_line_count + 1):end]
    handle.stdout_line_count = length(lines)
    return new_lines
end

function wait_for_watch_ready(handle::WatchHandle; timeout_seconds::Float64=READY_TIMEOUT_SECONDS)
    deadline = time() + timeout_seconds

    while time() < deadline
        stderr_text = read_file_or_empty(handle.stderr_path)
        occursin(READY_TOKEN, stderr_text) && return true
        process_exited(handle.process) && return false
        sleep(POLL_INTERVAL_SECONDS)
    end

    return false
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

    stdout_text = read_file_or_empty(handle.stdout_path)
    stderr_text = read_file_or_empty(handle.stderr_path)

    rm(handle.stdout_path; force=true)
    rm(handle.stderr_path; force=true)

    return stdout_text, stderr_text
end

function extract_sequence(line::AbstractString, prefix::String)
    occursin("\"event_type\":\"deleted\"", line) && return nothing
    matched = match(Regex("$(prefix)\\.(\\d+)\\.current\\.md"), line)
    matched === nothing && return nothing
    return parse(Int, matched.captures[1])
end

function write_move!(sequence::Int)
    path = joinpath(TABLE_DIR, "move.$sequence.current.md")
    write(path, MOVE_PAYLOADS[sequence])
    return path
end

function write_result!(sequence::Int)
    path = joinpath(TABLE_DIR, "result.$sequence.current.md")
    write(path, "{\"move\":$sequence,\"accepted\":true}")
    return path
end

function server_orchestrator!(handle::WatchHandle, channel::Channel, stop_flag::Base.RefValue{Bool})
    seen = Set{Int}()

    try
        while !stop_flag[]
            if process_exited(handle.process)
                put!(channel, (:error, "server watch exited early: " * read_file_or_empty(handle.stderr_path)))
                return
            end

            for line in read_new_stdout_lines!(handle)
                sequence = extract_sequence(line, "move")
                isnothing(sequence) && continue
                sequence in seen && continue

                push!(seen, sequence)
                write_result!(sequence)
                put!(channel, (:server_result_written, sequence))
            end

            sleep(POLL_INTERVAL_SECONDS)
        end
    catch e
        put!(channel, (:error, "server orchestrator failed: " * sprint(showerror, e)))
    end
end

function player_orchestrator!(handle::WatchHandle, channel::Channel, stop_flag::Base.RefValue{Bool})
    seen = Set{Int}()

    try
        while !stop_flag[]
            if process_exited(handle.process)
                put!(channel, (:error, "player watch exited early: " * read_file_or_empty(handle.stderr_path)))
                return
            end

            for line in read_new_stdout_lines!(handle)
                sequence = extract_sequence(line, "result")
                isnothing(sequence) && continue
                sequence in seen && continue

                push!(seen, sequence)
                put!(channel, (:player_result_observed, sequence))

                if sequence < 3
                    write_move!(sequence + 1)
                else
                    put!(channel, (:complete, sequence))
                    return
                end
            end

            sleep(POLL_INTERVAL_SECONDS)
        end
    catch e
        put!(channel, (:error, "player orchestrator failed: " * sprint(showerror, e)))
    end
end

function verify_results!()
    for sequence in 1:3
        result_path = joinpath(TABLE_DIR, "result.$sequence.current.md")
        isfile(result_path) || error("Missing result file $(basename(result_path))")
    end
end

function main()
    ensure_binary()
    reset_table_dir!()

    server_watch = spawn_watch("server", "move.**.current.md")
    player_watch = spawn_watch("player", "result.**.current.md")

    start_time = time()
    channel = Channel{Tuple}(32)
    stop_flag = Ref(false)

    server_task = nothing
    player_task = nothing

    try
        wait_for_watch_ready(server_watch) || error("Timed out waiting for server watch readiness")
        wait_for_watch_ready(player_watch) || error("Timed out waiting for player watch readiness")

        server_task = @async server_orchestrator!(server_watch, channel, stop_flag)
        player_task = @async player_orchestrator!(player_watch, channel, stop_flag)

        write_move!(1)

        deadline = time() + LOOP_TIMEOUT_SECONDS
        observed_result_count = 0

        while time() < deadline
            if isready(channel)
                message = take!(channel)
                kind = message[1]

                if kind == :error
                    error(message[2])
                elseif kind == :player_result_observed
                    observed_result_count = max(observed_result_count, message[2])
                elseif kind == :complete
                    observed_result_count = max(observed_result_count, message[2])
                    observed_result_count == 3 || error("Loop completed with only $(observed_result_count) observed results")
                    verify_results!()
                    elapsed = round(time() - start_time; digits=2)
                    println("3 moves produced 3 results in $(elapsed)s")
                    stop_flag[] = true
                    wait(server_task)
                    wait(player_task)
                    return
                end
            else
                sleep(POLL_INTERVAL_SECONDS)
            end
        end

        error("Timed out waiting for result.3.current.md")
    finally
        stop_flag[] = true

        if server_task !== nothing
            try
                wait(server_task)
            catch
            end
        end

        if player_task !== nothing
            try
                wait(player_task)
            catch
            end
        end

        stop_watch!(server_watch)
        stop_watch!(player_watch)
    end
end

function run_demo()
    try
        main()
    catch e
        println(stderr, "run_watch_demo.jl failed: ", sprint(showerror, e))
        exit(1)
    end
end

run_demo()
