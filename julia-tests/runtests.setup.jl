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

    # MVC Controller Actions (Task<IActionResult> signatures) - IMPROVEMENT7 extra.tests
    # Demonstrates hierarchical partial class pattern for MVC controllers:
    # - LevelController.CreateWizard3.Ai.cs contains AI-related controller actions
    # - Uses partial classes to organize controller actions across hierarchy
    # - Each action method (GenerateAiContent, ApplyAiContent) represents a view/endpoint
    # - Pattern works with any separator: NameController.Feature.Subfeature.cs
    # - Enables deep hierarchies: Nation.State.County.City.cs or any depth needed
    create_test_file("LevelController.CreateWizard3.Ai.cs", """
    public partial class LevelController {
        // Controller action methods with Task<IActionResult> signatures
        // These should be recognized as symbols just like regular methods
        public async Task<IActionResult> GenerateAiContent(int wizardId) {
            CreateChildComponent(wizardId);
            return Ok();
        }
        public async Task<IActionResult> ApplyAiContent(int wizardId) {
            CreateChildComponent(wizardId);
            TruncateString("test");
            return Ok();
        }
        // Helper methods called by controller actions
        private async Task CreateChildComponent(int id) { }
        private string TruncateString(string input) { return input; }
    }
    """)

    # Example of deeper hierarchy with base controller
    create_test_file("LocationController.cs", """
    public class LocationController : ControllerBase {
        protected void ValidateLocation(string location) { }
    }
    """)
    create_test_file("LocationController.Nation.cs", """
    public partial class LocationController {
        public async Task<IActionResult> GetNations() {
            ValidateLocation("nation");
            return Ok();
        }
    }
    """)
    create_test_file("LocationController.Nation.State.cs", """
    public partial class LocationController {
        public async Task<IActionResult> GetStates(string nation) {
            ValidateLocation(nation);
            return Ok();
        }
    }
    """)

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
function run_recur(args::Vector{String})
    # Add test directory flag if not already present
    if !("-d" in args)
        push!(args, "-d")
        push!(args, TEST_DIR)
    end

    # Build command string for display
    display_cmd = join(map(arg -> contains(arg, ' ') ? "\"$arg\"" : arg, args), " ")
    println("  -> recur $display_cmd")

    # Run the command
    cmd = `$RECUR_BIN $args`
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

# Overload for string input (backward compatibility)
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

    return run_recur(args_vec)
end

# Run recur command with piped input
function run_recur_piped(input_cmd::Cmd, recur_args::Vector{String})
    # Build command string for display
    display_cmd = join(map(arg -> contains(arg, ' ') ? "\"$arg\"" : arg, recur_args), " ")
    println("  -> <piped> | recur $display_cmd")

    # Run the command with piped input
    cmd = pipeline(input_cmd, `$RECUR_BIN $recur_args`)
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

# Run recur command with stdin from string (Julia-compatible)
function run_recur_stdin(input_string::String, recur_args::Vector{String})
    # Build command string for display
    display_cmd = join(map(arg -> contains(arg, ' ') ? "\"$arg\"" : arg, recur_args), " ")
    println("  -> <stdin> | recur $display_cmd")

    # Run the command with stdin from string
    cmd = `$RECUR_BIN $recur_args`
    out = IOBuffer()
    err = IOBuffer()
    stdin_buf = IOBuffer(input_string)
    success = true

    try
        # Use pipeline to connect stdin, stdout, stderr
        run(pipeline(cmd, stdin=stdin_buf, stdout=out, stderr=err))
    catch e
        if isa(e, ProcessFailedException)
            success = false
        else
            return (false, "", "Error running command: $e")
        end
    end

    return (success, String(take!(out)), String(take!(err)))
end

# Helper to log command execution
function log_command(description::String, success::Bool)
    status = success ? "PASS" : "FAIL"
    println("  $status")
end

# Export all public functions
export setup_test_environment, teardown_test_environment, create_test_file
export run_recur, run_recur_piped, run_recur_stdin, log_command, log_test, log_section, log_error
export RECUR_BIN, TEST_DIR, VERBOSE
