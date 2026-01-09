# Core Improvement 4: Code Intelligence & Symbol Navigation

## Overview

These enhancements transform `recur` from a hierarchical file search tool into a lightweight code intelligence platform, supporting workflows like "find definition → find callers → trace usage".

## 1. Call Graph Within Scope

### Problem

Common workflow in large codebases:
1. Find a method definition: `recur find "DeleteGameComponentAsync"`
2. Find who calls it: Manual grep/search through results
3. Trace call chain: Repeat for each caller

This is tedious and requires multiple search operations.

### Solution: `recur callers` and `recur callees`

Find function calls and invocations within hierarchical scopes.

#### Command: `recur callers`

Find all locations that call a given function/method.

```bash
# Find who calls DeleteGameComponentAsync within DynamicGameComponentService
recur callers "DeleteGameComponentAsync" \
  --scope "User Level Up Services\\DynamicGameComponentService.*" \
  --ext ".cs"
```

**Output:**
```
User Level Up Services\DynamicGameComponentService.Handlers.cs:42
  await DeleteGameComponentAsync(componentId);

User Level Up Services\DynamicGameComponentService.Cleanup.cs:87
  result = DeleteGameComponentAsync(id, force: true);

Found 2 call sites
```

#### Command: `recur callees`

Find all functions/methods called by a given function.

```bash
# Find what ApplyAiContent calls
recur callees "ApplyAiContent" \
  --scope "User Level Up\\Controllers\\LevelController.*" \
  --ext ".cs"
```

**Output:**
```
User Level Up\Controllers\LevelController.CreateWizard3.cs:156
  → ValidateAiContent(content)
  → ProcessQuestions(aiData.Questions)
  → SaveToDatabase(processed)

Found 3 callees in ApplyAiContent
```

#### Implementation Notes

**Heuristic-based (Phase 1):**
- Regex patterns for common call syntax:
  - C#: `methodName(...)`, `await methodName(...)`, `object.methodName(...)`
  - JavaScript: `functionName(...)`, `obj.method(...)`
  - Python: `function_name(...)`, `self.method(...)`
- Fast, no compilation required
- ~80% accuracy (good enough for exploration)

**AST-based (Phase 2 - Future):**
- Use tree-sitter for accurate parsing
- Language-specific parsers
- 99% accuracy
- Slower, requires language detection

#### CLI Specification

```
recur callers <SYMBOL> [OPTIONS]

Find call sites for a function/method

Arguments:
  <SYMBOL>  Function/method name to find callers for

Options:
  --scope <PATTERN>    Limit search to hierarchical scope
  --ext <EXT>          Filter by file extension
  -C <NUM>             Show N lines of context
  --json               Output as JSON
  -d, --dir <DIR>      Directory to search
  --language <LANG>    Hint language (auto-detected if omitted)
  -h, --help           Print help

recur callees <SYMBOL> [OPTIONS]

Find what a function/method calls

Arguments:
  <SYMBOL>  Function/method name to find callees of

Options:
  (same as callers)
```

#### JSON Output Format

```json
{
  "symbol": "DeleteGameComponentAsync",
  "call_sites": [
    {
      "file": "User Level Up Services\\DynamicGameComponentService.Handlers.cs",
      "line": 42,
      "column": 10,
      "context": "await DeleteGameComponentAsync(componentId);",
      "caller_function": "ProcessDeletion"
    }
  ],
  "total_sites": 2
}
```

---

## 2. Symbol Jump / Definition Resolution

### Problem

Finding where a symbol is defined requires:
1. Manual grep for "class SymbolName" or "function SymbolName"
2. Filtering through results
3. Multiple patterns for different languages

### Solution: `recur def` and `recur refs`

Lightweight symbol indexing - "ctags-lite" with hierarchical scoping.

#### Command: `recur def`

Find definition(s) of a symbol.

```bash
# Find where DeleteGameComponentAsync is defined
recur def "DeleteGameComponentAsync"
```

**Output:**
```
User Level Up Services\DynamicGameComponentService.cs:234
  public async Task DeleteGameComponentAsync(int id)

Found 1 definition
```

With context:
```bash
recur def "DeleteGameComponentAsync" -C 3
```

**Output:**
```
User Level Up Services\DynamicGameComponentService.cs:232-237
  232  /// <summary>Soft-deletes a game component</summary>
  233  /// <param name="id">Component ID</param>
  234: public async Task DeleteGameComponentAsync(int id)
  235  {
  236      var component = await _db.Components.FindAsync(id);
  237      component.IsDeleted = true;
```

#### Command: `recur refs`

Find all references/usages of a symbol.

```bash
# Find all usages of DeleteGameComponentAsync
recur refs "DeleteGameComponentAsync"
```

**Output:**
```
User Level Up Services\DynamicGameComponentService.cs:234
  [DEFINITION] public async Task DeleteGameComponentAsync(int id)

User Level Up Services\DynamicGameComponentService.Handlers.cs:42
  [CALL] await DeleteGameComponentAsync(componentId);

User Level Up Services\DynamicGameComponentService.Tests.cs:156
  [CALL] await service.DeleteGameComponentAsync(testId);

Found 1 definition, 2 references
```

#### CLI Specification

```
recur def <SYMBOL> [OPTIONS]

Find definition(s) of a symbol

Arguments:
  <SYMBOL>  Symbol name (class, function, method, etc.)

Options:
  --scope <PATTERN>    Limit search to hierarchical scope
  --ext <EXT>          Filter by file extension
  -C <NUM>             Show N lines of context
  --json               Output as JSON
  --all                Show all definitions (don't stop at first)
  -d, --dir <DIR>      Directory to search
  -h, --help           Print help

recur refs <SYMBOL> [OPTIONS]

Find all references to a symbol

Arguments:
  <SYMBOL>  Symbol name to find references for

Options:
  (same as def, plus:)
  --include-def        Include definition in results (default: true)
  --only-calls         Only show call sites, not definitions
```

#### Implementation Strategy

**Phase 1: Regex-based**
- Pattern matching for common definition syntax:
  - C#: `class|interface|struct SymbolName`, `public.*SymbolName\(`
  - JavaScript: `class SymbolName`, `function SymbolName`, `const SymbolName = `
  - Python: `class SymbolName:`, `def symbol_name(`
- Fast, no dependencies
- ~75% accuracy

**Phase 2: Symbol Index**
- Build lightweight index on first run: `.recur/symbol-index.db`
- SQLite database with: `(symbol, file, line, type, scope)`
- Incremental updates based on file modification times
- Near-instant lookups

**Phase 3: LSP Integration**
- Leverage Language Server Protocol for 100% accuracy
- Optional, requires LSP server for language
- Perfect for IDE integration

#### JSON Output Format

```json
{
  "symbol": "DeleteGameComponentAsync",
  "definitions": [
    {
      "file": "User Level Up Services\\DynamicGameComponentService.cs",
      "line": 234,
      "column": 25,
      "type": "method",
      "signature": "public async Task DeleteGameComponentAsync(int id)",
      "scope": "DynamicGameComponentService"
    }
  ],
  "references": [
    {
      "file": "User Level Up Services\\DynamicGameComponentService.Handlers.cs",
      "line": 42,
      "column": 10,
      "type": "call",
      "context": "await DeleteGameComponentAsync(componentId);"
    }
  ],
  "total_definitions": 1,
  "total_references": 2
}
```

---

## 3. Scope Aliases / Shortcuts

### Problem

Repeatedly typing long hierarchical patterns is tedious:
- `User Level Up Services\\DynamicGameComponentService.*`
- `User Level Up\\Controllers\\LevelController.CreateWizard3.*`

This slows down interactive workflows.

### Solution: Named scope aliases

Define shortcuts for frequently-used scope patterns.

#### Command: `recur scope add`

Create a named alias for a scope pattern.

```bash
# Define alias 'dgc' for DynamicGameComponentService
recur scope add dgc "User Level Up Services\\DynamicGameComponentService.*"

# Define alias 'cw3' for CreateWizard3 controller
recur scope add cw3 "User Level Up\\Controllers\\LevelController.CreateWizard3.*"

# Define alias 'ulu' for all User Level Up
recur scope add ulu "User Level Up**"
```

#### Command: `recur scope list`

List all defined scope aliases.

```bash
recur scope list
```

**Output:**
```
Defined scopes:
  dgc  → User Level Up Services\DynamicGameComponentService.*
  cw3  → User Level Up\Controllers\LevelController.CreateWizard3.*
  ulu  → User Level Up**

Use with: recur find "pattern" --scope <alias>
```

#### Command: `recur scope remove`

Remove a scope alias.

```bash
recur scope remove dgc
```

#### Using Scope Aliases

Once defined, use aliases in any command:

```bash
# Find "Soft delete" within CreateWizard3 controller
recur find "Soft delete" --scope cw3 --ext ".cs"

# Find all files in DynamicGameComponentService
recur files --scope dgc

# Find callers within scope
recur callers "DeleteAsync" --scope dgc

# Tree view of scope
recur tree --scope ulu
```

#### CLI Specification

```
recur scope <SUBCOMMAND>

Manage scope aliases for hierarchical patterns

Subcommands:
  add <NAME> <PATTERN>    Create a scope alias
  remove <NAME>           Remove a scope alias
  list                    List all scope aliases
  show <NAME>             Show expansion of a scope alias

Options:
  --global                Store globally (~/.recur/scopes)
  --local                 Store in .recur/scopes (project-specific)
  -h, --help              Print help
```

#### Storage

**Project-specific scopes** (default):
- Stored in: `.recur/scopes.toml`
- Committed to version control
- Shared across team

**Global scopes**:
- Stored in: `~/.config/recur/scopes.toml` (Linux/Mac) or `%APPDATA%\recur\scopes.toml` (Windows)
- Personal shortcuts

**Format** (`.recur/scopes.toml`):
```toml
[scopes]
dgc = "User Level Up Services\\DynamicGameComponentService.*"
cw3 = "User Level Up\\Controllers\\LevelController.CreateWizard3.*"
ulu = "User Level Up**"
```

#### Example Workflow

```bash
# Setup (once per project)
recur scope add services "Services\\**"
recur scope add controllers "Controllers\\**"
recur scope add tests "**.Tests"

# Daily usage
recur find "async" --scope services
recur files --scope controllers --ext .cs
recur gaps --scope services
recur stats --scope tests
```

---

## 4. Structural JSON Output Mode for Tooling

### Problem

Current JSON output varies by command, making it hard to:
- Parse consistently in scripts
- Build VS Code extensions
- Integrate with other tools

### Solution: Consistent, rich JSON schema

All commands support `--json` with consistent structure.

#### Unified JSON Schema

**Base structure** (all commands):
```json
{
  "command": "find",
  "pattern": "DeleteGameComponentAsync",
  "scope": "DynamicGameComponentService.*",
  "options": {
    "ext": ".cs",
    "context_lines": 2
  },
  "results": [...],
  "summary": {
    "total_results": 42,
    "total_files": 5,
    "execution_time_ms": 127
  }
}
```

#### Command-Specific Results

**`recur find --json`:**
```json
{
  "command": "find",
  "pattern": "Soft delete",
  "results": [
    {
      "file": "Services\\DynamicGameComponentService.cs",
      "line": 234,
      "column": 15,
      "match_start": 15,
      "match_end": 26,
      "match_type": "literal",
      "line_content": "    // Soft delete the component",
      "context_before": [
        "    if (component == null) return NotFound();",
        ""
      ],
      "context_after": [
        "    component.IsDeleted = true;",
        "    await _db.SaveChangesAsync();"
      ]
    }
  ],
  "summary": {
    "total_matches": 1,
    "total_files": 1,
    "execution_time_ms": 45
  }
}
```

**`recur files --json`:**
```json
{
  "command": "files",
  "pattern": "UserService.**",
  "results": [
    {
      "file": "Services\\UserService.cs",
      "relative_path": "Services\\UserService.cs",
      "depth": 0,
      "extension": ".cs",
      "size_bytes": 4521,
      "modified": "2026-01-09T10:30:00Z"
    },
    {
      "file": "Services\\UserService.Handlers.cs",
      "relative_path": "Services\\UserService.Handlers.cs",
      "depth": 1,
      "extension": ".cs",
      "size_bytes": 2341,
      "modified": "2026-01-08T15:20:00Z"
    }
  ],
  "summary": {
    "total_files": 2,
    "execution_time_ms": 12
  }
}
```

**`recur def --json`:**
```json
{
  "command": "def",
  "symbol": "DeleteGameComponentAsync",
  "results": [
    {
      "file": "Services\\DynamicGameComponentService.cs",
      "line": 234,
      "column": 25,
      "definition_type": "method",
      "signature": "public async Task DeleteGameComponentAsync(int id)",
      "scope": "DynamicGameComponentService",
      "modifiers": ["public", "async"],
      "return_type": "Task",
      "parameters": [
        {"name": "id", "type": "int"}
      ],
      "snippet": "public async Task DeleteGameComponentAsync(int id)\n{\n    var component = await _db.Components.FindAsync(id);"
    }
  ],
  "summary": {
    "total_definitions": 1,
    "execution_time_ms": 23
  }
}
```

**`recur gaps --json`:**
```json
{
  "command": "gaps",
  "pattern": "**",
  "results": [
    {
      "hierarchy_root": "README",
      "gaps": [
        {
          "missing_file": "README.CORE.md",
          "depth": 1,
          "parent": {
            "file": "README.md",
            "depth": 0
          },
          "children": [
            {
              "file": "README.CORE.SECTION.md",
              "depth": 2
            }
          ]
        }
      ]
    }
  ],
  "summary": {
    "total_gaps": 1,
    "total_missing_files": 1,
    "execution_time_ms": 67
  }
}
```

#### Schema Benefits

1. **Consistent parsing** - Same top-level structure across commands
2. **Rich metadata** - File size, modified time, depth, etc.
3. **Tooling-friendly** - Easy to consume in scripts/extensions
4. **Type-safe** - Clear schema for code generation
5. **Extensible** - Can add fields without breaking parsers

#### Example: VS Code Extension Usage

```typescript
// TypeScript interface for recur JSON output
interface RecurResult {
  command: string;
  pattern: string;
  scope?: string;
  options: Record<string, any>;
  results: Array<{
    file: string;
    line: number;
    column?: number;
    match_type?: string;
    line_content?: string;
  }>;
  summary: {
    total_results: number;
    execution_time_ms: number;
  };
}

// Parse recur output
const result: RecurResult = JSON.parse(stdout);

// Jump to first result
const firstMatch = result.results[0];
vscode.window.showTextDocument(
  vscode.Uri.file(firstMatch.file),
  { selection: new vscode.Range(firstMatch.line, 0, firstMatch.line, 0) }
);
```

---

## 5. Hierarchical Identifier Index (`recur id-tree`)

### Problem

When working with semantic/hierarchical naming in code (like keys):
- `ulu.lifecycle.deleted`
- `question.*`, `answer.*`
- `Section.*.visible`

You need to:
1. Find all occurrences
2. Group by scope/file
3. Count usage
4. Ensure governance of naming conventions

### Solution: `recur id-tree`

Enhanced identifier search with tree visualization and usage counts.

#### Command: `recur id-tree`

Show hierarchical identifiers and their usage, grouped by scope.

```bash
# Show all 'ulu.lifecycle.*' keys and usage counts
recur id-tree "ulu.lifecycle.*" --ext ".cs"
```

**Output:**
```
ulu.lifecycle
├── deleted (23 usages)
│   ├── DynamicGameComponentService.cs: 12
│   ├── ComponentCleanup.cs: 8
│   └── Tests.cs: 3
├── created (15 usages)
│   ├── ComponentFactory.cs: 10
│   └── Tests.cs: 5
└── updated (8 usages)
    └── ComponentUpdater.cs: 8

Total: 3 identifier patterns, 46 total usages
```

#### Command: `recur id-stats`

Statistics on identifier usage patterns.

```bash
# Stats on question/answer keys
recur id-stats "question.*,answer.*" --ext ".cs"
```

**Output:**
```
Identifier Statistics:

question.* (34 identifiers, 156 usages)
  Most used: question.text (45), question.type (32), question.id (28)
  Files: 12
  Scopes: LevelController.*, QuestionService.*

answer.* (28 identifiers, 98 usages)
  Most used: answer.text (38), answer.isCorrect (25), answer.id (20)
  Files: 8
  Scopes: LevelController.*, AnswerService.*

Section (12 usages)
  Files: 4
  Scopes: LevelController.CreateWizard3.*
```

#### CLI Specification

```
recur id-tree <PATTERN> [OPTIONS]

Show hierarchical identifier usage in tree format

Arguments:
  <PATTERN>  Identifier pattern (e.g., "ulu.lifecycle.*")

Options:
  --scope <PATTERN>    Limit to hierarchical file scope
  --ext <EXT>          Filter by file extension
  --min-usage <N>      Only show identifiers used N+ times
  --sort-by <FIELD>    Sort by: usage|name|scope (default: usage)
  --json               Output as JSON
  -d, --dir <DIR>      Directory to search
  -h, --help           Print help

recur id-stats <PATTERNS> [OPTIONS]

Show usage statistics for identifier patterns

Arguments:
  <PATTERNS>  Comma-separated patterns (e.g., "question.*,answer.*")

Options:
  (same as id-tree)
```

#### JSON Output Format

```json
{
  "command": "id-tree",
  "pattern": "ulu.lifecycle.*",
  "results": {
    "ulu.lifecycle.deleted": {
      "total_usages": 23,
      "files": [
        {
          "file": "DynamicGameComponentService.cs",
          "usages": 12,
          "lines": [45, 67, 89, 123, ...]
        },
        {
          "file": "ComponentCleanup.cs",
          "usages": 8,
          "lines": [23, 45, 67, 78, ...]
        }
      ]
    },
    "ulu.lifecycle.created": {
      "total_usages": 15,
      "files": [...]
    }
  },
  "summary": {
    "total_identifiers": 3,
    "total_usages": 46,
    "total_files": 5
  }
}
```

#### Use Case: Governance

Enforce naming conventions for semantic keys:

```bash
# Find all 'ulu.*' keys and ensure they follow convention
recur id-tree "ulu.*" --ext ".cs" --json > ulu-keys.json

# Script to validate:
# - All keys start with 'ulu.'
# - Second segment is valid category (lifecycle, state, config)
# - No orphaned single-usage keys (might be typos)
```

---

## Implementation Priority

### Phase 1 (High Value, Low Effort)
1. **Scope aliases** - Simple TOML storage, big productivity win
2. **Enhanced JSON output** - Refactor existing JSON to consistent schema
3. **`recur def` (regex-based)** - Heuristic definition search

### Phase 2 (Medium Value, Medium Effort)
4. **`recur callers/callees` (regex-based)** - Heuristic call graph
5. **`recur refs` (regex-based)** - Combined def + usages
6. **`recur id-tree`** - Identifier tree visualization

### Phase 3 (High Value, High Effort)
7. **Symbol index** - SQLite-based caching for fast lookups
8. **AST-based parsing** - Tree-sitter integration for accuracy
9. **LSP integration** - Optional for 100% accuracy

---

## Benefits Summary

| Feature | Benefit | Complexity |
|---------|---------|------------|
| `callers`/`callees` | Fast call graph exploration | Medium |
| `def`/`refs` | Quick symbol navigation | Medium |
| Scope aliases | Productivity boost for repetitive scopes | Low |
| Consistent JSON | Enables VS Code extensions, automation | Low |
| `id-tree` | Governance for semantic naming | Medium |

---

## Related Documents

- [README.md](README.md) - Main documentation
- [FEATURE-gaps.md](FEATURE-gaps.md) - Gap detection feature
- [COMPARISON-powershell.md](COMPARISON-powershell.md) - PowerShell vs recur

---

## Conclusion

These enhancements transform `recur` from a search tool into a **lightweight code intelligence platform** that:

1. **Understands code structure** - Not just files, but symbols and calls
2. **Supports workflows** - Definition → callers → callees → trace
3. **Boosts productivity** - Scope aliases eliminate repetitive typing
4. **Enables automation** - Consistent JSON for tooling
5. **Enforces governance** - Track identifier usage patterns

The result is a tool that bridges the gap between simple text search (grep) and full IDEs (VS Code), optimized for hierarchical codebases.
