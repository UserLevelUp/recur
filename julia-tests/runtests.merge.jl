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

    @testset "stdin mode with single JSON input" begin
        create_test_file("main.command.files.readme.md", "# Files Doc")

        # Generate JSON output from files command
        success_files, json_output, _ = run_recur([
            "files",
            "main.command.files.**",
            "-d",
            TEST_DIR,
            "--sep",
            ".",
            "--json",
        ])

        @test success_files

        # Pipe JSON into merge via stdin
        success, output, _ = run_recur(
            ["merge", "--stdin", "--base", "main.command.files", "--sep", "."],
            input=json_output,
        )

        passed = success &&
                 contains(output, "main.command.files") &&
                 contains(output, "readme.md")

        println(passed ? "  ✓ PASS" : "  ✗ FAIL")

        @test success
        @test contains(output, "main.command.files")
        @test contains(output, "readme.md")
        log_test("stdin mode with single JSON input works")
    end

    @testset "stdin mode with multiple JSON inputs" begin
        create_test_file("main.command.tree.readme.md", "# Tree Doc")
        create_test_file("main_command_tree_impl.rs", "fn tree_impl() {}")

        # Generate JSON from tree commands (simulating multiple inputs)
        success1, json1, _ = run_recur([
            "tree",
            "main.command.tree",
            "-d",
            TEST_DIR,
            "--sep",
            ".",
            "--json",
        ])

        success2, json2, _ = run_recur([
            "tree",
            "main_command_tree",
            "-d",
            TEST_DIR,
            "--sep",
            "_",
            "--json",
        ])

        @test success1 && success2

        # Concatenate JSON objects and pipe to merge
        combined_json = json1 * json2

        success, output, _ = run_recur(
            ["merge", "--stdin", "--base", "main.command.tree", "--sep", ".", "--sep", "_", "--show-sep"],
            input=combined_json,
        )

        passed = success &&
                 contains(output, "main.command.tree") &&
                 contains(output, "readme.md") &&
                 contains(output, "impl.rs") &&
                 contains(output, "[.]") &&
                 contains(output, "[_]")

        println(passed ? "  ✓ PASS" : "  ✗ FAIL")

        @test success
        @test contains(output, "main.command.tree")
        @test contains(output, "readme.md")
        @test contains(output, "impl.rs")
        @test contains(output, "[.]")
        @test contains(output, "[_]")
        log_test("stdin mode with multiple JSON inputs works")
    end

    @testset "auto-JSON when piped" begin
        create_test_file("main.auto.json.test.md", "# Auto JSON Test")

        # Run tree without --json flag, pipe to merge (should auto-enable JSON)
        # This simulates: recur tree X | recur merge --stdin
        success_tree, json_output, _ = run_recur([
            "tree",
            "main.auto",
            "-d",
            TEST_DIR,
            "--sep",
            ".",
        ])

        @test success_tree

        # The output should be JSON (auto-detected pipe)
        # Try to parse it as JSON to verify
        try
            parsed = JSON3.read(json_output)
            json_valid = true
        catch
            json_valid = false
        end

        passed = json_valid

        println(passed ? "  ✓ PASS" : "  ✗ FAIL")

        @test json_valid
        log_test("tree auto-enables JSON when piped")
    end
end
