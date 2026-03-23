"""
Display.jl — Terminal rendering for the Sudoku demo.

Knows nothing about Sudoku rules or recur internals.
Receives data structures and renders them as strings.
"""

module Display

# ---------------------------------------------------------------------------
# Grid rendering
# ---------------------------------------------------------------------------

"""
    render_grid(grid) -> String

Render a 9x9 grid as a multi-line ASCII board with box separators.
Empty cells (nothing) are shown as '.'.
"""
function render_grid(grid::Matrix{Union{Int,Nothing}})::String
    lines = String[]
    push!(lines, "+-------+-------+-------+")
    for row in 1:9
        cells = String[]
        for col in 1:9
            v = grid[row, col]
            push!(cells, v === nothing ? "." : string(v))
        end
        row_str = "| $(cells[1]) $(cells[2]) $(cells[3]) | $(cells[4]) $(cells[5]) $(cells[6]) | $(cells[7]) $(cells[8]) $(cells[9]) |"
        push!(lines, row_str)
        if row in (3, 6)
            push!(lines, "+-------+-------+-------+")
        end
    end
    push!(lines, "+-------+-------+-------+")
    return join(lines, "\n")
end

# ---------------------------------------------------------------------------
# Cascade rendering
# ---------------------------------------------------------------------------

"""
    render_cascade(cell_id, value, cascade) -> String

Format a trace-id cascade result for terminal output.
Shows the placed value and role counts (define/produce/consume/trigger).
"""
function render_cascade(cell_id::String, value::Int, cascade)::String
    define  = get(cascade, :define,  get(cascade, "define",  []))
    produce = get(cascade, :produce, get(cascade, "produce", []))
    consume = get(cascade, :consume, get(cascade, "consume", []))
    trigger = get(cascade, :trigger, get(cascade, "trigger", []))

    lines = String[
        "",
        "  Cascade: $(cell_id) = $(value)",
        "  ─────────────────────────────────",
        "  define  $(length(define))  site(s)",
        "  produce $(length(produce))  constraint(s) published",
        "  consume $(length(consume))  peer(s) notified",
        "  trigger $(length(trigger))  solver(s) activated",
        "",
    ]
    return join(lines, "\n")
end

end # module
