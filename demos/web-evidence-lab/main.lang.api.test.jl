using Test, HTTP, JSON3, Sockets
isdefined(@__MODULE__, :MainServer) || include("main.server.jl")

# Hand-authored from main.lang.api.contract.md, before adapter/route implementation.
function api_fixture()
    JSON3.read(raw"""
    {"schema":"recur-lang-query-v1","ir_schema":"recur-lang-warp-ir-v1",
     "source":"authored source.recur","source_hash":"fixture:original",
     "header":[
       {"identity":"query.q","meaning":"<img src=x onerror=alert(1)>",
        "input":{"local_identity":"query.i(a)","canonical_identity":"query.i(a)"},
        "output":{"local_identity":"query.o(b)","canonical_identity":"query.o(b)"}},
       {"identity":"other.q","meaning":"A second scoped letter",
        "input":{"local_identity":"other.i(b)","canonical_identity":"query.o(b)"},
        "output":{"local_identity":"other.o(c)","canonical_identity":"other.o(c)"}}],
     "contracts":[{"canonical_identity":"query.o(b)","fields":[{"name":"packet","type_name":"QueryPacket"}]}],
     "body":{"flows":[],"boundary_edges":[{"producer":"query.q","consumer":"other.q"}]},
     "coverage":{"whole_source_validated":false,"excluded":["Runtime behavior"]},
     "footer":{"validation":"findings","execution":"not-run",
       "findings":[{"code":"SGR001","message":"fixture cycle"}],
       "events":[{"scope":"query","recorded":[{"path":"example.complete.md"}]}]},
     "future_data":{"preserve":[1,2,3]}}
    """, Dict{String,Any})
end

@testset "Catalog inspector frozen API" begin
    # Intentional initial red stays standalone until implementation is green.
    implemented = isdefined(MainServer, :MainLangAPI)
    @test implemented
    if implemented
        calls = []
        packet = api_fixture()
        original = deepcopy(packet)
        adapter = args -> begin
            push!(calls, args)
            (code=0, stdout=JSON3.write(packet), stderr="")
        end
        request(path; method="GET", boundary=adapter) = MainServer.handler(
            HTTP.Request(method, path); lang_adapter=boundary, lang_binary="fake recur with spaces")
        decode(r) = JSON3.read(r.body, Dict{String,Any})
        result = request("/api/lang")
        @test result.status == 200
        @test isempty(calls)
        catalog = decode(result)
        @test catalog["selection"] === nothing
        @test [c["id"] for c in catalog["catalog"]] == ["greeting", "inspector"]
        @test catalog["catalog"][1]["scopes"] == ["greeting", "greeting.g"]
        @test catalog["catalog"][2]["scopes"] == ["query", "query.q", "model", "model.m", "view", "view.v"]
        for (path, status, code) in [
            ("?id=missing&scope=greeting",404,"unknown-capability"),
            ("?id=greeting&scope=q",400,"unknown-scope"),
            ("?id=greeting",400,"invalid-selection"),
            ("?scope=greeting",400,"invalid-selection"),
            ("?id=&scope=greeting",400,"invalid-selection"),
            ("?id=greeting&scope=",400,"invalid-selection"),
            ("?id=greeting&id=inspector&scope=greeting",400,"invalid-selection"),
            ("?id=greeting&scope=greeting&scope=greeting.g",400,"invalid-selection"),
            ("?binary=evil",400,"invalid-selection"),
            ("?id=greeting&scope=greeting&root=..",400,"invalid-selection"),
            ("?id=..%2Fsecret&scope=greeting",404,"unknown-capability"),
            ("?id=greeting&scope=greeting.g%3Bwhoami",400,"unknown-scope")]
            r = request("/api/lang" * path)
            @test r.status == status
            @test Set(keys(decode(r))) == Set(["error"])
            @test decode(r)["error"]["code"] == code
            @test Set(keys(decode(r)["error"])) == Set(["code", "message"])
        end
        @test isempty(calls)
        r = request("/api/lang?id=inspector&scope=model.m")
        @test r.status == 200
        @test only(calls) == ["fake recur with spaces", "lang", "report", "main.lang.inspector.recur", "--scope", "model.m", "-d", normpath(joinpath(@__DIR__, "..", "lang-inspector")), "--json"]
        data = decode(r)
        @test data["packet"] == original == packet
        @test data["selection"] == Dict("id"=>"inspector", "scope"=>"model.m")
        @test [c["identity"] for c in data["cards"]] == ["query.q", "other.q"]
        @test data["cards"][2]["input_canonical"] == "query.o(b)"
        @test isempty(data["observed_evidence"])
        for value in ["other.i(b) = query.o(b)", "query.q -> other.q", "SGR001", "fixture cycle", "example.complete.md", "Runtime behavior", "not-run", "<img src=x onerror=alert(1)>"]
            @test occursin(value, data["text"])
        end
        @test HTTP.header(r, "Content-Type") == "application/json; charset=utf-8"
        @test HTTP.header(r, "Cache-Control") == "no-store"
        empty!(packet["header"])
        @test occursin("No functions selected", decode(request("/api/lang?id=greeting&scope=greeting.g"))["text"])
        @test last(calls) == ["fake recur with spaces", "lang", "report", "main.greeting.recur", "--scope", "greeting.g", "-d", normpath(@__DIR__), "--json"]
        for field in ["coverage", "source_hash", "footer", "body", "contracts"]
            malformed = api_fixture(); delete!(malformed, field)
            failure = request("/api/lang?id=greeting&scope=greeting"; boundary=_ -> (code=0, stdout=JSON3.write(malformed), stderr=""))
            @test failure.status == 502
            @test decode(failure)["error"]["code"] == "query-failed"
        end
        malformed = api_fixture(); malformed["contracts"][1]["fields"] = [Dict("oops"=>true)]
        cir = api_fixture(); cir["ir_schema"] = "recur-lang-concurrent-ir-v1"
        for stdout in ["not json", "[]", JSON3.write(cir), JSON3.write(malformed)]
            failure = request("/api/lang?id=greeting&scope=greeting"; boundary=_ -> (code=0, stdout=stdout, stderr=""))
            @test failure.status == 502
            @test decode(failure)["error"]["code"] == "query-failed"
        end
        failure = request("/api/lang?id=greeting&scope=greeting"; boundary=_ -> (code=2, stdout="LANG003 original diagnostic", stderr="detail from CLI"))
        @test failure.status == 502
        @test occursin("LANG003 original diagnostic", decode(failure)["error"]["message"])
        @test occursin("detail from CLI", decode(failure)["error"]["message"])
        @test request("/api/lang?id=greeting&scope=greeting"; boundary=_ -> error("spawn failed")).status == 502
        empty!(calls)
        @test request("/api/lang"; method="POST").status == 405
        @test isempty(calls)
        head = request("/api/lang"; method="HEAD")
        @test head.status == 200
        @test isempty(head.body)
        @test parse(Int, HTTP.header(head, "Content-Length")) > 0
        @test request("/lang.html").status == 200
    end
end

@testset "Catalog inspector real loopback and read boundary" begin
    if isdefined(MainServer, :MainLangAPI)
        roots = [@__DIR__, joinpath(@__DIR__, "..", "lang-inspector")]
        snapshot() = Dict(joinpath(dir, f) => read(joinpath(dir, f)) for root in roots
            for (dir, _, files) in walkdir(root) for f in files)
        before = snapshot()
        config = read(joinpath(@__DIR__, "..", "..", ".recur", "config.toml"))
        server = MainServer.start(0)
        base = "http://127.0.0.1:$(Int(getsockname(server.listener.server)[2]))"
        try
            get(path) = HTTP.get(base * path; status_exception=false)
            @test get("/lang.html").status == 200
            @test get("/api/lang?id=missing&scope=query").status == 404
            for (id, scopes) in [("greeting", ["greeting", "greeting.g"]), ("inspector", ["query", "query.q", "model", "model.m", "view", "view.v"])]
                for scope in scopes
                    r = get("/api/lang?id=$id&scope=$scope")
                    @test r.status == 200
                    data = JSON3.read(r.body, Dict{String,Any})
                    @test data["packet"]["footer"]["execution"] == "not-run"
                    @test !data["packet"]["coverage"]["whole_source_validated"]
                    @test startswith(data["packet"]["source_hash"], "fnv1a64:")
                    @test isempty(data["observed_evidence"])
                    @test length(data["cards"]) == 1
                end
            end
            @test isempty(HTTP.head(base * "/api/lang?id=greeting&scope=greeting").body)
        finally
            close(server)
        end
        @test snapshot() == before
        @test read(joinpath(@__DIR__, "..", "..", ".recur", "config.toml")) == config
    end
end
