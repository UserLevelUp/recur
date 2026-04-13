"""
Recur.jl - thin Julia wrapper around recur subprocess calls.

recur does not know what Sudoku is. This module knows how to call
recur and parse its JSON output. The game engine uses these results
to drive UI, propagation, and hints.
"""

module Recur

using JSON3

function default_recur_bin()
    env_bin = get(ENV, "RECUR_BIN", nothing)
    env_bin !== nothing && return env_bin

    profile = get(ENV, "RECUR_PROFILE", "release-safe")
    repo_bin = normpath(
        joinpath(
            @__DIR__,
            "..", "..", "..",
            "target",
            profile,
            "recur" * (Sys.iswindows() ? ".exe" : ""),
        ),
    )
    return isfile(repo_bin) ? repo_bin : "recur"
end

# Locate recur binary: ENV override -> repo-local build -> PATH fallback.
const RECUR_BIN = default_recur_bin()

"""
    trace_id(identifier; dir, scope, ext, depth, save_run, reuse_if_fresh, run_name) -> NamedTuple

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
    save_run::Bool = false,
    reuse_if_fresh::Bool = false,
    run_name::Union{Nothing,String} = nothing,
)
    resolved_run_name = isnothing(run_name) && (save_run || reuse_if_fresh) ? identifier : run_name
    args = [
        RECUR_BIN, "trace-id", identifier,
        "--scope", scope,
        "--ext", ext,
        "--depth", string(depth),
        "--json",
        "-d", dir,
    ]

    save_run && push!(args, "--save-run")
    reuse_if_fresh && push!(args, "--reuse-if-fresh")
    !isnothing(resolved_run_name) && append!(args, ["--run-name", resolved_run_name])

    out = IOBuffer()
    err = IOBuffer()

    try
        run(pipeline(`$args`, stdout=out, stderr=err))
    catch e
        isa(e, ProcessFailedException) || rethrow()
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
