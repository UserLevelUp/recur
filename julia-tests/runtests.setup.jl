"""
Test Environment Setup and Utilities
=====================================

Provides test environment creation, cleanup, and helper functions
for running recur commands in the test suite.

This module can be included by other test files or run standalone.
"""

using Test
using JSON3

# Configuration
const RECUR_PROFILE = get(ENV, "RECUR_PROFILE", "release")
const RECUR_BIN = get(
    ENV,
    "RECUR_BIN",
    joinpath(@__DIR__, "..", "target", RECUR_PROFILE, "recur" * (Sys.iswindows() ? ".exe" : ""))
)
const TEST_DIR = "test_environment"
const VERBOSE = "--verbose" in ARGS

# Logging utilities
function log_test(msg::String)
    VERBOSE && println("  ok  $msg")
end

function log_section(msg::String)
    println("\n" * "="^60)
    println("  $msg")
    println("="^60)
end

function log_error(msg::String)
    println("  ERROR: $msg")
end

# Setup test environment
function setup_test_environment()
    log_section("Setting up test environment")

    # Create test directory
    isdir(TEST_DIR) && rm(TEST_DIR, recursive=true)
    mkdir(TEST_DIR)

    log_test("Created test directory: $TEST_DIR")

    # Service hierarchy
    create_test_file("UserService.cs", "public class UserService { public void ValidateEmail(string email) { } }")
    create_test_file("UserService.Handlers.cs", "public class Handlers { public void ProcessRequest() { ValidateEmail(email); } }")
    create_test_file("UserService.Handlers.Create.cs", "public async Task CreateUser() { ValidateEmail(email); SaveUser(user); }")
    create_test_file("UserService.Handlers.Update.cs", "public async Task UpdateUser() { ValidateEmail(email); SaveUser(user); }")
    create_test_file("UserService.Handlers.Delete.cs", "public void DeleteUser() { SaveUser(user); }")
    create_test_file("UserService.Models.cs", "public class UserModel { }")
    create_test_file("UserService.Models.Request.cs", "public class UserRequest { }")

    # Controller hierarchy
    create_test_file("ApiController.cs", "public class ApiController { public void Initialize() { } }")
    create_test_file("ApiController.Auth.cs", "public async Task Authenticate() { Initialize(); ValidateEmail(email); }")
    create_test_file("ApiController.Users.cs", "public async Task GetUsers() { Initialize(); }")

    # Trace-specific hierarchy
    create_test_file("LevelController.CreateWizard3.cs", "public class LevelController { public void CreateWizard3() { ApplyTemplate(\"base\"); SaveWizard(); } public void SaveWizard() { } }")
    create_test_file("LevelController.CreateWizard3.Template.cs", "public partial class LevelController { public void ApplyTemplate(string name) { RenderTemplate(); } }")
    create_test_file("LevelController.CreateWizard3.TemplateAlt.cs", "public partial class LevelController { public void ApplyTemplate(int id) { RenderTemplate(); } }")
    create_test_file("LevelController.CreateWizard3.Rendering.cs", "public partial class LevelController { public void RenderTemplate() { } }")
    create_test_file("DynamicGameComponentService.Delete.cs", "public class DynamicGameComponentService { public void DeleteGameComponentAsync() { ValidatePermissions(); CleanupComponents(); } }")
    create_test_file("DynamicGameComponentService.Validation.cs", "public partial class DynamicGameComponentService { public void ValidatePermissions() { LogAccess(); } }")
    create_test_file("DynamicGameComponentService.cs", "public partial class DynamicGameComponentService { public void GetDeletedComponentsAsync() { } public void LogAccess() { } }")
    create_test_file("MaintenanceService.cs", "public class MaintenanceService { public void CleanupComponents() { GetDeletedComponentsAsync(); } }")
    create_test_file("AddComponent.cshtml", "<div data-tab=\"CreateWizard3.Tab\"></div>")
    create_test_file("CycleService.cs", """
    public class CycleService {
        public void FunctionA() { FunctionB(); }
        public void FunctionB() { FunctionA(); }
    }
    """)
    create_test_file("WideService.cs", """
    public class WideService {
        public void WideRoot() {
            CallA();
            CallB();
            CallC();
        }
        public void CallA() { }
        public void CallB() { }
        public void CallC() { }
    }
    """)

    # Config hierarchy
    create_test_file("config.json", "{\"database\": {\"connection\": \"test\"}}")
    create_test_file("config.database.json", "{\"host\": \"localhost\"}")
    create_test_file("config.database.connection.json", "{\"timeout\": 30}")
    create_test_file("config.server.json", "{\"port\": 8080}")

    # Test files
    create_test_file("UserService.Tests.cs", "public class UserServiceTests { }")
    create_test_file("ApiController.Tests.cs", "public class ApiControllerTests { }")

    # Files with gaps (missing intermediate levels)
    # This creates: README.md, README.CORE.SECTION.md
    # Missing: README.CORE.md (gap at level 1)
    create_test_file("README.md", "# Main README")
    create_test_file("README.CORE.SECTION.md", "# Core Section (missing README.CORE.md)")

    # Multi-line file for testing context
    create_test_file("TestContext.cs", """
    line 1: start of file
    line 2: some code
    line 3: public async Task TestMethod()
    line 4: more code
    line 5: end of file
    """)

    log_test("Created $(length(readdir(TEST_DIR))) test files")
end

function create_test_file(filename::String, content::String)
    filepath = joinpath(TEST_DIR, filename)
    write(filepath, content)
    VERBOSE && println("    Created: $filename")
end

# Teardown test environment
function teardown_test_environment()
    log_section("Cleaning up test environment")

    if isdir(TEST_DIR)
        rm(TEST_DIR, recursive=true)
        log_test("Removed test directory: $TEST_DIR")
    end
end

# Run recur command and capture output
function run_recur(args::String)
    # Split args properly for command execution, preserving quoted strings
    args_vec = String[]
    current = ""
    in_quotes = false

    for c in args
        if c == '"'
            in_quotes = !in_quotes
        elseif c == ' ' && !in_quotes
            if !isempty(current)
                push!(args_vec, current)
                current = ""
            end
        else
            current *= c
        end
    end

    # Don't forget the last argument
    if !isempty(current)
        push!(args_vec, current)
    end

    # Add test directory flag
    push!(args_vec, "-d")
    push!(args_vec, TEST_DIR)

    # Build command string for display
    display_cmd = join(map(arg -> contains(arg, ' ') ? "\"$arg\"" : arg, args_vec), " ")
    println("  -> recur $display_cmd")

    # Run the command
    cmd = `$RECUR_BIN $args_vec`
    out = IOBuffer()
    err = IOBuffer()
    success = true

    try
        run(pipeline(cmd, stdout=out, stderr=err))
    catch e
        if isa(e, ProcessFailedException)
            success = false
        else
            return (false, "", "Error running command: $e")
        end
    end

    return (success, String(take!(out)), String(take!(err)))
end

# Export all public functions
export setup_test_environment, teardown_test_environment, create_test_file
export run_recur, log_test, log_section, log_error
export RECUR_BIN, TEST_DIR, VERBOSE
