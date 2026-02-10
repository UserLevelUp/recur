# Current Work: merge Command - Phase 1

## Active Task
Planning and design for `recur merge` command - Unix-style hierarchical result composition.

## Phase 1 Goals
1. ✅ Create eventness tracking files
2. ✅ Write comprehensive README
3. ✅ Design CLI interface
4. ✅ Plan merging algorithm
5. ⏳ Write Julia test cases
6. ⏳ Create Phase 1 plan document
7. ⏳ Review and get user alignment

## Context

### The Problem We're Solving
User has files with different naming conventions:
- Docs: `main.command.tree.readme.md` (dots)
- Source: `main_command_tree_impl.rs` (underscores)

Current multi-separator approach requires same base pattern. User wants explicit merge command for maximum flexibility.

### The Solution
```bash
# Explicit pattern merge
recur merge \
  --pattern "main.command.tree" --sep "." \
  --pattern "main_command_tree" --sep "_" \
  --show-sep

# Output:
main.command.tree
├── readme.md [.]
├── test.jl [.]
└── impl.rs [_]
```

### Unix Philosophy Benefits
1. **Explicit** - No surprising automatic conversions
2. **Composable** - Works with pipes
3. **Focused** - Does one thing (merging) well
4. **Flexible** - Can merge ANY searches

## Design Decisions

### CLI Interface
```
recur merge [OPTIONS]

OPTIONS:
    --pattern <PATTERN>              Pattern to search (repeatable)
    --sep <CHAR>                     Separator for pattern (repeatable)
    --show-sep                       Show markers [.] [_]
    --sep-replace-default <CHAR>     Normalize output
    --stdin                          Pipe mode (read JSON)
    --format <tree|files>            Output format
    --json                           JSON output
```

### Pairing Strategy
Patterns and separators are paired in order:
```bash
recur merge \
  --pattern "api.user" --sep "." \      # Pair 1
  --pattern "api_user" --sep "_"        # Pair 2
```

### Merging Algorithm
1. **Collect**: For each (pattern, sep) pair, find files
2. **Track**: Remember which separator found each file
3. **Merge**: Build unified hierarchy tree
4. **Display**: Show with markers if requested

## Next Steps

1. Write Julia test cases
2. Create detailed Phase 1 plan
3. Checkpoint with user for alignment
4. Begin Phase 2 implementation

## Files Created So Far
- `docs/main.command.merge.readme.md` - Comprehensive feature docs ✅
- `docs/main.command.merge.todo.md` - Implementation tracking ✅
- `docs/main.command.merge.todo.current.md` - This file ✅

## Branch
`merge-pipes` - clean start from main

## Questions for User
1. Should pipe mode be mandatory or optional first?
2. Start with tree format or files format?
3. Any specific use cases to prioritize?
