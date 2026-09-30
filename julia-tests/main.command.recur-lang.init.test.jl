# Standalone red-first acceptance contract. Do not include in runtests.jl yet.
# consumes: recur.lang.init.v1 CLI policy installation and preservation
module RecurLangInitContractTests
using Test, TOML, JSON3

const REPO = normpath(joinpath(@__DIR__,".."))
const EXT = Sys.iswindows() ? ".exe" : ""
const CORE = get(ENV,"RECUR_BIN",joinpath(REPO,"target",get(ENV,"RECUR_PROFILE","release-safe"),"recur"*EXT))
const ACTOR = get(ENV,"RECUR_LANG_BIN",joinpath(dirname(CORE),"recur-lang"*EXT))

function invoke(exe,args...)
    out,err = IOBuffer(),IOBuffer()
    p = run(pipeline(ignorestatus(Cmd([exe,args...])),stdout=out,stderr=err))
    text = String(take!(out))
    packet = try JSON3.read(text,Dict{String,Any}) catch; nothing end
    (code=p.exitcode,packet=packet,stdout=text,stderr=String(take!(err)))
end
init(root,args...) = invoke(ACTOR,"init","-d",root,args...,"--json")
function put(root,text)
    path = joinpath(root,".recur","config.toml")
    mkpath(dirname(path)); write(path,text); path
end
function snapshot(root)
    files = Dict{String,Vector{UInt8}}()
    dirs = String[]
    for (dir,children,names) in walkdir(root;follow_symlinks=false)
        append!(dirs,[relpath(joinpath(dir,c),root) for c in children])
        for file in names
            path = joinpath(dir,file)
            islink(path) || (files[relpath(path,root)] = read(path))
        end
    end
    (files=files,dirs=sort(dirs))
end
function successpacket(r)
    @test r.code == 0
    @test r.packet !== nothing
    isnothing(r.packet) && return nothing
    @test get(r.packet,"schema",nothing) == "recur-lang-init-v1"
    r.packet
end
function failurepacket(r,code)
    @test r.code == 2
    @test r.packet !== nothing
    isnothing(r.packet) && return
    @test get(r.packet,"schema",nothing) == "recur-lang-init-error-v1"
    ds = get(r.packet,"diagnostics",[])
    @test any(d -> get(d,"code",nothing) == code && !isempty(get(d,"message","")),ds)
end
const DEFAULT = Dict("schema_version"=>1,"target"=>"unspecified",
    "planning"=>Dict("specification_first"=>true,"prioritize_graph_findings"=>true,"include_test_plan"=>true))

@testset "recur-lang init v1 (standalone; expected red before implementation)" begin
    @testset "CLI discovery" begin
        result = invoke(ACTOR,"--help")
        @test result.code == 0
        @test occursin(r"(?m)^\s+init\s",result.stdout)
        help = invoke(ACTOR,"init","--help")
        @test help.code == 0
        @test occursin("--dry-run",help.stdout)
    end

    @testset "Fresh preview, install, exact defaults and repeat" begin
        mktempdir() do root
            before = snapshot(root)
            first = init(root,"--dry-run")
            packet = successpacket(first)
            @test snapshot(root) == before
            again = init(root,"--dry-run")
            @test first.stdout == again.stdout
            if !isnothing(packet)
                @test packet["state"] == "planned" && packet["changed"] === true && packet["dry_run"] === true
                @test length(packet["writes"]) == 1 && only(packet["writes"]) == packet["config_path"]
                @test TOML.parse(packet["preview"]) == Dict("recur-lang"=>DEFAULT)
            end
            written = successpacket(init(root))
            path = joinpath(root,".recur","config.toml")
            @test isfile(path)
            if isfile(path) && !isnothing(written)
                @test written["state"] == "written" && written["dry_run"] === false
                @test realpath(written["project_root"]) == realpath(root)
                @test realpath(written["config_path"]) == realpath(path)
                @test TOML.parsefile(path) == Dict("recur-lang"=>DEFAULT)
                @test read(path,String) == written["preview"]
                @test Set(keys(snapshot(root).files)) == Set([joinpath(".recur","config.toml")])
                before_repeat = snapshot(root)
                repeat = successpacket(init(root))
                @test snapshot(root) == before_repeat
                if !isnothing(repeat)
                    @test repeat["state"] == "unchanged" && repeat["changed"] === false
                    @test isempty(repeat["writes"])
                end
            end
        end
    end

    @testset "Inherited root, comments, false preferences and unrelated settings" begin
        mktempdir() do root
            initial = """
            # Keep this comment: Grüße / 東京
            [src]
            dir = "src"
            sep = "_"
            [status]
            current_suffix = ".current.md"
            [warp.creation]
            directory = "custom-warps"
            [reveal.skills]
            [recur-lang]
            target = "julia"
            custom_note = "emit no commands"
            [recur-lang.planning]
            specification_first = false
            [recur-lang.custom]
            owner = "Marc"
            """
            path = put(root,initial)
            child = joinpath(root,"nested","子 project"); mkpath(child)
            preview = successpacket(init(child,"--dry-run"))
            @test read(path,String) == initial
            packet = successpacket(init(child))
            @test !isdir(joinpath(child,".recur"))
            if !isnothing(packet)
                @test realpath(packet["config_path"]) == realpath(path)
                after = TOML.parsefile(path)
                old = TOML.parse(initial)
                for name in ["src","status","warp","reveal"]
                    @test after[name] == old[name]
                end
                @test after["recur-lang"]["target"] == "julia"
                @test after["recur-lang"]["planning"]["specification_first"] === false
                @test after["recur-lang"]["planning"]["include_test_plan"] === true
                @test after["recur-lang"]["planning"]["prioritize_graph_findings"] === true
                @test after["recur-lang"]["custom"] == old["recur-lang"]["custom"]
                @test after["recur-lang"]["custom_note"] == "emit no commands"
                @test occursin("# Keep this comment: Grüße / 東京",read(path,String))
                @test isnothing(preview) || preview["preview"] == read(path,String)
            end
        end
    end

    @testset "Inline and empty tables, no extra config file" begin
        for initial in ["[recur-lang]\n", "recur-lang = {target = 'rust', planning = {include_test_plan = false}}\n"]
            mktempdir() do root
                path = put(root,initial)
                packet = successpacket(init(root))
                if !isnothing(packet)
                    lang = TOML.parsefile(path)["recur-lang"]
                    @test lang["schema_version"] == 1
                    @test lang["planning"]["specification_first"] === true
                    @test lang["target"] == (occursin("rust",initial) ? "rust" : "unspecified")
                    @test lang["planning"]["include_test_plan"] === !occursin("false",initial)
                    before = snapshot(root)
                    successpacket(init(root))
                    @test snapshot(root) == before
                    @test !isfile(joinpath(root,".recur","recur-lang.toml"))
                end
            end
        end
    end

    @testset "Invalid nearest config is rejected without mutation" begin
        invalid = ["[broken", "recur-lang = 3", "[recur-lang]\nschema_version = 2",
            "[recur-lang]\nschema_version = true", "[recur-lang]\nschema_version = 1.0",
            "[recur-lang]\ntarget = 42", "[recur-lang]\ntarget = '  '",
            "[recur-lang]\nplanning = false",
            "[recur-lang.planning]\nspecification_first = 'true'",
            "[recur-lang.planning]\nprioritize_graph_findings = 1",
            "[recur-lang.planning]\ninclude_test_plan = []"]
        for text in invalid
            mktempdir() do root
                put(root,"[recur-lang]\ntarget = 'parent'\n")
                child = joinpath(root,"child"); mkpath(child); put(child,text)
                before = snapshot(root)
                failurepacket(init(child),"LINIT002")
                @test snapshot(root) == before
                failurepacket(init(child,"--dry-run"),"LINIT002")
                @test snapshot(root) == before
            end
        end
    end

    @testset "Invalid root and config directory" begin
        mktempdir() do root
            file = joinpath(root,"file.txt"); write(file,"keep")
            before = snapshot(root)
            failurepacket(init(joinpath(root,"missing")),"LINIT001")
            failurepacket(init(file),"LINIT001")
            @test snapshot(root) == before
            mkpath(joinpath(root,".recur","config.toml"))
            before = snapshot(root)
            failurepacket(init(root),"LINIT002")
            @test snapshot(root) == before
        end
    end

    @testset "Policy table must never be discovered as a file lane" begin
        mktempdir() do root
            put(root,"[recur-lang]\nschema_version=1\ntarget='custom-language'\ndir='trap'\nsep='_'\n")
            mkpath(joinpath(root,"trap"))
            write(joinpath(root,"trap","demo.file.txt"),"fixture")
            packet = successpacket(init(root))
            if !isnothing(packet)
                @test TOML.parsefile(joinpath(root,".recur","config.toml"))["recur-lang"]["dir"] == "trap"
                before = snapshot(root)
                result = invoke(CORE,"init","--analyze","-d",root,"--json")
                @test result.code == 0
                @test snapshot(root) == before
                @test result.packet !== nothing
                if !isnothing(result.packet)
                    @test !occursin("\"name\":\"recur-lang\"",JSON3.write(result.packet))
                end
            end
        end
    end

    @testset "Symlink escape, where the host permits symlink creation" begin
        mktempdir() do parent
            root=joinpath(parent,"project"); outside=joinpath(parent,"outside")
            mkpath(root); mkpath(outside)
            write(joinpath(outside,"config.toml"),"# untouched external configuration\n")
            linked = try
                symlink(outside,joinpath(root,".recur");dir_target=true); true
            catch error
                println("Symlink fixture unavailable: ",sprint(showerror,error)); false
            end
            if linked
                before=snapshot(outside)
                failurepacket(init(root),"LINIT003")
                @test snapshot(outside) == before
                @test islink(joinpath(root,".recur"))
            else
                @test_skip linked
            end
        end
    end
end
end
