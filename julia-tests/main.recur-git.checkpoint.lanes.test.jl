"""
Tests for recur-git checkpoint lane coverage
===========================================

Verifies that `recur-git checkpoint --snapshot` surfaces active state for the
agent vault lanes declared in `.recur/config.toml`.
"""

include("runtests.setup.jl")

const RECUR_GIT_BIN = let
    repo_root = normpath(joinpath(@__DIR__, ".."))
    ext = Sys.iswindows() ? ".exe" : ""
    debug_bin = joinpath(repo_root, "target", "debug", "recur-git" * ext)
    release_bin = joinpath(repo_root, "target", RECUR_PROFILE, "recur-git" * ext)
    isfile(debug_bin) ? debug_bin : release_bin
end

function run_recur_git(args::Vector{String})
    repo_root = normpath(joinpath(@__DIR__, ".."))
    display_cmd = join(map(arg -> contains(arg, ' ') ? "\"$arg\"" : arg, args), " ")
    println("  -> recur-git $display_cmd")

    cmd = Cmd(`$RECUR_GIT_BIN $args`, dir=repo_root)
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

@testset "recur-git checkpoint lane coverage" begin
    log_section("Testing: recur-git checkpoint lane coverage")

    @testset "checkpoint snapshot includes active agent vault lanes" begin
        _, output, error_output = run_recur_git(["checkpoint", "--snapshot"])
        snapshot_text = output * error_output

        @test contains(snapshot_text, "lane.state.skippy1.current:")
        @test contains(snapshot_text, "lane.state.skippy2.current:")
        @test contains(snapshot_text, "lane.state.test-monkey.current:")
        @test contains(snapshot_text, "lane.state.git-monkey.current:")
    end
end
