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

function set_test_file_mtime(path::String, unix_seconds::Real)
    timestamp = floor(Int, unix_seconds)
    escaped_path = replace(path, "'" => "''")

    if Sys.iswindows()
        ps_command =
            "\$path = '$escaped_path'; \$mtime = [DateTimeOffset]::FromUnixTimeSeconds($timestamp).UtcDateTime; [System.IO.File]::SetLastWriteTimeUtc(\$path, \$mtime)"
        run(`powershell -NoProfile -Command $ps_command`)
    else
        error("set_test_file_mtime is currently implemented only for Windows test hosts")
    end
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

    @testset "psyche detects stale current files with stale-seconds threshold" begin
        root = mktempdir()

        try
            vault_dir = joinpath(root, ".recur", "stale-agent")
            status_path = joinpath(vault_dir, "stale-agent.status.current.md")
            work_path = joinpath(vault_dir, "stale-agent.work.current.md")
            write_test_file(status_path, "STATE: active\n")
            write_test_file(work_path, "# stale-agent.work.current\n")
            write_test_file(joinpath(vault_dir, "stale-agent.recur.md"), "# stale-agent.recur\n")
            set_test_file_mtime(work_path, time() - 3600)

            success, output, error_output = run_recur_raw([
                "psyche", "--dir", root, "--stale-seconds", "60"
            ])
            combined = lowercase(output * error_output)

            @test !success
            @test occursin("stale-current", combined) ||
                  occursin("stale-agent", combined) ||
                  occursin("stale-agent.work.current.md", combined)
            @test !occursin("unexpected argument", combined)
        finally
            rm(root; recursive=true, force=true)
        end
    end

    @testset "psyche detects missing last-run after thrust" begin
        root = mktempdir()

        try
            vault_dir = joinpath(root, ".recur", "thrust-agent")
            write_test_file(
                joinpath(vault_dir, "thrust-agent.status.current.md"),
                "STATE: stopped-awaiting-merge\n",
            )
            write_test_file(
                joinpath(vault_dir, "thrust-agent.work.current.md"),
                "# thrust-agent.work.current\n",
            )
            write_test_file(
                joinpath(vault_dir, "thrust-agent.recur.md"),
                "# thrust-agent.recur\n",
            )

            success, output, error_output = run_recur_raw(["psyche", "--dir", root])
            combined = lowercase(output * error_output)

            @test !success
            @test occursin("missing-last-run-after-thrust", combined) ||
                  occursin("last-run", combined) ||
                  occursin("thrust-agent", combined)
            @test occursin("thrust-agent.status.current.md", combined) ||
                  occursin("stopped-awaiting-merge", combined)
        finally
            rm(root; recursive=true, force=true)
        end
    end

    @testset "psyche detects orphan work without status file" begin
        root = mktempdir()

        try
            vault_dir = joinpath(root, ".recur", "orphan-worker")
            write_test_file(
                joinpath(vault_dir, "orphan-worker.work.current.md"),
                "# orphan-worker.work.current\n",
            )
            write_test_file(
                joinpath(vault_dir, "orphan-worker.recur.md"),
                "# orphan-worker.recur\n",
            )

            success, output, error_output = run_recur_raw(["psyche", "--dir", root])
            combined = lowercase(output * error_output)

            @test !success
            @test occursin("orphan-work", combined) ||
                  occursin("orphan-worker", combined) ||
                  occursin("orphan-worker.work.current.md", combined)
            @test occursin("work", combined)
        finally
            rm(root; recursive=true, force=true)
        end
    end

    @testset "psyche filter json isolates one v2 finding kind" begin
        root = mktempdir()

        try
            orphan_dir = joinpath(root, ".recur", "orphan-json")
            write_test_file(
                joinpath(orphan_dir, "orphan-json.work.current.md"),
                "# orphan-json.work.current\n",
            )
            write_test_file(
                joinpath(orphan_dir, "orphan-json.recur.md"),
                "# orphan-json.recur\n",
            )

            thrust_dir = joinpath(root, ".recur", "thrust-json")
            write_test_file(
                joinpath(thrust_dir, "thrust-json.status.current.md"),
                "STATE: stopped-awaiting-merge\n",
            )
            write_test_file(
                joinpath(thrust_dir, "thrust-json.work.current.md"),
                "# thrust-json.work.current\n",
            )
            write_test_file(
                joinpath(thrust_dir, "thrust-json.recur.md"),
                "# thrust-json.recur\n",
            )

            success, output, _ = run_recur_raw([
                "psyche", "--dir", root, "--filter", "orphan-work", "--format", "json"
            ])
            parsed = try
                JSON3.read(output)
            catch
                nothing
            end

            findings = if parsed === nothing
                Any[]
            elseif parsed isa AbstractVector
                collect(parsed)
            else
                Any[parsed]
            end
            kinds = String[
                haskey(finding, :kind) ? String(finding[:kind]) : String(finding["kind"])
                for finding in findings
            ]

            @test !success
            @test parsed !== nothing
            @test length(findings) >= 1
            @test length(kinds) >= 1 && all(==("orphan-work"), kinds)
        finally
            rm(root; recursive=true, force=true)
        end
    end
end
