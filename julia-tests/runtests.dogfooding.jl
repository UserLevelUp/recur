"""
Tests for dogfooding hierarchy conventions with `recur tree`.

Focus:
1) Dot-separated hierarchy works end-to-end across code/tests/docs.
2) Underscore-separated hierarchy works for Rust-compatible naming.
3) The chosen separator controls parsing precedence (`--sep`).
"""

include("runtests.setup.jl")

"""
Create an isolated fixture for dogfooding tests inside TEST_DIR.
"""
function setup_dogfooding_fixture()
    fixture_dir = joinpath(TEST_DIR, "dogfooding")
    isdir(fixture_dir) && rm(fixture_dir, recursive=true)

    mkpath(joinpath(fixture_dir, "src"))
    mkpath(joinpath(fixture_dir, "julia-tests"))
    mkpath(joinpath(fixture_dir, "docs"))

    # Dot-separated artifacts for main.command.files
    write(joinpath(fixture_dir, "src", "main.command.files.impl.rs"), "// rust impl\n")
    write(joinpath(fixture_dir, "julia-tests", "main.command.files.test.jl"), "# julia test\n")
    write(joinpath(fixture_dir, "docs", "main.command.files.readme.md"), "# readme\n")
    write(joinpath(fixture_dir, "docs", "main.command.files.todo.md"), "# todo\n")
    write(joinpath(fixture_dir, "docs", "main.command.files.todo.priority.md"), "# priority\n")

    # Dot-separated artifacts for main.command.stats (intentionally no priority)
    write(joinpath(fixture_dir, "src", "main.command.stats.impl.rs"), "// rust impl\n")
    write(joinpath(fixture_dir, "julia-tests", "main.command.stats.test.jl"), "# julia test\n")
    write(joinpath(fixture_dir, "docs", "main.command.stats.readme.md"), "# readme\n")
    write(joinpath(fixture_dir, "docs", "main.command.stats.todo.md"), "# todo\n")

    # Separator-precedence sentinels
    write(joinpath(fixture_dir, "src", "main.command.dotonly.impl.rs"), "// dot only\n")
    write(joinpath(fixture_dir, "src", "main_command_underonly_impl.rs"), "// underscore only\n")

    # Rust-friendly underscore hierarchy
    write(joinpath(fixture_dir, "src", "command_files_impl.rs"), "// files impl\n")
    write(joinpath(fixture_dir, "src", "command_files_todo_priority.md"), "# files priority\n")
    write(joinpath(fixture_dir, "julia-tests", "command_files_test.jl"), "# files tests\n")
    write(joinpath(fixture_dir, "docs", "command_files_readme.md"), "# files docs\n")

    return fixture_dir
end

@testset "recur dogfooding hierarchy tests" begin
    log_section("Testing: dogfooding hierarchy with recur tree")

    fixture_dir = setup_dogfooding_fixture()

    @testset "dot separator tree across folders" begin
        success, output, _ = run_recur(["tree", "main", "-d", fixture_dir])

        @test success
        @test contains(output, "main")
        @test contains(output, "command")
        @test contains(output, "files")
        @test contains(output, "impl.rs")
        @test contains(output, "test.jl")
        @test contains(output, "readme.md")
        @test contains(output, "todo.md")
        @test contains(output, "priority.md")
    end

    @testset "dot branch shows missing priority by absence" begin
        success, output, _ = run_recur(["tree", "main.command.stats", "-d", fixture_dir])

        @test success
        @test contains(output, "main.command.stats")
        @test contains(output, "impl.rs")
        @test contains(output, "test.jl")
        @test contains(output, "readme.md")
        @test contains(output, "todo.md")
        @test !contains(output, "priority.md")
    end

    @testset "underscore separator works for rust naming" begin
        success, output, _ = run_recur(["tree", "command", "-d", fixture_dir, "--sep", "_"])

        @test success
        @test contains(output, "command")
        @test contains(output, "files")
        @test contains(output, "impl.rs")
        @test contains(output, "test.jl")
        @test contains(output, "readme.md")
        @test contains(output, "priority.md")
    end

    @testset "separator precedence follows chosen --sep" begin
        success_dot, output_dot, _ = run_recur(["tree", "main", "-d", fixture_dir])
        success_underscore, output_underscore, _ =
            run_recur(["tree", "main", "-d", fixture_dir, "--sep", "_"])

        @test success_dot
        @test success_underscore

        # Dot parsing should include only dot-only branch
        @test contains(output_dot, "dotonly")
        @test !contains(output_dot, "underonly")

        # Underscore parsing should include only underscore-only branch
        @test contains(output_underscore, "underonly")
        @test !contains(output_underscore, "dotonly")
    end
end
