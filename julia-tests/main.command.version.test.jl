"""
Tests for recur version and recur-version
=========================================

Executable specification for the pure version query surface and the write-side
artifact version companion.
"""

include("runtests.setup.jl")

const RECUR_VERSION_BIN = get(
    ENV,
    "RECUR_VERSION_BIN",
    joinpath(@__DIR__, "..", "target", RECUR_PROFILE, "recur-version" * (Sys.iswindows() ? ".exe" : "")),
)

repo_root() = normpath(joinpath(@__DIR__, ".."))

function run_recur_version(args::Vector{String})
    display_cmd = join(map(arg -> contains(arg, ' ') ? "\"$arg\"" : arg, args), " ")
    println("  -> recur-version $display_cmd")

    cmd = Cmd(`$RECUR_VERSION_BIN $args`, dir=repo_root())
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

function make_version_fixture()
    root = mktempdir()
    mkdir(joinpath(root, ".recur"))
    write(joinpath(root, ".recur", "config.toml"), """
[artifact."care.subject.routine"]
kind = "structured-routine"
format = "csv"
risk_class = "synthetic-clinical-fixture"
privacy_root = "fixtures/private/"
persona = "care_schedule_expert"

[artifact."care.subject.routine".fields]
identity = ["TaskOrItem", "Route"]
tracked = ["Time", "DoseOrAmount", "Route", "Status", "Notes"]
state = "Status"
notes = ["Notes"]

[artifact."care.subject.routine".states]
proposed = ["DRAFT", "PROPOSED"]
discontinued = ["DISCONTINUED", "OUT CURRENTLY"]

[artifact."care.subject.routine".versioning]
strategy = "letter-number"
manifest_required = true
queryable = true
operator_required_for = ["approved", "discontinued", "restart_candidate"]
""")

    write(joinpath(root, "care.subject.routine.proposed.current.csv"), """
TaskOrItem,Route,Time,DoseOrAmount,Status,Notes
item-a,route-a,time-a,amount-a,DISCONTINUED - OUT CURRENTLY,latest synthetic state
item-b,route-b,time-b,amount-b,DRAFT UNVERIFIED,still proposed
""")

    write(joinpath(root, "care.subject.routine.proposed.version.manifest.current.md"), """
# Version Manifest: care.subject.routine

Artifact: care.subject.routine.proposed.current.csv
Format: csv
Lifecycle: proposed
Latest version: a2

Versions:
- a1 - initial synthetic routine
- a2 - corrected discontinued item
""")

    write(joinpath(root, "care.subject.routine.proposed.version.a1.initial.csv"), """
TaskOrItem,Route,Time,DoseOrAmount,Status,Notes
item-a,route-a,time-a,amount-a,DRAFT UNVERIFIED,initial synthetic state
item-b,route-b,time-b,amount-b,DRAFT UNVERIFIED,still proposed
""")

    write(joinpath(root, "care.subject.routine.proposed.version.a2.corrected-discontinued-item.csv"), """
TaskOrItem,Route,Time,DoseOrAmount,Status,Notes
item-a,route-a,time-a,amount-a,DISCONTINUED - OUT CURRENTLY,corrected synthetic state
item-b,route-b,time-b,amount-b,DRAFT UNVERIFIED,still proposed
""")

    return root
end

@testset "recur version command" begin
    log_section("Testing: recur version command")

    root = make_version_fixture()
    try
        @testset "status reports manifest and next version" begin
            success, output, error_output = run_recur([
                "version", "status", "care.subject.routine", "-d", root
            ])

            @test success
            @test error_output == ""
            @test contains(output, "Current artifact:")
            @test contains(output, "Latest version: a2")
            @test contains(output, "Next version: a3")
            @test contains(output, "Policy configured: yes")
        end

        @testset "policy and schema expose config semantics" begin
            success, output, _ = run_recur([
                "version", "policy", "care.subject.routine", "-d", root
            ])
            @test success
            @test contains(output, "Risk class: synthetic-clinical-fixture")
            @test contains(output, "Operator required for: approved, discontinued, restart_candidate")

            success, output, _ = run_recur([
                "version", "schema", "care.subject.routine", "-d", root
            ])
            @test success
            @test contains(output, "Identity fields: TaskOrItem, Route")
            @test contains(output, "State field: Status")
            @test contains(output, "discontinued")
        end

        @testset "query answers with evidence from preserved versions" begin
            success, output, _ = run_recur([
                "version", "query", "care.subject.routine",
                "--question", "when did item-a become discontinued",
                "-d", root,
            ])

            @test success
            @test contains(output, "item-a first appears with discontinued state in version a2")
            @test contains(output, "manifest entry: corrected discontinued item")
            @test contains(output, "observed state: DISCONTINUED - OUT CURRENTLY")
        end
    finally
        rm(root; recursive=true, force=true)
    end
end

@testset "recur-version command" begin
    log_section("Testing: recur-version command")

    root = make_version_fixture()
    try
        @testset "next reports next version without writing" begin
            success, output, _ = run_recur_version([
                "next", "care.subject.routine.proposed.current.csv", "-d", root
            ])

            @test success
            @test contains(output, "Version: a3")
            @test !isfile(joinpath(root, "care.subject.routine.proposed.version.a3.item-a-discontinued.csv"))
        end

        @testset "save snapshots current artifact and writes ACK status" begin
            success, output, error_output = run_recur_version([
                "save", "care.subject.routine.proposed.current.csv",
                "--slug", "item-a-discontinued",
                "--reason", "synthetic state correction",
                "--operator", "operator-a",
                "-d", root,
            ])

            @test success
            @test error_output == ""
            @test contains(output, "Version: a3")

            snapshot = joinpath(root, "care.subject.routine.proposed.version.a3.item-a-discontinued.csv")
            manifest = joinpath(root, "care.subject.routine.proposed.version.manifest.current.md")
            status = joinpath(root, ".recur", "version", "recur-version.care.subject.routine.proposed.current.status.current.md")

            @test isfile(snapshot)
            @test contains(read(manifest, String), "Latest version: a3")
            @test contains(read(manifest, String), "operator=operator-a")
            @test isfile(status)
            @test contains(read(status, String), "ack = \"accepted\"")

            success, status_output, _ = run_recur([
                "version", "status", "care.subject.routine", "-d", root
            ])
            @test success
            @test contains(status_output, "Latest version: a3")
            @test contains(status_output, "Next version: a4")
        end

        @testset "save writes NAK status on rejected request" begin
            success, _, error_output = run_recur_version([
                "save", "missing.current.csv",
                "--slug", "bad-save",
                "--id", "bad-save",
                "-d", root,
            ])

            @test !success
            @test contains(lowercase(error_output), "not found")

            status = joinpath(root, ".recur", "version", "recur-version.bad-save.status.current.md")
            @test isfile(status)
            status_text = read(status, String)
            @test contains(status_text, "ack = \"rejected\"")
            @test contains(status_text, "nak_reason = ")
        end
    finally
        rm(root; recursive=true, force=true)
    end
end
