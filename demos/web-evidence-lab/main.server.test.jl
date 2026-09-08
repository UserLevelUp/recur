using Test, HTTP, JSON3, Sockets
include("main.server.jl")

@testset "main.server Julia HTTP contract" begin
    server = MainServer.start(0)
    port = Int(getsockname(server.listener.server)[2])
    base = "http://127.0.0.1:$(port)"
    get(path) = HTTP.get(base * path; status_exception=false)
    try
        @test get("/").status == 200
        @test occursin("Ten hellos", String(get("/").body))
        runtime = JSON3.read(get("/api/server").body)
        @test runtime.runtime == "Julia"
        @test runtime.version == string(VERSION)
        @test runtime.project == "main"
        @test HTTP.header(get("/main.report.html"), "Cache-Control") == "no-store"
        @test HTTP.header(get("/apps/10/main.js"), "Content-Type") == "text/javascript; charset=utf-8"
        for level in 1:10
            @test get("/apps/$(lpad(level, 2, '0'))/main.html").status == 200
        end
        @test JSON3.read(get("/api/greeting?name=Joe").body).message == "Hello, Joe!"
        @test JSON3.read(get("/api/greeting?name=%20Jos%C3%A9%20&locale=es").body).message == "Hola, José!"
        @test JSON3.read(get("/api/greeting?name=Joe&locale=xx").body).message == "Hello, Joe!"
        @test get("/api/greeting?name=").status == 400
        @test get("/api/greeting?name=" * repeat("x", 41)).status == 400
        @test HTTP.head(base * "/main.report.html").status == 200
        @test isempty(HTTP.head(base * "/main.report.html").body)
        @test HTTP.post(base * "/api/greeting"; status_exception=false).status == 405
        for path in ["/.recur/config.toml", "/main.server.jl", "/Project.toml", "/apps/", "/missing.html",
                     "/%2e%2e/main.report.html", "/apps/%2e%2e/main.report.html", "/C%3a/secret.md"]
            @test get(path).status == 404
        end
    finally
        close(server)
    end
end
