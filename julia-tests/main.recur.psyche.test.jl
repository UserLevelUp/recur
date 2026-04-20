"""
Tests for recur psyche command
==============================

Executable specification for the future `recur psyche` subcommand.
These tests are expected to fail until the command is implemented.
"""

include("runtests.setup.jl")

const RECUR_PSYCHE_BIN = RECUR_BIN

repo_root() = normpath(joinpath(@__DIR__, ".."))

function run_recur_raw(args::Vector{String})
    display_cmd = join(map(arg -> contains(arg, ' ') ? "\"$arg\"" : arg, args), " ")
    println("  -> recur $display_cmd")

    cmd = Cmd(`$RECUR_PSYCHE_BIN $args`, dir=repo_root())
    out = IOBuffer()
    err = IOBuffer()
    success = true

    try
        run(pipeline(cmd, stdout=out, stderr=err))
    catch e
        if isa(e, ProcessFailedException)
            success = false
        else
            return (false, "", "Error running command: $e")
        end
    end

    return (success, String(take!(out)), String(take!(err)))
end

function write_test_file(path::String, content::String)
    mkpath(dirname(path))
    write(path, content)
end

@testset "recur psyche command" begin
    log_section("Testing: recur psyche command")

    @testset "psyche help prints usage" begin
        success, output, error_output = run_recur_raw(["psyche", "--help"])
        help_text = lowercase(output * error_output)

        @test success
        @test occursin("psyche", help_text)
        @test occursin("dir", help_text)
        @test occursin("filter", help_text)
    end

    @testset "psyche on clean vault returns exit 0 with no findings" begin
        root = mktempdir()

        try
            success, output, _ = run_recur_raw(["psyche", "--dir", root])

            @test success
            @test strip(output) == ""
        finally
            rm(root; recursive=true, force=true)
        end
    end

    @testset "psyche detects orphan status without work file" begin
        root = mktempdir()

        try
            vault_dir = joinpath(root, ".recur", "test-agent")
            write_test_file(
                joinpath(vault_dir, "test-agent.status.current.md"),
                "STATE: active\n",
            )
            write_test_file(
                joinpath(vault_dir, "test-agent.recur.md"),
                "# test-agent.recur\n",
            )

            success, output, error_output = run_recur_raw(["psyche", "--dir", root])
            combined = lowercase(output * error_output)

            @test !success
            @test occursin("orphan", combined) ||
                  occursin("test-agent", combined) ||
                  occursin("test-agent.status.current.md", combined)
        finally
            rm(root; recursive=true, force=true)
        end
    end

    @testset "psyche detects missing capsule in agent vault" begin
        root = mktempdir()

        try
            vault_dir = joinpath(root, ".recur", "lonely-agent")
            write_test_file(
                joinpath(vault_dir, "lonely-agent.status.current.md"),
                "STATE: held-ready\n",
            )
            write_test_file(
                joinpath(vault_dir, "lonely-agent.work.current.md"),
                "# lonely-agent.work.current\n",
            )

            success, output, error_output = run_recur_raw(["psyche", "--dir", root])
            combined = lowercase(output * error_output)

            @test !success
            @test occursin("capsule", combined) ||
                  occursin("missing", combined) ||
                  occursin("lonely-agent", combined)
        finally
            rm(root; recursive=true, force=true)
        end
    end

    @testset "psyche json format emits parseable findings" begin
        root = mktempdir()

        try
            vault_dir = joinpath(root, ".recur", "json-agent")
            write_test_file(
                joinpath(vault_dir, "json-agent.status.current.md"),
                "STATE: active\n",
            )
            write_test_file(
                joinpath(vault_dir, "json-agent.recur.md"),
                "# json-agent.recur\n",
            )

            success, output, _ = run_recur_raw(["psyche", "--dir", root, "--format", "json"])
            parsed = try
                JSON3.read(output)
            catch
                nothing
            end

            @test !success
            @test parsed !== nothing

            if parsed !== nothing
                finding = parsed isa AbstractVector ? parsed[1] : parsed
                @test haskey(finding, :path) || haskey(finding, "path")
                @test haskey(finding, :kind) || haskey(finding, "kind")
            end
        finally
            rm(root; recursive=true, force=true)
        end
    end
end
