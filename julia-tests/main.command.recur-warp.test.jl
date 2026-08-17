"""Executable contract for the confirmation-gated `recur-warp` writer."""

include("runtests.setup.jl")

const RECUR_WARP_BIN = joinpath(
    @__DIR__, "..", "target", RECUR_PROFILE,
    "recur-warp" * (Sys.iswindows() ? ".exe" : "")
)
const RECUR_WARP_FIXTURE = joinpath(
    @__DIR__, "fixtures", "warp-bubble-v1", "partial"
)

function run_recur_warp(args::Vector{String})
    command = `$RECUR_WARP_BIN $args`
    out = IOBuffer()
    err = IOBuffer()
    success = true
    try
        run(pipeline(command, stdout=out, stderr=err))
    catch error
        if error isa ProcessFailedException
            success = false
        else
            rethrow(error)
        end
    end
    return success, String(take!(out)), String(take!(err))
end

@testset "recur-warp Slice completion" begin
    mktempdir() do temporary
        root = joinpath(temporary, "partial")
        cp(RECUR_WARP_FIXTURE, root)
        target = joinpath(root, "demo.partial.beta.attempt-beta-1.warp-layer.json")
        arguments = [
            "complete", "demo.partial", "beta",
            "--attempt-id", "attempt-beta-1",
            "--result-hash", "result-beta-v1",
            "--evidence", "tests=receipts/beta-tests.json",
            "-d", root, "--json",
        ]

        success, output, error_output = run_recur_warp(copy(arguments))
        @test success
        @test error_output == ""
        planned = JSON3.read(output)
        @test String(planned["schema"]) == "recur-warp-complete-v1"
        @test String(planned["state"]) == "planned"
        @test !isfile(target)

        success, output, error_output = run_recur_warp(vcat(arguments, ["--confirm"]))
        @test success
        @test error_output == ""
        written = JSON3.read(output)
        @test String(written["state"]) == "written"
        @test isfile(target)

        success, merge_output, merge_error = run_recur([
            "warp", "merge", "demo.partial", "-d", root, "--json",
        ])
        @test success
        @test merge_error == ""
        merged = JSON3.read(merge_output)
        @test String(merged["state"]) == "complete"
        @test Int(merged["counts"]["covered"]) == 2

        success, output, error_output = run_recur_warp(vcat(arguments, ["--confirm"]))
        @test success
        @test error_output == ""
        repeated = JSON3.read(output)
        @test String(repeated["state"]) == "idempotent"

        conflicting = copy(arguments)
        conflicting[findfirst(==("result-beta-v1"), conflicting)] = "result-beta-v2"
        conflicting[findfirst(==("attempt-beta-1"), conflicting)] = "attempt-beta-2"
        success, _, error_output = run_recur_warp(vcat(conflicting, ["--confirm"]))
        @test !success
        @test contains(error_output, "conflicting result hash")
        @test !isfile(joinpath(
            root, "demo.partial.beta.attempt-beta-2.warp-layer.json"
        ))
        nak_path = joinpath(
            root, ".recur", "warp",
            "recur-warp.demo.partial.beta.attempt-beta-2.status.nak.json",
        )
        @test isfile(nak_path)
        nak = JSON3.read(read(nak_path, String))
        @test String(nak["schema"]) == "recur-warp-nak-v1"
        @test String(nak["result_state"]) == "nak"
    end
end
