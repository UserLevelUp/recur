module TestSelection

const DEMOS = [
    "lang-inspector" => ["main.demo.lang-inspector.test.jl"],
    "web-evidence-lab" => ["main.demo.lang-dogfood.test.jl"],
    "holdem-lab" => ["main.demo.holdem-lab.test.jl"],
    "blackjack-lab" => ["main.demo.blackjack-lab.test.jl"],
    "blackjack-web" => ["main.demo.blackjack-web.test.jl"],
    "skippy-adaptive-comms" => ["main.demo.skippy.trace-id.test.jl"],
    "sudoku" => ["runtests.demo.sudoku.jl", "runtests.demo.sudoku.phase3.jl",
        "runtests.demo.sudoku.phase4.jl", "runtests.demo.sudoku.teaching.jl",
        "main.demo.sudoku.watch.test.jl"],
]

function parse_selection(args)
    selected = String[]
    with_core = false
    list = false
    dry_run = false
    i = 1
    while i <= length(args)
        arg = args[i]
        if arg == "--demo"
            i += 1
            i <= length(args) || throw(ArgumentError("--demo requires a demo name"))
            name = args[i]
            any(pair -> first(pair) == name, DEMOS) ||
                throw(ArgumentError("Unknown demo '$name'; use --list-demos"))
            name in selected || push!(selected, name)
        elseif arg == "--with-core"
            with_core = true
        elseif arg == "--list-demos"
            list = true
        elseif arg == "--dry-run"
            dry_run = true
        elseif arg != "--verbose"
            throw(ArgumentError("Unknown test option '$arg'"))
        end
        i += 1
    end
    files = String[]
    for name in selected
        append!(files, last(only(filter(pair -> first(pair) == name, DEMOS))))
    end
    return (core=isempty(selected) || with_core, demos=selected, files=files,
        list=list, dry_run=dry_run)
end

end
