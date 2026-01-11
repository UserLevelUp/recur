#!/usr/bin/env julia

const VERSION_FILE = joinpath(@__DIR__, "..", "VERSION")
const CARGO_TOML = joinpath(@__DIR__, "..", "Cargo.toml")

function read_version(path::AbstractString)
    text = strip(read(path, String))
    m = match(r"^([a-z])\.(\d{1,2})\.(\d{1,2})\.(\d{1,2})$", text)
    m === nothing && error("Expected VERSION like a.0.1.1, got: $(text)")

    captures = m.captures
    letter = captures[1]
    letter === nothing && error("Missing maturity letter in VERSION.")

    nums = Int[]
    for i in 2:4
        value = captures[i]
        value === nothing && error("Missing numeric part in VERSION.")
        push!(nums, parse(Int, value))
    end

    for n in nums
        (0 <= n <= 99) || error("Version parts must be 0-99, got: $(n)")
    end

    return String(letter), nums
end

function bump_numbers!(nums::Vector{Int})
    carry = 1
    for i in reverse(eachindex(nums))
        if carry == 0
            break
        end
        if nums[i] == 99
            nums[i] = 0
            carry = 1
        else
            nums[i] += 1
            carry = 0
        end
    end
    return carry == 0
end

function update_cargo_toml!(path::AbstractString, numeric_version::AbstractString)
    content = read(path, String)
    lines = split(content, '\n'; keepempty=true)
    in_package = false
    updated = false

    for i in eachindex(lines)
        line = lines[i]
        trimmed = strip(line)

        if startswith(trimmed, "[")
            in_package = trimmed == "[package]"
            continue
        end

        if in_package && occursin(r"^version\s*=", trimmed)
            indent = match(r"^(\s*)", line).captures[1]
            lines[i] = string(indent, "version = \"", numeric_version, "\"")
            updated = true
            break
        end
    end

    updated || error("Could not find [package] version in Cargo.toml.")
    write(path, join(lines, "\n"))
end

letter, nums = read_version(VERSION_FILE)
old_version = string(letter, ".", nums[1], ".", nums[2], ".", nums[3])

ok = bump_numbers!(nums)
ok || error("Version overflow; update maturity or reset numbers manually.")

new_version = string(letter, ".", nums[1], ".", nums[2], ".", nums[3])
numeric_version = string(nums[1], ".", nums[2], ".", nums[3])
write(VERSION_FILE, new_version * "\n")
update_cargo_toml!(CARGO_TOML, numeric_version)
println("Bumped VERSION: ", old_version, " -> ", new_version)
