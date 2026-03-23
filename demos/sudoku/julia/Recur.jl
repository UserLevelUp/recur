"""
Recur.jl — thin Julia wrapper around recur subprocess calls.

recur does not know what Sudoku is. This module knows how to call
recur and parse its JSON output. The game engine uses these results
to drive UI, propagation, and hints.
"""

module Recur

using JSON3

# Locate recur binary: ENV override → PATH fallback
const RECUR_BIN = get(ENV, "RECUR_BIN", "recur")

"""
    trace_id(identifier; dir, scope, ext, depth) -> NamedTuple

Call `recur trace-id` on the given identifier and return parsed JSON.
Returns a NamedTuple with fields: identifier, define, produce, consume, trigger, request.
Returns nothing on failure.
"""
function trace_id(
    identifier::String;
    dir::String = ".",
    scope::String = "sudoku.**",
    ext::String = ".txt",
    depth::Int = 2,
)
    args = [
        RECUR_BIN, "trace-id", identifier,
        "--scope", scope,
        "--ext", ext,
        "--depth", string(depth),
        "--json",
        "-d", dir,
    ]

    out = IOBuffer()
    err = IOBuffer()
    success = true

    try
        run(pipeline(`$args`, stdout=out, stderr=err))
    catch e
        isa(e, ProcessFailedException) && (success = false)
    end

    output = String(take!(out))
    isempty(output) && return nothing

    try
        return JSON3.read(output)
    catch
        return nothing
    end
end

"""
    files(pattern; dir) -> Vector{String}

Call `recur files` and return matching paths as a string vector.
"""
function files(pattern::String; dir::String = ".")
    args = [RECUR_BIN, "files", pattern, "-d", dir]
    out = IOBuffer()
    try
        run(pipeline(`$args`, stdout=out, stderr=devnull))
    catch
        return String[]
    end
    output = String(take!(out))
    isempty(output) && return String[]
    try
        return [String(p) for p in JSON3.read(output)]
    catch
        return String[]
    end
end

end # module
