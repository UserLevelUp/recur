"""
Tests for 'recur reveal' Command
================================

Reveal command validates the first shippable Improvement 22 slice:
- `recur init` scaffolds reveal defaults in `.recur/config.toml`
- `recur reveal` lists lane-local `*.recur.md` files
- `recur reveal <lane>` orders fields by the configured reveal doctrine
- missing reveal config falls back cleanly to built-in defaults
"""

include("runtests.setup.jl")

function seed_reveal_project(root::String)
    mkpath(joinpath(root, "src"))
    mkpath(joinpath(root, "docs"))
    mkpath(joinpath(root, ".recur", "skippy"))

    write(joinpath(root, "src", "main_command_trace_id_impl.rs"), "")
    write(joinpath(root, "docs", "main.command.trace-id.readme.md"), "")

    write(
        joinpath(root, "docs", "main.command.trace-id.recur.md"),
        """
# main.command.trace-id.recur

recur.gift = saved-run policy is the one real loose thread
persona = recur expert in the trace-id lane
agent = resume and verify
pull.first = recur files "main.command.trace-id.**" -d docs/
verify = julia julia-tests/main.command.trace-id.test.jl
ready.state = I know the lane and what to pull next
""",
    )

    write(
        joinpath(root, ".recur", "skippy", "skippy.recur.md"),
        """
# skippy.recur

recur.gift = Skippy wakes up, boasts briefly, and then does the work
persona = Skippy the Magnificent
agent = be theatrical for one breath and useful immediately after
pull.first = recur files "skippy.**" -d .recur/skippy --sep .
verify = recur find "Skippy" --scope "skippy.**" -d .recur/skippy --sep .
ready.state = I know who I am and which lane to pull first
""",
    )
end

@testset "recur reveal command" begin
    log_section("Testing: recur reveal")

    @testset "reveal CLI contract" begin
        success, output, _ = run_recur(["reveal", "--help"])

        @test success
        @test contains(lowercase(output), "reveal")
        @test contains(output, "main.command.trace-id")
        @test contains(output, "skippy")
    end

    @testset "init scaffolds reveal defaults and reveal lists entries" begin
        root = mktempdir()
        try
            seed_reveal_project(root)

            success, _, _ = run_recur(["init", "-d", root])
            @test success

            config_text = read(joinpath(root, ".recur", "config.toml"), String)
            @test contains(config_text, "[reveal]")
            @test contains(config_text, "entry_suffix = \".recur.md\"")
            @test contains(config_text, "[reveal.order]")

            success, output, _ = run_recur(["reveal", "-d", root, "--json"])
            @test success

            parsed = JSON3.read(output)
            entries = haskey(parsed, :entries) ? parsed[:entries] : parsed["entries"]

            @test any(item -> String(item["lane"]) == "main.command.trace-id", entries)
            @test any(item -> String(item["lane"]) == "skippy", entries)
        finally
            rm(root, recursive=true)
        end
    end

    @testset "reveal opens one lane and honors ordered fields" begin
        root = mktempdir()
        try
            seed_reveal_project(root)
            success, _, _ = run_recur(["init", "-d", root])
            @test success

            success, output, _ = run_recur(["reveal", "main.command.trace-id", "-d", root, "--json"])
            @test success

            parsed = JSON3.read(output)
            ordered = haskey(parsed, :ordered_fields) ? parsed[:ordered_fields] : parsed["ordered_fields"]

            @test String(parsed["lane"]) == "main.command.trace-id"
            @test String(parsed["path"]) == "docs/main.command.trace-id.recur.md"
            @test String(ordered[1]["key"]) == "recur.gift"
            @test String(ordered[2]["key"]) == "persona"
            @test String(ordered[3]["key"]) == "agent"
            @test String(ordered[4]["key"]) == "pull.first"
            @test String(ordered[5]["key"]) == "verify"
            @test String(ordered[6]["key"]) == "ready.state"
        finally
            rm(root, recursive=true)
        end
    end

    @testset "skill pointers are exposed without loading instructions" begin
        mktempdir() do root
            mkpath(joinpath(root,"docs")); mkpath(joinpath(root,"expert"))
            skill=joinpath(root,"expert","SKILL.md")
            write(skill,"---\nname: expert\ndescription: Fixture only\n---\nPRIVATE_SKILL_BODY_SENTINEL\n")
            capsule=joinpath(root,"docs","expert.recur.md")
            write(capsule,"skill.name = expert\nskill.path = expert/SKILL.md\nverify = do-not-execute-this\n")
            before=Dict(skill=>read(skill),capsule=>read(capsule))
            ok,out,_=run_recur(["reveal","expert","-d",root,"--json"])
            @test ok
            parsed=JSON3.read(out)
            fields=vcat(collect(parsed["ordered_fields"]),collect(parsed["extra_fields"]))
            @test any(f->f["key"]=="skill.path" && f["value"]=="expert/SKILL.md",fields)
            @test !occursin("PRIVATE_SKILL_BODY_SENTINEL",out)
            @test all(read(path)==bytes for (path,bytes) in before)
            ok,out,_=run_recur(["reveal","-d",root,"--json"])
            @test ok
            @test length(JSON3.read(out)["entries"])==1
        end
    end

    @testset "reveal falls back cleanly when config is absent" begin
        root = mktempdir()
        try
            mkpath(joinpath(root, "docs"))
            write(
                joinpath(root, "docs", "main.improvement.22.recur.md"),
                """
# main.improvement.22.recur

persona = recur expert
pull.first = recur reveal main.improvement.22
ready.state = I know the next command
""",
            )

            success, output, _ = run_recur(["reveal", "main.improvement.22", "-d", root])
            @test success
            @test contains(output, "Reveal for main.improvement.22")
            @test contains(output, "persona = recur expert")
            @test contains(output, "pull.first = recur reveal main.improvement.22")
        finally
            rm(root, recursive=true)
        end
    end
end
