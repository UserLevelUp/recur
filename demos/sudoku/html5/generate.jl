# Generate a validated additive playable package without overwriting legacy data.
# Usage: julia demos/sudoku/html5/generate.jl [output-directory]
include(joinpath(@__DIR__, "..", "julia", "Recur.jl"))
include(joinpath(@__DIR__, "..", "julia", "Generator.jl"))
const OUTPUT_DIR = isempty(ARGS) ? joinpath(@__DIR__, "data", "easy-001") : abspath(ARGS[1])
package = Generator.publish_playable(OUTPUT_DIR, Recur)
println("Published sudoku.playable.json: ", package["puzzle_id"])
for key in ("easy", "medium", "hard")
    preset = package["presets"][key]
    println(key, ": ", preset["gaps"], " gaps; ", preset["grade"]["label"])
end
