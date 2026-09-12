module MainLangAPI

using HTTP, JSON3
include(joinpath(@__DIR__, "..", "lang-inspector", "main.lang.inspector.jl"))

const CATALOG = (
    (id="greeting", label="Greeting boundary", root=normpath(@__DIR__),
     source="main.greeting.recur", scopes=["greeting", "greeting.g"]),
    (id="inspector", label="Lang inspector", root=normpath(joinpath(@__DIR__, "..", "lang-inspector")),
     source="main.lang.inspector.recur", scopes=["query", "query.q", "model", "model.m", "view", "view.v"]),
)
const DEFAULT_BINARY = get(ENV, "RECUR_BIN", normpath(joinpath(@__DIR__, "..", "..", "target", "release-safe", "recur" * (Sys.iswindows() ? ".exe" : ""))))

catalog() = [(id=c.id, label=c.label, scopes=copy(c.scopes)) for c in CATALOG]
json_response(status, value) = HTTP.Response(status,
    ["Content-Type"=>"application/json; charset=utf-8", "Cache-Control"=>"no-store",
     "X-Content-Type-Options"=>"nosniff", "X-Main-Server"=>"Julia/$(VERSION)"], JSON3.write(value))
failure(status, code, message) = json_response(status, (error=(code=code, message=message),))

"Resolve the exact public selection without accepting process or file options."
function select(uri)
    isempty(uri.query) && return nothing
    pairs = split(uri.query, '&'; keepempty=true)
    keys = [HTTP.URIs.unescapeuri(first(split(pair, '='; limit=2))) for pair in pairs]
    (length(keys) == 2 && Set(keys) == Set(["id", "scope"])) ||
        return failure(400, "invalid-selection", "Supply exactly one id and one scope.")
    params = HTTP.URIs.queryparams(uri)
    (isempty(get(params, "id", "")) || isempty(get(params, "scope", ""))) &&
        return failure(400, "invalid-selection", "Both id and scope must be nonempty.")
    index = findfirst(c -> c.id == params["id"], CATALOG)
    isnothing(index) && return failure(404, "unknown-capability", "Unknown capability ID.")
    entry = CATALOG[index]
    params["scope"] in entry.scopes || return failure(400, "unknown-scope", "Scope is not in this capability's catalog.")
    (entry=entry, scope=params["scope"])
end

function respond(uri; binary=DEFAULT_BINARY, adapter=LangInspector.capture_query)
    selected = select(uri)
    selected isa HTTP.Response && return selected
    isnothing(selected) && return json_response(200, (catalog=catalog(), selection=nothing))
    try
        entry, scope = selected.entry, selected.scope
        packet = LangInspector.query_report(binary, entry.root, entry.source, scope; adapter)
        view = LangInspector.build_view(packet)
        text = LangInspector.render_view(view)
        cards = [(identity=c.identity, meaning=c.meaning, input_local=c.input_local,
                  input_canonical=c.input_canonical, output_local=c.output_local,
                  output_canonical=c.output_canonical) for c in view.cards]
        json_response(200, (catalog=catalog(), selection=(id=entry.id, scope=scope),
            packet=packet, cards=cards, text=text, observed_evidence=[]))
    catch error
        failure(502, "query-failed", sprint(showerror, error))
    end
end

end
