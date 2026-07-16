"""
Tests for `recur capability`
============================

Executable specification for improvement 28 capability-card query commands.
"""

include("runtests.setup.jl")

function seed_capability_cards(root::String; include_all::Bool=true)
    write(joinpath(root, ".recur-warp"), """
# .recur-warp

Purpose: name the ideal completed eventness reality first.
""")
    write(joinpath(root, ".recur-watch"), """
# .recur-watch

Purpose: stream live eventness changes.
""")
    write(joinpath(root, ".recur-git"), """
# .recur-git

Purpose: make git state part of lane truth.
""")

    if include_all
        write(joinpath(root, ".recur-trace-id"), """
# .recur-trace-id

Purpose: classify trace-id role evidence.
""")
        write(joinpath(root, ".recur-reveal"), """
# .recur-reveal

Purpose: rehydrate lane-local persona context.
""")
    end
end

@testset "recur capability command" begin
    log_section("Testing: recur capability")

    @testset "capability help exposes improvement 28 surface" begin
        success, output, error_output = run_recur(["capability", "--help"])
        help_text = output * error_output

        @test success
        @test contains(lowercase(help_text), "capability cards")
        @test contains(help_text, "list")
        @test contains(help_text, "explain")
        @test contains(help_text, "doctor")
    end

    @testset "capability list and explain read root cards" begin
        root = mktempdir()
        try
            seed_capability_cards(root)

            success, output, error_output = run_recur(["capability", "list", "-d", root])
            @test success
            @test error_output == ""
            @test contains(output, "warp")
            @test contains(output, ".recur-warp")
            @test contains(output, "trace-id")

            success, explain, _ = run_recur(["capability", "explain", "warp", "-d", root])
            @test success
            @test contains(explain, "Capability: warp")
            @test contains(explain, "ideal completed eventness reality")

            success, explain_by_file, _ = run_recur(["capability", "explain", ".recur-watch", "-d", root])
            @test success
            @test contains(explain_by_file, "Capability: watch")
            @test contains(explain_by_file, "stream live eventness")
        finally
            rm(root; recursive=true, force=true)
        end
    end

    @testset "capability json and doctor expose card health" begin
        root = mktempdir()
        try
            seed_capability_cards(root; include_all=false)

            success, list_json, _ = run_recur(["--json", "capability", "list", "-d", root])
            @test success
            list_payload = JSON3.read(list_json)
            @test length(list_payload[:cards]) == 3
            @test any(card -> string(card[:name]) == "warp", list_payload[:cards])

            success, doctor_json, _ = run_recur(["--json", "capability", "doctor", "-d", root])
            @test success
            doctor_payload = JSON3.read(doctor_json)
            @test string(doctor_payload[:status]) == "missing"
            @test "trace-id" in [string(item) for item in doctor_payload[:missing]]
            @test "reveal" in [string(item) for item in doctor_payload[:missing]]
        finally
            rm(root; recursive=true, force=true)
        end
    end

    @testset "capability explain reports missing card" begin
        root = mktempdir()
        try
            seed_capability_cards(root; include_all=false)

            success, _, error_output = run_recur(["capability", "explain", "reveal", "-d", root])
            @test !success
            @test contains(error_output, "capability 'reveal' not found")
        finally
            rm(root; recursive=true, force=true)
        end
    end
end