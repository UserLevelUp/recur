using Test, HTTP, JSON3, Sockets, Unicode
isdefined(@__MODULE__, :MainServer) || include("main.server.jl")

# Hand-derived from main.greeting.recur and dogfood slice-1, before any changes
# to greeting. Expected statuses/messages are independent of production logic.
@testset "Greeting specification boundary table" begin
    cases = [
        ("missing", nothing, nothing, 400, "error", "Invalid name"),
        ("empty", "", "en", 400, "error", "Invalid name"),
        ("whitespace", " \t\n ", "es", 400, "error", "Invalid name"),
        ("trimmed Unicode", "  José  ", "es", 200, "message", "Hola, José!"),
        ("one character", "界", "en", 200, "message", "Hello, 界!"),
        ("40 multibyte characters", repeat("界", 40), "en", 200, "message", "Hello, $(repeat("界", 40))!"),
        ("41 characters", repeat("界", 41), "en", 400, "error", "Invalid name"),
        ("20 combining graphemes", repeat("e\u0301", 20), "en", 200, "message", "Hello, $(repeat("e\u0301", 20))!"),
        ("21 combining graphemes", repeat("e\u0301", 21), "en", 400, "error", "Invalid name"),
        ("default locale", "Joe", nothing, 200, "message", "Hello, Joe!"),
        ("unknown locale", "Joe", "xx", 200, "message", "Hello, Joe!"),
        ("locale exactness", "Joe", "ES", 200, "message", "Hello, Joe!")
    ]
    @test length(repeat("界", 40)) == 40 < sizeof(repeat("界", 40))
    @test length(repeat("e\u0301", 21)) == 42
    @test length(collect(Unicode.graphemes(repeat("e\u0301", 21)))) == 21
    server = MainServer.start(0)
    base = "http://127.0.0.1:$(Int(getsockname(server.listener.server)[2]))"
    try
        for (label, name, locale, status, field, value) in cases
            @testset "$label" begin
                params = String[]
                isnothing(name) || push!(params, "name=" * HTTP.URIs.escapeuri(name))
                isnothing(locale) || push!(params, "locale=" * HTTP.URIs.escapeuri(locale))
                url = base * "/api/greeting?" * join(params, "&")
                result = HTTP.get(url; status_exception=false)
                @test result.status == status
                @test JSON3.read(result.body, Dict{String,Any}) == Dict(field => value)
                @test HTTP.header(result, "Content-Type") == "application/json; charset=utf-8"
                @test HTTP.header(result, "Cache-Control") == "no-store"
                head = HTTP.head(url; status_exception=false)
                @test head.status == status
                @test isempty(head.body)
                @test parse(Int, HTTP.header(head, "Content-Length")) == length(result.body)
                rejected = HTTP.post(url; status_exception=false)
                @test rejected.status == 405
                @test HTTP.header(rejected, "Allow") == "GET, HEAD"
            end
        end
    finally
        close(server)
    end
end
