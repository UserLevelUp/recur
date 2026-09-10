using Test, JSON3

@testset "Lang inspector: specification before implementation" begin
    implementation = joinpath(@__DIR__, "..", "demos", "lang-inspector", "main.lang.inspector.jl")
    @test isfile(implementation)
    if isfile(implementation)
        include(implementation)
        # Hand-authored data fixture from the specified CLI boundary, not from
        # inspector implementation output. Contract aliases must stay distinct.
        fixture() = JSON3.read("""
        {"schema":"recur-lang-query-v1","ir_schema":"recur-lang-warp-ir-v1",
         "source":"sample.recur","source_hash":"fixture:1",
         "header":[{"identity":"model.m","meaning":"Build a view",
           "input":{"local_identity":"model.i(b)","canonical_identity":"query.o(b)"},
           "output":{"local_identity":"model.o(c)","canonical_identity":"model.o(c)"}}],
         "contracts":[], "body":{"boundary_edges":[{"producer":"query.q","consumer":"model.m"}]},
         "coverage":{"whole_source_validated":false,"excluded":["Runtime behavior"]},
         "footer":{"validation":"findings","execution":"not-run",
           "findings":[{"code":"SGR001","message":"dependency cycle"}],
           "events":[{"recorded":[{"path":"example.complete.md"}]}]}}
        """, Dict{String,Any})

        @testset "model.m and view.v contracts" begin
            packet = fixture()
            original = deepcopy(packet)
            view = LangInspector.build_view(packet)
            @test view.packet == original
            @test packet == original
            @test only(view.cards).identity == "model.m"
            @test only(view.cards).input_local == "model.i(b)"
            @test only(view.cards).input_canonical == "query.o(b)"
            rendered = LangInspector.render_view(view)
            @test rendered == LangInspector.render_view(view)
            for expected in ("model.m", "Build a view", "model.i(b) = query.o(b)",
                             "query.q -> model.m", "SGR001", "dependency cycle",
                             "Runtime behavior", "not-run", "example.complete.md",
                             "Whole source validated: false", "Recorded state (not acceptance)")
                @test occursin(expected, rendered)
            end
            empty_packet = fixture()
            empty!(empty_packet["header"])
            @test occursin("No functions selected", LangInspector.render_view(LangInspector.build_view(empty_packet)))
            second = deepcopy(packet["header"][1])
            second["identity"] = "view.v"
            push!(packet["header"], second)
            @test [c.identity for c in LangInspector.build_view(packet).cards] == ["model.m", "view.v"]
            for (field, value) in (("schema", "future"), ("ir_schema", "recur-lang-concurrent-ir-v1"))
                invalid = fixture(); invalid[field] = value
                @test_throws ArgumentError LangInspector.build_view(invalid)
            end
            invalid = fixture(); delete!(invalid, "coverage")
            @test_throws ArgumentError LangInspector.build_view(invalid)
        end

        @testset "query.q mock boundary" begin
            calls = []
            adapter = args -> begin
                push!(calls, args)
                (code=0, stdout=JSON3.write(fixture()), stderr="")
            end
            packet = LangInspector.query_report("fake recur", "root with spaces", "sample.recur", "model.m"; adapter)
            @test packet["source"] == "sample.recur"
            @test only(calls) == ["fake recur", "lang", "report", "sample.recur", "--scope", "model.m", "-d", "root with spaces", "--json"]
            @test_throws ArgumentError LangInspector.query_report("unused", ".", "bad", "m";
                adapter=_ -> (code=2, stdout="LANG003 unknown scope", stderr=""))
            @test_throws ArgumentError LangInspector.query_report("unused", ".", "bad", "m";
                adapter=_ -> (code=0, stdout="not json", stderr=""))
        end

        @testset "real Recur inspects its inspector specification" begin
            binary = get(ENV, "RECUR_BIN", joinpath(@__DIR__, "..", "target", "release-safe", "recur" * (Sys.iswindows() ? ".exe" : "")))
            root = joinpath(@__DIR__, "..", "demos", "lang-inspector")
            before = Dict(name => read(joinpath(root, name)) for name in readdir(root) if isfile(joinpath(root, name)))
            packet = LangInspector.query_report(binary, root, "main.lang.inspector.recur", "model.m")
            view = LangInspector.build_view(packet)
            @test only(view.cards).input_canonical == "query.o(b)"
            @test only(view.cards).identity == "model.m"
            @test packet["footer"]["execution"] == "not-run"
            @test !packet["coverage"]["whole_source_validated"]
            @test occursin("model.i(b) = query.o(b)", LangInspector.render_view(view))
            @test before == Dict(name => read(joinpath(root, name)) for name in readdir(root) if isfile(joinpath(root, name)))
        end
    end
end
