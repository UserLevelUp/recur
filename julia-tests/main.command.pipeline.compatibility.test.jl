# Real process pipes: preserve existing contracts; do not imply all JSON schemas compose.
include("runtests.setup.jl")

function pc_pipe(commands...)
    out,err=IOBuffer(),IOBuffer()
    chain=reduce((a,b)->pipeline(a,b),[ignorestatus(c) for c in commands])
    process=run(pipeline(chain,stdout=out,stderr=err))
    stages=hasproperty(process,:processes) ? process.processes : [process]
    return [p.exitcode for p in stages],String(take!(out)),String(take!(err))
end

@testset "Real pipeline compatibility" begin
    mktempdir() do root
        for file in ("demo.alpha.txt","demo.beta space.txt","demo.café.txt","other.txt")
            write(joinpath(root,file),"fixture")
        end
        before=Dict(f=>read(joinpath(root,f)) for f in readdir(root))
        tree=`$RECUR_BIN tree demo -d $root --json`
        merge=`$RECUR_BIN merge --stdin --base demo --json`
        codes,out,err=pc_pipe(tree,merge)
        @test codes==[0,0]
        @test isempty(err)
        parsed=JSON3.read(out)
        @test occursin("alpha",out)
        @test occursin("beta space",out)
        @test occursin("café",out)
        @test !occursin("other.txt",out)

        # An independent JSON-aware process can sit between Recur commands.
        adapter="using JSON3; value=JSON3.read(read(stdin,String)); print(JSON3.write(value))"
        external=`$(Base.julia_cmd()) --startup-file=no -e $adapter`
        codes,adapted,err=pc_pipe(tree,external,merge)
        @test codes==[0,0,0]
        @test isempty(err)
        @test JSON3.read(adapted)==parsed

        # A downstream tool consumes trait JSON, not prose or a universal path envelope.
        summarize="using JSON3; x=JSON3.read(read(stdin,String)); print(JSON3.write(Dict(\"name\"=>x.name,\"status\"=>x.catalog.status)))"
        codes,summary,err=pc_pipe(`$RECUR_BIN trait explain unmerge --json`,
            `$(Base.julia_cmd()) --startup-file=no -e $summarize`)
        @test codes==[0,0]
        @test isempty(err)
        @test JSON3.read(summary).status=="proposed"
        @test JSON3.read(summary).name=="unmerge"

        # JSON path arrays from an external command feed merge.
        producer="using JSON3; print(JSON3.write(ARGS))"
        paths=[joinpath(root,"demo.alpha.txt"),joinpath(root,"demo.beta space.txt"),joinpath(root,"demo.café.txt")]
        codes,produced,err=pc_pipe(`$(Base.julia_cmd()) --startup-file=no -e $producer $paths`,merge)
        @test codes==[0,0]
        @test isempty(err)
        @test occursin("beta space",produced)
        @test occursin("café",produced)
        @test JSON3.read(produced) isa JSON3.Object

        # Wrong JSON schemas and malformed streams fail the receiver on stderr.
        for source in ("print(\"{bad\")","print(\"{\\\"unrelated\\\":true}\")")
            codes,bad,err=pc_pipe(`$(Base.julia_cmd()) --startup-file=no -e $source`,merge)
            @test codes[1]==0
            @test codes[2]!=0
            @test isempty(bad)
            @test !isempty(err)
        end

        # Preserve the existing empty-stream contract, including its non-JSON stdout.
        codes,empty,err=pc_pipe(`$(Base.julia_cmd()) --startup-file=no -e "exit(0)"`,merge)
        @test codes==[0,0]
        @test strip(empty)=="No files found in stdin"
        @test isempty(err)

        # A valid payload does not erase a producer's failing exit status.
        failproducer="using JSON3; print(JSON3.write(ARGS)); flush(stdout); println(stderr,\"producer failed\"); exit(7)"
        codes,partial,err=pc_pipe(`$(Base.julia_cmd()) --startup-file=no -e $failproducer $paths`,merge)
        @test codes==[7,0]
        @test occursin("producer failed",err)
        @test JSON3.read(partial) isa JSON3.Object
        @test before==Dict(f=>read(joinpath(root,f)) for f in readdir(root))

        # Exercise the actual PowerShell | operator on the user's platform.
        shell=Sys.which("pwsh")
        shell===nothing && (shell=Sys.which("powershell"))
        if shell!==nothing
            quoteps(s)="'"*replace(s,"'"=>"''")*"'"
            encoding="[Console]::InputEncoding = [Console]::OutputEncoding = \$OutputEncoding = [System.Text.UTF8Encoding]::new(\$false); "
            script=encoding*"& "*quoteps(RECUR_BIN)*" tree demo -d "*quoteps(root)*" --json | & "*quoteps(RECUR_BIN)*" merge --stdin --base demo --json; exit \$LASTEXITCODE"
            codes,shellout,err=pc_pipe(`$shell -NoProfile -NonInteractive -Command $script`)
            @test codes==[0]
            @test isempty(err)
            @test JSON3.read(shellout)==parsed
        end
    end
    mktempdir() do root
        # Git's line-oriented output is a different contract from JSON.
        run(`git -C $root init -q`)
        write(joinpath(root,"demo.alpha.txt"),"a")
        write(joinpath(root,"demo.beta space.txt"),"b")
        write(joinpath(root,"other.txt"),"c")
        git=`git -C $root -c core.quotepath=false ls-files --others --exclude-standard`
        filter=`$RECUR_BIN files "demo.**" --stdin --json -d $root`
        codes,out,err=pc_pipe(git,filter)
        @test codes==[0,0]
        @test isempty(err)
        files=JSON3.read(out)
        @test length(files)==2
        @test any(p->occursin("beta space",p),files)
        @test all(p->!occursin("other",p),files)
        codes,merged,err=pc_pipe(git,filter,`$RECUR_BIN merge --stdin --base demo --json`)
        @test codes==[0,0,0]
        @test isempty(err)
        @test JSON3.read(merged) isa JSON3.Object
    end
end
