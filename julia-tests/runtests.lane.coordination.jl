"""
Tests for Lane Coordination via trace-id
=========================================

These tests prove that trace-id is the affordable, deterministic proxy
for multi-agent coordination correctness. Instead of simulating multiple
live agents (expensive, non-deterministic), we:

  1. Seed lane outputs the way a completed agent would have left them
  2. Run recur trace-id across the merged scope
  3. Verify the cascade is clean — or that wandering is detectable

Architecture:
  - Lane A publishes outputs (files with publish/define keywords)
  - Lane B subscribes to those outputs (files with subscribe keywords)
  - trace-id across the merged scope proves coordination was correct
  - An agent that wandered out of scope appears as an unmatched subscription

This is not a test of agents working simultaneously.
It is a test of the OUTPUT CONTRACT that correct coordination must satisfy.

Connection: docs/main.improvement.22.todo.future-plan.md
  "trace-id is the Handoff Contract"
"""

include("runtests.setup.jl")

# ── Seed helpers ──────────────────────────────────────────────────────────────

"""
Seed a two-lane project with a shared .recur/config.toml that enables
the full cascade vocabulary (publish/subscribe/trigger).
"""
function seed_coordination_project(root::String)
    mkpath(joinpath(root, ".recur"))
    mkpath(joinpath(root, "lanes", "docs"))
    mkpath(joinpath(root, "lanes", "impl"))

    # Project config — enables trigger keyword so full cascade fires
    write(joinpath(root, ".recur", "config.toml"), """
[docs]
dir = "lanes/docs"
sep = "."

[impl]
dir = "lanes/impl"
sep = "."

[traits.trace_id]
producer_keywords = "publish,define"
consumer_keywords = "subscribe,implement"
trigger_keywords  = "trigger"
""")
end

"""
Seed Lane A (docs) output: publishes lane.a.api.contract.
A completed docs agent would have left these files.
"""
function seed_lane_a_output(root::String)
    dir = joinpath(root, "lanes", "docs")

    write(joinpath(dir, "lane.a.api.contract.txt"), """
lane.a.api.contract

publish lane.a.api.contract
define  lane.a.api.contract = the API surface agreed by the docs lane
""")

    write(joinpath(dir, "lane.a.api.contract.changelog.txt"), """
lane.a.api.contract.changelog

publish lane.a.api.contract
""")
end

"""
Seed Lane B (impl) output: subscribes to lane.a.api.contract.
A completed impl agent would have left these files.
"""
function seed_lane_b_subscribes(root::String)
    dir = joinpath(root, "lanes", "impl")

    write(joinpath(dir, "lane.b.api.impl.txt"), """
lane.b.api.impl

subscribe lane.a.api.contract
implement lane.a.api.contract
""")
end

"""
Seed Lane B with a subscription to something Lane A never published.
Simulates an agent that wandered out of its declared scope.
"""
function seed_lane_b_wandered(root::String)
    dir = joinpath(root, "lanes", "impl")

    write(joinpath(dir, "lane.b.wandered.txt"), """
lane.b.wandered

subscribe lane.a.thing.that.does.not.exist
implement lane.a.thing.that.does.not.exist
""")
end

# ── Tests ─────────────────────────────────────────────────────────────────────

@testset "Lane coordination via trace-id" begin
    log_section("Testing: lane coordination via trace-id")

    @testset "clean cascade — lane A publishes, lane B subscribes, cascade resolves" begin
        root = mktempdir()
        try
            seed_coordination_project(root)
            seed_lane_a_output(root)
            seed_lane_b_subscribes(root)

            success, output, _ = run_recur([
                "trace-id", "lane.a.api.contract",
                "--scope", "**",
                "--ext", ".txt",
                "--json",
                "-d", root,
            ])
            @test success

            parsed = JSON3.read(output)

            # A clean cascade must have define + produce + consume
            @test !isempty(parsed["define"])
            @test !isempty(parsed["produce"])
            @test !isempty(parsed["consume"])
        finally
            rm(root, recursive=true)
        end
    end

    @testset "lane isolation — trace-id scoped to lane A sees only lane A files" begin
        root = mktempdir()
        try
            seed_coordination_project(root)
            seed_lane_a_output(root)
            seed_lane_b_subscribes(root)

            success, output, _ = run_recur([
                "trace-id", "lane.a.api.contract",
                "--scope", "**",
                "--ext", ".txt",
                "--json",
                "-d", joinpath(root, "lanes", "docs"),
            ])
            @test success

            parsed = JSON3.read(output)

            # Collect all paths from define + produce
            all_paths = vcat(
                [String(s["path"]) for s in parsed["define"]],
                [String(s["path"]) for s in parsed["produce"]],
            )

            # Scoped to lane A root — no lane B files should appear
            @test !isempty(all_paths)
            @test all(p -> !contains(replace(p, "\\" => "/"), "lanes/impl") &&
                           !contains(replace(p, "\\" => "/"), "lane.b"), all_paths)
        finally
            rm(root, recursive=true)
        end
    end

    @testset "out-of-scope wandering — unmatched subscription is detectable" begin
        root = mktempdir()
        try
            seed_coordination_project(root)
            seed_lane_a_output(root)
            seed_lane_b_wandered(root)

            # Lane B references something lane A never published
            success, output, _ = run_recur([
                "trace-id", "lane.a.thing.that.does.not.exist",
                "--scope", "**",
                "--ext", ".txt",
                "--json",
                "-d", root,
            ])
            @test success

            parsed = JSON3.read(output)

            # Consume sites exist (lane B referenced it)
            @test !isempty(parsed["consume"])

            # No lane A files appear anywhere — lane A never published this identifier
            all_paths = vcat(
                [replace(String(s["path"]), "\\" => "/") for s in parsed["define"]],
                [replace(String(s["path"]), "\\" => "/") for s in parsed["produce"]],
                [replace(String(s["path"]), "\\" => "/") for s in parsed["consume"]],
            )
            @test !any(p -> contains(p, "lanes/docs"), all_paths)
        finally
            rm(root, recursive=true)
        end
    end

    @testset "post-merge completeness — both lanes visible, cascade is complete" begin
        root = mktempdir()
        try
            seed_coordination_project(root)
            seed_lane_a_output(root)
            seed_lane_b_subscribes(root)

            # Merged view: run from repo root spanning both lanes
            success, output, _ = run_recur([
                "trace-id", "lane.a.api.contract",
                "--scope", "**",
                "--ext", ".txt",
                "--json",
                "-d", root,
            ])
            @test success

            parsed = JSON3.read(output)

            # All cascade roles present across the merged scope
            @test !isempty(parsed["define"])
            @test !isempty(parsed["produce"])
            @test !isempty(parsed["consume"])

            # Both lanes contributed to the cascade
            define_paths = [replace(String(s["path"]), "\\" => "/") for s in parsed["define"]]
            consume_paths = [replace(String(s["path"]), "\\" => "/") for s in parsed["consume"]]
            @test any(p -> contains(p, "lanes/docs"), define_paths)
            @test any(p -> contains(p, "lanes/impl"), consume_paths)
        finally
            rm(root, recursive=true)
        end
    end

    @testset "handoff completeness — lane B cannot declare done if define is missing" begin
        root = mktempdir()
        try
            seed_coordination_project(root)
            # Lane A never did its work — no publish, no define
            seed_lane_b_subscribes(root)

            success, output, _ = run_recur([
                "trace-id", "lane.a.api.contract",
                "--scope", "**",
                "--ext", ".txt",
                "--json",
                "-d", root,
            ])
            @test success

            parsed = JSON3.read(output)

            # Lane B subscribed but Lane A never defined — handoff is incomplete
            @test !isempty(parsed["consume"])

            # No lane A files appear anywhere — lane A did not do its work
            all_paths = vcat(
                [replace(String(s["path"]), "\\" => "/") for s in parsed["define"]],
                [replace(String(s["path"]), "\\" => "/") for s in parsed["produce"]],
                [replace(String(s["path"]), "\\" => "/") for s in parsed["consume"]],
            )
            @test !any(p -> contains(p, "lanes/docs"), all_paths)
            @test  any(p -> contains(p, "lanes/impl"), all_paths)
        finally
            rm(root, recursive=true)
        end
    end

    @testset "do not cross the streams — recur files confirms lane boundaries" begin
        root = mktempdir()
        try
            seed_coordination_project(root)
            seed_lane_a_output(root)
            seed_lane_b_subscribes(root)

            # Lane A scope must not contain any lane B files
            success_a, output_a, _ = run_recur([
                "files", "**",
                "--sep", ".",
                "-d", joinpath(root, "lanes", "docs"),
            ])
            @test success_a
            @test !contains(output_a, "lane.b")
            @test !contains(output_a, "impl")

            # Lane B scope must not contain any lane A output files
            success_b, output_b, _ = run_recur([
                "files", "**",
                "--sep", ".",
                "-d", joinpath(root, "lanes", "impl"),
            ])
            @test success_b
            @test !contains(output_b, "lane.a.api.contract.changelog")

            # Only the merged root sees both
            success_r, output_r, _ = run_recur([
                "files", "lane.**",
                "--sep", ".",
                "-d", root,
            ])
            @test success_r
            @test contains(output_r, "lane.a")
            @test contains(output_r, "lane.b")
        finally
            rm(root, recursive=true)
        end
    end

end
