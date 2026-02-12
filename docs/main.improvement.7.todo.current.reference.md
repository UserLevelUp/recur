# Reference: Improvement 7 Combined Phased Approach

## Existing Specs (Study These)

### Phase 1 References: Project Config
- `README.CORE.IMPROVEMENT7.recur-git.md` — `.recur/config.toml` design, lane detection, multi-dir awareness
- `README.CORE.IMPROVEMENTS10.md` — recur-git separation philosophy, why checkpoint is separate
- `src/recur_git_main.rs` — current checkpoint implementation (reads lanes, git state)

### Phase 2 References: More Flatten Formats
- `src/main_command_flatten_impl.rs` — current flatten impl (XML + JSON), format detection, output format
- `README.CORE.IMPROVEMENT12.md` — document transformation via filesystem vision (decompose/transform/recompose)

### Phase 3 References: Trace Stats
- `README.CORE.IMPROVEMENT7.md` — full trace-stats spec (sort, filter, risk scoring, output formats)
- `README.CORE.IMPROVEMENT7.extra.tests.md` — test plan for trace-stats
- `README.CORE.IMPROVEMENT5.md` — trace command foundation (trace-stats builds on this)

### Phase 4 References: Farming Tools
- `README.CORE.IMPROVEMENT13.md` — enterprise config management patterns (layered merging, environment promotion)
- `docs/main.command.merge.readme.md` — existing merge command (hierarchical merge already exists)

### Phase 5 References: Embeddings
- No existing spec yet — future vision
- `README.CORE.IMPROVEMENT8.md` — `recur index` (semantic code analysis, related concept)

## How to Study References

```bash
# Phase 1 config design
cat README.CORE.IMPROVEMENT7.recur-git.md

# Current flatten implementation (Phase 2 foundation)
cat src/main_command_flatten_impl.rs

# Trace-stats full spec (Phase 3)
cat README.CORE.IMPROVEMENT7.md

# Document transformation vision (Phase 2+4 intersection)
cat README.CORE.IMPROVEMENT12.md

# Config management patterns (Phase 4)
cat README.CORE.IMPROVEMENT13.md

# Current recur-git checkpoint (Phase 1 integration target)
cat src/recur_git_main.rs
```

## Recommended Approach

Start with Phase 1 (`.recur/config.toml`) because:
1. Every other phase benefits from project-aware config
2. It's the smallest standalone deliverable
3. It makes recur portable across projects immediately
4. The recur-git config spec already exists in `README.CORE.IMPROVEMENT7.recur-git.md`
5. It proves the pattern before building the bigger pieces
