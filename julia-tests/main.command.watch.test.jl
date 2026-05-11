"""
Tests for `recur watch` pure query command
==========================================

Executable specification for the synchronous core query surface.
The active subscription runner is `recur-watch`; `recur watch` reads
watcher state eventness and exits.
"""

include("runtests.setup.jl")

function seed_watch_state(root::String; id::String="docs-monkey", state::String="active", ack::String="accepted")
    watch_dir = joinpath(root, ".recur", "watch")
    mkpath(watch_dir)

    path = joinpath(watch_dir, "recur-watch.$id.status.current.md")
    write(
        path,
        """
state = $state
ack = $ack
nak_reason = ""
filter = monkey.**
dir = .recur/docs-monkey
mode = poll
poll_framing = 5
format = json
pid = 12345
started_at = 2026-05-11T00:00:00Z
last_event_at = 2026-05-11T00:00:12Z
events_seen = 12
filtered_out = 43
""",
    )

    return path
end

@testset "recur watch query command" begin
    log_section("Testing: recur watch query")

    @testset "watch help exposes pure query surface" begin
        success, output, error_output = run_recur(["watch", "--help"])
        help_text = output * error_output

        @test success
        @test contains(lowercase(help_text), "inspect watcher state")
        @test contains(help_text, "list")
        @test contains(help_text, "status")
        @test contains(help_text, "explain")
    end

    @testset "watch explain documents runner/query split" begin
        success, output, _ = run_recur(["watch", "explain"])

        @test success
        @test contains(output, "pure watcher-state query")
        @test contains(output, "active subscription runner")
        @test contains(output, ".recur/watch")
    end

    @testset "watch list is empty when no watcher state exists" begin
        root = mktempdir()
        try
            success, output, _ = run_recur(["watch", "-d", root])

            @test success
            @test isempty(strip(output))
        finally
            rm(root; recursive=true, force=true)
        end
    end

    @testset "watch list and status read ACK state files" begin
        root = mktempdir()
        try
            seed_watch_state(root)

            success, output, _ = run_recur(["watch", "-d", root])
            @test success
            @test contains(output, "docs-monkey")
            @test contains(output, "active")
            @test contains(output, "accepted")

            success, filtered, _ = run_recur(["watch", "list", "--filter", "**.active", "-d", root])
            @test success
            @test contains(filtered, "docs-monkey")

            success, missed, _ = run_recur(["watch", "list", "--filter", "**.stale", "-d", root])
            @test success
            @test isempty(strip(missed))

            success, status_json, _ = run_recur(["--json", "watch", "status", "docs-monkey", "-d", root])
            @test success

            parsed = JSON3.read(status_json)
            @test length(parsed) == 1
            record = parsed[1]
            @test string(record[:id]) == "docs-monkey"
            @test string(record[:state]) == "active"
            @test string(record[:ack]) == "accepted"
            @test string(record[:filter]) == "monkey.**"
            @test string(record[:mode]) == "poll"
        finally
            rm(root; recursive=true, force=true)
        end
    end

    @testset "watch status reports missing watcher" begin
        root = mktempdir()
        try
            success, _, error_output = run_recur(["watch", "status", "missing-watch", "-d", root])

            @test !success
            @test contains(error_output, "watch 'missing-watch' not found")
        finally
            rm(root; recursive=true, force=true)
        end
    end
end
