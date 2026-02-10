"""
Tests for 'recur merge' Command
===============================

Covers file mode JSON inputs for Phase 4.
"""

include("runtests.setup.jl")

@testset "recur merge command" begin
    log_section("Testing: recur merge")

    @testset "file mode JSON inputs" begin
        create_test_file("main.command.merge.readme.md", "# Merge Doc")
        create_test_file("main_command_merge_impl.rs", "fn merge_impl() {}")

        docs_json = joinpath(TEST_DIR, "merge_docs.json")
        src_json = joinpath(TEST_DIR, "merge_src.json")

        open(docs_json, "w") do io
            JSON3.write(io, [joinpath(TEST_DIR, "main.command.merge.readme.md")])
        end

        open(src_json, "w") do io
            JSON3.write(io, Dict("files" => [joinpath(TEST_DIR, "main_command_merge_impl.rs")]))
        end

        success, output, _ = run_recur([
            "merge",
            docs_json,
            "--sep",
            ".",
            src_json,
            "--sep",
            "_",
            "--base",
            "main.command.merge",
            "--show-sep",
        ])

        passed = success &&
                 contains(output, "main.command.merge") &&
                 contains(output, "readme.md") &&
                 contains(output, "impl.rs") &&
                 contains(output, "[.]") &&
                 contains(output, "[_]")

        println(passed ? "  ✓ PASS" : "  ✗ FAIL")

        @test success
        @test contains(output, "main.command.merge")
        @test contains(output, "readme.md")
        @test contains(output, "impl.rs")
        @test contains(output, "[.]")
        @test contains(output, "[_]")
        log_test("file mode JSON inputs work")
    end
end
