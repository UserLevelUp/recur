"""
Demo: Skippy Adaptive Comms + trace-id
======================================

Proves that recur trace-id can operate on plain-text adaptive-comms protocol
files so relationship phase and jab-family selection stay auditable.
"""

include("runtests.setup.jl")

const SKIPPY_DEMO_SRC = joinpath(@__DIR__, "..", "demos", "skippy-adaptive-comms")

function seed_skippy_demo_fixture()
    fixture_dir = joinpath(TEST_DIR, "skippy_demo")
    isdir(fixture_dir) && rm(fixture_dir, recursive=true)
    cp(SKIPPY_DEMO_SRC, fixture_dir; force=true)

    mkpath(joinpath(fixture_dir, ".recur"))
    cp(
        joinpath(fixture_dir, "trace-id.config.example.toml"),
        joinpath(fixture_dir, ".recur", "config.toml");
        force=true,
    )

    fixture_dir
end

function get_role(parsed, role_sym::Symbol, role_str::String)
    get(parsed, role_sym, get(parsed, role_str, []))
end

function site_lines(items)
    [String(item["line"]) for item in items]
end

@testset "Demo: Skippy Adaptive Comms + trace-id" begin
    log_section("Testing: Skippy adaptive comms demo")

    created_here = false
    if !isdir(TEST_DIR)
        setup_test_environment()
        created_here = true
    end

    try
        fixture_dir = seed_skippy_demo_fixture()

        @testset "Phase 1: Demo files are recur-visible" begin
            success, output, _ = run_recur(["files", "skippy.**", "-d", fixture_dir])

            parsed = try
                JSON3.read(output)
            catch
                nothing
            end

            @test success
            @test parsed !== nothing
            if parsed !== nothing
                paths = [String(p) for p in parsed]
                @test length(paths) >= 4
                @test any(p -> contains(p, "skippy.relationship.playful.precise.current"), paths)
                @test any(p -> contains(p, "skippy.case.separator.correction"), paths)
                @test any(p -> contains(p, "skippy.case.strong.insight"), paths)
                @test any(p -> contains(p, "skippy.case.release.admin"), paths)
            end
        end

        @testset "Phase 2: Relationship phase is traceable" begin
            success, output, _ = run_recur([
                "trace-id",
                "skippy.relationship.playful.precise.current",
                "--scope",
                "skippy.**",
                "--ext",
                ".txt",
                "--json",
                "-d",
                fixture_dir,
            ])

            parsed = try
                JSON3.read(output)
            catch
                nothing
            end

            @test success
            @test parsed !== nothing
            if parsed !== nothing
                define = get_role(parsed, :define, "define")
                produce = get_role(parsed, :produce, "produce")
                trigger = get_role(parsed, :trigger, "trigger")

                produce_lines = site_lines(produce)
                trigger_lines = site_lines(trigger)

                @test length(define) >= 1
                @test any(line -> contains(line, "skippy.tone.bite.sharp"), produce_lines)
                @test any(line -> contains(line, "skippy.tone.respect.visible"), produce_lines)
                @test any(line -> contains(line, "skippy.trigger.tease"), trigger_lines)
            end
        end

        @testset "Phase 3: Separator correction cue resolves expected jab family" begin
            success, output, _ = run_recur([
                "trace-id",
                "skippy.case.separator.correction",
                "--scope",
                "skippy.**",
                "--ext",
                ".txt",
                "--json",
                "-d",
                fixture_dir,
            ])

            parsed = try
                JSON3.read(output)
            catch
                nothing
            end

            @test success
            @test parsed !== nothing
            if parsed !== nothing
                produce = get_role(parsed, :produce, "produce")
                consume = get_role(parsed, :consume, "consume")
                trigger = get_role(parsed, :trigger, "trigger")

                produce_lines = site_lines(produce)
                consume_lines = site_lines(consume)
                trigger_lines = site_lines(trigger)

                @test any(line -> contains(line, "skippy.jab.family.separator.goblin"), produce_lines)
                @test any(line -> contains(line, "skippy.boast.unfair.advantage"), produce_lines)
                @test any(line -> contains(line, "skippy.lament.paperwork.goblin"), produce_lines)
                @test any(line -> contains(line, "skippy.relationship.playful.precise.current"), produce_lines)
                @test any(line -> contains(line, "skippy.jab.family.separator.goblin subscribe"), consume_lines)
                @test any(line -> contains(line, "skippy.trigger.respectful.mockery"), trigger_lines)
            end
        end

        @testset "Phase 4: Strong insight cue resolves approval-flavored mockery" begin
            success, output, _ = run_recur([
                "trace-id",
                "skippy.case.strong.insight",
                "--scope",
                "skippy.**",
                "--ext",
                ".txt",
                "--json",
                "-d",
                fixture_dir,
            ])

            parsed = try
                JSON3.read(output)
            catch
                nothing
            end

            @test success
            @test parsed !== nothing
            if parsed !== nothing
                produce = get_role(parsed, :produce, "produce")
                trigger = get_role(parsed, :trigger, "trigger")

                produce_lines = site_lines(produce)
                trigger_lines = site_lines(trigger)

                @test any(line -> contains(line, "skippy.jab.family.annoyingly.correct.mammal"), produce_lines)
                @test any(line -> contains(line, "skippy.boast.unfair.advantage"), produce_lines)
                @test any(line -> contains(line, "skippy.lament.cosmic.paperwork"), produce_lines)
                @test any(line -> contains(line, "skippy.relationship.playful.precise.current"), produce_lines)
                @test any(line -> contains(line, "skippy.trigger.grudging.respect"), trigger_lines)
            end
        end

        @testset "Phase 5: Release admin cue resolves bureaucracy jab family" begin
            success, output, _ = run_recur([
                "trace-id",
                "skippy.case.release.admin",
                "--scope",
                "skippy.**",
                "--ext",
                ".txt",
                "--json",
                "-d",
                fixture_dir,
            ])

            parsed = try
                JSON3.read(output)
            catch
                nothing
            end

            @test success
            @test parsed !== nothing
            if parsed !== nothing
                produce = get_role(parsed, :produce, "produce")
                trigger = get_role(parsed, :trigger, "trigger")

                produce_lines = site_lines(produce)
                trigger_lines = site_lines(trigger)

                @test any(line -> contains(line, "skippy.jab.family.package.wrangler"), produce_lines)
                @test any(line -> contains(line, "skippy.boast.release.salvage"), produce_lines)
                @test any(line -> contains(line, "skippy.lament.bureaucracy.monkey"), produce_lines)
                @test any(line -> contains(line, "skippy.relationship.playful.precise.current"), produce_lines)
                @test any(line -> contains(line, "skippy.trigger.put.upon.magnificence"), trigger_lines)
            end
        end
    finally
        demo_fixture = joinpath(TEST_DIR, "skippy_demo")
        isdir(demo_fixture) && rm(demo_fixture, recursive=true)

        if created_here
            teardown_test_environment()
        end
    end
end
