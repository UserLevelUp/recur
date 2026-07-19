"""Standalone regression for wildcard tree rendering of current-state leaves."""

include("runtests.setup.jl")

@testset "recur tree wildcard current" begin
    mktempdir() do root
        write(joinpath(root, "main.alpha.current.md"), "alpha current")
        write(joinpath(root, "main.beta.inner.current.md"), "beta current")
        write(joinpath(root, "main.beta.current.child.md"), "current child")
        write(joinpath(root, "other.alpha.current.md"), "outside main")

        success, output, error_output = run_recur([
            "tree", "main.**.current.**", "-d", root, "--json",
        ])

        @test success
        @test error_output == ""
        tree = JSON3.read(output)
        @test String(tree["name"]) == "main"
        @test contains(output, "main.alpha.current.md")
        @test contains(output, "main.beta.inner.current.md")
        @test contains(output, "main.beta.current.child.md")
        @test !contains(output, "other.alpha.current.md")
    end
end
