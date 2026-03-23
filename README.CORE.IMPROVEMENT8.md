> **SUPERSEDED** — This spec proposed a Roslyn-based `recur index` approach.
> The implemented path diverged: `recur callers`, `recur trace`, and `recur trace-stats`
> were built as language-agnostic text-based tools — no Roslyn dependency.
> The `recur index` command was never implemented. This doc is a historical artifact.
> Improvement 8 in the eventness system refers to `trace-id` MVP, not this spec.

## 🎯 GitHub Issues: `recur index` Feature Proposal (3 Phases)

---

### **Issue #1: Phase 1 - Core Indexer & File Generation**

**Title:** `feat: Add recur index command for semantic code analysis`

**Labels:** `enhancement`, `phase-1`, `indexer`

**Description:**

Implement the core `recur index` command that generates hierarchical JSON index files from C# codebases using Roslyn.

**Goals:**
- [ ] New subcommand: `recur index --lang csharp --output .recur-index/`
- [ ] Parse `.sln` or `.csproj` files using Roslyn
- [ ] Generate per-file JSON with methods, calls, and line numbers
- [ ] Mirror hierarchical file structure in index output
- [ ] Support incremental indexing (only changed files)

**Output Structure:**
```
.recur-index/
├── DynamicLevelService/
│   ├── DynamicLevelService.cs.json
│   ├── DynamicLevelService.Levels.cs.json
│   └── ...
└── _meta/
    └── index.json  # timestamp, file hashes
```

**Acceptance Criteria:**
- [ ] `recur index MySolution.sln` generates valid JSON files
- [ ] Index files contain method names, signatures, line numbers, and outgoing calls
- [ ] Existing `recur tree/find` commands work on `.recur-index/` directory
- [ ] Performance: Index 100 files in < 30 seconds

---

### **Issue #2: Phase 2 - Reverse Index & Caller Queries**

**Title:** `feat: Add recur callers command with reverse index generation`

**Labels:** `enhancement`, `phase-2`, `callers`

**Depends on:** #1

**Description:**

Build the reverse index (callers) during indexing and add the `recur callers` command to query "who calls this method?"

**Goals:**
- [ ] Generate `_callers/` directory with per-method caller files
- [ ] New command: `recur callers <method-name>`
- [ ] Support `--depth N` for recursive caller chains
- [ ] Support `--exclude-external` to hide System/Framework calls

**Output Structure:**
```
.recur-index/
├── _callers/
│   ├── GetCurrentUserId.json
│   ├── GetAllLevelsAsync.json
│   └── MapCollection.json
```

**Example Usage:**
```powershell
recur callers GetCurrentUserId
recur callers GetCurrentUserId --depth 2
```

**Acceptance Criteria:**
- [ ] `recur callers X` shows all methods that call X
- [ ] `--depth 2` shows callers of callers
- [ ] Output includes file:line references
- [ ] Works with existing `recur` output formats (tree, json)

---

### **Issue #3: Phase 3 - Trace, Stats & Analytics**

**Title:** `feat: Add recur trace and recur stats for call graphs and analytics`

**Labels:** `enhancement`, `phase-3`, `analytics`

**Depends on:** #2

**Description:**

Add execution trace visualization and codebase analytics (hotspots, dead code, complexity).

**Goals:**

**`recur trace`:**
- [ ] New command: `recur trace <method-name> --depth N`
- [ ] Generate call tree from entry point downward (callees)
- [ ] Pre-compute traces for common entry points (controllers, handlers)
- [ ] Support `--exclude-external` flag

**`recur stats`:**
- [ ] New command: `recur stats --hotspots` (most called methods)
- [ ] New command: `recur stats --orphans` (methods with 0 callers)
- [ ] New command: `recur stats --complexity --threshold N`
- [ ] Generate `_stats/` directory during indexing

**Output Structure:**
```
.recur-index/
├── _traces/
│   ├── PublishGame.trace.json
│   └── CreateWizard3.trace.json
└── _stats/
    ├── hotspots.json
    ├── orphans.json
    └── complexity.json
```

**Example Usage:**
```powershell
recur trace PublishGame --depth 5
recur stats --hotspots --top 10
recur stats --orphans
recur stats --complexity --threshold 5
```

**Acceptance Criteria:**
- [ ] `recur trace X` shows full call tree as hierarchical output
- [ ] `recur stats --hotspots` lists top N most-called methods
- [ ] `recur stats --orphans` identifies potential dead code
- [ ] Complexity calculation uses cyclomatic complexity metric
- [ ] All commands support `--json` output format

---

## 📋 Summary

| Phase | Issue | Key Deliverable |
|-------|-------|-----------------|
| **1** | Core Indexer | `recur index` generates per-file JSON |
| **2** | Reverse Index | `recur callers` queries who-calls-what |
| **3** | Analytics | `recur trace` + `recur stats` for insights |

**Milestone:** `recur` becomes a semantic code query engine, not just a file hierarchy tool.