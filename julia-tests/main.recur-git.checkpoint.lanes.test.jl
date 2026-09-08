"""
Tests for recur-git checkpoint lane coverage
===========================================

Verifies that `recur-git checkpoint --snapshot` surfaces active state for the
agent vault lanes declared in `.recur/config.toml`.
"""

include("runtests.setup.jl")

const RECUR_GIT_BIN = let
    ext = Sys.iswindows() ? ".exe" : ""
    joinpath(dirname(RECUR_BIN), "recur-git" * ext)
end

function run_checkpoint_fixture(root::String, args::Vector{String})
    display_cmd = join(map(arg -> contains(arg, ' ') ? "\"$arg\"" : arg, args), " ")
    println("  -> recur-git $display_cmd")

    cmd = Cmd(`$RECUR_GIT_BIN $args`, dir=root)
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
        mktempdir() do root
            lanes = ["skippy1", "skippy2", "test-monkey", "git-monkey"]
            mkpath(joinpath(root, ".recur"))
            write(joinpath(root, ".recur", "config.toml"), join([
                "[$lane]\ndir = '.recur/$lane'\nsep = '.'\n" for lane in lanes
            ], "\n"))
            for lane in lanes
                mkpath(joinpath(root, ".recur", lane))
                write(joinpath(root, ".recur", lane, "$lane.work.current.md"), "fixture")
            end
            success, output, error_output = run_checkpoint_fixture(root, ["checkpoint", "--snapshot"])
            @test success
            snapshot_text = output * error_output
            for lane in lanes
                @test contains(snapshot_text, "lane.state.$lane.current:")
                @test contains(snapshot_text, "$lane.work.current.md")
            end
        end
    end
end
