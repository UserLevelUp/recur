# Trigger Events: Separator Merge Trait

## On Start (Discovery)
```bash
# Check current work
recur files "**.current" -d docs/

# See trait structure
recur tree "main.trait" -d docs/
recur files "trait_**" -d src/ --sep _

# Check existing traits for reference
cat src/trait/stdin.rs
cat src/trait/content_search.rs
```

## After Placeholders Setup (Critical - Run ALL Tests!)
```bash
# Run ALL Julia tests - verify baseline
cd julia-tests && julia runtests.jl

# Run ALL Rust tests - verify baseline
cargo test

# Expected: Existing tests pass, new separator-merge tests fail
cd julia-tests && julia runtests.jl 2>&1 | grep "separator"
```

## During Work (Validation)
```bash
# Build Rust code
cargo build

# Run specific tests
cargo test separator
cd julia-tests && julia runtests.jl 2>&1 | grep "separator"
```

## On Complete (Cleanup)
```bash
# Verify all tests pass
cd julia-tests && julia runtests.jl
cargo test

# Mark phase complete
recur files "main.trait.separator-merge.**" -d docs/

# Clean up ephemeral files
rm docs/main.trait.separator-merge.todo.current.md
rm docs/main.trait.separator-merge.todo.trigger.event.md

# Create completion marker
echo "Phase 1 complete" > docs/main.trait.separator-merge.phase1.complete.md
```
