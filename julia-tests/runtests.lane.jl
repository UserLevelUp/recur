"""
Tests for 'recur lane' Command
==============================

Lane command validates the Improvement 21 phase 1 slice:
- `recur init` scaffolds a [lanes] section in `.recur/config.toml`
- `recur lane <name>` creates a named sub-root directory
- the sub-root gets its own `.recur/config.toml`
- the sub-root gets a `<name>.recur.md` capsule in `.recur/`
- `recur lane` lists known scaffolded lanes
- `recur reveal` works from within a scaffolded lane root
- missing [lanes] config falls back cleanly to `lanes/<name>/` at current root

Architecture note:
  The existing LaneConfig (dir + sep) is the separator-inference layer.
  [lanes] root is the lane-scaffolding doctrine. These are distinct concerns
  and must not collide in config or tests.
"""

include("runtests.setup.jl")

function seed_lane_project(root::String)
    mkpath(joinpath(root, "docs"))
    mkpath(joinpath(root, "src"))
    write(joinpath(root, "docs", "main.readme.md"), "# project")
    write(joinpath(root, "src", "main_impl.rs"), "// impl")
end

@testset "recur lane command" begin
    log_section("Testing: recur lane")

    @testset "lane CLI contract" begin
        success, output, _ = run_recur(["lane", "--help"])

        @test success
        @test contains(lowercase(output), "lane")
        @test contains(output, "recur lane docs")
        @test contains(output, "recur lane impl")
    end

    @testset "init scaffolds [lanes] section" begin
        root = mktempdir()
        try
            seed_lane_project(root)
            success, _, _ = run_recur(["init", "-d", root])
            @test success

            config_text = read(joinpath(root, ".recur", "config.toml"), String)
            @test contains(config_text, "[lanes]")
            @test contains(config_text, "root =")
            @test contains(config_text, "entry_suffix =")
        finally
            rm(root, recursive=true)
        end
    end

    @testset "recur lane creates sub-root directory" begin
        root = mktempdir()
        try
            seed_lane_project(root)
            success, _, _ = run_recur(["init", "-d", root])
            @test success

            success, _, _ = run_recur(["lane", "docs", "-d", root])
            @test success
            @test isdir(joinpath(root, "lanes", "docs"))
        finally
            rm(root, recursive=true)
        end
    end

    @testset "recur lane creates nested .recur/config.toml" begin
        root = mktempdir()
        try
            seed_lane_project(root)
            success, _, _ = run_recur(["init", "-d", root])
            @test success

            success, _, _ = run_recur(["lane", "impl", "-d", root])
            @test success

            lane_config = joinpath(root, "lanes", "impl", ".recur", "config.toml")
            @test isfile(lane_config)

            config_text = read(lane_config, String)
            @test contains(config_text, "[reveal]")
            @test contains(config_text, "entry_suffix")
        finally
            rm(root, recursive=true)
        end
    end

    @testset "recur lane scaffolds reveal capsule in .recur/" begin
        root = mktempdir()
        try
            seed_lane_project(root)
            success, _, _ = run_recur(["init", "-d", root])
            @test success

            success, _, _ = run_recur(["lane", "tests", "-d", root])
            @test success

            capsule = joinpath(root, "lanes", "tests", ".recur", "tests.recur.md")
            @test isfile(capsule)

            capsule_text = read(capsule, String)
            @test contains(capsule_text, "pull.first")
            @test contains(capsule_text, "verify")
            @test contains(capsule_text, "ready.state")
        finally
            rm(root, recursive=true)
        end
    end

    @testset "recur reveal works from within a scaffolded lane root" begin
        root = mktempdir()
        try
            seed_lane_project(root)
            success, _, _ = run_recur(["init", "-d", root])
            @test success

            success, _, _ = run_recur(["lane", "docs", "-d", root])
            @test success

            lane_root = joinpath(root, "lanes", "docs")
            success, output, _ = run_recur(["reveal", "docs", "-d", lane_root, "--json"])
            @test success

            parsed = JSON3.read(output)
            @test String(parsed["lane"]) == "docs"
            @test contains(String(parsed["path"]), ".recur")
        finally
            rm(root, recursive=true)
        end
    end

    @testset "recur lane lists known lanes as JSON" begin
        root = mktempdir()
        try
            seed_lane_project(root)
            success, _, _ = run_recur(["init", "-d", root])
            @test success

            run_recur(["lane", "docs",  "-d", root])
            run_recur(["lane", "impl",  "-d", root])
            run_recur(["lane", "tests", "-d", root])

            success, output, _ = run_recur(["lane", "-d", root, "--json"])
            @test success

            parsed = JSON3.read(output)
            names = [String(entry["name"]) for entry in parsed["lanes"]]
            @test "docs"  in names
            @test "impl"  in names
            @test "tests" in names
        finally
            rm(root, recursive=true)
        end
    end

    @testset "recur lane is idempotent — second scaffold does not clobber capsule" begin
        root = mktempdir()
        try
            seed_lane_project(root)
            success, _, _ = run_recur(["init", "-d", root])
            @test success

            run_recur(["lane", "docs", "-d", root])

            capsule = joinpath(root, "lanes", "docs", ".recur", "docs.recur.md")
            original = read(capsule, String)
            write(capsule, original * "\nextra = hand-written line\n")

            # Running lane again must not overwrite the existing capsule
            success, _, _ = run_recur(["lane", "docs", "-d", root])
            @test success
            @test contains(read(capsule, String), "hand-written line")
        finally
            rm(root, recursive=true)
        end
    end

    @testset "recur lane falls back to lanes/<name>/ when [lanes] root absent" begin
        root = mktempdir()
        try
            # No recur init — no [lanes] config at all
            success, _, _ = run_recur(["lane", "docs", "-d", root])
            @test success
            @test isdir(joinpath(root, "lanes", "docs"))
        finally
            rm(root, recursive=true)
        end
    end

    @testset "recur lane rejects blank name" begin
        root = mktempdir()
        try
            success, _, stderr = run_recur(["lane", "", "-d", root])
            @test !success
        finally
            rm(root, recursive=true)
        end
    end

end
