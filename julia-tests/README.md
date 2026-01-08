# Recur Julia Integration Tests

Comprehensive integration test suite for the `recur` hierarchical search tool.

## Features

- **Automatic environment setup/teardown** - Creates test files, runs tests, cleans up
- **Comprehensive coverage** - Tests all commands with various options
- **Verbose logging** - Optional detailed output for debugging
- **Exit code validation** - Ensures proper exit codes (0/1/2)
- **JSON validation** - Parses and validates JSON output
- **Placeholder structure** - Easy to fill in actual test implementations

## Requirements

- Julia 1.6+ ([Download Julia](https://julialang.org/downloads/))
- `recur` binary built in release mode

## Setup

1. Build recur in release mode:
   ```bash
   cd ..
   cargo build --release
   ```

2. Install Julia dependencies (if any added later):
   ```bash
   julia --project=. -e 'using Pkg; Pkg.instantiate()'
   ```

## Usage

### Run all tests:
```bash
julia runtests.jl
```

### Run with verbose output:
```bash
julia runtests.jl --verbose
```

### Run specific test set:
```julia
julia -e 'include("runtests.jl"); @testset "Command: files" begin ... end'
```

## Test Structure

### Test Categories

1. **Command Tests**
   - `files` - Pattern matching, depth filtering, extensions
   - `find` - Text search, context lines, regex
   - `tree` - Visualization, depth limits, statistics
   - `related` - Sibling discovery, exclude-self
   - `children` - Descendant discovery, counting
   - `id` - Identifier search, context
   - `stats` - Statistics, depth levels, pagination

2. **Exit Code Tests**
   - Exit 0: Success with results
   - Exit 1: No results found
   - Exit 2: Errors (invalid args, etc.)

3. **Pattern Matching Tests**
   - Single wildcards (`*`)
   - Recursive wildcards (`**`)
   - Prefix/suffix patterns
   - Complex patterns

### Test File Hierarchy

The test suite creates this hierarchy:

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
└── ApiController.Tests.cs
```

## Adding Tests

To add a new test, replace `@test_skip` with actual test logic:

```julia
@testset "My new test" begin
    # Replace this:
    @test_skip "description"

    # With this:
    success, output, error = run_recur("files \"UserService.*\"")
    @test success
    @test contains(output, "UserService.Handlers.cs")
    log_test("My new test passed")
end
```

## Example Tests

### Test files command:
```julia
@testset "files with wildcard" begin
    success, output, _ = run_recur("files \"UserService.*\"")
    @test success
    @test contains(output, "UserService.Handlers.cs")
    @test contains(output, "UserService.Models.cs")
    log_test("files wildcard test passed")
end
```

### Test JSON output:
```julia
@testset "stats JSON output" begin
    success, output, _ = run_recur("stats \"UserService.**\" --json")
    @test success

    data = JSON3.read(output)
    @test data.total_files > 0
    @test data.max_depth >= 0
    log_test("stats JSON validation passed")
end
```

### Test exit codes:
```julia
@testset "exit code validation" begin
    # Success (exit 0)
    cmd = `$RECUR_BIN files "UserService.*" -d $TEST_DIR`
    @test success(cmd)

    # No results (exit 1)
    cmd = `$RECUR_BIN files "NonExistent.*" -d $TEST_DIR`
    @test !success(cmd)

    log_test("exit codes validated")
end
```

## Continuous Integration

Add to CI/CD pipeline:

```yaml
# .github/workflows/test.yml
- name: Run Julia tests
  run: |
    cd julia-tests
    julia runtests.jl
```

## Contributing

When adding new features to `recur`:

1. Add placeholder test in `runtests.jl`
2. Implement the feature
3. Fill in the test implementation
4. Run tests: `julia runtests.jl --verbose`
5. Ensure all tests pass

## License

MIT - Same as recur
