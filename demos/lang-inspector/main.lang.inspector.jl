module LangInspector

using JSON3

"A presentation model inferred from the Lang contracts; it does not execute Lang."
struct FunctionCard
    identity::String
    meaning::String
    input_local::String
    input_canonical::String
    output_local::String
    output_canonical::String
end

struct InspectorView
    cards::Vector{FunctionCard}
    packet::Dict{String,Any}
end

function capture_query(args)
    out, err = IOBuffer(), IOBuffer()
    process = run(pipeline(ignorestatus(Cmd(args)), stdout=out, stderr=err))
    (code=process.exitcode, stdout=String(take!(out)), stderr=String(take!(err)))
end

"query.q: the only process boundary, replaceable with a fixture adapter."
function query_report(binary, root, source, scope; adapter=capture_query)
    args = String[binary, "lang", "report", source, "--scope", scope, "-d", root, "--json"]
    result = adapter(args)
    result.code == 0 || throw(ArgumentError("Recur query failed ($(result.code)): $(result.stdout) $(result.stderr)"))
    packet = try
        JSON3.read(result.stdout, Dict{String,Any})
    catch
        throw(ArgumentError("Recur query did not return a JSON object"))
    end
    build_view(packet) # Validate the supported boundary before passing data on.
    packet
end

"model.m: retain evidence verbatim and derive only the displayed function cards."
function build_view(packet::Dict{String,Any})
    get(packet, "schema", nothing) == "recur-lang-query-v1" ||
        throw(ArgumentError("Expected recur-lang-query-v1"))
    get(packet, "ir_schema", nothing) == "recur-lang-warp-ir-v1" ||
        throw(ArgumentError("This demo supports WIR1 function contracts only"))
    try
        # Required display fields must exist; absent evidence is not inferred.
        String(packet["source"]); String(packet["source_hash"])
        packet["contracts"] isa AbstractVector || error("contracts must be an array")
        packet["coverage"]["whole_source_validated"] isa Bool || error("coverage must be boolean")
        packet["coverage"]["excluded"] isa AbstractVector || error("excluded must be an array")
        packet["body"]["boundary_edges"] isa AbstractVector || error("boundaries must be an array")
        String(packet["footer"]["execution"]); String(packet["footer"]["validation"])
        packet["footer"]["findings"] isa AbstractVector || error("findings must be an array")
        packet["footer"]["events"] isa AbstractVector || error("events must be an array")
        cards = FunctionCard[
            FunctionCard(h["identity"], h["meaning"],
                h["input"]["local_identity"], h["input"]["canonical_identity"],
                h["output"]["local_identity"], h["output"]["canonical_identity"])
            for h in packet["header"]
        ]
        InspectorView(cards, deepcopy(packet))
    catch err
        throw(ArgumentError("Incomplete or malformed WIR1 query packet: $(sprint(showerror, err))"))
    end
end

contract_label(local_id, canonical) = local_id == canonical ? local_id : "$local_id = $canonical"

"view.v: plain text, preserving the distinction between evidence and acceptance."
function render_view(view::InspectorView)
    packet = view.packet
    io = IOBuffer()
    println(io, "Lang inspector: ", packet["source"])
    println(io, "Source revision: ", packet["source_hash"])
    println(io, "\nHEADER")
    isempty(view.cards) && println(io, "No functions selected")
    for card in view.cards
        println(io, card.identity, " — ", card.meaning)
        println(io, "  in:  ", contract_label(card.input_local, card.input_canonical))
        println(io, "  out: ", contract_label(card.output_local, card.output_canonical))
    end
    for contract in packet["contracts"]
        fields = join((string(f["name"], ": ", f["type_name"]) for f in contract["fields"]), ", ")
        println(io, "  ", contract["canonical_identity"], " := (", fields, ")")
    end
    println(io, "\nBODY — boundary references")
    for edge in packet["body"]["boundary_edges"]
        println(io, "  ", edge["producer"], " -> ", edge["consumer"])
    end
    println(io, "\nFOOTER")
    println(io, "Validation: ", packet["footer"]["validation"])
    println(io, "Execution: ", packet["footer"]["execution"])
    println(io, "Whole source validated: ", packet["coverage"]["whole_source_validated"])
    for excluded in packet["coverage"]["excluded"]
        println(io, "  Outside coverage: ", excluded)
    end
    for finding in packet["footer"]["findings"]
        println(io, "  Finding: ", JSON3.write(finding))
    end
    println(io, "Recorded state (not acceptance):")
    for event in packet["footer"]["events"]
        recorded = get(event, "recorded", [])
        println(io, "  ", get(event, "scope", "selection"), ": ", isempty(recorded) ? "none" : JSON3.write(recorded))
    end
    String(take!(io))
end

function main(args=ARGS)
    length(args) == 3 || throw(ArgumentError("Usage: julia main.lang.inspector.jl ROOT SOURCE SCOPE; set RECUR_BIN to a Lang-capable binary"))
    binary = get(ENV, "RECUR_BIN", "recur")
    print(render_view(build_view(query_report(binary, args...))))
end

end

if abspath(PROGRAM_FILE) == @__FILE__
    LangInspector.main()
end
