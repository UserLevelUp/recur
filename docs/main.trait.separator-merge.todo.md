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
- [x] Phase 1: Documentation + failing tests + placeholder code ✅
- [x] Phase 2: Implement trait and basic multi-separator support ✅
- [x] Phase 3: Implement --sep-replace-default normalization logic ✅
- [x] Phase 4: Implement --show-sep display logic ✅
- [x] **Phase 5: Extended test cases + 3+ separator scenarios** ✅
- [ ] Phase 6: Enhanced help examples (OPTIONAL - deferred)

## Current Status
**🎉 FEATURE COMPLETE 🎉**

### Core Functionality Delivered
- ✅ Multiple `--sep` flags accepted (unlimited count)
- ✅ `tree` command merges hierarchies from different separators
- ✅ `files` command lists files from all domains
- ✅ `--sep-replace-default` normalization working
- ✅ `--show-sep` markers working (gap analysis enabled)
- ✅ All baseline tests passing
- ✅ 6 new separator-merge tests passing

### Documentation Complete
- ✅ Feature README with use cases
- ✅ Computer science significance documented
- ✅ Decomposition/recomposition scenarios documented
- ✅ 30+ test cases for real-world scenarios
- ✅ 3+ separator behavior fully specified
- ✅ Gap analysis patterns documented
- ✅ UX analysis and design decisions captured

### Test Coverage
- ✅ Basic multi-separator merging
- ✅ Normalization edge cases
- ✅ Marker display logic
- ✅ 3+ separator scenarios (stress tested to 6 separators)
- ✅ Real-world decomposition patterns (XSLT, JSON, configs, docs)
- ✅ Performance considerations documented

See:
- `main.trait.separator-merge.decomposition-test-cases.md` - 30+ real-world scenarios
- `main.trait.separator-merge.multi-separator-test-cases.md` - 3+ separator edge cases
