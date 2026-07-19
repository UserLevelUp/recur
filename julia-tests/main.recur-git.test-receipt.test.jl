"""Eventness receipt contract for bounded `recur-git` test execution."""

include("runtests.setup.jl")

const RECUR_GIT_BIN = joinpath(
    @__DIR__, "..", "target", RECUR_PROFILE, "recur-git" * (Sys.iswindows() ? ".exe" : ""),
)

function run_recur_git(root::String, args::Vector{String})
    cmd = Cmd(`$RECUR_GIT_BIN $args`; dir=root)
    out = IOBuffer()
    err = IOBuffer()
    success = true
    try
        run(pipeline(cmd, stdout=out, stderr=err))
    catch error
        if isa(error, ProcessFailedException)
            success = false
        else
            rethrow()
        end
    end
    return (success, String(take!(out)), String(take!(err)))
end

function initialize_receipt_repo(root::String)
    run(Cmd(`git init`; dir=root))
    run(Cmd(`git config user.email recur-tests@example.invalid`; dir=root))
    run(Cmd(`git config user.name "Recur Tests"`; dir=root))
    write(joinpath(root, ".gitignore"), ".recur/\n")
    write(joinpath(root, "pass.jl"), "println(\"pass\")\n")
    write(joinpath(root, "fail.jl"), "error(\"expected receipt failure\")\n")
    run(Cmd(`git add .`; dir=root))
    run(Cmd(`git commit -m "test fixture"`; dir=root))
    return strip(read(Cmd(`git rev-parse --short=12 HEAD`; dir=root), String))
end

@testset "recur-git test receipt" begin
    mktempdir() do root
        head = initialize_receipt_repo(root)

        passed, output, error_output = run_recur_git(root, [
            "test-receipt", "main.command.tree.wildcard-current", "--julia-file", "pass.jl",
        ])
        @test passed
        @test error_output == ""
        @test contains(output, "Wrote test receipt")
        pass_receipt = joinpath(
            root,
            ".recur",
            "tests",
            "main.command.tree.wildcard-current.test.$head.passed.complete.md",
        )
        @test isfile(pass_receipt)
        @test contains(read(pass_receipt, String), "test.state: passed.complete")
        @test contains(read(pass_receipt, String), "test.tested-head: $head")

        failed, _, failure_output = run_recur_git(root, [
            "test-receipt", "main.command.tree.wildcard-current.failure", "--julia-file", "fail.jl",
        ])
        @test !failed
        @test contains(failure_output, "test command failed; recorded")
        fail_receipt = joinpath(
            root,
            ".recur",
            "tests",
            "main.command.tree.wildcard-current.failure.test.$head.failed.strange.md",
        )
        @test isfile(fail_receipt)
        @test contains(read(fail_receipt, String), "test.state: failed.strange")

        snapshotted, snapshot_output, snapshot_error = run_recur_git(root, ["checkpoint", "--snapshot"])
        @test snapshotted
        @test snapshot_error == ""
        @test contains(snapshot_output, "lane.state.tests.passed: .recur\\tests\\main.command.tree.wildcard-current.test.$head.passed.complete.md")
        @test contains(snapshot_output, "lane.state.tests.failed: .recur\\tests\\main.command.tree.wildcard-current.failure.test.$head.failed.strange.md")
    end
end
