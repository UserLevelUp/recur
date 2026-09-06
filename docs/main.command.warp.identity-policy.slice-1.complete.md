# slice-1: editable-init
Status: complete

Implemented nearest-project `recur-warp init`, dry-run, comment-preserving partial
defaults, inline tables, starter templates and byte-idempotent retries. Existing
user templates survive. Publication stages bytes and rolls back a new template
if config replacement fails.

Acceptance gate: editable-init.
Observed evidence: main.command.warp.identity-policy.verification.current.md.
