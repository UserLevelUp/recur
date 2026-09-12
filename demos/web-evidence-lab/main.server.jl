"""Julia development hub for main: static demos, reports, and greeting API."""
module MainServer

using HTTP, JSON3, Sockets
include("main.lang.api.jl")

const ROOT = realpath(@__DIR__)
const MIME = Dict(
    ".html" => "text/html; charset=utf-8", ".css" => "text/css; charset=utf-8",
    ".js" => "text/javascript; charset=utf-8", ".json" => "application/json; charset=utf-8",
    ".md" => "text/plain; charset=utf-8", ".png" => "image/png",
)

function response(status, body, mime="text/plain; charset=utf-8")
    HTTP.Response(status, ["Content-Type" => mime, "Cache-Control" => "no-store",
        "X-Content-Type-Options" => "nosniff", "X-Main-Server" => "Julia/$(VERSION)"], body)
end

# produces: main.greeting.api.response validated JSON greeting
function greeting(uri)
    query = HTTP.URIs.queryparams(uri)
    name = strip(get(query, "name", ""))
    locale = get(query, "locale", "en")
    valid = 0 < length(name) <= 40
    value = valid ? (message="$(locale == "es" ? "Hola" : "Hello"), $(name)!",) : (error="Invalid name",)
    response(valid ? 200 : 400, JSON3.write(value), MIME[".json"])
end

function static_file(path)
    # Only public demo/report artifacts are served; no directory listings or source runners.
    parts = split(path, '/'; keepempty=false)
    any(part -> part in (".", "..") || startswith(part, ".") || occursin(r"[\\:\x00]", part), parts) &&
        return response(404, "Not found")
    isempty(parts) && return response(404, "Not found")
    file = joinpath(ROOT, parts...)
    isfile(file) || return response(404, "Not found")
    resolved = realpath(file)
    relative = relpath(resolved, ROOT)
    (isabspath(relative) || first(splitpath(relative)) == "..") && return response(404, "Not found")
    extension = lowercase(splitext(resolved)[2])
    haskey(MIME, extension) || return response(404, "Not found")
    response(200, read(resolved), MIME[extension])
end

function handler(request::HTTP.Request; lang_adapter=MainLangAPI.LangInspector.capture_query,
                 lang_binary=MainLangAPI.DEFAULT_BINARY)
    request.method in ("GET", "HEAD") || return HTTP.Response(405, ["Allow" => "GET, HEAD"], "Method not allowed")
    result = try
        uri = HTTP.URI(request.target)
        path = HTTP.URIs.unescapeuri(uri.path)
        if path == "/api/greeting"
            greeting(uri)
        elseif path == "/api/lang"
            MainLangAPI.respond(uri; binary=lang_binary, adapter=lang_adapter)
        elseif path == "/api/server"
            response(200, JSON3.write((runtime="Julia", version=string(VERSION), project="main")), MIME[".json"])
        else
            static_file(path == "/" ? "/main.report.html" : path)
        end
    catch error
        if error isa ArgumentError
            response(400, "Invalid request")
        else
            @error "main.server request failed" exception=(error, catch_backtrace())
            response(500, "Request failed")
        end
    end
    if request.method == "HEAD"
        HTTP.setheader(result, "Content-Length" => string(length(result.body)))
        result.body = UInt8[]
    end
    result
end

function start(port=8765)
    HTTP.serve!(handler, ip"127.0.0.1", port; verbose=false)
end

function main(args=ARGS)
    port = 8765
    if !isempty(args)
        length(args) == 2 && args[1] == "--port" || error("Usage: main.server.jl [--port PORT]")
        port = parse(Int, args[2])
    end
    0 <= port <= 65535 || error("Port must be between 0 and 65535")
    server = start(port)
    actual_port = Int(getsockname(server.listener.server)[2])
    println("MAIN_SERVER_READY http://127.0.0.1:$(actual_port) Julia/$(VERSION)")
    flush(stdout)
    try
        wait(server)
    finally
        close(server)
    end
end

end # module

abspath(PROGRAM_FILE) == (@__FILE__) && MainServer.main()
