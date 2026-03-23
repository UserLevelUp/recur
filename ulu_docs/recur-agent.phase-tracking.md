# Recur Agent: Phase & Epic Tracking

> Extracted from `recur-agent.md` — run `recur tree "recur-agent" -d docs/agents/` to see all sections.

## Phase and Epic Tracking with Recur

**Phases are temporal containers.** They organize work into stages but their tracking files
should not accumulate forever. Use the hierarchy to track phases, then collapse.

### Phase File Naming Convention

```
<Epic>.Phase<N>.<concern>.<suffix>.md

Examples:
  MongoDB.Users.Collection.Standardization.Phase2.eventness.md               ? analysis
  MongoDB.Users.Collection.Standardization.Phase3.migration.todo.current.md  ? active work
  MongoDB.Users.Collection.Standardization.Phase3.migration.complete.md      ? done
```

### Phase Discovery

```bash
# Full epic tree
recur tree "MongoDB.Users.Collection.Standardization" -d docs/

# What phase is active?
recur files "**Phase**current**" -d docs/

# What phases are done?
recur files "**Phase**complete**" -d docs/

# Master tracker (absorbs collapsed knowledge)
cat docs/MongoDB.Users.Collection.Standardization.todo.md
```

### Phase Lifecycle

```bash
# BEFORE starting a phase:
#   1. Create .todo.current.md           ? marks active work
#   2. Create .todo.current.reference.md ? points to patterns from prior phases
#   3. Create jl/*.verify-phaseN.jl      ? Julia verification ready
#   4. Create cross-lane doc stubs       ? recur can bridge lanes

# DURING a phase:
#   5. recur files "**.current" -d docs/  ? check progress
#   6. Run jl scripts                     ? verify data
#   7. Run tests                          ? verify code

# AFTER completing a phase:
#   8. Delete .current.md                 ? close eventness window
#   9. Create .complete.md (5–10 lines)   ? summary record
#  10. Update master .todo.md             ? absorb key findings
#  11. recur files "**.current" -d docs/  ? confirm cleanup

# WHEN NEXT PHASE COMPLETES:
#  12. Collapse prior phase .eventness.md ? fold into epic issue or .todo.md
#  13. Consider deleting prior .complete.md (master .todo.md has the knowledge)
```

### Cross-Lane Phase Alignment

**Tests and Julia scripts survive phase collapse. Docs and analysis don't need to.**

```bash
# Phase tests (permanent)
recur tree "MongoContentOwnership" -d "User_Level_Up_Tests_Data_Mongo/"

# Phase Julia scripts (permanent)
recur tree "mongo.verify" -d jl/ --sep .

# Phase docs (temporal — collapse when cold)
recur tree "MongoDB.Users.Collection.Standardization" -d docs/
```

## Code-Centric Cross-Lane Discovery

**Code is the canonical hierarchy.** Docs, tests, and Julia scripts mirror it.

```bash
# Working on CreateWizard3.Tab.Publish? Check all lanes:
recur tree "CreateWizard3.Tab.Publish" -d "User Level Up/Views/Level/"   # Code
recur tree "CreateWizard3.Tab.Publish" -d docs/                          # Docs
recur tree "CreateWizard3.Tab.Publish" -d jl/                            # Julia
recur files "CreateWizard3.Tab.Publish**" -d "User Level Up Test/"       # Tests
```

**Gap analysis is automatic:** If a lane returns nothing, that's a visible gap.

| Lane | What you see |
|------|-------------|
| **Code** | `.cshtml`, `.cs` — the actual implementation |
| **docs/** | `.todo.md`, `.current.md`, `.complete.md` — eventness state |
| **jl/** | `.patch-*.jl`, `.verify.jl` — Julia automation scripts |
| **Test/** | `.Tests.cs` — test coverage (or gap by absence) |

### Placeholder Docs Pattern

When touching code, create a matching doc stub:

```markdown
# DashboardController

## Status
Prepped - nothing yet

## Cross-Lane
- Code: `User Level Up/Controlllers/DashboardController.cs`
- Julia: `recur tree "DashboardController" -d jl/`
- Tests: TODO
```

**The file just needs to exist** so recur can find it. Content comes when the eventness window opens.

### Eventness Window Rule

**If you're touching the code, create the matching doc/jl in the same session.**

Don't batch it up for later. When the `.current.md` gets deleted, the window closes.
Anything not mirrored by then is a visible gap next time you query.

## Cross-Lane
- Parent: `docs/agents/recur-agent.md`
