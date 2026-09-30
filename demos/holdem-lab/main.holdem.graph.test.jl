using Test, JSON3
# consumes: demo.holdem.graph independent CLI falsification of declared graphs
@testset "Lang symbols, lineage and adversarial graphs" begin
    binary = get(ENV,"RECUR_BIN","recur")
    root = @__DIR__
    function cli(args...)
        out, err = IOBuffer(), IOBuffer()
        process = run(pipeline(ignorestatus(Cmd([binary,args...])),stdout=out,stderr=err))
        text = String(take!(out))
        (code=process.exitcode, data=isempty(text) ? nothing : JSON3.read(text), stderr=String(take!(err)))
    end
    specs = sort(filter(f -> endswith(f,".recur"),readdir(root)))
    @test length(specs) == 6
    for file in specs
        result = cli("lang","check",file,"-d",root,"--json")
        @test result.code == 0
        @test result.data.coverage.whole_source_validated == false
        @test result.data.footer.execution == "not-run"
    end
    result = cli("lang","show","main.holdem.05.play.recur","--scope","view.v","-d",root,"--json")
    @test result.data.header[1].input.canonical_identity == "command.o(d)"
    @test length(result.data.body.boundary_edges) >= 1
    compact = cli("lang","show","main.holdem.06.coordination.recur","--scope","settle.s","-d",root,"--json")
    expanded = cli("lang","show","main.holdem.06.coordination.recur","--scope","settle.s","--expand","-d",root,"--json")
    @test length(compact.data.header[1].input.members) == 3
    @test length(expanded.data.header[1].input.messages) == 3
    @test compact.data.footer.graph == expanded.data.footer.graph
    @test compact.data.footer.graph.orchestration_sound
    base = read(joinpath(root,"main.holdem.06.coordination.recur"),String)
    # These exact edits are the reproducible fault injections. The original
    # valid spec stays untouched; check returns 1, not parse-error exit 2.
    variants = [
        ("self-cycle", replace(base,"i(a) := table.plan.o(a)" => "i(a) := join(table.plan.o(a), left.o(b))"; count=1), "SGR001"),
        ("dependency-cycle", replace(base,"i(a) := table.plan.o(a)" => "i(a) := join(table.plan.o(a), audit.o(b))"; count=1), "SGR001"),
        ("wait-cycle", replace(base,"-> table.finish(a)" => "-> settle(a) -> table.finish(a)"), "SGR002"),
        ("missing-join", replace(base,"await [left.o(b), right.o(b)]" => "await left.o(b)"), "SGR004"),
    ]
    mktempdir() do tmp
        for (name,source,code) in variants
            file = "main.holdem.$name.recur"
            write(joinpath(tmp,file),source)
            report = cli("lang","check",file,"-d",tmp,"--json")
            @test report.code == 1
            findings = report.data.footer.graph.findings
            @test any(f -> f.code == code, findings)
            if code in ("SGR001","SGR002")
                cycle = first(f for f in findings if f.code == code)
                @test first(cycle.path) == last(cycle.path)
                println("  $name: $code ",join(cycle.path," -> "))
            else
                println("  $name: ",join([f.code for f in findings],", "))
            end
            scoped = cli("lang","check",file,"--scope","right","-d",tmp,"--json")
            @test scoped.code == 1
            @test scoped.data.footer.graph == report.data.footer.graph
        end
        # A runtime dependency is deliberately hidden behind a binding. Lang
        # cannot inspect this Julia module, so its static verdict remains green.
        spec = replace(read(joinpath(root,"main.holdem.01.hello.recur"),String),"by HoldemLab.hello" => "by HiddenCycle.hello")
        write(joinpath(tmp,"main.holdem.hidden.recur"),spec)
        write(joinpath(tmp,"main.holdem.hidden.jl"), "module HiddenCycle\nhello() = \"Hello, World!\"\nend\n")
        before = cli("lang","check","main.holdem.hidden.recur","-d",tmp,"--json")
        write(joinpath(tmp,"main.holdem.hidden.jl"), "module HiddenCycle\nhello(n=0) = n == 4 ? error(\"bounded dependency cycle\") : caller(n+1)\ncaller(n) = hello(n)\nend\n")
        report = cli("lang","check","main.holdem.hidden.recur","-d",tmp,"--json")
        @test report.code == 0
        @test before.data.source_hash == report.data.source_hash
        hidden = include(joinpath(tmp,"main.holdem.hidden.jl"))
        @test_throws ErrorException Base.invokelatest(() -> getproperty(hidden,:hello)())
        println("  hidden Julia cycle: static check passes; bounded runtime probe fails (coverage limit)")
    end
    trace = cli("trace-id","demo.holdem.play","--scope","main.holdem.**","-d",root,"--format","full","--json")
    @test trace.code == 0
    text = JSON3.write(trace.data)
    @test occursin("main.holdem.requirements.md",text)
    @test occursin("main.holdem.05.jl",text)
    @test occursin("main.holdem.05.test.jl",text)
    @test occursin("trigger",text) && occursin("produce",text) && occursin("consume",text)
    # Claims of equal shape do not invent an alias, and output-to-output aliasing
    # remains explicitly unsupported by WIR1 (the experiment hit RLIR011).
    mktempdir() do tmp
        original = read(joinpath(root,"main.holdem.05.play.recur"),String)
        altered = replace(original,r"o\(d\) := \([^\n]+\)" => "o(d) := start.o(b)")
        write(joinpath(tmp,"main.holdem.alias.recur"),altered)
        report = cli("lang","check","main.holdem.alias.recur","-d",tmp,"--json")
        @test report.code == 2
        @test occursin("RLIR011",JSON3.write(report.data))
    end
end
