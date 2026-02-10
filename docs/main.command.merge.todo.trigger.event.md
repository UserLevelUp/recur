# merge Command: Trigger Events

## Discovery Commands

### Find all merge-related docs
```bash
recur files "main.command.merge.**" --sep "."
```

### Show merge docs tree
```bash
recur tree "main.command.merge" --sep "."
```

### Find merge implementation files
```bash
recur files "main_command_merge**" --sep "_" -d src
```

## Validation Commands

### Run merge tests
```bash
julia julia-tests/main.command.merge.test.jl
```

### Run all tests
```bash
julia julia-tests/run_all_tests.jl
```

### Build and install
```bash
cargo build
cargo install --path . --profile release-safe --force --offline
```

### Test merge command exists
```bash
recur merge --help
```

## Checkpoint Commands

### Git snapshot
```bash
recur-git checkpoint --snapshot
```

### Commit current work
```bash
git add docs/main.command.merge.*.md src/main_command_merge*.rs julia-tests/main.command.merge.test.jl
git commit -m "Phase X: Description here

Details...

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

### Push to remote
```bash
git push -u origin merge-pipes
```

## Testing Workflow

### Manual smoke test
```bash
# Create test files
mkdir -p test/docs test/src
echo "# Test" > test/docs/api.user.service.md
echo "// Test" > test/src/api_user_service.rs

# Test basic merge
recur merge \
  --pattern "api.user" --sep "." -d test/docs \
  --pattern "api_user" --sep "_" -d test/src \
  --show-sep

# Cleanup
rm -rf test
```

### Test with real recur codebase
```bash
# Merge recur docs and source
recur merge \
  --pattern "main.command.tree" --sep "." \
  --pattern "main_command_tree" --sep "_" \
  --show-sep
```

## Phase Transitions

### Complete Phase 1
```bash
# Review docs
cat docs/main.command.merge.readme.md
cat docs/main.command.merge.todo.md

# Commit
git add docs/main.command.merge.*.md
git commit -m "Phase 1 complete: Planning and design for merge command"

# Snapshot
recur-git checkpoint --snapshot
```

### Start Phase 2
```bash
# Update current work
vim docs/main.command.merge.todo.current.md

# Create Phase 2 plan
vim docs/main.command.merge.phase2.plan.md
```

## Cleanup Commands

### Remove test artifacts
```bash
rm -rf test/ temp/ *.tmp
```

### Reset branch (if needed)
```bash
git checkout main
git branch -D merge-pipes
git checkout -b merge-pipes
```

## Useful Queries

### Check what's documented
```bash
recur files "main.command.merge.**" --sep "."
```

### Check what's implemented
```bash
recur files "main_command_merge**" --sep "_" -d src
```

### See unified view (once merge is implemented!)
```bash
recur merge \
  --pattern "main.command.merge" --sep "." \
  --pattern "main_command_merge" --sep "_" \
  --show-sep
```
