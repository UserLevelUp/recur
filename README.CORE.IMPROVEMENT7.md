# Core Improvement 7: Statistical Call Graph Analysis (`trace-stats`)

## Current Status

**🔴 NOT STARTED** - This is a planned enhancement building on IMPROVEMENT5 (trace).

### Dependencies
- ✅ **IMPROVEMENT5** (trace command) - **COMPLETE** - Provides foundation for call graph analysis
- 🟡 **IMPROVEMENT6** (--stdin flag) - **IN PROGRESS** - Needed for Git integration workflows

### What This Adds
A new `trace-stats` command that provides **statistical analysis of call graph complexity** across your codebase. Instead of tracing one function at a time, analyze all functions and rank them by complexity metrics.

**Key Value Proposition**: Instant visibility into which functions are highest-risk for refactoring, which have circular dependencies, and where to focus testing efforts.

---

## Overview

### What is `trace-stats`?

A statistical analysis command that ranks functions by call graph complexity. Helps developers identify:
- **High-impact functions** (many transitive dependencies)
- **Refactoring hotspots** (deep call chains)
- **Circular reference patterns** (potential design issues)

### Why This Matters

**Developer Pain Point**: When planning refactoring or reviewing code changes, developers ask:
- "Which functions are most complex to change?"
- "Where should I focus my testing?"
- "Are there circular dependencies I should know about?"

**Current Solution**: Manually trace each function, keep notes, try to remember complexity.

**With trace-stats**: One command gives objective metrics for the entire codebase.

---

## Example Usage

### Basic Usage

```bash
# Find the 5 most complex functions in the codebase
recur trace-stats --scope "**" --ext .rs --sort-by transitive --top 5

# Output:
Function              | Direct | Transitive | Circular | Depth | Risk
print_trace_result    | 2      | 41         | 0        | 3     | Medium
cmd_trace             | 15     | 38         | 2        | 3     | High
format_output         | 8      | 29         | 1        | 3     | Medium
parse_args            | 5      | 15         | 0        | 2     | Low
validate_input        | 2      | 8          | 0        | 2     | Low

Summary: 5 functions analyzed
  - 2 with circular references (cmd_trace, format_output)
  - Average transitive count: 26.2
  - Deepest call chain: 3 levels
```

### Column Definitions

| Column | Meaning | Interpretation |
|--------|---------|----------------|
| **Function** | Function name | The analyzed function |
| **Direct** | Number of unique functions called directly (depth 1) | Shows immediate dependencies |
| **Transitive** | Total unique functions reachable in call graph | Shows full impact - more = harder to refactor |
| **Circular** | Number of distinct circular reference patterns detected | 0 = no cycles, >0 = potential design smell (but may be intentional) |
| **Depth** | Maximum depth of call chain from this function | Shows call stack depth risk |
| **Risk** | Refactoring risk assessment | Low (<10 transitive), Medium (10-30), High (>30) |

---

## Real-World Use Cases

### 1. Pre-Refactoring Analysis

**Scenario**: You need to refactor `UserService` but don't know the impact.

```bash
# Analyze all methods in UserService hierarchy
recur trace-stats --scope "UserService.**" --ext .cs --sort-by transitive

# Output shows which methods have most dependencies
Function                      | Direct | Transitive | Circular | Depth | Risk
UserService.CreateUser        | 8      | 45         | 1        | 4     | High
UserService.ValidateEmail     | 2      | 12         | 0        | 3     | Medium
UserService.SaveUser          | 3      | 8          | 0        | 2     | Low
```

**Decision**: `CreateUser` has 45 transitive dependencies - needs extensive testing. `SaveUser` only has 8 - safer to change.

### 2. Identify Circular Reference Patterns

**Scenario**: You suspect there are circular dependencies but don't know where.

```bash
# Find all functions with circular references
recur trace-stats --scope "**" --ext .rs --sort-by circular --filter circular-only

Function                  | Direct | Transitive | Circular | Depth | Risk
EventDispatcher.dispatch  | 8      | 24         | 3        | 4     | Medium
ObserverManager.notify    | 5      | 18         | 2        | 3     | Medium
StateManager.transition   | 6      | 15         | 1        | 3     | Low

Summary: 3 functions with circular references
  - EventDispatcher.dispatch has 3 distinct circular patterns
  - ObserverManager.notify has 2 distinct circular patterns
  - StateManager.transition has 1 distinct circular pattern
```

**Investigation**:
- `EventDispatcher.dispatch` with 3 circular patterns might be intentional (event loop design)
- Or it might indicate tight coupling that needs refactoring
- **Recur reports facts, you decide** if it's a problem

### 3. Git Impact Analysis (Requires IMPROVEMENT6)

**Scenario**: You're reviewing a PR and want to focus testing on complex changed functions.

```bash
# Which changed functions have the highest complexity?
git diff main..feature --name-only | \
  recur trace-stats --scope "**" --stdin --sort-by transitive --top 10

# Output shows complexity of only the changed functions
Function                    | Direct | Transitive | Circular | Depth | Risk
ModifiedFunction1           | 12     | 55         | 0        | 5     | High
ModifiedFunction2           | 3      | 8          | 1        | 2     | Low
```

**Decision**: Focus code review and testing on `ModifiedFunction1` (55 transitive deps, depth 5). `ModifiedFunction2` is low-risk.

### 4. Code Review Prioritization

**Scenario**: Large PR with 50+ changed files. Where do you focus review time?

```bash
# Get JSON output for tooling integration
git diff main..feature --name-only | \
  recur trace-stats --scope "**" --stdin --sort-by risk --format json | \
  jq '.functions[] | select(.risk == "High") | {name, transitive, file}'

# Output:
{
  "name": "ProcessPayment",
  "transitive": 67,
  "file": "PaymentService.cs"
}
{
  "name": "ValidateOrder",
  "transitive": 42,
  "file": "OrderService.Validation.cs"
}
```

**Decision**: Review `ProcessPayment` and `ValidateOrder` first - highest complexity, highest risk.

### 5. Architecture Health Metrics

**Scenario**: You want to track codebase complexity over time.

```bash
# Get stats for entire codebase
recur trace-stats --scope "**" --ext .cs --json > complexity-report.json

# Summary shows:
# - Total functions analyzed
# - Functions with circular references
# - Average transitive count
# - Deepest call chain

# Track in Git to see if complexity is growing
git add complexity-report.json
git commit -m "Weekly complexity metrics"
```

**Continuous Improvement**: Compare reports week-over-week to see if refactoring is reducing complexity.

---

## Command-Line Interface

### Required Arguments

```bash
--scope <PATTERN>    # Scope pattern (e.g., "UserService.**", "**")
```

### Optional Arguments

```bash
# Sorting Options
--sort-by <MODE>     # Sort order (default: transitive)
                     # Options: transitive, direct, circular, depth, risk

# Filtering
--filter <TYPE>      # Filter results
                     # Options: circular-only, high-risk, medium-risk, low-risk

# Limiting
--top <N>            # Show only top N results (default: all)

# File Filtering
--ext <EXTENSIONS>   # File extensions (e.g., ".cs", ".rs", ".js")

# Input Source
--stdin              # Read file paths from stdin (Git integration)

# Output Format
--json               # Output as JSON
--format <TYPE>      # Output format: table (default), json, csv

# Search Options
--ignore-case        # Case-insensitive matching
-d, --dir <PATH>     # Root directory (default: current)
```

### Sort Options Explained

```bash
--sort-by transitive    # Functions with most total dependencies (default)
                        # Best for: Finding highest-impact functions

--sort-by direct        # Functions calling many others directly
                        # Best for: Finding functions with high immediate coupling

--sort-by circular      # Functions with most circular patterns
                        # Best for: Identifying potential design issues

--sort-by depth         # Functions with deepest call chains
                        # Best for: Finding stack depth risks

--sort-by risk          # Combined complexity score
                        # Best for: General refactoring prioritization
```

### Filter Options Explained

```bash
--filter circular-only  # Show only functions with circular > 0
                        # Use when: Investigating circular dependencies

--filter high-risk      # Show only functions with Risk = High (>30 transitive)
                        # Use when: Prioritizing testing/refactoring

--filter medium-risk    # Show only functions with Risk = Medium (10-30 transitive)

--filter low-risk       # Show only functions with Risk = Low (<10 transitive)
```

---

## Output Formats

### Table Format (Default)

```bash
recur trace-stats --scope "**" --ext .rs --top 3

Function              | Direct | Transitive | Circular | Depth | Risk
print_trace_result    | 2      | 41         | 0        | 3     | Medium
cmd_trace             | 15     | 38         | 2        | 3     | High
format_output         | 8      | 29         | 1        | 3     | Medium

Summary: 3 functions analyzed
  - 2 with circular references (cmd_trace, format_output)
  - Average transitive count: 36.0
  - Deepest call chain: 3 levels
```

### JSON Format

```bash
recur trace-stats --scope "**" --ext .rs --top 3 --json

{
  "functions": [
    {
      "name": "print_trace_result",
      "file": "src/output.rs",
      "line": 252,
      "direct": 2,
      "transitive": 41,
      "circular": 0,
      "depth": 3,
      "risk": "Medium"
    },
    {
      "name": "cmd_trace",
      "file": "src/main.rs",
      "line": 1017,
      "direct": 15,
      "transitive": 38,
      "circular": 2,
      "depth": 3,
      "risk": "High"
    },
    {
      "name": "format_output",
      "file": "src/output.rs",
      "line": 180,
      "direct": 8,
      "transitive": 29,
      "circular": 1,
      "depth": 3,
      "risk": "Medium"
    }
  ],
  "summary": {
    "total_functions": 3,
    "with_circular": 2,
    "avg_transitive": 36.0,
    "max_depth": 3,
    "risk_distribution": {
      "low": 0,
      "medium": 2,
      "high": 1
    }
  }
}
```

### CSV Format

```bash
recur trace-stats --scope "**" --ext .rs --top 3 --format csv

Function,File,Line,Direct,Transitive,Circular,Depth,Risk
print_trace_result,src/output.rs,252,2,41,0,3,Medium
cmd_trace,src/main.rs,1017,15,38,2,3,High
format_output,src/output.rs,180,8,29,1,3,Medium
```

---

## Design Philosophy

### 1. Neutral Reporting, Not Judgement

**Core Principle**: Recur reports facts. Developers decide what's acceptable.

**Circular References Example**:
- ✅ Recur reports: "2 circular patterns detected"
- ❌ Recur does NOT say: "WARNING: Fix these circular references!"

**Why**: Circular references can be intentional design patterns (event loops, observers) or unintended coupling. Only the developer knows the difference.

**Examples of Acceptable Circular Patterns**:
- Event loops (dispatcher ↔ handler)
- Observer patterns (subject ↔ observer)
- State machines (state ↔ transition manager)
- Recursive data structures with proper termination

**Examples of Problematic Circular Patterns**:
- Constructor chains causing initialization deadlocks
- Memory leaks from strong reference cycles
- Unintended coupling from poor architecture

**Recur's role**: Shine a light on the structure. You decide if it's a feature or a bug.

### 2. Circular Count = Distinct Patterns, Not Frequency

**Important**: The `Circular` column counts **distinct circular reference patterns**, not how many times a function appears in cycles.

**Example**:
```
CreateWizard3() → ApplyTemplate() → RenderTemplate() → CreateWizard3()  [pattern 1]
CreateWizard3() → SaveWizard() → ValidateWizard() → CreateWizard3()    [pattern 2]
```

This shows `Circular: 2` for `CreateWizard3()` - meaning 2 distinct circular patterns exist.

**Not**: A count of every time the function appears in a cycle (which would be meaningless).

### 3. Risk Scoring Thresholds

Risk levels based on transitive dependency count:

| Risk Level | Transitive Count | Interpretation |
|-----------|------------------|----------------|
| **Low** | < 10 | Easy to refactor, low testing burden |
| **Medium** | 10 - 30 | Moderate complexity, test carefully |
| **High** | > 30 | High complexity, extensive testing needed |

**Note**: These thresholds are heuristics, not absolute truth. A function with 35 transitive deps in a well-tested codebase might be safer than one with 15 deps in an untested codebase.

---

## Implementation Plan

### Phase 1: Core Functionality (4-6 hours)

#### Step 1: Add TraceStats Command (1 hour)

**File**: `src/main.rs`

Add new command to CLI:

```rust
#[derive(Parser, Debug)]
enum Commands {
    // ... existing commands ...

    /// Analyze call graph complexity statistics
    ///
    /// Ranks functions by complexity metrics (direct/transitive callees, circular patterns, depth).
    /// Useful for identifying refactoring hotspots and high-risk functions.
    ///
    /// Examples:
    ///   # Find 5 most complex functions
    ///   recur trace-stats --scope "**" --ext .rs --top 5
    ///
    ///   # Find all functions with circular references
    ///   recur trace-stats --scope "**" --ext .cs --filter circular-only
    ///
    ///   # Analyze changed files (requires --stdin from IMPROVEMENT6)
    ///   git diff --name-only | recur trace-stats --scope "**" --stdin --sort-by risk
    TraceStats {
        /// Scope pattern (e.g., "UserService.**", "**")
        #[arg(long, required = true)]
        scope: String,

        /// File extensions to include (e.g., ".cs", ".rs")
        #[arg(long)]
        ext: Option<String>,

        /// Sort by metric (transitive, direct, circular, depth, risk)
        #[arg(long, default_value = "transitive")]
        sort_by: String,

        /// Filter results (circular-only, high-risk, medium-risk, low-risk)
        #[arg(long)]
        filter: Option<String>,

        /// Show only top N results
        #[arg(long)]
        top: Option<usize>,

        /// Output format (table, json, csv)
        #[arg(long, default_value = "table")]
        format: String,

        /// Read file paths from stdin (Git integration)
        #[arg(long)]
        stdin: bool,

        /// Case-insensitive matching
        #[arg(long)]
        ignore_case: bool,

        /// Root directory to search
        #[arg(short = 'd', long, default_value = ".")]
        dir: PathBuf,
    },
}
```

Add match arm:

```rust
Commands::TraceStats {
    scope,
    ext,
    sort_by,
    filter,
    top,
    format,
    stdin,
    ignore_case,
    dir,
} => {
    cmd_trace_stats(
        scope,
        ext,
        sort_by,
        filter,
        top,
        format,
        stdin,
        ignore_case,
        dir,
        cli.json,
        cli.color,
    )
}
```

#### Step 2: Implement Statistical Collection (2 hours)

**File**: `src/search.rs`

Add new structures:

```rust
/// Statistics for a single function's call graph
#[derive(Debug, Clone)]
pub struct FunctionStats {
    pub name: String,
    pub file: PathBuf,
    pub line: usize,
    pub direct_callees: usize,      // Depth 1 only
    pub transitive_callees: usize,  // All depths
    pub circular_patterns: usize,   // Distinct circular patterns
    pub max_depth: usize,           // Maximum call chain depth
    pub risk: RiskLevel,            // Computed risk score
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RiskLevel {
    Low,     // < 10 transitive
    Medium,  // 10-30 transitive
    High,    // > 30 transitive
}

impl FunctionStats {
    /// Calculate risk level from transitive count
    pub fn calculate_risk(transitive: usize) -> RiskLevel {
        if transitive < 10 {
            RiskLevel::Low
        } else if transitive <= 30 {
            RiskLevel::Medium
        } else {
            RiskLevel::High
        }
    }
}
```

Extend TraceSearcher:

```rust
impl TraceSearcher {
    /// Collect statistics for all functions in scope
    pub fn collect_stats(
        &mut self,
        scope: &HierarchyPattern,
        extensions: Vec<String>,
    ) -> Result<Vec<FunctionStats>> {
        let mut stats = Vec::new();

        // Find all functions in scope
        let functions = self.find_all_functions(scope, extensions)?;

        for func in functions {
            // Run trace for this function
            let trace = self.trace_function(&func.name, scope, 5)?; // Max depth 5

            // Count metrics
            let direct = self.count_direct_callees(&trace);
            let transitive = self.count_transitive_callees(&trace);
            let circular = self.count_circular_patterns(&trace);
            let depth = self.max_call_depth(&trace);

            stats.push(FunctionStats {
                name: func.name,
                file: func.file,
                line: func.line,
                direct_callees: direct,
                transitive_callees: transitive,
                circular_patterns: circular,
                max_depth: depth,
                risk: FunctionStats::calculate_risk(transitive),
            });
        }

        Ok(stats)
    }

    /// Count direct callees (depth 1 only)
    fn count_direct_callees(&self, trace: &TraceNode) -> usize {
        trace.children.len()
    }

    /// Count all unique transitive callees
    fn count_transitive_callees(&self, trace: &TraceNode) -> usize {
        let mut seen = HashSet::new();
        self.collect_all_callees(trace, &mut seen);
        seen.len()
    }

    fn collect_all_callees(&self, node: &TraceNode, seen: &mut HashSet<String>) {
        for child in &node.children {
            let key = format!("{}:{}:{}", child.name, child.file, child.line);
            if seen.insert(key) {
                self.collect_all_callees(child, seen);
            }
        }
    }

    /// Count distinct circular patterns
    fn count_circular_patterns(&self, trace: &TraceNode) -> usize {
        let mut patterns = HashSet::new();
        self.find_circular_patterns(trace, &Vec::new(), &mut patterns);
        patterns.len()
    }

    fn find_circular_patterns(
        &self,
        node: &TraceNode,
        path: &Vec<String>,
        patterns: &mut HashSet<Vec<String>>,
    ) {
        let key = format!("{}:{}:{}", node.name, node.file, node.line);

        if let Some(pos) = path.iter().position(|p| p == &key) {
            // Found a cycle - extract the circular pattern
            let cycle: Vec<String> = path[pos..].iter().cloned().collect();
            patterns.insert(cycle);
            return;
        }

        let mut new_path = path.clone();
        new_path.push(key);

        for child in &node.children {
            self.find_circular_patterns(child, &new_path, patterns);
        }
    }

    /// Find maximum call depth
    fn max_call_depth(&self, trace: &TraceNode) -> usize {
        if trace.children.is_empty() {
            return 1;
        }

        let max_child_depth = trace.children
            .iter()
            .map(|c| self.max_call_depth(c))
            .max()
            .unwrap_or(0);

        1 + max_child_depth
    }
}
```

#### Step 3: Implement Sorting & Filtering (1 hour)

**File**: `src/search.rs`

```rust
#[derive(Debug, Clone, Copy)]
pub enum SortBy {
    Transitive,
    Direct,
    Circular,
    Depth,
    Risk,
}

impl SortBy {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "transitive" => Ok(Self::Transitive),
            "direct" => Ok(Self::Direct),
            "circular" => Ok(Self::Circular),
            "depth" => Ok(Self::Depth),
            "risk" => Ok(Self::Risk),
            _ => Err(anyhow!("Invalid sort option: {}. Valid options: transitive, direct, circular, depth, risk", s)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum FilterBy {
    All,
    CircularOnly,
    HighRisk,
    MediumRisk,
    LowRisk,
}

impl FilterBy {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "circular-only" => Ok(Self::CircularOnly),
            "high-risk" => Ok(Self::HighRisk),
            "medium-risk" => Ok(Self::MediumRisk),
            "low-risk" => Ok(Self::LowRisk),
            _ => Err(anyhow!("Invalid filter option: {}. Valid options: circular-only, high-risk, medium-risk, low-risk", s)),
        }
    }
}

pub fn sort_stats(stats: &mut Vec<FunctionStats>, sort_by: SortBy) {
    match sort_by {
        SortBy::Transitive => {
            stats.sort_by(|a, b| b.transitive_callees.cmp(&a.transitive_callees));
        }
        SortBy::Direct => {
            stats.sort_by(|a, b| b.direct_callees.cmp(&a.direct_callees));
        }
        SortBy::Circular => {
            stats.sort_by(|a, b| b.circular_patterns.cmp(&a.circular_patterns));
        }
        SortBy::Depth => {
            stats.sort_by(|a, b| b.max_depth.cmp(&a.max_depth));
        }
        SortBy::Risk => {
            // Sort by risk level, then transitive within same risk
            stats.sort_by(|a, b| {
                match (&b.risk, &a.risk) {
                    (RiskLevel::High, RiskLevel::High) => b.transitive_callees.cmp(&a.transitive_callees),
                    (RiskLevel::High, _) => std::cmp::Ordering::Less,
                    (_, RiskLevel::High) => std::cmp::Ordering::Greater,
                    (RiskLevel::Medium, RiskLevel::Medium) => b.transitive_callees.cmp(&a.transitive_callees),
                    (RiskLevel::Medium, _) => std::cmp::Ordering::Less,
                    (_, RiskLevel::Medium) => std::cmp::Ordering::Greater,
                    (RiskLevel::Low, RiskLevel::Low) => b.transitive_callees.cmp(&a.transitive_callees),
                }
            });
        }
    }
}

pub fn filter_stats(stats: Vec<FunctionStats>, filter: FilterBy) -> Vec<FunctionStats> {
    match filter {
        FilterBy::All => stats,
        FilterBy::CircularOnly => stats.into_iter().filter(|s| s.circular_patterns > 0).collect(),
        FilterBy::HighRisk => stats.into_iter().filter(|s| s.risk == RiskLevel::High).collect(),
        FilterBy::MediumRisk => stats.into_iter().filter(|s| s.risk == RiskLevel::Medium).collect(),
        FilterBy::LowRisk => stats.into_iter().filter(|s| s.risk == RiskLevel::Low).collect(),
    }
}
```

#### Step 4: Implement Output Formatting (1 hour)

**File**: `src/output.rs`

```rust
use crate::search::{FunctionStats, RiskLevel};

pub fn print_trace_stats(
    stats: &[FunctionStats],
    format: &str,
    color: bool,
) -> Result<()> {
    match format {
        "table" => print_stats_table(stats, color),
        "json" => print_stats_json(stats),
        "csv" => print_stats_csv(stats),
        _ => Err(anyhow!("Invalid format: {}. Valid options: table, json, csv", format)),
    }
}

fn print_stats_table(stats: &[FunctionStats], color: bool) -> Result<()> {
    if stats.is_empty() {
        println!("No functions found in scope");
        return Ok(());
    }

    // Print header
    println!("{:<30} | {:>6} | {:>10} | {:>8} | {:>5} | {:>6}",
        "Function", "Direct", "Transitive", "Circular", "Depth", "Risk");
    println!("{}", "-".repeat(80));

    // Print rows
    for stat in stats {
        let risk_str = match stat.risk {
            RiskLevel::Low => "Low",
            RiskLevel::Medium => "Medium",
            RiskLevel::High => "High",
        };

        println!("{:<30} | {:>6} | {:>10} | {:>8} | {:>5} | {:>6}",
            truncate(&stat.name, 30),
            stat.direct_callees,
            stat.transitive_callees,
            stat.circular_patterns,
            stat.max_depth,
            risk_str);
    }

    // Print summary
    println!();
    let with_circular = stats.iter().filter(|s| s.circular_patterns > 0).count();
    let avg_transitive = stats.iter().map(|s| s.transitive_callees).sum::<usize>() as f64 / stats.len() as f64;
    let max_depth = stats.iter().map(|s| s.max_depth).max().unwrap_or(0);

    println!("Summary: {} functions analyzed", stats.len());
    if with_circular > 0 {
        let circular_funcs: Vec<&str> = stats.iter()
            .filter(|s| s.circular_patterns > 0)
            .map(|s| s.name.as_str())
            .collect();
        println!("  - {} with circular references ({})", with_circular, circular_funcs.join(", "));
    }
    println!("  - Average transitive count: {:.1}", avg_transitive);
    println!("  - Deepest call chain: {} levels", max_depth);

    Ok(())
}

fn print_stats_json(stats: &[FunctionStats]) -> Result<()> {
    use serde_json::json;

    let functions: Vec<serde_json::Value> = stats.iter().map(|s| {
        json!({
            "name": s.name,
            "file": s.file.display().to_string(),
            "line": s.line,
            "direct": s.direct_callees,
            "transitive": s.transitive_callees,
            "circular": s.circular_patterns,
            "depth": s.max_depth,
            "risk": format!("{:?}", s.risk),
        })
    }).collect();

    let with_circular = stats.iter().filter(|s| s.circular_patterns > 0).count();
    let avg_transitive = if stats.is_empty() {
        0.0
    } else {
        stats.iter().map(|s| s.transitive_callees).sum::<usize>() as f64 / stats.len() as f64
    };
    let max_depth = stats.iter().map(|s| s.max_depth).max().unwrap_or(0);

    let high = stats.iter().filter(|s| s.risk == RiskLevel::High).count();
    let medium = stats.iter().filter(|s| s.risk == RiskLevel::Medium).count();
    let low = stats.iter().filter(|s| s.risk == RiskLevel::Low).count();

    let output = json!({
        "functions": functions,
        "summary": {
            "total_functions": stats.len(),
            "with_circular": with_circular,
            "avg_transitive": avg_transitive,
            "max_depth": max_depth,
            "risk_distribution": {
                "low": low,
                "medium": medium,
                "high": high,
            }
        }
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn print_stats_csv(stats: &[FunctionStats]) -> Result<()> {
    println!("Function,File,Line,Direct,Transitive,Circular,Depth,Risk");

    for stat in stats {
        println!("{},{},{},{},{},{},{},{:?}",
            stat.name,
            stat.file.display(),
            stat.line,
            stat.direct_callees,
            stat.transitive_callees,
            stat.circular_patterns,
            stat.max_depth,
            stat.risk);
    }

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len-3])
    }
}
```

#### Step 5: Implement Command Handler (30 minutes)

**File**: `src/main.rs`

```rust
fn cmd_trace_stats(
    scope: String,
    ext: Option<String>,
    sort_by: String,
    filter: Option<String>,
    top: Option<usize>,
    format: String,
    stdin: bool,
    ignore_case: bool,
    dir: PathBuf,
    json: bool,
    color: bool,
) -> anyhow::Result<()> {
    use recur::search::{TraceSearcher, HierarchyPattern, SortBy, FilterBy, sort_stats, filter_stats};
    use recur::output::print_trace_stats;

    // Parse scope pattern
    let scope_pattern = HierarchyPattern::parse(&scope)?;

    // Parse sort option
    let sort_option = SortBy::from_str(&sort_by)?;

    // Parse filter option
    let filter_option = if let Some(f) = filter {
        FilterBy::from_str(&f)?
    } else {
        FilterBy::All
    };

    // Parse extensions
    let extensions = parse_extensions(ext);

    // Create searcher
    let mut searcher = TraceSearcher::new(SearchOptions {
        root: dir,
        extensions: extensions.clone(),
        ignore_case,
        ..Default::default()
    });

    // Collect statistics
    let mut stats = searcher.collect_stats(&scope_pattern, extensions)?;

    // Sort
    sort_stats(&mut stats, sort_option);

    // Filter
    stats = filter_stats(stats, filter_option);

    // Limit to top N
    if let Some(n) = top {
        stats.truncate(n);
    }

    // Output
    print_trace_stats(&stats, &format, color)?;

    Ok(())
}
```

---

## Testing Strategy

### Julia Test Suite

**File**: `julia-tests/runtests.trace-stats.jl` (already created as placeholder)

**Test Coverage**:
1. Contract tests (help, missing args, invalid options)
2. Basic output (default sort, JSON, CSV formats)
3. Sorting options (all 5 sort modes)
4. Filtering options (circular-only, high-risk, etc.)
5. Git integration with --stdin (requires IMPROVEMENT6)
6. Circular pattern counting accuracy
7. Risk scoring thresholds
8. Top N limiting
9. Performance on large codebases

**Activation**: Replace `@test_skip` with actual assertions once implemented.

---

## Why This Saves Time & Energy

### Before trace-stats:

1. Developer wants to refactor UserService
2. Manually trace each method with `recur trace`
3. Keep notes on complexity
4. Try to remember which methods are risky
5. Spend 30+ minutes gathering context
6. Still miss some dependencies

### With trace-stats:

1. Run: `recur trace-stats --scope "UserService.**" --ext .cs`
2. Instantly see all methods ranked by complexity
3. Objective metrics guide decisions
4. 2 minutes to full understanding

**Time saved**: 28 minutes per refactoring session

**Energy saved**: No mental burden of tracking complexity manually

---

## Implementation Checklist

- [ ] **Step 1**: Add TraceStats command to CLI (1 hour)
  - [ ] Add command struct with all arguments
  - [ ] Add match arm calling cmd_trace_stats
  - [ ] Add help text with examples

- [ ] **Step 2**: Implement statistical collection (2 hours)
  - [ ] Add FunctionStats struct
  - [ ] Add RiskLevel enum with thresholds
  - [ ] Extend TraceSearcher with collect_stats method
  - [ ] Implement count_direct_callees
  - [ ] Implement count_transitive_callees
  - [ ] Implement count_circular_patterns
  - [ ] Implement max_call_depth
  - [ ] Add find_all_functions helper

- [ ] **Step 3**: Implement sorting & filtering (1 hour)
  - [ ] Add SortBy enum with from_str parser
  - [ ] Add FilterBy enum with from_str parser
  - [ ] Implement sort_stats function (all 5 modes)
  - [ ] Implement filter_stats function (all 4 modes)

- [ ] **Step 4**: Implement output formatting (1 hour)
  - [ ] Add print_trace_stats dispatcher
  - [ ] Implement table format with summary
  - [ ] Implement JSON format
  - [ ] Implement CSV format
  - [ ] Add color support (optional)

- [ ] **Step 5**: Implement command handler (30 minutes)
  - [ ] Add cmd_trace_stats function
  - [ ] Parse all arguments
  - [ ] Call searcher.collect_stats
  - [ ] Sort and filter results
  - [ ] Apply top N limit
  - [ ] Output results

- [ ] **Step 6**: Activate Julia tests (1 hour)
  - [ ] Replace @test_skip with actual tests
  - [ ] Add golden output validation
  - [ ] Test all sort modes
  - [ ] Test all filter modes
  - [ ] Test edge cases (empty results, no circular refs)
  - [ ] Uncomment include in runtests.jl

- [ ] **Step 7**: Manual testing (30 minutes)
  - [ ] Test on recur codebase itself
  - [ ] Test all sort options
  - [ ] Test all filter options
  - [ ] Test JSON/CSV output
  - [ ] Test --stdin with Git (requires IMPROVEMENT6)

---

## Dependencies

### Required
- ✅ **IMPROVEMENT5** (trace command) - Provides TraceSearcher foundation

### Optional but Recommended
- 🟡 **IMPROVEMENT6** (--stdin flag) - Enables Git integration workflows

---

## Estimated Effort

**Total**: 4-6 hours

| Task | Time | Priority |
|------|------|----------|
| CLI command setup | 1 hour | High |
| Statistical collection | 2 hours | High |
| Sorting & filtering | 1 hour | High |
| Output formatting | 1 hour | High |
| Command handler | 30 min | High |
| Julia tests | 1 hour | Medium |
| Manual testing | 30 min | High |

---

## Success Criteria

1. ✅ Command executes without errors
2. ✅ All 5 sort modes work correctly
3. ✅ All 4 filter modes work correctly
4. ✅ Table, JSON, and CSV output formats work
5. ✅ Circular pattern counting is accurate (distinct patterns, not frequency)
6. ✅ Risk scoring thresholds are applied correctly (Low <10, Medium 10-30, High >30)
7. ✅ Summary statistics are accurate
8. ✅ Julia tests pass (once activated)
9. ✅ --stdin integration works with Git (once IMPROVEMENT6 is complete)

---

## Future Enhancements

### Phase 2 (Optional):

1. **Trend analysis**: Compare complexity over time
   ```bash
   recur trace-stats --scope "**" --compare-to complexity-last-week.json
   ```

2. **Threshold customization**: Custom risk thresholds
   ```bash
   recur trace-stats --risk-thresholds 5,20,50  # Low: <5, Medium: 5-20, High: >20
   ```

3. **Visualization**: Generate complexity graphs
   ```bash
   recur trace-stats --scope "**" --visualize complexity.svg
   ```

4. **CI/CD integration**: Fail build if complexity exceeds threshold
   ```bash
   recur trace-stats --scope "**" --max-avg-transitive 30 || exit 1
   ```

---

## Conclusion

`trace-stats` provides **instant visibility** into codebase complexity, enabling:
- **Data-driven refactoring** decisions
- **Objective testing** prioritization
- **Early detection** of circular dependencies
- **Continuous monitoring** of code health

**Philosophy**: Recur reports. You decide.

---

**Status**: Ready for implementation once IMPROVEMENT6 (--stdin) is complete.

**Next Steps**: Complete IMPROVEMENT6, then implement trace-stats following this specification.
