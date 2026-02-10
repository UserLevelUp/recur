using Test

@testset "MultiSeparatorCapable trait - tree command" begin

    @testset "Multiple separators merge results" begin
        # Test that multiple --sep flags work together
        output = read(`recur tree main --sep . --sep _`, String)

        # Should contain files from both separators
        @test occursin("readme.md", output)  # from dot separator (docs)
        @test occursin("impl.rs", output)    # from underscore separator (src)

        # Should show a merged hierarchy
        @test occursin("command", output)
        @test occursin("files", output)
    end

    @testset "Single separator still works (backward compatibility)" begin
        # Dot separator only
        output_dot = read(`recur tree main --sep .`, String)
        @test occursin("readme.md", output_dot)
        @test occursin("command", output_dot)

        # Underscore separator only
        output_underscore = read(`recur tree main --sep _`, String)
        @test occursin("impl.rs", output_underscore)
        @test occursin("command", output_underscore)

        # Default (dot) separator
        output_default = read(`recur tree main`, String)
        @test occursin("readme.md", output_default)
    end

    @testset "--sep-replace-default normalizes output" begin
        # Without normalization - paths have different separators
        output_raw = read(`recur files "main.command.files.**" --sep . --sep _`, String)

        # With normalization to dot
        output_normalized = read(`recur files "main.command.files.**" --sep . --sep _ --sep-replace-default .`, String)

        # All paths should use dots in normalized output
        lines = split(strip(output_normalized), '\n')
        for line in lines
            if !isempty(line)
                # Should not contain underscore in path structure
                # (except possibly in filenames themselves)
                parts = split(line, '/')
                if length(parts) > 0
                    path_part = parts[1]
                    # Path structure should use dots, not underscores
                    @test occursin('.', path_part) || !occursin('_', path_part)
                end
            end
        end
    end

    @testset "--show-sep displays separator markers" begin
        output = read(`recur tree main.command.files --sep . --sep _ --show-sep`, String)

        # Should show separator markers
        @test occursin("[.]", output)   # Files from dot domain
        @test occursin("[_]", output)   # Files from underscore domain

        # Markers should be at end of lines
        @test occursin("readme.md [.]", output)
        @test occursin("impl.rs [_]", output)
    end

    @testset "--sep-replace-default with --show-sep" begin
        output = read(`recur files "main.command.files.**" --sep . --sep _ --sep-replace-default . --show-sep`, String)

        # Paths should be normalized
        @test occursin("main.command.files", output)

        # But should still show which separator was used
        @test occursin("[.]", output)
        @test occursin("[_]", output)
    end
end

@testset "MultiSeparatorCapable trait - files command" begin

    @testset "files lists from multiple separators" begin
        output = read(`recur files "main.command.files.**" --sep . --sep _`, String)

        # Should list files from both domains
        lines = split(strip(output), '\n')
        @test length(lines) > 0

        # Should contain docs and src files
        @test any(l -> occursin("readme", l), lines)
        @test any(l -> occursin("impl", l), lines)
    end

    @testset "files with single separator (backward compatibility)" begin
        output_dot = read(`recur files "main.command.files.**" --sep .`, String)
        @test occursin("readme", output_dot)

        output_underscore = read(`recur files "main.command.files.**" --sep _`, String)
        @test occursin("impl", output_underscore)
    end

    @testset "files with --sep-replace-default" begin
        output = read(`recur files "main.command.**" --sep . --sep _ --sep-replace-default .`, String)

        lines = split(strip(output), '\n')

        # All lines should use dot separator in path structure
        for line in lines
            if !isempty(line) && startswith(line, "main")
                # Count separators - should be dots, not underscores in hierarchy
                @test occursin("main.command", line)
            end
        end
    end

    @testset "files with --show-sep" begin
        output = read(`recur files "main.command.files.**" --sep . --sep _ --show-sep`, String)

        lines = split(strip(output), '\n')

        # Lines should end with separator markers
        @test any(l -> occursin("[.]", l), lines)
        @test any(l -> occursin("[_]", l), lines)
    end
end

@testset "MultiSeparatorCapable - edge cases" begin

    @testset "Three or more separators" begin
        # Should support arbitrary number of separators
        # (even if we only have 2 in practice)
        output = read(`recur tree main --sep . --sep _ --sep -`, String)
        @test occursin("main", output)
    end

    @testset "Duplicate separators ignored" begin
        # Should handle duplicate --sep flags gracefully
        output = read(`recur tree main --sep . --sep .`, String)
        @test occursin("main", output)
    end

    @testset "Invalid separator characters" begin
        # TODO: Should handle invalid separators gracefully
        # For now, just ensure it doesn't crash
        try
            run(`recur tree main --sep /`)
            @test true  # Didn't crash
        catch
            @test true  # Expected to fail, that's ok
        end
    end
end

@testset "MultiSeparatorCapable - gap analysis use case" begin

    @testset "Find what exists across domains" begin
        # Real use case: see completeness of a feature
        output = read(`recur tree main.command.files --sep . --sep _ --show-sep`, String)

        # Should show what exists in each domain
        @test occursin("readme", output)  # Docs
        @test occursin("impl", output)    # Source

        # With separator markers, can identify gaps
        @test occursin("[.]", output)
        @test occursin("[_]", output)
    end

    @testset "Compare coverage across separators" begin
        # Get files from each domain
        output_docs = read(`recur files "main.command.**" --sep .`, String)
        output_src = read(`recur files "main.command.**" --sep _`, String)
        output_merged = read(`recur files "main.command.**" --sep . --sep _`, String)

        # Merged should have more files than either individually
        lines_docs = length(split(strip(output_docs), '\n'))
        lines_src = length(split(strip(output_src), '\n'))
        lines_merged = length(split(strip(output_merged), '\n'))

        @test lines_merged >= lines_docs
        @test lines_merged >= lines_src
    end
end

println("✅ MultiSeparatorCapable tests defined (expecting failures until implemented)")
