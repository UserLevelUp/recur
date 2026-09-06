"""
Tests for demos/sudoku/julia/run_watch_demo.jl
==============================================

Drives the Lane L cross-loop demo as a subprocess and asserts the
3-move / 3-result ping-pong closes cleanly with all expected files
on disk.
"""

include("runtests.setup.jl")

# Mirror only the script and watcher into a disposable root. The demo clears its
# table, so a regression run must never target the user's live table directory.
const DEMO_TEST_ROOT = mktempdir(prefix="recur-sudoku-watch-test-")
const DEMO_SCRIPT = joinpath(DEMO_TEST_ROOT,"demos","sudoku","julia","run_watch_demo.jl")
const DEMO_TABLE_DIR = joinpath(DEMO_TEST_ROOT,"demos","sudoku","table")
mkpath(dirname(DEMO_SCRIPT))
cp(joinpath(@__DIR__,"..","demos","sudoku","julia","run_watch_demo.jl"),DEMO_SCRIPT)
mkpath(joinpath(DEMO_TEST_ROOT,"target","release-safe"))
cp(joinpath(@__DIR__,"..","target","release-safe","recur-watch.exe"),
   joinpath(DEMO_TEST_ROOT,"target","release-safe","recur-watch.exe"))
const DEMO_RUN_TIMEOUT_SECONDS = 30.0

function run_demo_subprocess()
    stdout_path = tempname()
    stderr_path = tempname()
    stdout_io = open(stdout_path, "w")
    stderr_io = open(stderr_path, "w")
    cmd = Cmd(Cmd(["julia", DEMO_SCRIPT]); dir=normpath(joinpath(@__DIR__, "..")))

    process = run(pipeline(cmd, stdout=stdout_io, stderr=stderr_io); wait=false)
    close(stdout_io)
    close(stderr_io)

    deadline = time() + DEMO_RUN_TIMEOUT_SECONDS
    timed_out = false

    while time() < deadline
        process_running(process) || break
        sleep(0.1)
    end

    if process_running(process)
        timed_out = true
        kill(process)
    end

    try
        wait(process)
    catch
    end

    stdout_text = isfile(stdout_path) ? read(stdout_path, String) : ""
    stderr_text = isfile(stderr_path) ? read(stderr_path, String) : ""
    rm(stdout_path; force=true)
    rm(stderr_path; force=true)

    return (process.exitcode, stdout_text, stderr_text, timed_out)
end

@testset "demo.sudoku.watch cross-loop ping-pong" begin
    isfile(DEMO_SCRIPT) || error("Missing demo script at $(DEMO_SCRIPT)")

    exit_code, stdout_text, stderr_text, timed_out = run_demo_subprocess()
    timed_out && error("Demo subprocess timed out")

    @test exit_code == 0
    @test occursin("3 moves produced 3 results", stdout_text)
    @test all(
        isfile(joinpath(DEMO_TABLE_DIR, "result.$sequence.current.md"))
        for sequence in 1:3
    )

    isempty(stderr_text) || @info "demo stderr" stderr_text
end
