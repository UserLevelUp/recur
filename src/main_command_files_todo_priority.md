# 🔥 High-Priority TODOs: Files Command

## P0: Critical Performance
- [ ] Fix O(n²) complexity in pattern matching for deep hierarchies
- [ ] Add benchmark tests before optimizing
- **Impact**: Slow on repos with >10k files
- **ETA**: 1 week

## P1: Stdin Bug
- [ ] Extension filtering doesn't work correctly with stdin + multiple extensions
- **Impact**: Users can't filter `git diff --name-only` by multiple extensions
- **Blocker**: Needs parser fix first (see `main_parser_todo_blocker.md`)
- **ETA**: 2 days after parser fix

---
**Status**: 2 P0 items, 1 blocked
**Last Updated**: 2026-02-07
