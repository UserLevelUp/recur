# Phase 3 Stdin - Current Active Work

Status: `todo.current`

## Active Command: find

Currently implementing stdin support for the `find` command.

**Track with:**
```bash
recur files "**.current" -d docs/
```

## Context Files (Ephemeral - Remove When Done)

Created for current work session:
- `main.command.find.stdin.todo.current.md` - Active task description
- `main.command.find.stdin.todo.current.reference.md` - Points to working example (files command)
- `main.command.find.stdin.todo.trigger.event.md` - Commands to run at key moments

**These files are temporary** - delete them when find stdin is complete, then create similar files for next command.

## Eventness Pattern

**Key Events:**
1. **Start work** → Run discovery commands (see trigger.event)
2. **During work** → Run progress checks
3. **Complete** → Run validation, clean up tracking files, move to next

**Discovery commands always available:**
```bash
# What am I working on?
recur files "**.current" -d docs/

# What's left to do?
recur files "**.stdin.todo" -d docs/

# What's my reference?
recur files "**.reference" -d docs/

# What commands should I run?
recur files "**.trigger.event" -d docs/
```

## Progress Tracking

**Phase 3 Status (4/10 complete):**
- ✅ files, stats, tree, related
- 🔄 **find** ← Current work
- ⏳ children, id, callers, callees, trace

**Next up:** children (after find is complete)

## Philosophy

Use the file hierarchy as external memory:
1. Create context files when starting work
2. Query with recur to discover state
3. Delete context files when done
4. The presence/absence of files IS the state

No hidden automation - every step is explicit and queryable!
