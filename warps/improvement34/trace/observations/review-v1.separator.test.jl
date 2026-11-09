# Review counterexample: run explicitly; intentionally red until the scanner is fixed.
# trigger: main.improvement.34.implementation.trace.review.separator
include(joinpath(@__DIR__, "../../../../julia-tests/runtests.setup.jl"))

@testset "Trace path discovery keeps sentence punctuation separate" begin
    for separator in (".", "_", "-")
        mktempdir() do root
            identifier = join(["capability", "baseline"], separator)
            write(joinpath(root, "lab.reference.md"), "Reference " * identifier * ".\n")
            ok, output, errors = run_recur([
                "trace-id", identifier, "-d", root, "--sep", separator, "--json",
            ])
            @test ok && isempty(errors) &&
                  JSON3.read(output).files == ["lab.reference.md"]
        end
    end
end
