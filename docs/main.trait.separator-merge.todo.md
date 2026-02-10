# Trait: Separator Merge

## Overview
Enable commands to accept multiple `--sep` flags and merge results from different separator domains (docs with `.` + src with `_`) into a unified hierarchical view.

## Goals
1. Create `MultiSeparatorCapable` trait in Rust
2. Implement for `tree` command (primary use case)
3. Add `--sep-replace-default` flag to normalize output display
4. Add `--show-sep` flag to show which separator each file uses
5. Support in `files` command (list all paths from both domains)

## Expected Behavior

### tree command
```bash
recur tree "main" --sep "." --sep "_"
# Shows merged hierarchy with docs + src files
```

With flags:
```bash
recur tree "main" --sep "." --sep "_" --sep-replace-default "." --show-sep
# Normalized paths + separator indicators
```

### files command
```bash
recur files "main.command.**" --sep "." --sep "_" --sep-replace-default "."
# Lists all paths with normalized separators
```

## Implementation Phases
- [x] Phase 1: Documentation + failing tests + placeholder code
- [x] Phase 2: Implement trait and basic multi-separator support ✅ COMPLETE!
- [ ] Phase 3: Implement --sep-replace-default normalization logic
- [ ] Phase 4: Implement --show-sep display logic
- [ ] Phase 5: Enhanced help examples showing multi-separator usage patterns

## Current Status
**Phase 2 COMPLETE** - Multi-separator merging is WORKING!

Core functionality delivered:
- Multiple `--sep` flags accepted
- `tree` command merges hierarchies from different separators
- `files` command lists files from all domains
- All baseline tests passing
- 6 new separator-merge tests passing
