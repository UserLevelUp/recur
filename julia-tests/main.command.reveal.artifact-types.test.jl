# Acceptance for the frozen shared artifact-types contract.
include("runtests.setup.jl")

function rat_write(root, name, body)
    path = joinpath(root, name)
    mkpath(dirname(path))
    write(path, body)
    path
end

function rat_query(root, args...)
    ok, out, err = run_recur(vcat(["reveal", "-d", root, "--json"], collect(args)))
    @test ok
    ok ? JSON3.read(out) : nothing
end

rat_artifact(entry) = get(entry, "artifact", Dict())
rat_type(entry) = get(rat_artifact(entry), "type", nothing)
rat_status(entry) = get(rat_artifact(entry), "status", "absent")

@testset "reveal artifact types" begin
    @testset "metadata, legacy and diagnostics" begin
        mktempdir() do root
            for kind in ["skill", "persona", "agent", "team.Custom-v1"]
                rat_write(root, "$kind.recur.md", "artifact.type = \"$kind\"\n")
                p = rat_query(root, kind)
                @test rat_type(p) == kind
                @test get(rat_artifact(p), "source", "absent") == "metadata"
            end
            rat_write(root, "legacy.recur.md", "persona = Skippy\nagent = worker\nskill.path = nowhere/SKILL.md\n")
            @test rat_status(rat_query(root, "legacy")) == "untyped"
            for (name, body, expected) in [
                ("empty", "artifact.type =\n", "invalid"),
                ("bad", "artifact.type = not a type\n", "invalid"),
                ("conflict", "artifact.type = skill\nartifact.type = agent\n", "conflict"),
                ("same", "artifact.type = skill\nartifact.type = skill\n", "resolved")]
                rat_write(root, "$name.recur.md", body)
                p = rat_query(root, name)
                @test rat_status(p) == expected
                @test !isempty(get(rat_artifact(p), "diagnostics", []))
            end
            p = rat_query(root)
            @test length(p["entries"]) == 9
            @test all(haskey(e, "artifact") for e in p["entries"])
            lanes = [String(e["lane"]) for e in p["entries"]]
            @test lanes == sort(lanes)
        end
    end

    @testset "prefix policy and filtering" begin
        mktempdir() do root
            config = "[reveal.types]\nprefix_hints = true\n[reveal.types.prefixes]\nskill = \"skill\"\npersona = \"persona\"\n\"skill.team\" = \"team-skill\"\n"
            rat_write(root, ".recur/config.toml", config)
            rat_write(root, "skill.expert.recur.md", "persona = ignored\n")
            rat_write(root, "persona.expert.recur.md", "")
            rat_write(root, "skill.team.expert.recur.md", "")
            rat_write(root, "skill.override.recur.md", "artifact.type = agent\n")
            @test rat_type(rat_query(root, "skill.expert")) == "skill"
            @test rat_type(rat_query(root, "skill.team.expert")) == "team-skill"
            p = rat_query(root, "skill.override")
            @test rat_type(p) == "agent"
            @test !isempty(get(rat_artifact(p), "diagnostics", []))
            p = rat_query(root, "--type", "skill")
            if p !== nothing
                @test [e["lane"] for e in p["entries"]] == ["skill.expert"]
            end
            p = rat_query(root, "expert")
            @test get(p, "status", "absent") == "ambiguous"
            p = rat_query(root, "expert", "--type", "persona")
            if p !== nothing
                @test p["lane"] == "persona.expert"
            end
            p = rat_query(root, "skill.expert", "--type", "persona")
            if p !== nothing
                @test p["status"] == "type-mismatch"
                @test isempty(p["entries"])
            end
            p = rat_query(root, "--type", "absent")
            if p !== nothing
                @test isempty(p["entries"])
            end
            rat_write(root, ".recur/config.toml", replace(config, "true" => "false"))
            @test rat_status(rat_query(root, "skill.expert")) == "untyped"
            @test rat_type(rat_query(root, "skill.override")) == "agent"
        end
    end

    @testset "explicit roots and configured hierarchy" begin
        mktempdir() do root
            rat_write(root, ".recur/config.toml", "[reveal]\nentry_suffix = \".capsule.md\"\n[reveal.types.prefixes]\nskill = \"skill\"\n[private]\ndir = \".recur/private\"\nsep = \"_\"\n")
            inside = joinpath(root, ".recur", "private")
            rat_write(root, ".recur/private/skill_team_expert.capsule.md", "")
            rat_write(root, "sibling/skill.outside.capsule.md", "")
            p = rat_query(inside)
            @test length(p["entries"]) == 1
            @test rat_type(first(p["entries"])) == "skill"
            p = rat_query(inside, "expert")
            @test get(p, "lane", "absent") == "skill_team_expert"
            p = rat_query(inside, "./skill_team_expert.capsule.md")
            @test get(p, "lane", "absent") == "skill_team_expert"
            p = rat_query(inside, "--sep", ".")
            if p !== nothing
                @test rat_status(first(p["entries"])) == "untyped"
            end
        end
    end

    @testset "invalid policy and inert queries" begin
        mktempdir() do root
            rat_write(root, "skill.expert.recur.md", "artifact.type = skill\nverify = touch EXECUTED\nskill.path = secret.md\n")
            rat_write(root, "secret.md", "PRIVATE_BODY_SENTINEL")
            before = Dict(joinpath(root, f) => read(joinpath(root, f)) for f in readdir(root))
            p = rat_query(root, "skill.expert")
            @test !occursin("PRIVATE_BODY_SENTINEL", string(p))
            @test all(read(path) == bytes for (path, bytes) in before)
            @test length(readdir(root)) == length(before)
            for config in [
                "[reveal.types]\nprefix_hints = \"yes\"\n",
                "[reveal.types]\nexecute = \"bad\"\n",
                "[reveal.types.prefixes]\nskill = 42\n",
                "[reveal.types.prefixes]\n\"\" = \"skill\"\n",
                "[reveal.types.prefixes]\nskill = \"bad type\"\n"]
                rat_write(root, ".recur/config.toml", config)
                ok, _, _ = run_recur(["reveal", "-d", root, "--json"])
                @test !ok
            end
        end
    end

    @testset "human output, identity, and selection boundaries" begin
        mktempdir() do root
            rat_write(root, ".recur/config.toml", "[reveal]\nentry_suffix = \".capsule.md\"\nmode = \"fixture-mode\"\n[reveal.types.prefixes]\nskill = \"skill\"\n")
            rat_write(root, "a/skill.expert.capsule.md", "artifact.type = persona\n")
            rat_write(root, "b/skill.expert.capsule.md", "artifact.type = skill\n")
            rat_write(root, "skill.expert.extra.capsule.md", "artifact.type = agent\n")
            rat_write(root, "skillish.capsule.md", "persona = skill\n")
            rat_write(root, "skill.conflict.capsule.md", "artifact.type = agent\nartifact.type = skill\n")
            p = rat_query(root, "skill.expert")
            @test p["status"] == "ambiguous"
            @test p["entry_suffix"] == ".capsule.md"
            @test p["mode"] == "fixture-mode"
            @test length(p["entries"]) == 2
            p = rat_query(root, "skill.expert", "--type", "skill")
            @test p["path"] == "b/skill.expert.capsule.md"
            p = rat_query(root, "./a/skill.expert.capsule.md")
            @test p["path"] == "a/skill.expert.capsule.md"
            p = rat_query(root, "skill.expert", "--type", "agent")
            @test p["status"] == "type-mismatch"
            @test isempty(p["entries"])
            p = rat_query(root, "missing")
            @test p["status"] == "missing"
            @test p["entry_suffix"] == ".capsule.md"
            @test rat_status(rat_query(root, "skillish")) == "untyped"
            p = rat_query(root, "--type", "skill")
            @test [e["path"] for e in p["entries"]] == ["b/skill.expert.capsule.md"]
            for args in [String[], ["a/skill.expert.capsule.md"]]
                ok, out, _ = run_recur(vcat(["reveal", "-d", root], args))
                @test ok
                @test occursin("type: persona (status=resolved, source=metadata)", out)
                @test occursin("Configured prefix hint disagrees", out)
            end
            for args in [["--type", "bad type"], ["--sep", "too-long"]]
                ok, _, _ = run_recur(vcat(["reveal", "-d", root], args))
                @test !ok
            end
        end
    end

    @testset "prefix overlap, custom types, and no implicit defaults" begin
        mktempdir() do root
            rat_write(root, "skill.team.expert.recur.md", "")
            @test rat_status(rat_query(root, "skill.team.expert")) == "untyped"
            rat_write(root, ".recur/config.toml", "[reveal.types.prefixes]\n\"skill.team\" = \"first\"\n\"skill_team\" = \"second\"\n")
            p = rat_query(root, "skill.team.expert", "--sep", ".", "--sep", "_")
            @test rat_status(p) == "conflict"
            @test get(rat_artifact(p), "source", "absent") == "prefix"
            rat_write(root, "skill.team.expert.recur.md", "artifact.type = Team.Custom\n")
            p = rat_query(root, "--type", "Team.Custom", "--sep", ".", "--sep", "_")
            @test length(p["entries"]) == 1
            @test rat_type(first(p["entries"])) == "Team.Custom"
            p = rat_query(root, "--type", "team.custom")
            @test isempty(p["entries"])
            for invalid in ["[reveal.types]\nprefixes = []\n",
                            "[reveal.types.prefixes]\n\"skill..team\" = \"skill\"\n",
                            "[reveal.types]\nprefix_hints = false\n[reveal.types.prefixes]\nskill = 1\n"]
                rat_write(root, ".recur/config.toml", invalid)
                ok, _, _ = run_recur(["reveal", "-d", root])
                @test !ok
            end
        end
    end

    @testset "hierarchical files remain generic query artifacts" begin
        mktempdir() do root
            for name in ["skill.recur.eventness", "persona.skippy", "agent.backend"]
                rat_write(root, ".recur/$name.recur.md", "")
            end
            hidden = joinpath(root, ".recur")
            for args in [["tree", "skill"], ["files", "skill.**"]]
                ok, out, _ = run_recur(vcat(args, ["-d", hidden, "--sep", ".", "--json"]))
                @test ok
                @test occursin("skill.recur.eventness.recur.md", out)
            end
            @test length(rat_query(hidden)["entries"]) == 3
        end
    end

    @testset "nearest nested config, default root and no sibling evidence" begin
        mktempdir() do root
            rat_write(root, ".recur/config.toml", "[reveal.types.prefixes]\nskill = \"parent\"\n")
            rat_write(root, "skill.parent.recur.md", "")
            rat_write(root, "child/.recur/config.toml", "[reveal.types.prefixes]\nskill = \"child\"\n")
            rat_write(root, "child/docs/skill.local.recur.md", "")
            docs = joinpath(root, "child", "docs")
            p = rat_query(docs)
            @test length(p["entries"]) == 1
            @test rat_type(first(p["entries"])) == "child"
            # Bypass run_recur's automatic -d to exercise omitted-directory behavior.
            out = read(Cmd(`$RECUR_BIN reveal --json`, dir=docs), String)
            p = JSON3.read(out)
            @test first(p["entries"])["path"] == "docs/skill.local.recur.md"
            @test rat_type(first(p["entries"])) == "child"
            rat_write(root, "child/docs/escape.recur.md", "warp.root = ..\n")
            ok, _, err = run_recur(["reveal", "escape", "-d", docs])
            @test !ok
            @test occursin("escapes", err)
        end
    end
end
