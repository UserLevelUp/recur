# Command: checkpoint - OUT OF SCOPE

Status: `out-of-scope`

## Decision

The `checkpoint` command is NOT part of core recur.

It will be implemented in a future extension called `recur-git` which will handle Git-specific workflows.

## Rationale

Recur should remain focused on:
- Managing hierarchical files
- Hierarchical list operations
- File pattern matching and querying

Git integration is a separate concern better handled by a dedicated extension.

## Future Work

See `main.git.checkpoint` for future `recur-git` planning.

## References

- Future improvement for `recur-git` integration
