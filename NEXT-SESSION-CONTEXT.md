# Next Session Context - Julia Test Implementation

## Where We Left Off

This document provides complete context for continuing Julia test implementation in a new session.

## Current Status (as of 2026-01-09)

### ✅ Completed
1. **Julia test framework is fully functional**
   - Test environment auto-setup/teardown works
   - First test implemented and passing
   - 92 total tests defined (1 implemented, 88 placeholders)

2. **Test infrastructure**
   - File: `julia-tests/runtests.jl`
   - Helper function `run_recur(args)` executes recur commands
   - Test environment creates 18 hierarchical test files
   - Proper command output display with `→` prefix
   - Pass/fail status display with `✓ PASS` / `✗ FAIL`

3. **Documentation created**
   - `README.julia-tests.md` - Complete test framework guide
   - `TEST-COVERAGE-SUMMARY.md` - Overview of all 92 tests
   - `FEATURE-gaps.md` - Gap detection feature spec
   - `README.CORE.IMPROVEMENT4.md` - Future code intelligence features
   - `COMPARISON-powershell.md` - PowerShell vs recur comparison

### 📝 Next Steps

**Implement the next batch of tests in `julia-tests/runtests.jl`**

## Test Implementation Pattern

### Example: The First Implemented Test

Located at line ~135-158 in `runtests.jl`:

```julia
# Command: recur files "UserService.Handlers"
# Should match: UserService.Handlers.cs
# Should NOT match: UserService.Handlers.Create.cs (child)
@testset "files with exact pattern" begin
    success, output, _ = run_recur("files \"UserService.Handlers\"")

    passed = success &&
             contains(output, "UserService.Handlers.cs") &&
             !contains(output, "UserService.Handlers.Create.cs") &&
             !contains(output, "UserService.Handlers.Update.cs")

    println(passed ? "  ✓ PASS" : "  ✗ FAIL")

    @test success
    @test contains(output, "UserService.Handlers.cs")
    # Should not match children with additional segments
    @test !contains(output, "UserService.Handlers.Create.cs")
    @test !contains(output, "UserService.Handlers.Update.cs")
    log_test("exact pattern matching works")
end
```

### Key Pattern Elements

1. **Comment header** - Shows the exact command and expected behavior
2. **run_recur()** - Executes command, returns `(success, output, error)`
3. **Pre-compute pass/fail** - Check all conditions before assertions
4. **Status output** - Print `✓ PASS` or `✗ FAIL` immediately
5. **Assertions** - Use `@test` for each condition
6. **Log message** - Optional verbose message with `log_test()`

## Next Tests to Implement

### Priority Order

#### 1. Complete `files` command tests (9 remaining)

**Location**: Lines ~160-167

```julia
# TODO: Test wildcard pattern
@test_skip "files with * wildcard"

# TODO: Test recursive pattern
@test_skip "files with ** wildcard"
```

**Suggested implementation for "files with * wildcard"**:

```julia
# Command: recur files "UserService.*"
# Should match: UserService.Handlers.cs, UserService.Models.cs
# Should NOT match: UserService.Handlers.Create.cs (too deep)
@testset "files with * wildcard" begin
    success, output, _ = run_recur("files \"UserService.*\"")

    passed = success &&
             contains(output, "UserService.Handlers.cs") &&
             contains(output, "UserService.Models.cs") &&
             !contains(output, "UserService.Handlers.Create.cs")

    println(passed ? "  ✓ PASS" : "  ✗ FAIL")

    @test success
    @test contains(output, "UserService.Handlers.cs")
    @test contains(output, "UserService.Models.cs")
    @test !contains(output, "UserService.Handlers.Create.cs")
    log_test("single-segment wildcard works")
end
```

**Suggested implementation for "files with ** wildcard"**:

```julia
# Command: recur files "UserService.**"
# Should match all UserService files at any depth
@testset "files with ** wildcard" begin
    success, output, _ = run_recur("files \"UserService.**\"")

    passed = success &&
             contains(output, "UserService.cs") &&
             contains(output, "UserService.Handlers.cs") &&
             contains(output, "UserService.Handlers.Create.cs") &&
             contains(output, "UserService.Models.Request.cs")

    println(passed ? "  ✓ PASS" : "  ✗ FAIL")

    @test success
    @test contains(output, "UserService.cs")
    @test contains(output, "UserService.Handlers.cs")
    @test contains(output, "UserService.Handlers.Create.cs")
    @test contains(output, "UserService.Models.Request.cs")
    log_test("recursive wildcard works")
end
```

#### 2. Implement depth filtering tests (4 tests)

**Location**: Lines ~169-180

```julia
@testset "Depth filtering" begin
    # TODO: Test --min-depth
    @test_skip "files with --min-depth"

    # TODO: Test --max-depth
    @test_skip "files with --max-depth"

    # TODO: Test depth range
    @test_skip "files with depth range"

    # TODO: Test depth validation
    @test_skip "files depth validation"
end
```

**Suggested implementation for "--min-depth"**:

```julia
# Command: recur files "UserService.**" --min-depth 2
# Should match only files at depth 2+
# Should NOT match: UserService.cs (depth 0), UserService.Handlers.cs (depth 1)
@testset "files with --min-depth" begin
    success, output, _ = run_recur("files \"UserService.**\" --min-depth 2")

    passed = success &&
             contains(output, "UserService.Handlers.Create.cs") &&
             !contains(output, "UserService.cs") &&
             !contains(output, "UserService.Handlers.cs")

    println(passed ? "  ✓ PASS" : "  ✗ FAIL")

    @test success
    @test contains(output, "UserService.Handlers.Create.cs")
    @test !contains(output, "UserService.cs")
    @test !contains(output, "UserService.Handlers.cs")
    log_test("min-depth filtering works")
end
```

#### 3. Implement extension filtering tests (2 tests)

**Location**: Lines ~182-188

```julia
@testset "Extension filtering" begin
    # TODO: Test single extension
    @test_skip "files with single extension"

    # TODO: Test multiple extensions
    @test_skip "files with multiple extensions"
end
```

#### 4. Implement output format tests (2 tests)

**Location**: Lines ~190-196

```julia
@testset "Output formats" begin
    # TODO: Test --count
    @test_skip "files with --count"

    # TODO: Test --json
    @test_skip "files with --json"
end
```

## Test Environment Files

The test environment creates these files (in `test_environment/`):

```
UserService.cs                        # Depth 0
UserService.Handlers.cs               # Depth 1
UserService.Handlers.Create.cs        # Depth 2
UserService.Handlers.Update.cs        # Depth 2
UserService.Handlers.Delete.cs        # Depth 2
UserService.Models.cs                 # Depth 1
UserService.Models.Request.cs         # Depth 2
ApiController.cs                      # Depth 0
ApiController.Auth.cs                 # Depth 1
ApiController.Users.cs                # Depth 1
config.json                           # Depth 0
config.database.json                  # Depth 1
config.database.connection.json       # Depth 2
config.server.json                    # Depth 1
UserService.Tests.cs                  # With .Tests suffix
ApiController.Tests.cs                # With .Tests suffix
README.md                             # Depth 0
README.CORE.SECTION.md                # Depth 2 (gap - missing README.CORE.md)
```

## Helper Functions Available

### `run_recur(args::String) -> (success, output, error)`

Executes a recur command and returns results.

**Example:**
```julia
success, output, error = run_recur("files \"UserService.*\" --ext .cs")
```

**Returns:**
- `success::Bool` - true if exit code 0, false otherwise
- `output::String` - stdout content
- `error::String` - stderr content (if failed)

**Features:**
- Automatically adds `-d test_environment` to all commands
- Handles quoted arguments properly
- Displays command being run with `→` prefix

### `log_test(msg::String)`

Prints a message only in verbose mode.

**Example:**
```julia
log_test("wildcard pattern matching works")
```

### `log_section(msg::String)`

Prints a section header (always visible).

**Example:**
```julia
log_section("Testing: recur files")
```

## Running Tests

```bash
# Run all tests
cd julia-tests
julia runtests.jl

# Run with verbose output
julia runtests.jl --verbose
```

## Test Output Example

```
============================================================
  Testing: recur files
============================================================
  → recur files UserService.Handlers -d test_environment
  ✓ PASS
  → recur files "UserService.*" -d test_environment
  ✓ PASS
```

## Converting Placeholders to Tests

### Step 1: Find the placeholder

```julia
# TODO: Test wildcard pattern
@test_skip "files with * wildcard"
```

### Step 2: Replace with implementation

```julia
# Command: recur files "UserService.*"
# Should match: UserService.Handlers.cs, UserService.Models.cs
# Should NOT match: UserService.Handlers.Create.cs (too deep)
@testset "files with * wildcard" begin
    success, output, _ = run_recur("files \"UserService.*\"")

    passed = success &&
             contains(output, "UserService.Handlers.cs") &&
             contains(output, "UserService.Models.cs") &&
             !contains(output, "UserService.Handlers.Create.cs")

    println(passed ? "  ✓ PASS" : "  ✗ FAIL")

    @test success
    @test contains(output, "UserService.Handlers.cs")
    @test contains(output, "UserService.Models.cs")
    @test !contains(output, "UserService.Handlers.Create.cs")
    log_test("single-segment wildcard works")
end
```

### Step 3: Run tests to verify

```bash
cd julia-tests
julia runtests.jl
```

## Common Patterns

### Testing for Success with Specific Output

```julia
success, output, _ = run_recur("command args")
@test success
@test contains(output, "expected text")
```

### Testing for Failure (Error Cases)

```julia
success, output, error = run_recur("command --invalid-args")
@test !success  # Should fail
@test contains(error, "error message")
```

### Testing JSON Output

```julia
using JSON3

success, output, _ = run_recur("command --json")
@test success

data = JSON3.read(output)
@test data.total_files > 0
@test haskey(data, "expected_field")
```

### Testing Count Output

```julia
success, output, _ = run_recur("files \"**\" --count")
@test success
@test contains(output, r"\d+")  # Contains a number
```

## Important Notes

### File Paths in Output

On Windows, recur outputs paths with backslashes:
```
UserService\Handlers.cs
```

Use `contains()` with just the filename if path separators might vary:
```julia
@test contains(output, "UserService.Handlers.cs")
```

### Depth Counting

Depth is counted by dots in the hierarchical name, NOT filesystem directories:
- `UserService.cs` → depth 0
- `UserService.Handlers.cs` → depth 1
- `UserService.Handlers.Create.cs` → depth 2

### Extensions

When testing extension filtering:
```julia
run_recur("files \"**\" --ext .cs")
run_recur("files \"**\" --ext \".cs,.json\"")  # Multiple extensions
```

## Tips for Next Session

1. **Start with the simplest tests first** - Build confidence with easy wins
2. **Follow the established pattern** - Keep tests consistent
3. **Test one thing at a time** - Don't combine multiple features in one test
4. **Run tests frequently** - Verify each test as you implement it
5. **Update the count** - Keep `TEST-COVERAGE-SUMMARY.md` updated

## Recommended Session Goals

### Session 1: Complete `files` command (9 tests)
- ✅ files with exact pattern (DONE)
- ⏳ files with * wildcard
- ⏳ files with ** wildcard
- ⏳ files with --min-depth
- ⏳ files with --max-depth
- ⏳ files with depth range
- ⏳ files depth validation
- ⏳ files with single extension
- ⏳ files with multiple extensions
- ⏳ files with --count
- ⏳ files with --json

### Session 2: Complete `find` command (10 tests)
### Session 3: Complete `tree` command (5 tests)
### Session 4: Complete remaining core commands (27 tests)

## File Locations

- **Test file**: `c:\src\recur\julia-tests\runtests.jl`
- **Test README**: `c:\src\recur\README.julia-tests.md`
- **Coverage summary**: `c:\src\recur\TEST-COVERAGE-SUMMARY.md`
- **Binary**: `c:\src\recur\target\release\recur.exe`

## Quick Start for Next Session

```julia
# 1. Navigate to julia-tests
cd c:\src\recur\julia-tests

# 2. Open runtests.jl in editor
# Look for: @test_skip "files with * wildcard" (around line 161)

# 3. Replace placeholder with implementation (see examples above)

# 4. Run tests
julia runtests.jl

# 5. Verify new test passes
# Look for: ✓ PASS in output

# 6. Repeat for next test
```

## Questions to Ask User

If starting a new session, you might ask:

1. "Should I continue implementing the `files` command tests where we left off?"
2. "Do you want me to implement tests in a specific order, or continue with the priority list?"
3. "Should I implement tests one at a time with verification, or batch several together?"

## Success Criteria

A test is successfully implemented when:
- ✅ No syntax errors in Julia
- ✅ Test passes (shows `✓ PASS`)
- ✅ Follows the established pattern
- ✅ Has clear comments explaining what it tests
- ✅ Tests the right behavior for that feature

## End State Goal

When all 92 tests are implemented:
- 92 passing tests
- 0 broken/placeholder tests
- Complete test coverage of all recur features
- Confidence in refactoring and new features
- CI/CD ready

---

**Last Updated**: 2026-01-09
**Status**: 1/92 tests implemented (1.1% complete)
**Next Up**: Implement remaining 9 `files` command tests
