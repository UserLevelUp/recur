# Phase 3 Complete: Provenance Tracking (--show-sep)

## Summary
Phase 3 is complete for the merge command. Provenance tracking is implemented and functional for two separators, with normalization in place so merged output renders correctly.

## What Works
- `--show-sep` markers show source separator per file
- Path normalization produces a unified tree view
- Multiple `--pattern`/`--sep` pairs merge and de-duplicate correctly

## Known Issue
- Three separators crash the merge flow (see main.command.merge.bug.md)

## Next Phase
Phase 4: File mode (merge JSON files).
