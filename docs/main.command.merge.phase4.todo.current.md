# Current Work: merge Command - Phase 4 (File Mode)

## Goal
Enable file mode for `recur merge`: read JSON from file inputs and merge results.

## Scope
- Accept JSON file arguments
- Parse tree/files JSON into file lists
- Merge multiple inputs with provenance markers
- Add basic tests with cached JSON fixtures

## Next Actions
- Define accepted JSON schema for file mode (avoid blocking Phase 5 stdin)
- Add CLI path for file inputs without impacting pattern mode
- Implement file reader + parser to return (files, separator, base)
- Add tests for two-separator merge from cached JSON

## Notes (Eventness)
- Phase 5 uses stdin and may rely on Bash process substitution: `<(recur tree ... --json)`
- On Windows/PowerShell, prefer pipes or temp files instead of `<( )`
- File mode should mirror stdin parsing to avoid divergence between Phase 4 and Phase 5
