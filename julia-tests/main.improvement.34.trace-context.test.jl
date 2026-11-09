module TraceContextTests
include("runtests.setup.jl")
using SHA

function query(root, args...; pattern="capability.**", success=true)
    ok, output, errors = run_recur(["trace-id", pattern, "-d", root, "--json", args...])
    @test ok == success
    @test !isempty(output)
    isempty(output) && return nothing
    @test isempty(errors)
    JSON3.read(output)
end

function put(root, name, text)
    file=joinpath(root,name)
    mkpath(dirname(file))
    write(file,text)
end

@testset "Improvement 34 trace sets and context" begin
    @testset "Distinct ID sets and selected source context" begin
        mktempdir() do root
            source = "# Capability contract\nWhy the baseline matters.\ndefines: capability.baseline\nconsumer: capability.delivery\ncapability.baseline capability.baseline\nKeep this explanation.\n\nUnrelated paragraph.\n\nLast note.\ntrigger: capability.baseline.child\n"
            put(root,"one/lab.contract.md",source)
            put(root,"two/lab.consumer.md","Uses capability.baseline\nWHY this consumer exists.\n")
            put(root,"two/lab.generator.jl","println(\"capability.generated\")\n")
            put(root,"other/elsewhere.md","capability.excluded\n")
            put(root,"one/lab.case.md","Capability.Baseline\n")
            before = Dict(joinpath(dir,name)=>read(joinpath(dir,name)) for (dir,_,names) in walkdir(root) for name in names)
            paths=query(root)
            paths === nothing || @test !haskey(paths,:ids) && !haskey(paths,:excerpts)
            ids=query(root,"--scope","lab.**","--list-ids","--ext",".md")
            if ids !== nothing
                @test ids.schema == "recur-trace-id-inventory-v1"
                @test ids.complete && ids.coverage.complete && ids.output.complete
                @test [x.id for x in ids.ids] == ["capability.baseline","capability.baseline.child","capability.delivery"]
                baseline=only(filter(x->x.id=="capability.baseline",ids.ids))
                @test baseline.namespace == "capability"
                @test baseline.file_count == 2 && baseline.occurrences == 4
                @test baseline.files == ["one/lab.contract.md","two/lab.consumer.md"]
                @test ids.provenance == "token-occurrences"
                @test ids.output.returned_bytes == sum(ncodeunits(JSON3.write(x)) for x in ids.ids)
            end
            exact=query(root,"--scope","lab.**","--list-ids","--select","capability.baseline")
            exact === nothing || @test [x.id for x in exact.ids] == ["capability.baseline"]
            family=query(root,"--scope","lab.**","--list-ids","--select","capability.baseline.**","--select","capability.delivery")
            family === nothing || @test [x.id for x in family.ids] == ["capability.baseline","capability.baseline.child","capability.delivery"]
            narrow=query(root,"--scope","lab.**","--list-ids","--select","capability.delivery";pattern="capability.baseline",success=false)
            narrow === nothing || @test narrow.complete && isempty(narrow.ids)
            incidental=query(root,"--scope","lab.**","--list-ids","--ext",".jl")
            incidental === nothing || @test only(incidental.ids).id == "capability.generated"
            case=query(root,"--scope","LAB.CASE","--list-ids","--ignore-case")
            case === nothing || @test only(case.ids).id == "Capability.Baseline"
            inside=query(joinpath(root,"two"),"--scope","lab.**","--list-ids","--ext",".md")
            inside === nothing || @test only(inside.ids).files == ["lab.consumer.md"] && only(inside.ids).occurrences == 1
            excerpts=query(root,"--scope","lab.contract","--context","1","--select","capability.baseline","--select","capability.delivery")
            if excerpts !== nothing
                @test excerpts.schema == "recur-trace-id-context-v1"
                @test excerpts.complete
                excerpt=only(excerpts.excerpts)
                @test excerpt.path == "one/lab.contract.md"
                @test (excerpt.start_line,excerpt.end_line) == (2,6)
                @test excerpt.ids == ["capability.baseline","capability.delivery"]
                @test excerpt.lines == split(source,'\n')[2:6]
                @test !occursin("Unrelated",join(excerpt.lines,'\n'))
                @test excerpts.output.returned_bytes == ncodeunits(JSON3.write(excerpt))
            end
            zero=query(root,"--scope","lab.contract","--context","0","--select","capability.baseline")
            zero === nothing || @test [(e.start_line,e.end_line) for e in zero.excerpts] == [(3,3),(5,5)]
            ok,text,_=run_recur(["trace-id","capability.baseline","-d",root,"--scope","lab.contract","--context","1"])
            @test ok && occursin("one/lab.contract.md:2-6",text) && occursin("Why the baseline matters.",text)
            ok,text,_=run_recur(["trace-id","capability.baseline","-d",root,"--scope","lab.**","--list-ids"])
            @test ok && occursin("capability.baseline",text) && occursin("2 files",text)
            after = Dict(joinpath(dir,name)=>read(joinpath(dir,name)) for (dir,_,names) in walkdir(root) for name in names)
            @test before == after
        end
    end
    @testset "Bounds, omissions, rejection and repaired token handling" begin
        mktempdir() do root
            put(root,"lab.one.md","capability.alpha\n\n\ncapability.beta\n")
            limited=query(root,"--scope","lab.**","--list-ids","--max-results","1")
            limited === nothing || @test !limited.complete && limited.coverage.complete && !limited.output.complete && length(limited.ids)==1
            bounded=query(root,"--scope","lab.**","--context","0","--max-results","1")
            bounded === nothing || @test !bounded.complete && length(bounded.excerpts)==1
            tiny=query(root,"--scope","lab.**","--context","1","--max-output-bytes","1")
            tiny === nothing || @test !tiny.complete && isempty(tiny.excerpts) && tiny.output.returned_bytes==0
            small=query(root,"--scope","lab.**","--list-ids","--max-output-bytes","1")
            small === nothing || @test !small.complete && isempty(small.ids)
            short=query(root,"--scope","lab.**","--list-ids","--max-bytes","1")
            short === nothing || @test !short.coverage.complete && isempty(short.ids)
            empty=query(root,"--scope","missing.**","--list-ids";success=false)
            empty === nothing || @test empty.complete && isempty(empty.ids)
            put(root,"lab.utf8.md","Explanation: café λ 🙂\ncapability.unicode\n")
            utf8=query(root,"--scope","lab.utf8","--context","1")
            utf8 === nothing || @test first(only(utf8.excerpts).lines)=="Explanation: café λ 🙂"
            put(root,"lab.binary.dat",UInt8[0,1,2,3])
            binary=query(root,"--scope","lab.binary","--list-ids","--max-bytes","4";success=false)
            binary === nothing || @test binary.coverage.bytes_read==4 && isempty(binary.ids)
            put(root,"lab.encoding.md",UInt8[0xff,0xfe,0xff])
            invalid=query(root,"--scope","lab.encoding","--list-ids")
            invalid === nothing || @test !invalid.coverage.complete && haskey(invalid.coverage.omissions,:invalid_utf8)
            put(root,"lab.long.md","capability.long "*repeat("λ",500)*"\n")
            long=query(root,"--scope","lab.long","--context","0","--max-output-bytes","128")
            long === nothing || @test !long.complete && isempty(long.excerpts) && long.output.returned_bytes <= 128
            for options in (["--list-ids"],["--context","2"],["--scope","**","--select","capability.alpha"],
                            ["--scope","**","--list-ids","--context","2"],["--scope","**","--list-ids","--declared"],
                            ["--scope","**","--context","21"],["--scope","**","--list-ids","--max-results","0"],
                            ["--scope","**","--list-ids","--max-output-bytes","0"],["--scope","**","--list-ids","--format","full"],
                            ["--scope","**","--list-ids","--select"," "],["--scope"," ","--list-ids"],
                            ["--scope","**","--list-ids","--max-results","10001"],["--scope","**","--context","0","--max-output-bytes","1048577"])
                ok,_,errors=run_recur(vcat(["trace-id","capability.**","-d",root],options))
                @test !ok && !isempty(errors)
            end
        end
        for sep in (".","_","-")
            mktempdir() do root
                id=join(["capability","baseline"],sep)
                put(root,"lab.match.md","Reference "*id*".\n")
                put(root,"lab.suffix.md",id*"_extra\n")
                put(root,"lab.child.md",id*sep*"child\n")
                p=query(root,"--sep",sep;pattern=id)
                p === nothing || @test p.files == ["lab.match.md"]
            end
        end
    end
end
end
