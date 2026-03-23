# Recur Agent: Workflow & Reference Patterns

> Extracted from `recur-agent.md` — run `recur tree "recur-agent" -d docs/agents/` to see all sections.

## Practical Development Workflow

**Example: Fixing stdin tests (real session)**

```bash
# 1. Discovery
recur tree "main.improvement.6" -d docs/
recur files "**.current" -d docs/

# 2. Find active work details
cat docs/main.command.find.stdin.todo.current.md
cat docs/main.command.find.stdin.todo.current.reference.md

# 3. Study reference implementation (don't guess!)
recur files "main_command_files_*" -d src/ --sep _
cat src/main_command_files_stdin.rs

# 4. Run tests
cd julia-tests && julia runtests.jl 2>&1 | grep "find.*stdin"

# 5. Fix the issue using discovered knowledge

# 6. Verify fix
cd julia-tests && julia runtests.jl 2>&1 | tail -30

# 7. Update tracking files
rm docs/main.command.find.stdin.todo.current.md
echo "complete" > docs/main.command.find.stdin.complete.md

# 8. Discover what's next
recur files "**.stdin.todo" -d docs/
```

## The Reference Pattern

**When re-implementing an existing pattern**, create a `.reference.md` file pointing to working implementations.

**Use references when:**
- Implementing the same capability for a different command
- Re-applying a known pattern to a new context

**Don't use references when:**
- First time implementing something completely new
- No existing examples exist

**Reference file structure:**
```markdown
# Reference: <Feature> Implementation Patterns

## Pattern 1: <Approach Name> (Recommended)
- ? `src/working_example.rs` - Description
- ? Tests passing

## How to Study References
Commands to run to understand the pattern

## Recommended Approach
Why to use this pattern and implementation steps
```

## Creating Good Reference Files

Include these five elements:

1. **Multiple patterns** — show different approaches available
2. **Working examples** — point to actual files that work
3. **Study commands** — explicit `cat`/`grep` commands to run
4. **Recommendation** — which pattern to use and why
5. **Implementation steps** — concrete next steps

This creates a **decision record** that helps future you (or another agent) understand not just what to do, but why.

## Gap Analysis

**Use recur to find what's missing:**

```bash
# All implementations vs. those with stdin
recur files "main_command_*_impl" -d src/ --sep _ --count
recur files "main_command_*_stdin" -d src/ --sep _ --count

# Coverage across lanes
recur files "main.command.*.readme" -d docs/ --count    # Docs
recur files "main.command.*.test" -d julia-tests/ --count # Tests
```

**Missing files = missing capabilities** (visible by absence!)

## Cross-Lane
- Parent: `docs/agents/recur-agent.md`
