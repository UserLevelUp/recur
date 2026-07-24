module MainLang

export Boundary,
       LanguageError,
       Program,
       SOURCE_PATH,
       describe,
       execute_all,
       execute_scope,
       json_string,
       load_program,
       main,
       parse_bindings,
       parse_program

const SOURCE_PATH = joinpath(@__DIR__, "main.lang.algorithm-lab.recur")

struct LanguageError <: Exception
    message::String
end

Base.showerror(io::IO, error::LanguageError) = print(io, error.message)

struct Field
    name::String
    type_name::String
end

Base.:(==)(left::Field, right::Field) =
    left.name == right.name && left.type_name == right.type_name

struct Shape
    symbol::String
    role::String
    fields::Vector{Field}
    canonical_name::String
    alias_of::Union{Nothing,String}
end

function source_text(shape::Shape)
    marker = shape.role == "input" ? "i" : "o"
    !isnothing(shape.alias_of) &&
        return "$marker($(shape.symbol)) := $(shape.alias_of)"
    fields = join(["$(field.name): $(field.type_name)" for field in shape.fields], ", ")
    return "$marker($(shape.symbol)) := ($fields)"
end

struct FunctionDef
    symbol::String
    input_symbol::String
    output_symbol::String
    familiar_name::String
    intrinsic::String
end

function source_text(definition::FunctionDef)
    return (
        "$(definition.symbol) : i($(definition.input_symbol)) -> " *
        "o($(definition.output_symbol)) ~ \"$(definition.familiar_name)\" " *
        "by $(definition.intrinsic)"
    )
end

struct Event
    edge::String
    identifier::String
end

struct Warp
    current::String
    slice_symbol::String
    desired::String
end

source_text(warp::Warp) =
    "E0($(warp.current)) -> dE($(warp.slice_symbol)) -> Ef($(warp.desired))"

mutable struct Scope
    name::String
    shapes::Dict{String,Shape}
    definition::FunctionDef
    mode::String
    flow::String
    expansion::String
    events::Vector{Event}
    warp::Union{Nothing,Warp}
end

struct Flow
    name::String
    mode::String
    expression::String
end

struct Boundary
    producer_scope::String
    symbol::String
    consumer_scope::String
end

source_text(boundary::Boundary) =
    "$(boundary.producer_scope).o($(boundary.symbol)) -> " *
    "$(boundary.consumer_scope).i($(boundary.symbol))"

struct Program
    version::String
    class_name::String
    scopes::Dict{String,Scope}
    scope_order::Vector{String}
    flows::Dict{String,Flow}
    flow_order::Vector{String}
    exports::Vector{String}
    boundaries::Vector{Boundary}
end

squash_space(value::AbstractString) = strip(replace(value, r"\s+" => " "))

function block_from_opening(text::String, opening::Int)
    depth = 0
    index = opening
    while index <= lastindex(text)
        character = text[index]
        character == '{' && (depth += 1)
        if character == '}'
            depth -= 1
            if depth == 0
                first = nextind(text, opening)
                last = prevind(text, index)
                body = first > last ? "" : String(SubString(text, first, last))
                return body, nextind(text, index)
            end
        end
        index = nextind(text, index)
    end
    throw(LanguageError("source block has no closing brace"))
end

function extract_block(text::String, pattern::Regex)
    found = match(pattern, text)
    isnothing(found) && throw(LanguageError("missing source block matching $(pattern.pattern)"))
    opening = findnext(==('{'), text, found.offset)
    isnothing(opening) && throw(LanguageError("source block has no opening brace"))
    return block_from_opening(text, opening)
end

function dedent_block(body::String)
    lines = split(replace(body, "\r\n" => "\n"), '\n'; keepempty = true)
    while !isempty(lines) && isempty(strip(first(lines)))
        popfirst!(lines)
    end
    while !isempty(lines) && isempty(strip(last(lines)))
        pop!(lines)
    end
    isempty(lines) && return ""
    indents = [
        length(match(r"^ *", line).match)
        for line in lines
        if !isempty(strip(line))
    ]
    indent = isempty(indents) ? 0 : minimum(indents)
    trimmed = [
        length(line) <= indent ? "" : line[(indent + 1):end]
        for line in lines
    ]
    return join(trimmed, "\n")
end

function extract_named_blocks(text::String, keyword::String)
    blocks = Pair{String,String}[]
    position = firstindex(text)
    pattern = Regex("\\b$(keyword)\\s+([A-Za-z][A-Za-z0-9_.]*)\\s*\\{", "m")
    while position <= ncodeunits(text)
        found = match(pattern, text, position)
        isnothing(found) && break
        name = String(found.captures[1])
        any(first(block) == name for block in blocks) &&
            throw(LanguageError("duplicate $keyword block '$name'"))
        opening = findnext(==('{'), text, found.offset)
        isnothing(opening) && throw(LanguageError("$keyword block '$name' has no opening brace"))
        body, after = block_from_opening(text, opening)
        push!(blocks, name => dedent_block(body))
        position = after
    end
    return blocks
end

function parse_shape(
    scope_name::String,
    role::String,
    symbol::String,
    fields_source::String,
)
    fields = Field[]
    for item in split(fields_source, ',')
        found = match(
            r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([A-Za-z][A-Za-z0-9_<>?]*)\s*$",
            item,
        )
        isnothing(found) && throw(LanguageError("invalid field in shape '$symbol': '$item'"))
        push!(fields, Field(String(found.captures[1]), String(found.captures[2])))
    end
    isempty(fields) && throw(LanguageError("shape '$symbol' cannot be empty"))
    marker = role == "input" ? "i" : "o"
    return Shape(symbol, role, fields, "$scope_name.$marker($symbol)", nothing)
end

function validate!(program::Program)
    for name in program.scope_order
        scope = program.scopes[name]
        definition = scope.definition
        haskey(scope.shapes, definition.input_symbol) ||
            throw(LanguageError("$name.$(definition.symbol) has unknown input shape"))
        haskey(scope.shapes, definition.output_symbol) ||
            throw(LanguageError("$name.$(definition.symbol) has unknown output shape"))
        scope.shapes[definition.input_symbol].role == "input" ||
            throw(LanguageError("$name.$(definition.input_symbol) must be declared with i(...)"))
        scope.shapes[definition.output_symbol].role == "output" ||
            throw(LanguageError("$name.$(definition.output_symbol) must be declared with o(...)"))
        haskey(program.flows, name) ||
            throw(LanguageError("scope '$name' has no body flow"))

        flow = program.flows[name]
        expected = (
            "i($(definition.input_symbol)) -> " *
            "$(definition.symbol)($(definition.input_symbol)) -> " *
            "o($(definition.output_symbol))"
        )
        squash_space(flow.expression) == squash_space(expected) ||
            throw(
                LanguageError(
                    "'$name' contract mismatch: expected '$expected', " *
                    "found '$(flow.expression)'",
                ),
            )
        scope.mode = flow.mode
        scope.flow = flow.expression
        isnothing(scope.warp) && throw(LanguageError("scope '$name' has no Warp projection"))
        expected_slice = "$name.$(definition.symbol)"
        scope.warp.slice_symbol == expected_slice ||
            throw(
                LanguageError(
                    "'$name' Warp uses '$(scope.warp.slice_symbol)'; " *
                    "expected '$expected_slice'",
                ),
            )
    end

    for name in program.exports
        (haskey(program.scopes, name) || haskey(program.flows, name)) ||
            throw(LanguageError("footer exposes unknown flow '$name'"))
    end

    for flow in values(program.flows)
        flow.mode == "async" && !occursin("await", flow.expression) &&
            throw(LanguageError("async flow '$(flow.name)' has no visible await"))
    end

    for boundary in program.boundaries
        producer = get(program.scopes, boundary.producer_scope, nothing)
        consumer = get(program.scopes, boundary.consumer_scope, nothing)
        (isnothing(producer) || isnothing(consumer)) &&
            throw(LanguageError("shared boundary has an unknown scope: $(source_text(boundary))"))
        producer.definition.output_symbol == boundary.symbol ||
            throw(
                LanguageError(
                    "$(boundary.producer_scope).o($(boundary.symbol)) is not that " *
                    "function's declared output",
                ),
            )
        consumer.definition.input_symbol == boundary.symbol ||
            throw(
                LanguageError(
                    "$(boundary.consumer_scope).i($(boundary.symbol)) is not that " *
                    "function's declared input",
                ),
            )
        produced = producer.shapes[boundary.symbol]
        consumed = consumer.shapes[boundary.symbol]
        produced.canonical_name == consumed.canonical_name ||
            throw(
                LanguageError(
                    "shared boundary $(source_text(boundary)) does not resolve to one " *
                    "contract: '$(produced.canonical_name)' != '$(consumed.canonical_name)'",
                ),
            )
        produced.fields == consumed.fields ||
            throw(
                LanguageError(
                    "shared boundary $(source_text(boundary)) does not have an exact signature",
                ),
            )
    end
    return program
end

function parse_program(source::String)
    declaration = match(
        r"(?m)^\s*recur\s+([0-9.]+)\s+class\s+([A-Za-z][A-Za-z0-9_]*)\s*$",
        source,
    )
    isnothing(declaration) &&
        throw(LanguageError("missing 'recur <version> class <Name>' declaration"))

    header, _ = extract_block(source, r"\bheader\s*\{")
    body, _ = extract_block(source, r"\bbody\s*\{")
    footer, _ = extract_block(source, r"\bfooter\s*\{")

    scopes = Dict{String,Scope}()
    scope_order = String[]
    for (scope_name, scope_source) in extract_named_blocks(header, "scope")
        shapes = Dict{String,Shape}()
        for found in eachmatch(
            r"(?m)^\s*([io])\(([a-z])\)\s*:=\s*\(([^)]*)\)\s*$",
            scope_source,
        )
            marker = String(found.captures[1])
            role = marker == "i" ? "input" : "output"
            symbol = String(found.captures[2])
            shapes[symbol] =
                parse_shape(scope_name, role, symbol, String(found.captures[3]))
        end

        for found in eachmatch(
            r"(?m)^\s*([io])\(([a-z])\)\s*:=\s*([A-Za-z][A-Za-z0-9_]*)\.([io])\(([a-z])\)\s*$",
            scope_source,
        )
            marker = String(found.captures[1])
            role = marker == "i" ? "input" : "output"
            symbol = String(found.captures[2])
            target_scope_name = String(found.captures[3])
            target_marker = String(found.captures[4])
            target_symbol = String(found.captures[5])
            qualified = "$target_scope_name.$target_marker($target_symbol)"
            target_scope = get(scopes, target_scope_name, nothing)
            (isnothing(target_scope) || !haskey(target_scope.shapes, target_symbol)) &&
                throw(LanguageError("$scope_name.$symbol aliases unknown bundle '$qualified'"))
            target = target_scope.shapes[target_symbol]
            target.role == (target_marker == "i" ? "input" : "output") ||
                throw(LanguageError("$qualified uses the wrong input/output marker"))
            role == "input" && target.role == "output" ||
                throw(LanguageError("bundle aliases must connect i(...) to a prior o(...)"))
            shapes[symbol] =
                Shape(symbol, role, target.fields, target.canonical_name, qualified)
        end

        found = match(
            r"(?m)^\s*([a-z])\s*:\s*i\(([a-z])\)\s*->\s*o\(([a-z])\)\s*~\s*\"([^\"]+)\"\s+by\s+([A-Za-z][A-Za-z0-9_.]*)\s*$",
            scope_source,
        )
        isnothing(found) &&
            throw(LanguageError("scope '$scope_name' has no valid function declaration"))
        definition = FunctionDef(
            String(found.captures[1]),
            String(found.captures[2]),
            String(found.captures[3]),
            String(found.captures[4]),
            String(found.captures[5]),
        )
        scopes[scope_name] =
            Scope(scope_name, shapes, definition, "", "", "", Event[], nothing)
        push!(scope_order, scope_name)
    end

    flows = Dict{String,Flow}()
    flow_order = String[]
    for found in eachmatch(
        r"(?m)^\s*([A-Za-z][A-Za-z0-9_.]*)\s+(sync|async)\s*:\s*(.+?)\s*$",
        body,
    )
        name = String(found.captures[1])
        flows[name] =
            Flow(name, String(found.captures[2]), String(found.captures[3]))
        push!(flow_order, name)
    end

    boundaries = Boundary[]
    for found in eachmatch(
        r"(?m)^\s*share\s+([A-Za-z][A-Za-z0-9_.]*)\.o\(([a-z])\)\s*->\s*([A-Za-z][A-Za-z0-9_.]*)\.i\(\2\)\s*$",
        body,
    )
        push!(
            boundaries,
            Boundary(
                String(found.captures[1]),
                String(found.captures[2]),
                String(found.captures[3]),
            ),
        )
    end

    for (qualified, expansion) in extract_named_blocks(body, "expand")
        occursin('.', qualified) ||
            throw(LanguageError("expansion '$qualified' must be scope-qualified"))
        scope_name, symbol = rsplit(qualified, '.'; limit = 2)
        haskey(scopes, scope_name) ||
            throw(LanguageError("expansion has unknown scope '$scope_name'"))
        symbol == scopes[scope_name].definition.symbol ||
            throw(LanguageError("expansion has unknown function '$qualified'"))
        scopes[scope_name].expansion = expansion
    end

    expose = match(r"(?m)^\s*expose\s+(.+?)\s*$", footer)
    exports = isnothing(expose) ? String[] :
              [strip(item) for item in split(String(expose.captures[1]), ',')]

    for (scope_name, event_source) in extract_named_blocks(footer, "event")
        haskey(scopes, scope_name) ||
            throw(LanguageError("event block has unknown scope '$scope_name'"))
        events = Event[]
        for found in eachmatch(
            r"(?m)^\s*(consume|trigger|produce|state)\s+([A-Za-z0-9_.-]+)\s*$",
            event_source,
        )
            push!(
                events,
                Event(String(found.captures[1]), String(found.captures[2])),
            )
        end
        scopes[scope_name].events = events
    end

    for found in eachmatch(
        r"(?m)^\s*warp\s+([A-Za-z][A-Za-z0-9_.]*)\s*:\s*E0\(([A-Za-z0-9_.-]+)\)\s*->\s*dE\(([A-Za-z0-9_.-]+)\)\s*->\s*Ef\(([A-Za-z0-9_.-]+)\)\s*$",
        footer,
    )
        scope_name = String(found.captures[1])
        haskey(scopes, scope_name) ||
            throw(LanguageError("Warp has unknown scope '$scope_name'"))
        scopes[scope_name].warp = Warp(
            String(found.captures[2]),
            String(found.captures[3]),
            String(found.captures[4]),
        )
    end

    program = Program(
        String(declaration.captures[1]),
        String(declaration.captures[2]),
        scopes,
        scope_order,
        flows,
        flow_order,
        exports,
        boundaries,
    )
    return validate!(program)
end

load_program(path::String = SOURCE_PATH) = parse_program(read(path, String))

const Bundle = Dict{String,Any}

function gcd_euclid(bundle::Bundle)
    left = abs(bundle["left"])
    right = abs(bundle["right"])
    while right != 0
        left, right = right, left % right
    end
    return Bundle("value" => left)
end

function bubble_sort(bundle::Bundle)
    values = copy(bundle["values"])
    for last_index in length(values):-1:2
        swapped = false
        for index in 1:(last_index - 1)
            if values[index] > values[index + 1]
                values[index], values[index + 1] = values[index + 1], values[index]
                swapped = true
            end
        end
        !swapped && break
    end
    return Bundle("values" => values)
end

function merge_values(values::Vector{Int})
    length(values) <= 1 && return values
    middle = length(values) ÷ 2
    left = merge_values(values[1:middle])
    right = merge_values(values[(middle + 1):end])
    merged = Int[]
    left_index = 1
    right_index = 1
    while left_index <= length(left) && right_index <= length(right)
        if left[left_index] <= right[right_index]
            push!(merged, left[left_index])
            left_index += 1
        else
            push!(merged, right[right_index])
            right_index += 1
        end
    end
    append!(merged, left[left_index:end])
    append!(merged, right[right_index:end])
    return merged
end

merge_sort(bundle::Bundle) =
    Bundle("values" => merge_values(copy(bundle["values"])))

function prime_sieve(bundle::Bundle)
    limit = bundle["limit"]
    limit < 2 && return Bundle("values" => Int[])
    possible = trues(limit + 1)
    possible[1] = false
    possible[2] = false
    candidate = 2
    while candidate * candidate <= limit
        if possible[candidate + 1]
            for multiple in (candidate * candidate):candidate:limit
                possible[multiple + 1] = false
            end
        end
        candidate += 1
    end
    return Bundle(
        "values" => [
            value
            for value in 0:limit
            if possible[value + 1]
        ],
    )
end

function centered_pyramid(bundle::Bundle)
    rows = bundle["rows"]
    glyph = bundle["glyph"]
    rows < 0 && throw(LanguageError("pyramid rows cannot be negative"))
    isempty(glyph) && throw(LanguageError("pyramid glyph cannot be empty"))
    lines = [
        repeat(" ", rows - row) * repeat(glyph, (2 * row) - 1)
        for row in 1:rows
    ]
    return Bundle("lines" => lines)
end

const INTRINSICS = Dict{String,Function}(
    "gcd.euclid" => gcd_euclid,
    "sort.bubble" => bubble_sort,
    "sort.merge" => merge_sort,
    "prime.sieve" => prime_sieve,
    "pyramid.centered" => centered_pyramid,
)

const DEFAULT_INPUTS = Dict{String,Bundle}(
    "gcd" => Bundle("left" => 1071, "right" => 462),
    "bubble" => Bundle("values" => [5, 1, 4, 2, 8]),
    "merge" => Bundle("values" => [9, 3, 7, 1, 4]),
    "primes" => Bundle("limit" => 30),
    "pyramid" => Bundle("rows" => 5, "glyph" => "*"),
)

function matches_type(type_name::String, value)
    type_name == "Int" && return value isa Integer && !(value isa Bool)
    type_name == "Text" && return value isa AbstractString
    type_name == "List<Int>" &&
        return value isa AbstractVector &&
               all(item isa Integer && !(item isa Bool) for item in value)
    type_name == "List<Text>" &&
        return value isa AbstractVector && all(item isa AbstractString for item in value)
    return false
end

function validate_bundle!(scope_name::String, shape::Shape, bundle::Bundle)
    expected = Set(field.name for field in shape.fields)
    actual = Set(keys(bundle))
    expected == actual ||
        throw(
            LanguageError(
                "$scope_name.$(shape.symbol) fields do not match; " *
                "missing=$(sort!(collect(setdiff(expected, actual)))), " *
                "extra=$(sort!(collect(setdiff(actual, expected))))",
            ),
        )
    for field in shape.fields
        matches_type(field.type_name, bundle[field.name]) ||
            throw(
                LanguageError(
                    "$scope_name.$(shape.symbol).$(field.name) expects " *
                    "$(field.type_name), found $(repr(bundle[field.name]))",
                ),
            )
    end
end

event_dict(event::Event) =
    Dict{String,Any}("edge" => event.edge, "identifier" => event.identifier)

warp_dict(warp::Warp) = Dict{String,Any}(
    "current" => warp.current,
    "slice_symbol" => warp.slice_symbol,
    "desired" => warp.desired,
)

function execute_scope(program::Program, scope_name::String, inputs::Bundle)
    haskey(program.scopes, scope_name) ||
        throw(LanguageError("unknown executable scope '$scope_name'"))
    scope = program.scopes[scope_name]
    definition = scope.definition
    input_shape = scope.shapes[definition.input_symbol]
    output_shape = scope.shapes[definition.output_symbol]
    validate_bundle!(scope_name, input_shape, inputs)
    haskey(INTRINSICS, definition.intrinsic) ||
        throw(LanguageError("no intrinsic registered for '$(definition.intrinsic)'"))
    output = INTRINSICS[definition.intrinsic](deepcopy(inputs))
    validate_bundle!(scope_name, output_shape, output)
    return Dict{String,Any}(
        "scope" => scope_name,
        "mode" => scope.mode,
        "flow" => scope.flow,
        "input" => Dict{String,Any}("symbol" => input_shape.symbol, "value" => inputs),
        "output" => Dict{String,Any}("symbol" => output_shape.symbol, "value" => output),
        "events" => [event_dict(event) for event in scope.events],
        "warp" => warp_dict(scope.warp),
    )
end

function execute_all(program::Program)
    flow = get(program.flows, "all", nothing)
    (isnothing(flow) || flow.mode != "async") &&
        throw(LanguageError("program has no 'all async' flow"))
    found = match(r"^\[([^\]]+)\]\s*->\s*await\s*->", flow.expression)
    isnothing(found) && throw(LanguageError("'all async' flow has an invalid lane list"))
    names = [String(strip(name)) for name in split(String(found.captures[1]), ',')]
    tasks = [
        Threads.@spawn execute_scope(program, name, deepcopy(DEFAULT_INPUTS[name]))
        for name in names
    ]
    results = fetch.(tasks)
    lanes = Dict{String,Any}()
    for result in results
        lanes[result["scope"]] = result
    end
    return Dict{String,Any}(
        "scope" => "all",
        "mode" => "async",
        "flow" => flow.expression,
        "lane_order" => names,
        "output" => Dict{String,Any}(
            "symbol" => "results",
            "value" => lanes,
        ),
    )
end

function unqualify(program::Program, target::String)
    prefix = program.class_name * "."
    return startswith(target, prefix) ? target[(length(prefix) + 1):end] : target
end

function relevant_boundaries(program::Program, scope_name::String)
    return [
        source_text(boundary)
        for boundary in program.boundaries
        if boundary.producer_scope == scope_name ||
           boundary.consumer_scope == scope_name
    ]
end

function describe(program::Program, raw_target::String; expand::Bool = false)
    target = unqualify(program, raw_target)
    if haskey(program.flows, target) && !haskey(program.scopes, target)
        flow = program.flows[target]
        return Dict{String,Any}(
            "name" => target,
            "mode" => flow.mode,
            "flow" => flow.expression,
            "boundaries" => relevant_boundaries(program, target),
        )
    end

    role_lookup = match(
        r"^([A-Za-z][A-Za-z0-9_]*)\.([io])\(([a-z])\)$",
        target,
    )
    parts = split(target, '.')
    scope_name = isnothing(role_lookup) ?
                 String(first(parts)) :
                 String(role_lookup.captures[1])
    requested_role = isnothing(role_lookup) ?
                     nothing :
                     (String(role_lookup.captures[2]) == "i" ? "input" : "output")
    symbol = isnothing(role_lookup) ?
             (length(parts) == 2 ? String(parts[2]) : "") :
             String(role_lookup.captures[3])
    haskey(program.scopes, scope_name) ||
        throw(LanguageError("unknown symbol or scope '$target'"))
    scope = program.scopes[scope_name]

    if isnothing(role_lookup) && length(parts) == 1
        result = Dict{String,Any}(
            "scope" => scope.name,
            "mode" => scope.mode,
            "flow" => scope.flow,
            "input" => source_text(scope.shapes[scope.definition.input_symbol]),
            "output" => source_text(scope.shapes[scope.definition.output_symbol]),
            "shapes" => Dict(
                name => source_text(shape)
                for (name, shape) in scope.shapes
            ),
            "function" => source_text(scope.definition),
            "events" => [event_dict(event) for event in scope.events],
            "warp" => source_text(scope.warp),
            "boundaries" => relevant_boundaries(program, scope_name),
        )
        expand && (result["expansion"] = scope.expansion)
        return result
    end

    (!isnothing(role_lookup) || length(parts) == 2) ||
        throw(LanguageError("symbol lookup is at most scope.symbol, found '$target'"))
    if haskey(scope.shapes, symbol)
        shape = scope.shapes[symbol]
        !isnothing(requested_role) && shape.role != requested_role &&
            throw(LanguageError("'$target' uses the wrong input/output marker"))
        marker = shape.role == "input" ? "i" : "o"
        return Dict{String,Any}(
            "symbol" => "$scope_name.$marker($symbol)",
            "kind" => "bundle",
            "role" => shape.role,
            "definition" => source_text(shape),
            "fields" => [
                Dict{String,Any}("name" => field.name, "type_name" => field.type_name)
                for field in shape.fields
            ],
            "canonical" => shape.canonical_name,
        )
    end
    if symbol == scope.definition.symbol
        result = Dict{String,Any}(
            "symbol" => "$scope_name.$symbol",
            "kind" => "function",
            "definition" => source_text(scope.definition),
            "collapsed" => scope.flow,
            "warp" => source_text(scope.warp),
        )
        expand && (result["expansion"] = scope.expansion)
        return result
    end
    throw(LanguageError("unknown local symbol '$target'"))
end

function parse_value(type_name::String, raw::AbstractString)
    if type_name == "Int"
        value = tryparse(Int, raw)
        isnothing(value) && throw(LanguageError("expected Int, found '$raw'"))
        return value
    elseif type_name == "Text"
        return String(raw)
    elseif type_name == "List<Int>"
        isempty(raw) && return Int[]
        values = Int[]
        for item in split(raw, ',')
            value = tryparse(Int, strip(item))
            isnothing(value) &&
                throw(LanguageError("expected comma-separated Int values, found '$raw'"))
            push!(values, value)
        end
        return values
    elseif type_name == "List<Text>"
        return isempty(raw) ? String[] : [strip(item) for item in split(raw, ',')]
    end
    throw(LanguageError("unsupported prototype type '$type_name'"))
end

function parse_bindings(program::Program, scope_name::String, bindings::Vector{String})
    haskey(program.scopes, scope_name) ||
        throw(LanguageError("unknown executable scope '$scope_name'"))
    scope = program.scopes[scope_name]
    input_shape = scope.shapes[scope.definition.input_symbol]
    field_types = Dict(field.name => field.type_name for field in input_shape.fields)
    values = deepcopy(DEFAULT_INPUTS[scope_name])
    for binding in bindings
        occursin('=', binding) ||
            throw(LanguageError("input must use name=value syntax, found '$binding'"))
        name, raw = split(binding, '='; limit = 2)
        haskey(field_types, name) ||
            throw(LanguageError("$scope_name.$(input_shape.symbol) has no field '$name'"))
        values[name] = parse_value(field_types[name], raw)
    end
    return values
end

function json_escape(value::AbstractString)
    return replace(
        value,
        "\\" => "\\\\",
        "\"" => "\\\"",
        "\b" => "\\b",
        "\f" => "\\f",
        "\n" => "\\n",
        "\r" => "\\r",
        "\t" => "\\t",
    )
end

function write_json(io::IO, value, level::Int = 0)
    padding = repeat("  ", level)
    child_padding = repeat("  ", level + 1)
    if isnothing(value)
        print(io, "null")
    elseif value isa Bool
        print(io, value ? "true" : "false")
    elseif value isa Number
        print(io, value)
    elseif value isa AbstractString
        print(io, '"', json_escape(value), '"')
    elseif value isa AbstractVector
        isempty(value) && return print(io, "[]")
        println(io, "[")
        for (index, item) in enumerate(value)
            print(io, child_padding)
            write_json(io, item, level + 1)
            index < length(value) && print(io, ',')
            println(io)
        end
        print(io, padding, "]")
    elseif value isa AbstractDict
        isempty(value) && return print(io, "{}")
        entries = sort!(collect(pairs(value)); by = pair -> string(first(pair)))
        println(io, "{")
        for (index, pair) in enumerate(entries)
            print(io, child_padding, '"', json_escape(string(first(pair))), "\": ")
            write_json(io, last(pair), level + 1)
            index < length(entries) && print(io, ',')
            println(io)
        end
        print(io, padding, "}")
    else
        throw(LanguageError("cannot encode $(typeof(value)) as JSON"))
    end
end

json_string(value) = sprint(io -> write_json(io, value))

function bundle_text(bundle::AbstractDict)
    entries = sort!(collect(pairs(bundle)); by = pair -> string(first(pair)))
    return "{" * join(["$(first(pair))=$(repr(last(pair)))" for pair in entries], ", ") * "}"
end

function print_human(value::Dict{String,Any})
    kind = get(value, "kind", nothing)
    if kind == "bundle" || kind == "function"
        println(value["symbol"])
        println(value["definition"])
        canonical = get(value, "canonical", nothing)
        !isnothing(canonical) && canonical != value["symbol"] &&
            println("shared contract: $canonical")
        haskey(value, "collapsed") && println(value["collapsed"])
        haskey(value, "warp") && println("warp: $(value["warp"])")
        if haskey(value, "expansion")
            println("\nexpand {")
            for line in split(value["expansion"], '\n')
                println("  $line")
            end
            println("}")
        end
        return
    end

    if haskey(value, "shapes") && haskey(value, "function")
        println("$(value["scope"]) $(value["mode"]) : $(value["flow"])")
        println("symbols:")
        println("  $(value["input"])")
        println("  $(value["output"])")
        println("  $(value["function"])")
        for boundary in value["boundaries"]
            println("share: $boundary")
        end
        println("warp: $(value["warp"])")
        return
    end

    if get(value, "scope", nothing) == "all"
        println("all $(value["mode"]) : $(value["flow"])")
        lanes = value["output"]["value"]
        for name in value["lane_order"]
            output = lanes[name]["output"]
            println(rpad(name, 8), " o($(output["symbol"])) = ", bundle_text(output["value"]))
        end
        return
    end

    if haskey(value, "output") && haskey(value, "input")
        println("$(value["scope"]) $(value["mode"]) : $(value["flow"])")
        println("i($(value["input"]["symbol"])) = $(bundle_text(value["input"]["value"]))")
        println("o($(value["output"]["symbol"])) = $(bundle_text(value["output"]["value"]))")
        if !isempty(value["events"])
            println("events:")
            for event in value["events"]
                println("  ", rpad(event["edge"], 7), " ", event["identifier"])
            end
        end
        warp = value["warp"]
        println(
            "warp: E0($(warp["current"])) -> dE($(warp["slice_symbol"])) -> " *
            "Ef($(warp["desired"]))",
        )
        return
    end

    if all(haskey(value, key) for key in ("name", "mode", "flow"))
        println("$(value["name"]) $(value["mode"]) : $(value["flow"])")
        return
    end
    println(json_string(value))
end

function take_flag!(arguments::Vector{String}, flag::String)
    index = findfirst(==(flag), arguments)
    isnothing(index) && return false
    deleteat!(arguments, index)
    return true
end

function take_source!(arguments::Vector{String})
    index = findfirst(==("--source"), arguments)
    isnothing(index) && return SOURCE_PATH
    index == length(arguments) &&
        throw(LanguageError("--source requires a file path"))
    path = arguments[index + 1]
    deleteat!(arguments, index:(index + 1))
    return path
end

function usage(io::IO = stdout)
    println(
        io,
        """
        usage:
          julia main.lang.cli.jl list [--json]
          julia main.lang.cli.jl show <scope|scope.symbol> [--expand] [--json]
          julia main.lang.cli.jl run <scope|all> [name=value ...] [--json]
        """,
    )
end

function main(raw_arguments::Vector{String} = copy(ARGS))
    try
        arguments = copy(raw_arguments)
        source_path = take_source!(arguments)
        as_json = take_flag!(arguments, "--json")
        expand = take_flag!(arguments, "--expand")
        if take_flag!(arguments, "--help") || take_flag!(arguments, "-h")
            usage()
            return 0
        end
        program = load_program(source_path)
        command = isempty(arguments) ? "list" : popfirst!(arguments)

        if command == "list"
            rows = [
                Dict{String,Any}(
                    "name" => name,
                    "mode" => program.flows[name].mode,
                    "flow" => program.flows[name].expression,
                )
                for name in program.flow_order
            ]
            if as_json
                println(json_string(rows))
            else
                println("$(program.class_name) (Recur $(program.version))")
                for row in rows
                    println(
                        rpad(row["name"], 8),
                        " ",
                        rpad(row["mode"], 5),
                        " : ",
                        row["flow"],
                    )
                end
            end
            return 0
        elseif command == "show"
            isempty(arguments) && throw(LanguageError("show requires a target"))
            value = describe(program, popfirst!(arguments); expand = expand)
            isempty(arguments) || throw(LanguageError("unexpected show arguments"))
        elseif command == "run"
            isempty(arguments) && throw(LanguageError("run requires a target"))
            target = unqualify(program, popfirst!(arguments))
            if target == "all"
                isempty(arguments) ||
                    throw(LanguageError("'all' uses the demo input bundle for each lane"))
                value = execute_all(program)
            else
                inputs = parse_bindings(program, target, arguments)
                value = execute_scope(program, target, inputs)
            end
        else
            throw(LanguageError("unknown command '$command'"))
        end

        as_json ? println(json_string(value)) : print_human(value)
        return 0
    catch error
        if error isa LanguageError || error isa SystemError
            println(stderr, "main.lang: ", sprint(showerror, error))
            return 2
        end
        rethrow()
    end
end

end # module MainLang
