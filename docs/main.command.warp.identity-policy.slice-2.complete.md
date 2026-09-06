# slice-2: stable-identities
Status: complete

Creation assigns distinct UUIDv7 bubble/slice identities and replaces template
identities. Readers validate UUIDs and duplicates, expose metadata to JSON, and
preserve legacy maps without inventing IDs. Completion and rename preserve IDs;
evolution preserves predecessor bytes and requires a distinct successor UUID
when the predecessor has one.

Acceptance gate: stable-identities.
Observed evidence: main.command.warp.identity-policy.verification.current.md.
