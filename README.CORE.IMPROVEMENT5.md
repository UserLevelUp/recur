# Core Improvement 5: Call Tracing (`trace` command)

## Overview

The `trace` command provides multi-level call graph visualization, showing execution paths (what a function calls) and usage paths (who calls a function) in a single command. This eliminates the need to run multiple `callers`/`callees` queries manually.

## Motivation

### Current Workflow (Manual Tracing)
```bash
# Want to see what ApplyAiContent calls and what calls it
recur callees "ApplyAiContent" --scope "LevelController.**"
# Output: calls ValidateInput, GetAiModel, SaveChanges

recur callees "ValidateInput" --scope "**"
# Output: calls CheckPermissions, SanitizeData

recur callees "GetAiModel" --scope "**"
# Output: calls LoadModel, InitializeContext

# Now trace backwards
recur callers "ApplyAiContent" --scope "**"
# Output: called by OnSubmitClick, ProcessWizard
```

**Problem**: Requires 4+ commands to understand the full call context. Each command must be run sequentially, and you must manually track the tree structure.

### With `trace` Command
```bash
recur trace "ApplyAiContent" --depth 2 --scope "LevelController.**"
```

**Output**: Complete execution tree in one view, showing hierarchical file organization.

## Use Cases

### 1. Execution Path Analysis (Debugging)
**Scenario**: Understanding what happens when a user clicks "Submit" in the wizard.

```bash
recur trace "ApplyAiContent" --depth 3 --scope "LevelController.**" --ext ".cs"
```

**Shows**:
- What `ApplyAiContent` calls
- What those functions call (2 levels deep)
- Where each function is defined (hierarchical vs flat files)
- Full execution path without manual exploration

### 2. Impact Analysis (Refactoring)
**Scenario**: Need to refactor `GetDeletedComponentsAsync` - who depends on it?

```bash
recur trace "GetDeletedComponentsAsync" --direction callers --depth 2 --scope "**"
```

**Shows**:
- Direct callers (who calls this function)
- Transitive callers (who calls the callers)
- Impact radius of changes

### 3. Cross-Hierarchy Call Flow
**Scenario**: Tracing calls across service boundaries (Controller → Service → Repository).

```bash
recur trace "DeleteGameComponentAsync" --depth 3 --scope "**" --ext ".cs"
```

**Shows**:
- Method calls within `DynamicGameComponentService.*` hierarchy
- Cross-hierarchy calls to other services
- Database/persistence calls
- Complete dependency graph

## Command Design

### Basic Syntax
```bash
recur trace <function> --scope <pattern> [options]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--depth <N>` | How many levels to trace (1-5) | 2 |
| `--direction <callers\|callees\|both>` | Trace direction | `callees` |
| `--scope <pattern>` | Hierarchical scope pattern | (required) |
| `--ext <exts>` | File extensions filter | none |
| `--max-width <N>` | Max branches per level | 10 |
| `--json` | Output as JSON tree | false |
| `--format <tree\|flat\|graph>` | Output format | `tree` |
| `-i, --ignore-case` | Case-insensitive search | false |

### Direction Modes

#### 1. `--direction callees` (Execution Path - Default)
**Shows what the function calls** (dependencies).

```
ApplyAiContent (LevelController.CreateWizard3.cs:145)
├─ ValidateInput (LevelController.CreateWizard3.Validation.cs:23)
├─ GetAiModel (LevelController.CreateWizard3.AI.cs:67)
└─ SaveChanges (LevelController.CreateWizard3.Persistence.cs:102)
```

#### 2. `--direction callers` (Usage Path)
**Shows who calls the function** (reverse dependencies).

```
GetDeletedComponentsAsync (DynamicGameComponentService.cs:234)
↑ DeleteGameComponentAsync (DynamicGameComponentService.Delete.cs:45)
↑ CleanupComponents (MaintenanceService.cs:67)
```

#### 3. `--direction both` (Full Context)
**Shows both directions** (complete call graph around the function).

```
                ↑ OnSubmitClick
                ↑ ProcessWizard
                │
    ┌───────────┴───────────┐
    │  ApplyAiContent       │
    └───────────┬───────────┘
                │
    ├─ ValidateInput
    ├─ GetAiModel
    └─ SaveChanges
```

## Output Formats

### 1. Tree Format (Default) - Abbreviated Paths with Color

**Design Philosophy**: Use **abbreviated paths** with **color coding** to reduce visual clutter while maintaining clarity. Paths are abbreviated by showing only the unique part when files share a common hierarchy prefix with their parent.

**Terminal output** (colors shown as text annotations):

```
ApplyAiContent (LevelController.CreateWizard3.cs:145) [h:1]

├─ ValidateInput (…Validation.cs:23) [h:2]                    ← cyan (same hierarchy)
│  ├─ CheckPermissions (AuthService.cs:89) [flat]             ← magenta (flat file)
│  ├─ SanitizeData (ValidationHelpers.cs:45) [flat]           ← magenta
│  └─ LogValidation (Logger.cs:34) [flat]                     ← magenta
│
├─ GetAiModel (…AI.cs:67) [h:2]                               ← yellow (different subsection)
│  ├─ LoadModel (AiModelCache.cs:34) [flat]                   ← magenta
│  ├─ InitializeContext (AiService.cs:12) [flat]              ← magenta
│  └─ ConfigureParameters (AiConfig.cs:56) [flat]             ← magenta
│
└─ SaveChanges (…Persistence.cs:102) [h:2]                    ← green (different subsection)
   ├─ UpdateDatabase (DbContext.cs:456) [flat]                ← magenta
   ├─ ClearCache (CacheService.cs:78) [flat]                  ← magenta
   └─ TriggerWebhook (WebhookService.cs:23) [flat]            ← magenta

Summary: 3 direct callees, 9 transitive callees (depth 2)
```

**Color scheme** (optimized for black/dark terminals):
- **Function names**: Bold white
- **Same hierarchy path** (e.g., `…Validation.cs`): **Bright Cyan** (`Color::Cyan`) - shares `LevelController.CreateWizard3.*` prefix
- **Different subsections** (e.g., `…AI.cs`, `…Persistence.cs`):
  - **Bright Green** (`Color::Green`) - good contrast on black
  - **Bright Blue** (`Color::Blue`) - readable on dark backgrounds
  - **Bright Yellow** (`Color::Rgb(255, 255, 0)`) - use bright yellow instead of dark yellow for readability
- **Flat files** (e.g., `AuthService.cs`): **Bright Magenta** (`Color::Magenta`) - external/non-hierarchical file
- **[h:N] markers**: Dim green (`Color::Rgb(0, 200, 0)`) for hierarchical, dim red (`Color::Rgb(200, 0, 0)`) for `[flat]`
- **Tree lines** (`├─`, `│`, `└─`): Dim gray (`Color::Rgb(128, 128, 128)`)

**Note**: All colors tested for readability on black/dark terminal backgrounds. Bright variants used to ensure visibility.

**Path Abbreviation Rules**:
1. **Same hierarchy as parent**: Show `…` + unique part
   - Parent: `LevelController.CreateWizard3.cs`
   - Child: `LevelController.CreateWizard3.Validation.cs`
   - Display: `…Validation.cs` (in cyan)

2. **Different subsection of hierarchy**: Show `…` + differentiating part
   - Parent: `LevelController.CreateWizard3.cs`
   - Child: `LevelController.CreateWizard3.AI.cs`
   - Display: `…AI.cs` (in yellow)

3. **Flat/external files**: Show full filename (no abbreviation)
   - Child: `AuthService.cs`
   - Display: `AuthService.cs` (in magenta, no `…`)

**Visual Benefits**:
- ✅ **Reduced clutter**: `…Validation.cs` instead of full path
- ✅ **Instant hierarchy recognition**: Color indicates relationship to parent
- ✅ **Scannable**: Eye quickly finds hierarchical vs external calls
- ✅ **Compact**: Fits more information on screen

### 2. Verbose Mode (Full Paths, No Abbreviation)

Use `--verbose` flag to show full paths without abbreviation. Useful when you need exact file paths for scripting or when working with unfamiliar code.

```bash
recur trace "ApplyAiContent" --depth 2 --scope "**" --verbose
```

**Output**:
```
ApplyAiContent (LevelController.CreateWizard3.cs:145) [h:1]

├─ ValidateInput (LevelController.CreateWizard3.Validation.cs:23) [h:2]
│  ├─ CheckPermissions (AuthService.cs:89) [flat]
│  ├─ SanitizeData (ValidationHelpers.cs:45) [flat]
│  └─ LogValidation (Logger.cs:34) [flat]
│
├─ GetAiModel (LevelController.CreateWizard3.AI.cs:67) [h:2]
│  ├─ LoadModel (AiModelCache.cs:34) [flat]
│  └─ InitializeContext (AiService.cs:12) [flat]
│
└─ SaveChanges (LevelController.CreateWizard3.Persistence.cs:102) [h:2]
   ├─ UpdateDatabase (DbContext.cs:456) [flat]
   └─ ClearCache (CacheService.cs:78) [flat]
```

**Colors still applied** (same scheme as default), but no path abbreviation.

### 3. Flat Format (No Tree Lines)

Use `--format flat` for linear output without tree characters. Easier to grep/parse.

```bash
recur trace "ApplyAiContent" --depth 2 --scope "**" --format flat
```

**Output**:
```
ApplyAiContent (LevelController.CreateWizard3.cs:145) [h:1]
  ValidateInput (…Validation.cs:23) [h:2]
    CheckPermissions (AuthService.cs:89) [flat]
    SanitizeData (ValidationHelpers.cs:45) [flat]
  GetAiModel (…AI.cs:67) [h:2]
    LoadModel (AiModelCache.cs:34) [flat]
    InitializeContext (AiService.cs:12) [flat]
  SaveChanges (…Persistence.cs:102) [h:2]
    UpdateDatabase (DbContext.cs:456) [flat]
    ClearCache (CacheService.cs:78) [flat]
```

**Uses**: Piping to other tools, easier text processing

### 4. JSON Format

Machine-readable tree structure for tooling integration.

```json
{
  "root": {
    "function": "ApplyAiContent",
    "path": "LevelController.CreateWizard3.cs",
    "line": 145,
    "is_hierarchical": true,
    "depth": 1
  },
  "direction": "callees",
  "trace_depth": 2,
  "callees": [
    {
      "function": "ValidateInput",
      "path": "LevelController.CreateWizard3.Validation.cs",
      "line": 23,
      "is_hierarchical": true,
      "depth": 2,
      "callees": [
        {
          "function": "CheckPermissions",
          "path": "AuthService.cs",
          "line": 89,
          "is_hierarchical": false,
          "depth": 0,
          "callees": []
        },
        {
          "function": "SanitizeData",
          "path": "ValidationHelpers.cs",
          "line": 45,
          "is_hierarchical": false,
          "depth": 0,
          "callees": []
        }
      ]
    }
  ],
  "stats": {
    "total_nodes": 9,
    "direct_callees": 3,
    "transitive_callees": 6,
    "max_depth_reached": 2,
    "cycles_detected": 0
  }
}
```

## Example Workflows

### Workflow 1: Debug Method Call Chain
**Goal**: Understand execution flow when wizard submits AI content.

```bash
# See what ApplyAiContent does (2 levels deep)
recur trace "ApplyAiContent" --depth 2 --scope "LevelController.**" --ext ".cs"
```

**Output** (with color-coded abbreviated paths):
```
ApplyAiContent (LevelController.CreateWizard3.cs:145) [h:1]

├─ ValidateInput (…Validation.cs:23) [h:2]                    ← cyan
│  ├─ CheckPermissions (AuthService.cs:89) [flat]             ← magenta
│  ├─ SanitizeData (ValidationHelpers.cs:45) [flat]           ← magenta
│  └─ LogValidation (Logger.cs:34) [flat]                     ← magenta
│
├─ GetAiModel (…AI.cs:67) [h:2]                               ← yellow
│  ├─ LoadModel (AiModelCache.cs:34) [flat]                   ← magenta
│  ├─ InitializeContext (AiService.cs:12) [flat]              ← magenta
│  └─ ConfigureParameters (AiConfig.cs:56) [flat]             ← magenta
│
└─ SaveChanges (…Persistence.cs:102) [h:2]                    ← green
   ├─ UpdateDatabase (DbContext.cs:456) [flat]                ← magenta
   ├─ ClearCache (CacheService.cs:78) [flat]                  ← magenta
   └─ TriggerWebhook (WebhookService.cs:23) [flat]            ← magenta

Summary: 3 direct callees, 9 transitive callees (depth 2)
```

**Insight**: Clear execution path showing validation → AI processing → persistence. Color coding shows:
- **Cyan** paths stay within same hierarchy (CreateWizard3.Validation)
- **Yellow/Green** paths are different subsections (AI, Persistence)
- **Magenta** paths are external flat files (services, helpers)

### Workflow 2: Find All Usages Before Refactoring
**Goal**: Before modifying `GetDeletedComponentsAsync`, see who depends on it.

```bash
# Trace all callers (2 levels up)
recur trace "GetDeletedComponentsAsync" --direction callers --depth 2 --scope "**"
```

**Output**:
```
GetDeletedComponentsAsync (DynamicGameComponentService.cs:234) [flat]
↑ called by:
├─ DeleteGameComponentAsync (DynamicGameComponentService.Delete.cs:45) [hierarchical, depth=1]
│  ↑ called by:
│  ├─ OnDeleteClick (LevelController.Delete.UI.cs:89) [hierarchical, depth=2]
│  ├─ BulkDelete (AdminController.Bulk.cs:156) [hierarchical, depth=1]
│  └─ ApiDeleteEndpoint (ApiController.Delete.cs:67) [hierarchical, depth=1]
└─ CleanupComponents (MaintenanceService.cs:67) [flat]
   ↑ called by:
   ├─ ScheduledCleanup (BackgroundJobs.cs:23) [flat]
   └─ ManualCleanup (AdminTools.cs:45) [flat]

Summary: 2 direct callers, 5 transitive callers (depth 2)
Impact: 7 call sites across 6 files
```

**Insight**: Method is called from UI, API, admin tools, and background jobs - refactoring requires careful consideration.

### Workflow 3: Cross-Hierarchy Dependency Analysis
**Goal**: See how `DynamicGameComponentService` interacts with other services.

```bash
# Trace callees from DeleteGameComponentAsync
recur trace "DeleteGameComponentAsync" --depth 3 --scope "DynamicGameComponentService.**"
```

**Output**:
```
DeleteGameComponentAsync (DynamicGameComponentService.Delete.cs:45) [hierarchical, depth=1]
├─ GetDeletedComponentsAsync (DynamicGameComponentService.cs:234) [flat]
│  ├─ QueryDatabase (DbContext.Components.cs:123) [hierarchical, depth=1]
│  │  └─ ExecuteSql (DbContext.cs:456) [flat]
│  └─ FilterDeleted (ComponentFilters.cs:78) [flat]
├─ ValidatePermissions (DynamicGameComponentService.Validation.cs:34) [hierarchical, depth=1]
│  ├─ CheckUserRole (AuthService.cs:89) [flat]
│  └─ LogAccess (AuditLog.cs:45) [flat]
└─ NotifyDeletion (DynamicGameComponentService.Events.cs:56) [hierarchical, depth=1]
   ├─ PublishEvent (EventBus.cs:23) [flat]
   └─ UpdateCache (CacheService.cs:78) [flat]
      └─ InvalidateKeys (Redis.cs:234) [flat]

Summary: 3 direct callees, 7 transitive callees (depth 3)
External dependencies: AuthService, DbContext, EventBus, CacheService
```

**Insight**: Service has dependencies on auth, database, events, and caching - clear service boundaries visible.

### Workflow 4: Both Directions (Full Context)
**Goal**: See complete call graph around a critical method.

```bash
recur trace "ApplyAiContent" --direction both --depth 1 --scope "LevelController.**"
```

**Output**:
```
Callers (who calls ApplyAiContent):
↑ OnSubmitClick (LevelController.CreateWizard3.UI.cs:12) [hierarchical, depth=2]
↑ ProcessWizard (LevelController.CreateWizard3.cs:89) [hierarchical, depth=1]
↑ RetryFailed (LevelController.CreateWizard3.Retry.cs:45) [hierarchical, depth=2]

─────────────────────────────────────────
ApplyAiContent (LevelController.CreateWizard3.cs:145) [hierarchical, depth=1]
─────────────────────────────────────────

Callees (what ApplyAiContent calls):
↓ ValidateInput (LevelController.CreateWizard3.Validation.cs:23) [hierarchical, depth=2]
↓ GetAiModel (LevelController.CreateWizard3.AI.cs:67) [hierarchical, depth=2]
↓ SaveChanges (LevelController.CreateWizard3.Persistence.cs:102) [hierarchical, depth=2]
↓ LogCompletion (Logger.cs:56) [flat]

Summary: 3 callers, 4 callees
```

**Insight**: Complete picture of method's role in the system - called by UI/retry logic, calls validation/AI/persistence.

## Performance Considerations

### Exponential Growth Prevention

Call trees can grow exponentially:
- Depth 1: ~10 functions
- Depth 2: ~100 functions
- Depth 3: ~1,000 functions
- Depth 4: ~10,000 functions

**Mitigations**:
1. **Max depth limit**: Default 2, hard limit 5
2. **Max width per level**: Default 10 children per node (show top 10 most relevant)
3. **Cycle detection**: Stop when encountering A → B → A
4. **Hierarchical prioritization**: Show hierarchical files first (most relevant in this codebase)
5. **Scope restriction**: Use `--scope` to limit search space

### Width Limiting Strategy

When a function has >10 callees/callers:

```
ApplyAiContent (LevelController.CreateWizard3.cs:145) [hierarchical, depth=1]
├─ ValidateInput (LevelController.CreateWizard3.Validation.cs:23) [hierarchical, depth=2]
├─ GetAiModel (LevelController.CreateWizard3.AI.cs:67) [hierarchical, depth=2]
├─ SaveChanges (LevelController.CreateWizard3.Persistence.cs:102) [hierarchical, depth=2]
├─ LogInfo (Logger.cs:34) [flat]
├─ UpdateMetrics (Metrics.cs:56) [flat]
... 15 more callees (use --max-width to show more)

Summary: 20 total callees (showing top 5)
```

### Cycle Detection

When encountering cycles:

```
ProcessLoop (LoopHandler.cs:12) [flat]
├─ ValidateState (StateValidator.cs:34) [flat]
├─ UpdateCounter (Counter.cs:56) [flat]
└─ ProcessLoop [cycle detected - already visited at depth 0]
```

## Implementation Architecture

### High-Level Design

```rust
// src/search.rs

pub struct TraceNode {
    pub function: String,
    pub path: PathBuf,
    pub line_number: usize,
    pub is_hierarchical: bool,
    pub depth: usize,
    pub children: Vec<TraceNode>,  // callees or callers
    pub is_cycle: bool,
    pub parent_path: Option<PathBuf>,  // For path abbreviation
}

pub struct TraceSearcher {
    callee_searcher: CalleeSearcher,
    caller_searcher: CallerSearcher,
    options: TraceOptions,
    visited: HashSet<String>,  // cycle detection
}

impl TraceSearcher {
    pub fn trace(
        &mut self,
        function: &str,
        direction: TraceDirection,
        max_depth: usize,
    ) -> anyhow::Result<TraceNode> {
        self.trace_recursive(function, direction, 0, max_depth, None)
    }

    fn trace_recursive(
        &mut self,
        function: &str,
        direction: TraceDirection,
        current_depth: usize,
        max_depth: usize,
        parent_path: Option<PathBuf>,  // Pass parent for abbreviation
    ) -> anyhow::Result<TraceNode> {
        // Depth limit check
        if current_depth >= max_depth {
            return Ok(/* leaf node */);
        }

        // Cycle detection
        if self.visited.contains(function) {
            return Ok(/* cycle marker */);
        }
        self.visited.insert(function.to_string());

        // Find children (callees or callers)
        let children = match direction {
            TraceDirection::Callees => self.callee_searcher.find_callees(function)?,
            TraceDirection::Callers => self.caller_searcher.find_callers(function)?,
            TraceDirection::Both => /* both searches */,
        };

        // Sort by hierarchical priority (hierarchical first, then by depth)
        children.sort_by_hierarchical_priority();

        // Limit width
        let children = children.into_iter()
            .take(self.options.max_width)
            .collect();

        // Get current node's path for passing to children
        let current_path = children.first().map(|c| c.path.clone());

        // Recursively trace children
        let child_nodes = children.iter()
            .map(|child| self.trace_recursive(
                child.function,
                direction,
                current_depth + 1,
                max_depth,
                current_path.clone()  // Pass as parent for abbreviation
            ))
            .collect()?;

        Ok(TraceNode {
            parent_path,
            /* ... */
        })
    }
}
```

### Output Formatting with Color and Abbreviation

```rust
// src/output.rs

// Colors optimized for dark/black terminal backgrounds
const SAME_HIERARCHY_COLOR: Color = Color::Cyan;  // Bright cyan - good on black
const DIFF_SUBSECTION_COLORS: [Color; 3] = [
    Color::Green,                  // Bright green - excellent on black
    Color::Blue,                   // Bright blue - good on dark backgrounds
    Color::Rgb(255, 255, 0),       // Bright yellow - readable (not dark yellow!)
];
const FLAT_FILE_COLOR: Color = Color::Magenta;  // Bright magenta
const TREE_LINE_COLOR: Color = Color::Rgb(128, 128, 128);  // Medium gray
const HIERARCHICAL_MARKER_COLOR: Color = Color::Rgb(0, 200, 0);  // Dim green
const FLAT_MARKER_COLOR: Color = Color::Rgb(200, 0, 0);  // Dim red

impl TerminalFormatter {
    pub fn print_trace_tree(&mut self, node: &TraceNode, prefix: &str, is_last: bool, subsection_index: usize) {
        // Print current node with color and abbreviation
        self.print_trace_node(node, prefix, is_last, subsection_index);

        // Print children recursively
        let mut current_subsection = subsection_index;
        for (i, child) in node.children.iter().enumerate() {
            let is_last_child = i == node.children.len() - 1;
            let child_prefix = if is_last { "   " } else { "│  " };

            // Determine if child is in different subsection
            if child.is_hierarchical && node.is_hierarchical {
                if !paths_share_subsection(&node.path, &child.path) {
                    current_subsection += 1;
                }
            }

            self.print_trace_tree(
                child,
                &format!("{}{}", prefix, child_prefix),
                is_last_child,
                current_subsection
            );
        }
    }

    fn print_trace_node(&mut self, node: &TraceNode, prefix: &str, is_last: bool, subsection_index: usize) {
        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(TREE_LINE_COLOR)));
        }

        let branch = if is_last { "└─ " } else { "├─ " };
        let _ = write!(self.stdout, "{}{}", prefix, branch);

        if self.color {
            let _ = self.stdout.reset();
        }

        // Print function name (bold white)
        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_bold(true));
        }
        let _ = write!(self.stdout, "{} ", node.function);
        if self.color {
            let _ = self.stdout.reset();
        }

        // Determine path color and abbreviation
        let (abbreviated_path, path_color) = self.format_path_with_color(node, subsection_index);

        if self.color {
            let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(path_color)));
        }
        let _ = write!(self.stdout, "({}:{}) ", abbreviated_path, node.line_number);
        if self.color {
            let _ = self.stdout.reset();
        }

        // Print hierarchical marker
        let marker = if node.is_hierarchical {
            format!("[h:{}]", node.depth)
        } else {
            "[flat]".to_string()
        };

        if self.color {
            let marker_color = if node.is_hierarchical {
                HIERARCHICAL_MARKER_COLOR
            } else {
                FLAT_MARKER_COLOR
            };
            let _ = self.stdout.set_color(ColorSpec::new().set_fg(Some(marker_color)));
        }
        let _ = writeln!(self.stdout, "{}", marker);
        if self.color {
            let _ = self.stdout.reset();
        }
    }

    fn format_path_with_color(&self, node: &TraceNode, subsection_index: usize) -> (String, Color) {
        // Flat files: show full filename, magenta color
        if !node.is_hierarchical {
            let filename = node.path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            return (filename.to_string(), FLAT_FILE_COLOR);
        }

        // Hierarchical files: abbreviate path based on parent
        if let Some(parent_path) = &node.parent_path {
            if let Some(common_prefix) = find_common_hierarchy_prefix(&node.path, parent_path) {
                // Same hierarchy prefix: use cyan, abbreviate
                if common_prefix == extract_hierarchy_prefix(parent_path) {
                    let unique_part = extract_unique_part(&node.path, &common_prefix);
                    return (format!("…{}", unique_part), SAME_HIERARCHY_COLOR);
                } else {
                    // Different subsection: use rotating color, abbreviate
                    let unique_part = extract_unique_part(&node.path, &common_prefix);
                    let color = DIFF_SUBSECTION_COLORS[subsection_index % DIFF_SUBSECTION_COLORS.len()];
                    return (format!("…{}", unique_part), color);
                }
            }
        }

        // No parent or no common prefix: show full path
        (node.path.display().to_string(), SAME_HIERARCHY_COLOR)
    }
}

// Helper functions for path processing
fn find_common_hierarchy_prefix(path1: &Path, path2: &Path) -> Option<String> {
    let name1 = path1.file_name()?.to_str()?;
    let name2 = path2.file_name()?.to_str()?;

    // Extract hierarchical names (before extension)
    let hier1 = name1.rsplit_once('.').map(|(n, _)| n)?;
    let hier2 = name2.rsplit_once('.').map(|(n, _)| n)?;

    // Find common prefix in hierarchical names
    let parts1: Vec<&str> = hier1.split('.').collect();
    let parts2: Vec<&str> = hier2.split('.').collect();

    let common_parts: Vec<&str> = parts1.iter()
        .zip(parts2.iter())
        .take_while(|(a, b)| a == b)
        .map(|(a, _)| *a)
        .collect();

    if common_parts.is_empty() {
        None
    } else {
        Some(common_parts.join("."))
    }
}

fn extract_unique_part(path: &Path, common_prefix: &str) -> String {
    let filename = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    let hier_name = filename.rsplit_once('.')
        .map(|(n, ext)| {
            let unique = n.strip_prefix(common_prefix)
                .unwrap_or(n)
                .trim_start_matches('.');
            format!("{}.{}", unique, ext)
        })
        .unwrap_or_else(|| filename.to_string());

    hier_name
}
```

## Validation & Testing

### Smoke Tests (Real-World Scenarios)

```bash
# 1. Method execution path within scoped hierarchy
recur trace "DeleteGameComponentAsync" --scope "DynamicGameComponentService.**" --ext ".cs" --depth 2

# 2. Controller action dependencies
recur trace "ApplyAiContent" --scope "LevelController.CreateWizard3.**" --ext ".cs" --depth 2

# 3. Cross-scope caller analysis
recur trace "GetDeletedComponentsAsync" --direction callers --scope "**" --ext ".cs" --depth 2

# 4. Both directions for refactoring
recur trace "ValidateInput" --direction both --depth 1 --scope "LevelController.**"

# 5. Deep trace with width limiting
recur trace "ProcessRequest" --depth 3 --max-width 5 --scope "**"
```

### Test Cases (Julia Test Suite)

Create `runtests.trace.jl`:

```julia
@testset "recur trace command" begin
    @testset "Basic callees trace" begin
        # Trace CreateUser callees (should find ValidateEmail, SaveUser)
        success, output, _ = run_recur("trace \"CreateUser\" --depth 1 --scope \"**\"")
        @test success
        @test contains(output, "ValidateEmail")
        @test contains(output, "SaveUser")
    end

    @testset "Callers trace" begin
        # Trace who calls ValidateEmail
        success, output, _ = run_recur("trace \"ValidateEmail\" --direction callers --depth 1 --scope \"**\"")
        @test success
        @test contains(output, "CreateUser")
    end

    @testset "Depth limiting" begin
        # Depth 0 should show only root
        success, output, _ = run_recur("trace \"CreateUser\" --depth 0 --scope \"**\"")
        @test success
        @test contains(output, "CreateUser")
        @test !contains(output, "ValidateEmail")
    end

    @testset "Cycle detection" begin
        # Create test file with cycle: A calls B, B calls A
        # Trace should mark cycle
        success, output, _ = run_recur("trace \"FunctionA\" --depth 3 --scope \"**\"")
        @test success
        @test contains(output, "cycle detected")
    end

    @testset "Hierarchical prioritization" begin
        # Hierarchical files should appear before flat files
        success, output, _ = run_recur("trace \"CreateUser\" --depth 1 --scope \"**\"")
        @test success
        @test contains(output, "[hierarchical")
    end

    @testset "JSON output" begin
        success, output, _ = run_recur("trace \"CreateUser\" --depth 1 --scope \"**\" --json")
        @test success
        data = JSON3.read(output)
        @test haskey(data, :root)
        @test haskey(data, :callees)
        @test haskey(data, :stats)
    end
end
```

## Benefits

### Time Savings
- **Before**: 4-6 manual commands + mental tree construction (~2-3 minutes)
- **After**: 1 command with visual tree (~10 seconds)
- **Savings**: ~90% reduction in debugging time

### Clarity
- Visual tree structure easier to parse than multiple flat lists
- Hierarchical file organization visible at a glance
- Cycle detection prevents infinite exploration

### Discoverability
- Reveals unexpected dependencies (calls you didn't know existed)
- Shows cross-hierarchy interactions
- Highlights potential refactoring opportunities

## Future Enhancements

### Phase 2 Features
1. **Interactive Mode**: Navigate tree with arrow keys, expand/collapse nodes
2. **Call Context**: Show actual call sites (not just function names)
3. **Filtering**: `--exclude-external` to hide calls to external libraries
4. **Metrics**: Show execution counts, performance data if available
5. **Graph Export**: Output DOT format for GraphViz visualization

### Phase 3 Features
1. **Type-Aware Tracing**: Distinguish method calls from function calls
2. **Interface Tracing**: Follow interface implementations
3. **Async Tracing**: Show async/await chains
4. **Test Coverage**: Show which functions have test coverage

## Design Decisions (Finalized)

### 1. Output Format: Abbreviated Paths with Color (Concept 3)
**Decision**: Use color-coded path abbreviation as the default output format.

**Rationale**:
- Reduces visual clutter by showing only unique parts of paths (`…Validation.cs` instead of full path)
- Color instantly communicates hierarchy relationships without reading full paths
- Compact format fits more information on screen
- Still clear and scannable

**Implementation**:
- Cyan for same hierarchy
- Yellow/Green/Blue for different subsections (rotating)
- Magenta for flat files
- `--verbose` flag shows full paths when needed

### 2. Default Depth: 2 Levels
**Decision**: Default `--depth 2`, allow 0-5, recommend 1-3 for interactive use.

**Rationale**:
- Depth 1: Too shallow for understanding execution flow
- Depth 2: Sweet spot - shows immediate dependencies + their dependencies
- Depth 3+: Useful for deep analysis but can get overwhelming
- Hard limit of 5 to prevent exponential explosion

### 3. Width Limiting: Hierarchical Priority
**Decision**: Prioritize hierarchical files, then limit to top 10 per level.

**Rationale**:
- Hierarchical files are most relevant in this codebase
- Showing all calls leads to information overload
- If function has >10 callees, user can increase `--max-width` or use `--scope` to filter

### 4. Cycle Display: Simple Marker
**Decision**: Show `[cycle detected - already visited]` and stop tracing that branch.

**Rationale**:
- Simple and clear
- Prevents infinite loops
- Showing full path back to cycle start adds complexity without much value

### 5. Both Direction Format: Two Sections
**Decision**: Show callers above, callees below, with clear separator.

**Rationale**:
- Vertical layout easier to read than side-by-side
- Clear separation prevents confusion
- Consistent with existing tools (git log --graph, etc.)

## Open Questions (For Future Consideration)

1. **Performance caching**: Should we cache callee/caller results between depth levels?
   - Pro: Faster for deep traces
   - Con: More memory usage, complexity
   - **Defer**: Implement basic version first, optimize if needed

2. **Interactive mode**: Arrow keys to expand/collapse nodes?
   - Pro: Explore large trees easily
   - Con: Significant implementation complexity
   - **Defer**: Phase 2 feature

3. **Color customization**: Allow users to customize color scheme?
   - Pro: Accessibility, personal preference
   - Con: More configuration, testing burden
   - **Defer**: Use sensible defaults first

## Success Metrics

- ✅ Reduces multi-command workflows to single command
- ✅ Preserves hierarchical file organization in output
- ✅ Handles large codebases without explosion (depth/width limits)
- ✅ Detects and displays cycles gracefully
- ✅ Supports JSON output for tooling integration
- ✅ Works with existing `callers`/`callees` infrastructure
