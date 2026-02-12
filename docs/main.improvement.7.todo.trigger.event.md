# Trigger Events: Improvement 7 Phase 1

## On Start
```bash
# Discover current state
recur files "**.current" -d docs/
recur tree "main.improvement.7" -d docs/

# Read the config design spec
cat README.CORE.IMPROVEMENT7.recur-git.md

# Check current recur-git checkpoint implementation
recur files "main_command_*" -d src/ --sep _ --count
cat src/recur_git_main.rs
```

## During Work
```bash
# Verify docs hierarchy
recur tree "main.improvement.7" -d docs/

# Check source changes
git diff --name-only

# Run tests
cargo test
```

## On Complete
```bash
# Run full test suite
cargo test

# Verify cleanup
recur files "**.current" -d docs/

# Clean up ephemeral files
rm docs/main.improvement.7.todo.current.md
rm docs/main.improvement.7.todo.current.reference.md
rm docs/main.improvement.7.todo.trigger.event.md

# Create completion marker
# docs/main.improvement.7.phase1.complete.md
```
