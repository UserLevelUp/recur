# Feature Proposal: `recur gaps` - Detect Missing Intermediate Files

## Problem

In hierarchical file naming, it's easy to accidentally skip intermediate levels:

```
README.md                        ← Level 0 (exists)
README.CORE.SECTION.md           ← Level 2 (exists)
README.CORE.md                   ← Level 1 (MISSING - gap!)
```

This creates "gaps" in the hierarchy where intermediate files should exist but don't. These gaps can indicate:

1. **Incomplete documentation** - Missing intermediate docs between high-level and detailed docs
2. **Broken conventions** - Violation of naming patterns
3. **Refactoring artifacts** - Files that were moved/deleted but children remain
4. **Organizational issues** - Need for intermediate organization files

## Solution: `recur gaps` Command

A new command that analyzes hierarchical file structures and reports missing intermediate levels.

### Basic Usage

```bash
# Find all gaps in current directory
recur gaps "**"

# Find gaps in specific hierarchy
recur gaps "README.**"

# Find gaps with detailed missing file suggestions
recur gaps "**" --show-missing
```

### Example Output

```bash
$ recur gaps "**"

Gap detected in README hierarchy:
  README.md (level 0)
  README.CORE.md (level 1) ← MISSING
  README.CORE.SECTION.md (level 2)

Gap detected in UserService hierarchy:
  UserService.cs (level 0)
  UserService.Advanced.cs (level 1) ← MISSING
  UserService.Advanced.Features.cs (level 2)

Found 2 hierarchies with gaps (2 missing files)
```

### Example Output with `--show-missing`

```bash
$ recur gaps "**" --show-missing

README hierarchy:
  Missing: README.CORE.md
    Would connect: README.md → README.CORE.SECTION.md

UserService hierarchy:
  Missing: UserService.Advanced.cs
    Would connect: UserService.cs → UserService.Advanced.Features.cs
```

## Algorithm

1. **Collect all matching files** using the pattern
2. **Parse hierarchical structure** from filenames (split by `.`)
3. **Build hierarchy tree** with depth levels
4. **Detect gaps** - for each file, check if all parent levels exist
5. **Report missing intermediates** - list files that should exist

### Gap Detection Logic

For a file like `Module.Feature.Detail.cs`:
- Split: `["Module", "Feature", "Detail"]`
- Required parents:
  - `Module.cs` (level 0)
  - `Module.Feature.cs` (level 1)
- Check each parent exists
- Report any missing

## Command Specification

```
recur gaps <PATTERN> [OPTIONS]

Find missing intermediate files in hierarchical naming structures

Arguments:
  <PATTERN>  File pattern to search (e.g., "**" or "Module.**")

Options:
  --show-missing       List specific missing intermediate files
  --json              Output results as JSON
  -d, --dir <DIR>     Directory to search (default: current)
  --ext <EXT>         Filter by extension (e.g., .cs, .md)
  -h, --help          Print help
```

## Exit Codes

- **0** - Gaps found (success - found what we searched for)
- **1** - No gaps found (all hierarchies complete)
- **2** - Error (invalid arguments, etc.)

## JSON Output Format

```json
{
  "total_gaps": 2,
  "total_missing_files": 2,
  "gaps": [
    {
      "hierarchy_root": "README",
      "missing_files": [
        {
          "path": "README.CORE.md",
          "depth": 1,
          "connects": {
            "parent": "README.md",
            "child": "README.CORE.SECTION.md"
          }
        }
      ]
    },
    {
      "hierarchy_root": "UserService",
      "missing_files": [
        {
          "path": "UserService.Advanced.cs",
          "depth": 1,
          "connects": {
            "parent": "UserService.cs",
            "child": "UserService.Advanced.Features.cs"
          }
        }
      ]
    }
  ]
}
```

## Use Cases

### 1. Documentation Verification

```bash
# Check if all README levels exist
recur gaps "README.**"
```

**Detects:**
- `README.md` exists
- `README.API.ENDPOINTS.md` exists
- Missing: `README.API.md`

### 2. Code Organization

```bash
# Verify service hierarchy is complete
recur gaps "UserService.**" --ext .cs
```

**Detects:**
- Partial implementations where detail files exist but no intermediate module file

### 3. Refactoring Validation

```bash
# After refactoring, check for orphaned deep files
recur gaps "**" --show-missing
```

**Detects:**
- Files that were moved/deleted but their children remain

### 4. CI/CD Integration

```bash
# Fail build if gaps detected
recur gaps "src/**" --json > gaps.json
if [ $? -eq 0 ]; then
  echo "ERROR: Gaps detected in hierarchy"
  exit 1
fi
```

## Implementation Notes

### Core Logic

```rust
struct Gap {
    hierarchy_root: String,
    missing_file: String,
    depth: usize,
    parent: Option<String>,
    child: String,
}

fn detect_gaps(files: Vec<PathBuf>, pattern: &str) -> Vec<Gap> {
    // 1. Group files by hierarchy root (first segment)
    // 2. For each hierarchy:
    //    - Build depth map: depth -> Vec<files>
    //    - For each file at depth N:
    //      - Check all levels 0..N exist
    //      - Record missing levels as gaps
    // 3. Return all gaps found
}
```

### Edge Cases

1. **Extension handling** - Gap at `Module.Feature` needs `.cs` if checking `.cs` files
2. **Multiple extensions** - What if `Module.cs` exists but `Module.Feature.md` is missing `Module.md`?
3. **Naming conflicts** - Handle both `Module.cs` and `Module.md` existing
4. **Deep hierarchies** - Efficiently check many levels

## Testing

Test cases in `julia-tests/runtests.jl`:

```julia
@testset "Command: gaps" begin
    # Test 1: Basic gap detection
    # Files: README.md, README.CORE.SECTION.md
    # Missing: README.CORE.md
    # Expected: 1 gap found, exit code 0

    # Test 2: No gaps
    # Files: Module.cs, Module.Feature.cs, Module.Feature.Detail.cs
    # Expected: No gaps, exit code 1

    # Test 3: Pattern scoping
    # Test gaps within specific pattern only

    # Test 4: JSON output
    # Validate JSON structure
end
```

## Related Commands

- `recur files` - Find files (may reveal incomplete hierarchies visually)
- `recur tree` - Visualize hierarchy (gaps visible as missing nodes)
- `recur stats` - Statistics by depth (unusual depth distributions suggest gaps)
- `recur children` - List children (may show orphaned children)

## Benefits

1. **Quality control** - Catch incomplete hierarchies early
2. **Documentation completeness** - Ensure all README levels exist
3. **Refactoring safety** - Detect when intermediate files were accidentally deleted
4. **Convention enforcement** - Maintain consistent hierarchical naming
5. **CI/CD integration** - Automated gap detection in builds

## Future Enhancements

1. **Auto-fix mode** - `--create-missing` to generate stub files
2. **Ignore patterns** - `--ignore-depth 0` to only check certain levels
3. **Template support** - Use templates for generated stub files
4. **Suggestion mode** - Suggest content for missing files based on siblings

## Priority

**Medium** - Useful quality-of-life feature, not critical for core functionality

## Dependencies

- Builds on existing pattern matching system
- Reuses hierarchical parsing from `files`, `tree`, `related` commands
- No new external dependencies required

## Example Real-World Scenario

Your codebase:
```
UserService.cs                           ← Level 0 ✓
UserService.Handlers.cs                  ← Level 1 ✓
UserService.Handlers.Create.cs           ← Level 2 ✓
UserService.Handlers.Update.cs           ← Level 2 ✓
UserService.Models.Request.cs            ← Level 2 ✓
```

Gap detected:
```
UserService.Models.cs is MISSING (level 1)
```

This helps you realize you forgot to create the `UserService.Models.cs` intermediate file.

## Conclusion

The `gaps` command fills a quality assurance niche by detecting structural inconsistencies in hierarchical file naming. It's particularly valuable for:

- Documentation projects with deep README hierarchies
- Large codebases with namespaced module files
- Teams enforcing hierarchical naming conventions
- CI/CD pipelines ensuring code organization standards

The feature leverages recur's existing hierarchical understanding to provide a unique capability that grep, find, and even ripgrep cannot offer.
