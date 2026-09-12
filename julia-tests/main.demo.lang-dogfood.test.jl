# Green dogfood suites share the normal runner and remain runnable standalone.
module LangDogfoodTests
include(joinpath(@__DIR__, "..", "demos", "web-evidence-lab", "main.server.test.jl"))
include(joinpath(@__DIR__, "..", "demos", "web-evidence-lab", "main.greeting.fixtures.test.jl"))
include(joinpath(@__DIR__, "..", "demos", "web-evidence-lab", "main.lang.api.test.jl"))
end
