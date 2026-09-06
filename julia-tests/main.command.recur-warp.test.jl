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

@testset "recur-warp evolution" begin
    mktempdir() do root
        write(joinpath(root, "demo.source.warp-map.json"), """
        {
          "schema": "warp-bubble-map-v1",
          "warp_id": "demo.source",
          "required_slices": [
            {"slice_id":"alpha","contract_hash":"alpha-v1","evidence_gates":["tests"]},
            {"slice_id":"beta","contract_hash":"beta-v1","evidence_gates":["tests"]}
          ]
        }
        """)
        for (attempt, slice, contract, result) in [
            ("a1", "alpha", "alpha-v1", "alpha-result"),
            ("b1", "beta", "beta-v1", "beta-result-1"),
            ("b2", "beta", "beta-v1", "beta-result-2"),
        ]
            write(joinpath(root, "demo.source.$slice.$attempt.warp-layer.json"), """
            {
              "schema":"warp-slice-layer-v1", "warp_id":"demo.source",
              "slice_id":"$slice", "contract_hash":"$contract",
              "attempt_id":"$attempt", "result_state":"accepted",
              "result_hash":"$result", "evidence":{"tests":["$slice-tests.json"]}
            }
            """)
        end
        candidate = joinpath(root, "candidate.json")
        write(candidate, """
        {
          "schema": "warp-bubble-map-v1",
          "warp_id": "demo.evolved",
          "required_slices": [
            {"slice_id":"alpha","contract_hash":"alpha-v1","evidence_gates":["tests"]},
            {"slice_id":"beta","contract_hash":"beta-v2","evidence_gates":["tests"]},
            {"slice_id":"gamma","contract_hash":"gamma-v1","evidence_gates":["tests"]}
          ]
        }
        """)
        arguments = ["evolve", "demo.source", "candidate.json", "-d", root, "--json"]
        target = joinpath(root, "demo.evolved.warp-map.json")
        receipt = joinpath(
            root, ".recur", "warp",
            "recur-warp.demo.source.to.demo.evolved.supersession.ack.json",
        )

        success, output, error_output = run_recur_warp(copy(arguments))
        @test success
        @test isempty(strip(error_output))
        planned = JSON3.read(output)
        @test String(planned["state"]) == "planned"
        @test String.(planned["carried_slices"]) == ["alpha"]
        @test !isfile(target)
        @test !isfile(receipt)

        success, output, error_output = run_recur_warp(vcat(arguments, ["--confirm"]))
        @test success
        @test isempty(strip(error_output))
        written = JSON3.read(output)
        @test String(written["state"]) == "written"
        @test isfile(target)
        @test isfile(receipt)
        @test isfile(joinpath(root, "demo.evolved.alpha.evolved-a1.warp-layer.json"))
        @test !isfile(joinpath(root, "demo.evolved.beta.evolved-b1.warp-layer.json"))
        @test "beta" in String.(written["invalidated_slices"])

        success, merge_output, merge_error = run_recur([
            "warp", "merge", "demo.evolved", "-d", root, "--json",
        ])
        @test success
        @test isempty(strip(merge_error))
        projection = JSON3.read(merge_output)
        @test String(projection["state"]) == "incomplete"
        @test String.(projection["covered"]) == ["alpha"]
    end
end

@testset "recur-warp collapse" begin
    mktempdir() do root
        complete = joinpath(root, "demo.lane.alpha.complete.md")
        interesting = joinpath(root, "demo.lane.question.strange.md")
        write(complete, "verified completion evidence\n")
        write(interesting, "preserve this unresolved observation\n")
        arguments = ["collapse", "demo.lane", "-d", root, "--json"]

        success, plan_output, plan_error = run_recur([
            "warp", "collapse-plan", "demo.lane", "-d", root, "--json",
        ])
        @test success
        @test isempty(strip(plan_error))
        query_plan = JSON3.read(plan_output)

        success, output, error_output = run_recur_warp(copy(arguments))
        @test success
        @test isempty(strip(error_output))
        planned = JSON3.read(output)
        @test String(planned["state"]) == "planned"
        @test String.(planned["collapse_known"]) == ["demo.lane.alpha.complete.md"]
        @test String.(planned["preserve_interesting"]) == ["demo.lane.question.strange.md"]
        @test String.(item["path"] for item in query_plan["collapse_known"]) ==
              String.(planned["collapse_known"])
        @test String.(item["path"] for item in query_plan["preserve_interesting"]) ==
              String.(planned["preserve_interesting"])
        @test isfile(complete)

        success, output, error_output = run_recur_warp(vcat(arguments, ["--confirm"]))
        @test success
        @test isempty(strip(error_output))
        written = JSON3.read(output)
        archive = joinpath(
            root, ".recur", "warp", "archive", "demo.lane",
            "demo.lane.alpha.complete.md",
        )
        receipt = joinpath(
            root, ".recur", "warp", "recur-warp.demo.lane.collapse.ack.json",
        )
        @test String(written["state"]) == "written"
        @test !isfile(complete)
        @test isfile(archive)
        @test isfile(interesting)
        @test isfile(receipt)
        recorded = JSON3.read(read(receipt, String))
        @test String(recorded["result_state"]) == "accepted"
    end

    mktempdir() do root
        current = joinpath(root, "demo.active.work.todo.current.md")
        write(current, "active and ambiguous\n")
        success, _, error_output = run_recur_warp([
            "collapse", "demo.active", "-d", root, "--json", "--confirm",
        ])
        @test !success
        @test occursin("operator resolution", error_output)
        @test isfile(current)
        @test !isdir(joinpath(root, ".recur", "warp", "archive"))
    end
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
