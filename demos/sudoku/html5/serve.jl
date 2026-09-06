"""
serve.jl — Local dev server for the HTML5 Sudoku demo.

Usage: julia demos/sudoku/html5/serve.jl
Then open: http://localhost:8787

Serves static files AND a /api/generate endpoint that creates
new puzzles on demand using Julia + recur.

Security: only predefined API endpoints are exposed.
No user-supplied code is ever executed.
"""

using HTTP
using JSON3

const DIR = @__DIR__
const PORT = parse(Int, get(ENV, "SUDOKU_PORT", "8787"))

# Load Generator + Recur modules (one-time cost at startup)
const JULIA_DIR = normpath(joinpath(DIR, "..", "julia"))
include(joinpath(JULIA_DIR, "Recur.jl"))
include(joinpath(JULIA_DIR, "Generator.jl"))
using .Recur
using .Generator

const MIME_TYPES = Dict(
    ".html" => "text/html",
    ".css"  => "text/css",
    ".js"   => "application/javascript",
    ".json" => "application/json",
    ".txt"  => "text/plain",
)

const DATA_DIR = get(ENV, "SUDOKU_DATA_DIR", joinpath(DIR, "data", "easy-001"))

function handler(req)
    # Strip query string first, then handle root
    path_clean = split(req.target, "?")[1]
    if path_clean == "/"
        path_clean = "/index.html"
    end

    # ── API endpoints ──────────────────────────────────────────────
    if path_clean == "/api/generate" && req.method == "POST"
        return handle_generate()
    end

    # ── Static file serving ────────────────────────────────────────
    filepath = path_clean == "/data/easy-001/sudoku.playable.json" ? joinpath(DATA_DIR,"sudoku.playable.json") : joinpath(DIR, lstrip(path_clean, '/'))

    if !isfile(filepath)
        return HTTP.Response(404, "Not found: $path_clean")
    end

    ext = lowercase(last(splitext(filepath)))
    mime = get(MIME_TYPES, ext, "application/octet-stream")
    body = read(filepath)
    return HTTP.Response(200, ["Content-Type" => mime, "Access-Control-Allow-Origin" => "*"], body)
end

"""
Generate a new random puzzle: solution → flow events → recur trace-id → cascades JSON.
Publishes one complete sudoku.playable.json; legacy files remain unchanged.
"""
function handle_generate()
    try
        println("  [API] Generating new puzzle...")
        t0 = time()

        package = Generator.publish_playable(DATA_DIR, Recur)
        response = JSON3.write(merge(package, Dict("status"=>"ok", "elapsed_ms"=>round(Int,(time()-t0)*1000))))

        println("  [API] Done in $(round(time() - t0, digits=2))s")
        return HTTP.Response(200, [
            "Content-Type" => "application/json",
            "Access-Control-Allow-Origin" => "*",
        ], response)

    catch e
        msg = sprint(showerror, e)
        println("  [API] Error: $msg")
        return HTTP.Response(500, [
            "Content-Type" => "application/json",
        ], JSON3.write(Dict("status"=>"error", "message"=>msg)))
    end
end

println("Serving at http://localhost:$PORT")
println("API: POST /api/generate — creates a new random puzzle")
println("Open that URL in your browser, then Ctrl+C to stop.")
HTTP.serve(handler, "127.0.0.1", PORT)
