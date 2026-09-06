# Included by Generator: domain validation and additive playable package v1.
using SHA: sha256

function valid_board(grid::Matrix{Int})
    size(grid) == (9, 9) || return false
    all(v -> 0 <= v <= 9, grid) || return false
    for r in 1:9, c in 1:9
        v = grid[r,c]
        v == 0 && continue
        any(grid[pr,pc] == v for (pr,pc) in all_peers(r,c)) && return false
    end
    return true
end

"""Count up to two solutions. Exceeding a node budget is a failure, never uniqueness."""
function count_solutions(grid::Matrix{Int}; node_budget::Int=100_000)
    valid_board(grid) || return 0
    work = copy(grid)
    nodes = Ref(0)
    function visit()
        nodes[] += 1
        nodes[] <= node_budget || error("Solution-count node budget exceeded")
        best = nothing
        choices = collect(1:10)
        for r in 1:9, c in 1:9
            work[r,c] == 0 || continue
            values = candidates(work,r,c)
            isempty(values) && return 0
            if length(values) < length(choices)
                best = (r,c); choices = values
            end
        end
        best === nothing && return 1
        total = 0
        for value in choices
            work[best...] = value
            total += visit()
            total >= 2 && break
        end
        work[best...] = 0
        return min(total,2)
    end
    return visit()
end

"""Deterministic naked-single solve trace; stalled means ungraded, not 'hard'."""
function grade_playable(grid::Matrix{Int})
    valid_board(grid) || error("Invalid board")
    work = copy(grid)
    steps = []
    while any(==(0),work)
        moved = false
        for r in 1:9, c in 1:9
            values = candidates(work,r,c)
            if length(values) == 1
                push!(steps, Dict("cell"=>cell_id(r,c), "value"=>only(values), "technique"=>"naked-single"))
                work[r,c] = only(values); moved = true; break
            end
        end
        moved || break
    end
    return Dict("rubric"=>"naked-singles-v1", "label"=>any(==(0),work) ? "ungraded" : "naked-single-solvable",
        "steps"=>steps, "remaining"=>count(==(0),work))
end

"""One bounded removal pass. Presets specify target gaps, not technique difficulty."""
function make_playable(solution::Matrix{Int}, preset::String; node_budget::Int=100_000, removal_budget::Int=81)
    targets = Dict("easy"=>25,"medium"=>35,"hard"=>45)
    haskey(targets,preset) || error("Unknown preset")
    valid_board(solution) && all(!=(0),solution) || error("Invalid solution")
    grid = copy(solution)
    cells = shuffle!([(r,c) for r in 1:9 for c in 1:9])
    for (attempt,(r,c)) in enumerate(cells)
        count(==(0),grid) >= targets[preset] && break
        attempt <= removal_budget || error("Removal budget exceeded")
        old = grid[r,c]; grid[r,c] = 0
        count_solutions(grid;node_budget) == 1 || (grid[r,c] = old)
    end
    count(==(0),grid) == targets[preset] || error("Could not reach gap target in bounded pass")
    return Dict("givens"=>[collect(grid[r,:]) for r in 1:9], "grade"=>grade_playable(grid), "gaps"=>targets[preset])
end

"""Generate off to the side; publish one complete JSON via same-directory rename.
Legacy files are left untouched. Failure leaves the previous playable package intact.
"""
function publish_playable(output_dir::String, recur_module; solution=generate_solution(), node_budget::Int=100_000, removal_budget::Int=81)
    presets = Dict(key=>make_playable(solution,key;node_budget,removal_budget) for key in ("easy","medium","hard"))
    mkpath(output_dir)
    return mktempdir(output_dir; prefix=".teaching-stage-") do stage
        # Explicit domain-authored keywords, even when tests generate outside the demo.
        mkpath(joinpath(stage,".recur"))
        write(joinpath(stage,".recur","config.toml"), """
        [traits.trace_id]
        enabled = true
        producer_keywords = "publish,emit,propagate"
        consumer_keywords = "subscribe,bind,consume"
        trigger_keywords = "trigger,register,solve"
        """)
        pairs = solution_to_pairs(solution)
        solution_path = write_solution_file(solution,stage)
        # Author the entire relationship graph before querying any cell.
        for r in 1:9, c in 1:9
            write_flow_event(r,c,solution[r,c],stage)
        end
        path = generate_cascades(pairs,stage,recur_module;save_run=false,reuse_if_fresh=false)
        cascades = JSON3.read(read(path,String))
        length(cascades) == 81 || error("Incomplete cascade package")
        solution_text = read(solution_path,String)
        identity = bytes2hex(sha256(solution_text * JSON3.write(presets)))
        package = Dict("schema"=>"sudoku-playable-v1", "puzzle_id"=>identity,
            "solution_text"=>solution_text, "cascades"=>cascades, "presets"=>presets)
        pending = joinpath(stage,"sudoku.playable.json")
        write(pending,JSON3.write(package))
        destination = joinpath(output_dir,"sudoku.playable.json")
        Base.Filesystem.rename(pending,destination)
        return package
    end
end
