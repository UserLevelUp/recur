# UX Analysis: Normalization Defaults

## Current Behavior (Phase 3)

**Normalization is OPT-IN:**
```bash
# Default: Mixed notation (raw)
recur tree main --sep "." --sep "_"
# Output:
#   main.command.files.readme.md
#   main_command_files_impl.rs      # Different separator!

# With flag: Normalized
recur tree main --sep "." --sep "_" --sep-replace-default "."
# Output:
#   main.command.files.readme.md
#   main.command.files.impl.rs      # Consistent!
```

## User Perspective Analysis

### When Users Use Multi-Separator

**Primary use case:** "Show me everything about this feature"
- Want to see docs + tests + source together
- Goal: UNIFIED view of the codebase
- Mixed notation is **visually jarring**
- Forces mental translation: "is `main_command_files` the same as `main.command.files`?"

**Secondary use case:** "Which domain am I missing?"
- Want to see which files exist where
- Need to identify gaps (docs but no code, code but no tests)
- This is better served by `--show-sep` flag showing domain markers

### Proposed: Normalization by DEFAULT

```bash
# Default: Normalized to first separator
recur tree main --sep "." --sep "_"
# Output (auto-normalized to dot):
#   main.command.files.readme.md
#   main.command.files.impl.rs      # Normalized!

# Show which domain (for gap analysis)
recur tree main --sep "." --sep "_" --show-sep
# Output:
#   main.command.files.readme.md [.]
#   main.command.files.impl.rs [_]

# Keep original notation (opt-out)
recur tree main --sep "." --sep "_" --no-normalize
# Output:
#   main.command.files.readme.md
#   main_command_files_impl.rs      # Original separator
```

## Rationale

### 1. **Principle of Least Surprise**
When I ask for multiple separators, I want them MERGED, not just listed side-by-side with different notation.

**Expected:** Unified view
**Current:** Mixed notation (surprising!)

### 2. **Common Case is Unified View**
99% of the time, users want:
- "Show me all files for this feature"
- "Show me the complete picture"

NOT:
- "Show me raw file paths with their actual separators"

### 3. **Opt-out is Better than Opt-in**
**Opt-in normalization (current):**
- Every query needs extra flag
- Most users won't discover the feature
- Verbose: `--sep "." --sep "_" --sep-replace-default "."`

**Opt-out normalization (proposed):**
- Works great by default
- Power users can opt-out with `--no-normalize`
- Simpler: `--sep "." --sep "_"` (just works!)

### 4. **Consistency with Other Tools**
When tools merge data from multiple sources, they normalize by default:
- Git merge: unifies code
- Database JOIN: consistent schema
- Multi-separator: should unify notation

## Proposed Behavior

### Default: Auto-Normalize to First Separator

```bash
recur tree main --sep "." --sep "_"
# Automatically normalizes to "." (first separator)
```

**Why first separator?**
- Predictable (always uses what you listed first)
- User controls it (put preferred separator first)
- No magic heuristics

### Explicit Normalization Target

```bash
recur tree main --sep "_" --sep "." --sep-replace-default "."
# Normalizes to dot even though underscore is first
```

### Show Domain Markers (Gap Analysis)

```bash
recur tree main --sep "." --sep "_" --show-sep
# Normalized output + domain markers
#   main.command.files.readme.md [.]
#   main.command.files.impl.rs [_]
```

This serves the "which domain?" use case better than raw mixed notation.

### Opt-out: Keep Original Notation

```bash
recur tree main --sep "." --sep "_" --no-normalize
# Shows original separators (rare use case)
```

## Implementation Changes

### Current (Phase 3)
```rust
if let Some(replace_sep) = replace_default {
    // Normalize
} else {
    // Don't normalize (default)
}
```

### Proposed (Phase 3.1)
```rust
// Default: normalize to first separator
let should_normalize = !no_normalize;  // New flag
let target_sep = if should_normalize {
    replace_default.unwrap_or(separators[0])  // First separator by default
} else {
    None
};

if let Some(sep) = target_sep {
    // Normalize
}
```

## Migration Path

### For Backward Compatibility

Since multi-separator is NEW (Phase 2), there's no existing behavior to break!

**Current users:**
- Single separator: No change
- Multi-separator: NEW feature, can set good defaults

**No migration needed** - this IS the first release of multi-separator.

## User Testing Scenarios

### Scenario 1: "Show me the files command"
```bash
recur tree main.command.files --sep "." --sep "_"
```

**User expectation:** See docs + tests + source in unified view
**Proposed behavior:** ✅ Normalized by default
**Current behavior:** ❌ Mixed notation (confusing)

### Scenario 2: "Find what's missing"
```bash
recur tree main.command.files --sep "." --sep "_" --show-sep
```

**User expectation:** See which files exist in which domain
**Proposed behavior:** ✅ Normalized + domain markers
**Current behavior:** ✅ Works (when implemented)

### Scenario 3: "Debug file paths"
```bash
recur tree main.command.files --sep "." --sep "_" --no-normalize
```

**User expectation:** See actual file paths as-is
**Proposed behavior:** ✅ Opt-out works
**Current behavior:** ✅ This is current default (wrong!)

## Recommendation

**CHANGE DEFAULT TO AUTO-NORMALIZE**

1. ✅ Better user experience
2. ✅ Matches user expectations
3. ✅ No backward compatibility issues (new feature)
4. ✅ Power users can opt-out
5. ✅ Simpler commands (no extra flag needed)

## Proposed Flag Names

```bash
--sep-replace-default <CHAR>   # Explicit normalization target (keeps this)
--no-normalize                  # Opt-out flag (new)
--show-sep                      # Show domain markers (Phase 4)
```

**Alternative names for opt-out:**
- `--raw-separators`
- `--keep-separators`
- `--preserve-separators`
- `--no-normalize` ✅ (clearest)

## Summary

**Current:** Normalization is opt-in (requires `--sep-replace-default`)
**Proposed:** Normalization is DEFAULT (can opt-out with `--no-normalize`)

**Why:** Multi-separator queries are about MERGING views, not just listing files. Unified notation is the expected and desired behavior.
