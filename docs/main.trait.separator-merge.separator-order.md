# Design Decision: Separator Order and Normalization

## The Question

What happens when you change the order of `--sep` flags?

```bash
recur tree main --sep "." --sep "_"   # Dot first
recur tree main --sep "_" --sep "."   # Underscore first
```

## Proposed Behavior: First Wins

**With auto-normalization to first separator:**

```bash
# Normalize to DOT
recur tree main --sep "." --sep "_"
# Output: main.command.files.impl.rs  (normalized from _)

# Normalize to UNDERSCORE
recur tree main --sep "_" --sep "."
# Output: main_command_files_impl.rs  (normalized from .)
```

## Design Considerations

### Option 1: First Separator Wins (PROPOSED)

**Pros:**
✅ User has explicit control
✅ Predictable behavior
✅ No magic heuristics
✅ Order is meaningful

**Cons:**
❌ Might not be obvious to new users
❌ Order matters (could be surprising)
❌ No "smart" default

**Example:**
```bash
--sep "." --sep "_"     # Normalizes to dot
--sep "_" --sep "."     # Normalizes to underscore
```

### Option 2: Always Normalize to Dot

**Pros:**
✅ Consistent across all queries
✅ Dot is most common in docs/tests
✅ No surprises

**Cons:**
❌ Less flexible
❌ Rust projects might prefer underscore
❌ One-size-fits-all doesn't fit all

**Example:**
```bash
--sep "." --sep "_"     # Normalizes to dot
--sep "_" --sep "."     # Still normalizes to dot (!)
```

### Option 3: Heuristic (Most Common Separator)

**Pros:**
✅ "Smart" default
✅ Adapts to project

**Cons:**
❌ Unpredictable
❌ Magic behavior
❌ Harder to understand
❌ Complex implementation

**Example:**
```bash
--sep "." --sep "_"     # Counts which separator has more files, normalizes to that
# If more .md files than .rs files → normalizes to dot
```

### Option 4: Require Explicit (Current Phase 3)

**Pros:**
✅ No assumptions
✅ Clear intent

**Cons:**
❌ Verbose
❌ Most users want normalization
❌ Extra flag always needed

**Example:**
```bash
--sep "." --sep "_" --sep-replace-default "."   # Explicit
```

## Recommendation: Option 1 (First Wins)

**Rationale:**

### 1. User Control
Users can choose normalization target by ordering separators:
```bash
# I want dots
recur tree main --sep "." --sep "_"

# I want underscores
recur tree main --sep "_" --sep "."
```

### 2. Predictable
No magic, no heuristics. First wins. Always.

### 3. Composable
Works with other flags:
```bash
# Override default
recur tree main --sep "_" --sep "." --sep-replace-default "."
# First is underscore, but explicitly normalize to dot
```

### 4. Precedent
Many tools use "first wins" for ordering:
- CSS: first matching rule wins
- Shell: first PATH match wins
- Git: first remote is "origin"

## User Mental Model

"List your preferred separator first"

```bash
--sep "." --sep "_"
       ↑
   Primary separator (normalization target)
```

## Documentation Strategy

Make it CLEAR in help text:

```
--sep <CHAR>
    Hierarchy separator (default: '.')

    Use multiple times to merge different hierarchies.
    Output is normalized to the FIRST separator listed.

    Examples:
      --sep "." --sep "_"   # Merge and normalize to dot
      --sep "_" --sep "."   # Merge and normalize to underscore

    Override normalization:
      --sep-replace-default <CHAR>   # Explicit target
      --no-normalize                  # Keep original separators
```

## Edge Cases

### Same Separator Multiple Times
```bash
recur tree main --sep "." --sep "."
# Duplicate, same as --sep "."
```

**Behavior:** Deduplicate, treat as single separator.

### Three or More Separators
```bash
recur tree main --sep "." --sep "_" --sep "-"
```

**Behavior:** Still normalize to first (dot).

### Empty Separator List
```bash
recur tree main
# No --sep flags
```

**Behavior:** Use default separator (`.`). No normalization needed.

## Implementation

```rust
// Get separators from CLI
let separators: Vec<char> = /* ... */;

// Determine normalization target
let normalize_to = if no_normalize {
    None  // User opted out
} else if let Some(explicit) = replace_default {
    Some(explicit)  // User specified explicit target
} else if separators.len() > 1 {
    Some(separators[0])  // Auto-normalize to first separator
} else {
    None  // Single separator, no normalization needed
};

// Apply normalization
if let Some(target_sep) = normalize_to {
    // Normalize all paths to target_sep
}
```

## Summary

**First separator wins** is the best default because:
1. ✅ User has control
2. ✅ Predictable behavior
3. ✅ Can be overridden
4. ✅ Simple to understand
5. ✅ Works for all use cases

**Key insight:** Order is meaningful but optional. Users who care can control it, users who don't care will get sensible defaults.
