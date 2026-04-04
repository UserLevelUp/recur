"""
Tests for 'recur init' Command
==============================

Init command validates project-local `.recur/config.toml` generation,
analyze-mode suggestions, and lane-name collision handling for generated
section names.
"""

include("runtests.setup.jl")

function seed_init_fixture(root::String)
    mkpath(joinpath(root, "src"))
    mkpath(joinpath(root, "docs"))
    mkpath(joinpath(root, "julia-tests"))

    write(joinpath(root, "src", "main_command_find_impl.rs"), "")
    write(joinpath(root, "docs", "main.command.find.readme.md"), "")
    write(joinpath(root, "julia-tests", "main.command.find.test.jl"), "")
end

@testset "recur init command" begin
    log_section("Testing: recur init")

    @testset "init CLI contract" begin
        success, output, _ = run_recur(["init", "-d", TEST_DIR, "--help"])

        @test success
        @test contains(lowercase(output), "init")
        @test contains(output, "--analyze")
        @test contains(output, "--force")
    end

    @testset "init creates config and trait surface" begin
        root = mktempdir()
        try
            seed_init_fixture(root)

            success, _, _ = run_recur(["init", "-d", root])
            @test success
            @test isfile(joinpath(root, ".recur", "config.toml"))
            @test isfile(joinpath(root, ".recur", "checkpoints.md"))

            config_text = read(joinpath(root, ".recur", "config.toml"), String)
            @test contains(config_text, "[src]")
            @test contains(config_text, "[docs]")
            @test contains(config_text, "[julia-tests]")
            @test contains(config_text, "[traits.content_search]")
            @test contains(config_text, "[traits.separator_merge]")
            @test contains(config_text, "[traits.stdin]")
            @test contains(config_text, "[traits.trace_id]")
            @test contains(config_text, "[traits.traversal_budget]")

            success, trait_output, _ = run_recur(["trait", "-d", root, "list"])
            @test success
            @test contains(trait_output, "[traits.content_search]")
            @test contains(trait_output, "[traits.stdin]")
            @test contains(trait_output, "[traits.trace_id]")
            @test contains(trait_output, "[traits.traversal_budget]")
        finally
            rm(root, recursive=true)
        end
    end

    @testset "init analyze mode reports additions and separator updates" begin
        root = mktempdir()
        try
            mkpath(joinpath(root, "src"))
            mkpath(joinpath(root, "docs"))
            mkpath(joinpath(root, ".recur"))

            write(joinpath(root, "src", "main_command_tree_impl.rs"), "")
            write(joinpath(root, "docs", "main.command.tree.readme.md"), "")
            write(
                joinpath(root, ".recur", "config.toml"),
                """
[src]
dir = "src/"
sep = "."
""",
            )

            success, output, _ = run_recur(["--json", "init", "-d", root, "--analyze"])
            @test success

            parsed = JSON3.read(output)
            additions = haskey(parsed, :additions) ? parsed[:additions] : parsed["additions"]
            separator_updates = haskey(parsed, :separator_updates) ? parsed[:separator_updates] : parsed["separator_updates"]

            @test any(item -> String(item["name"]) == "docs", additions)
            @test any(
                item -> String(item["name"]) == "src" && String(item["suggested_sep"]) == "_",
                separator_updates,
            )
        finally
            rm(root, recursive=true)
        end
    end

    @testset "init deduplicates colliding lane names and trait reads still work" begin
        root = mktempdir()
        try
            mkpath(joinpath(root, "test-quick"))
            mkpath(joinpath(root, "test_quick"))

            write(joinpath(root, "test-quick", "main-command-tree-notes.md"), "")
            write(joinpath(root, "test_quick", "main.command.tree.readme.md"), "")

            success, _, _ = run_recur(["init", "-d", root])
            @test success

            config_text = read(joinpath(root, ".recur", "config.toml"), String)
            @test contains(config_text, "[test-quick]")
            @test contains(config_text, "dir = \"test-quick/\"")
            @test contains(config_text, "[test-quick-2]")
            @test contains(config_text, "dir = \"test_quick/\"")

            success, trait_output, _ = run_recur(["trait", "-d", root, "list"])
            @test success
            @test contains(trait_output, "[traits.traversal_budget]")

            success, get_output, _ = run_recur(["trait", "-d", root, "get", "traversal_budget.depth_guard"])
            @test success
            @test contains(get_output, "traits.traversal_budget.depth_guard")
            @test contains(get_output, "hard-fail")
        finally
            rm(root, recursive=true)
        end
    end
end
