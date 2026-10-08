# Optional web-evidence-lab suites, selected explicitly or run standalone.
module LangDogfoodTests
include(joinpath(@__DIR__, "..", "demos", "web-evidence-lab", "main.server.test.jl"))
include(joinpath(@__DIR__, "..", "demos", "web-evidence-lab", "main.greeting.fixtures.test.jl"))
include(joinpath(@__DIR__, "..", "demos", "web-evidence-lab", "main.lang.api.test.jl"))
end
