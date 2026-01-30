# setup.packages.jl
# Install required Julia packages for recur test suite

import Pkg

println("Installing recur Julia test dependencies...")
println("Note: Test module is part of Julia's standard library and does NOT need Pkg.add().")

packages = [
    "JSON3"         # JSON handling for test data
]

stdlibs = [
    "Test"          # Testing framework (stdlib)
]

for pkg in packages
    println("Installing $pkg...")
    try
        Pkg.add(pkg)
    catch ex
        println("  Skipped $pkg (already installed or unavailable): $(ex)")
    end
end

println("Stdlib modules available without install: $(join(stdlibs, ", "))")
println("OK: dependencies installed.")
