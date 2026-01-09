# Julia Integration Tests for Recur

This document explains the Julia-based integration test suite for `recur`.

## Why Julia?

Julia was chosen for the integration test framework because:

1. **Excellent file system capabilities** - Built-in functions for creating/managing test environments
2. **Simple test syntax** - Clear, readable test assertions with `@test` and `@testset`
3. **Process execution** - Easy command execution and output capture
4. **Fast startup** - Reasonable startup time for a test suite
5. **Cross-platform** - Works on Windows, macOS, and Linux

## Test Structure

### Location

All tests are in the `julia-tests/` directory:

```
julia-tests/
├── runtests.jl          # Main test suite
├── README.md            # Test-specific documentation
└── .gitignore          # Ignores test_environment/
```

### Test Organization

Tests are organized into functions by command:

- `run_files_tests()` - Tests for `recur files` command
- `run_find_tests()` - Tests for `recur find` command
- `run_tree_tests()` - Tests for `recur tree` command
- `run_related_tests()` - Tests for `recur related` command
- `run_children_tests()` - Tests for `recur children` command
- `run_id_tests()` - Tests for `recur id` command
- `run_stats_tests()` - Tests for `recur stats` command
- `run_gaps_tests()` - Tests for `recur gaps` command (detect missing intermediate files)
- `run_callers_tests()` - Tests for `recur callers` command (find call sites) [Future]
- `run_callees_tests()` - Tests for `recur callees` command (find what a function calls) [Future]
- `run_def_tests()` - Tests for `recur def` command (find definitions) [Future]
- `run_refs_tests()` - Tests for `recur refs` command (find all references) [Future]
- `run_scope_tests()` - Tests for `recur scope` command (scope aliases) [Future]
- `run_id_tree_tests()` - Tests for `recur id-tree` command (identifier tree visualization) [Future]
- `run_id_stats_tests()` - Tests for `recur id-stats` command (identifier statistics) [Future]
- `run_exit_code_tests()` - Exit code validation tests
- `run_pattern_tests()` - Pattern matching tests

## Running Tests

### Prerequisites

1. **Install Julia 1.6+**: [Download Julia](https://julialang.org/downloads/)
2. **Build recur in release mode**:
   ```bash
   cargo build --release
   ```

### Run All Tests

```bash
cd julia-tests
julia runtests.jl
```

### Run with Verbose Output

```bash
cd julia-tests
julia runtests.jl --verbose
```

Verbose mode shows:
- Each test file being created
- The exact command being executed
- Detailed pass/fail information

## Test Output Format

The test suite produces clean, readable output:

```
============================================================
  Testing: recur files
============================================================
  → recur files UserService.Handlers -d test_environment
  ✓ PASS
```

Each test shows:
1. **Command executed** - The exact `recur` command run (with `→`)
2. **Status** - `✓ PASS` or `✗ FAIL`
3. **Summary** - Overall test statistics at the end

## Test Environment

### Automatic Setup/Teardown

The test suite automatically:

1. **Creates** a `test_environment/` directory
2. **Populates** it with hierarchical test files
3. **Runs** all tests against these files
4. **Cleans up** by removing the test environment

### Test File Hierarchy

The test environment creates this structure:

```
test_environment/
├── UserService.cs
├── UserService.Handlers.cs
├── UserService.Handlers.Create.cs
├── UserService.Handlers.Update.cs
├── UserService.Handlers.Delete.cs
├── UserService.Models.cs
├── UserService.Models.Request.cs
├── ApiController.cs
├── ApiController.Auth.cs
├── ApiController.Users.cs
├── config.json
├── config.database.json
├── config.database.connection.json
├── config.server.json
├── UserService.Tests.cs
├── ApiController.Tests.cs
├── README.md
└── README.CORE.SECTION.md  (gap: missing README.CORE.md)
```

This hierarchy is designed to test:
- Exact pattern matching
- Wildcard patterns (`*` and `**`)
- Depth-based filtering
- Extension filtering
- Sibling/child relationships
- Gap detection (missing intermediate levels)

## Writing Tests

### Test Format

Each test follows this pattern:

```julia
# Command: recur files "UserService.Handlers"
# Should match: UserService.Handlers.cs
# Should NOT match: UserService.Handlers.Create.cs (child)
@testset "files with exact pattern" begin
    success, output, _ = run_recur("files \"UserService.Handlers\"")

    passed = success &&
             contains(output, "UserService.Handlers.cs") &&
             !contains(output, "UserService.Handlers.Create.cs")

    println(passed ? "  ✓ PASS" : "  ✗ FAIL")

    @test success
    @test contains(output, "UserService.Handlers.cs")
    @test !contains(output, "UserService.Handlers.Create.cs")
    log_test("exact pattern matching works")
end
```

### Key Components

1. **Comment header** - Shows the command being tested and expected behavior
2. **run_recur()** - Helper function that executes recur and captures output
3. **Status check** - Pre-computes pass/fail for display
4. **Status output** - Prints `✓ PASS` or `✗ FAIL`
5. **Assertions** - Multiple `@test` calls to validate behavior
6. **Log message** - Optional verbose logging with `log_test()`

### Helper Functions

#### `run_recur(args::String)`

Executes a recur command and returns `(success, output, error)`:

```julia
success, output, error = run_recur("files \"UserService.*\"")
```

- **success** - `true` if command succeeded (exit code 0), `false` otherwise
- **output** - String containing stdout
- **error** - String containing stderr (if failed)

#### `log_test(msg::String)`

Prints a message only in verbose mode:

```julia
log_test("pattern matching works correctly")
```

#### `log_section(msg::String)`

Prints a section header (always visible):

```julia
log_section("Testing: recur files")
```

### Placeholder Tests

Tests not yet implemented use `@test_skip`:

```julia
# TODO: Test wildcard pattern
@test_skip "files with * wildcard"
```

These show as "Broken" in the test summary, indicating they're planned but not yet implemented.

## Test Status

The test suite uses Julia's built-in test framework which reports:

- **Pass** - Test assertions succeeded
- **Fail** - Test assertions failed
- **Broken** - Test is a placeholder (`@test_skip`)
- **Error** - Test crashed or threw an exception

### Current Coverage

As of now:
- ✅ 1 implemented test (files with exact pattern)
- 📝 88 placeholder tests waiting for implementation
  - 56 tests for current/planned features
  - 32 tests for future code intelligence features (callers, callees, def, refs, scope, id-tree, id-stats)

### Example Summary

```
Test Summary:           | Pass  Broken  Total
Recur Integration Tests |    4      88     92
  Command: files        |    4      10     14
  Command: find         |           10     10
  Command: callers      |            4      4
  Command: callees      |            3      3
  Command: def          |            5      5
  Command: refs         |            4      4
  Command: scope        |            9      9
  Command: id-tree      |            5      5
  Command: id-stats     |            3      3
  Command: tree         |            5      5
  ...
```

## Adding New Tests

To add a new test:

1. **Find the appropriate test function** (e.g., `run_files_tests()`)
2. **Locate the `@test_skip` placeholder** for your test
3. **Replace with actual test code**:

```julia
# Before:
@test_skip "files with * wildcard"

# After:
# Command: recur files "UserService.*"
# Should match: UserService.Handlers.cs, UserService.Models.cs
@testset "files with * wildcard" begin
    success, output, _ = run_recur("files \"UserService.*\"")

    passed = success &&
             contains(output, "UserService.Handlers.cs") &&
             contains(output, "UserService.Models.cs")

    println(passed ? "  ✓ PASS" : "  ✗ FAIL")

    @test success
    @test contains(output, "UserService.Handlers.cs")
    @test contains(output, "UserService.Models.cs")
    log_test("wildcard pattern matching works")
end
```

## Continuous Integration

To run tests in CI/CD:

```yaml
# .github/workflows/test.yml
- name: Install Julia
  uses: julia-actions/setup-julia@v1
  with:
    version: '1.6'

- name: Build recur
  run: cargo build --release

- name: Run Julia integration tests
  run: |
    cd julia-tests
    julia runtests.jl
```

## Exit Codes

The test suite validates that recur returns correct exit codes:

- **0** - Success, results found
- **1** - No results found (not an error)
- **2** - Error (invalid arguments, etc.)

Example test:

```julia
@testset "exit code 0 on success" begin
    cmd = `$RECUR_BIN files "UserService.*" -d $TEST_DIR`
    @test success(cmd)  # Verifies exit code 0
end
```

## JSON Validation

Tests can validate JSON output:

```julia
using JSON3

@testset "stats JSON output" begin
    success, output, _ = run_recur("stats \"UserService.**\" --json")
    @test success

    data = JSON3.read(output)
    @test data.total_files > 0
    @test data.max_depth >= 0
end
```

## Best Practices

1. **Always show the command** - Include a comment showing the exact recur command
2. **Document expectations** - Explain what should/shouldn't match
3. **Check success first** - Verify the command succeeded before checking output
4. **Test edge cases** - Include tests for empty results, errors, etc.
5. **Keep tests focused** - Each test should verify one specific behavior
6. **Use descriptive names** - Test names should clearly indicate what's being tested

## Troubleshooting

### Tests fail unexpectedly

1. **Rebuild recur**: `cargo build --release`
2. **Check binary path**: Ensure `../target/release/recur` exists
3. **Run verbose**: `julia runtests.jl --verbose` to see details

### Environment issues

1. **Clean test environment**: Remove `test_environment/` if it exists
2. **Check permissions**: Ensure you can create/delete files in `julia-tests/`

### Julia not found

Install Julia from [julialang.org](https://julialang.org/downloads/) and ensure it's in your PATH.

## Contributing Tests

When adding a new feature to recur:

1. Add a `@test_skip` placeholder in the appropriate test function
2. Implement the feature
3. Replace the placeholder with actual test code
4. Run tests to verify: `julia runtests.jl`
5. Commit both the feature and the tests together

## License

MIT - Same as recur
