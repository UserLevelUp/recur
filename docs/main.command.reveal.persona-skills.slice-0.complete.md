# slice-0: complete
Status: complete

Accepted 2026-09-10 against contract v2 and the gate baseline-contract.
See main.command.reveal.persona-skills.verification.md and the corresponding
reveal-v2-20260910 completion layer for observed evidence.

## Historical planning record (superseded by acceptance above)

# slice-0: baseline-contract
Status: todo.current

Follow the readme acceptance matrix. Implementation and acceptance remain pending.
Gate: baseline-contract.

Contract revision: v2 (2026-09-09). Freeze discovery for agents, personas and
skills, explicit typed associations, identity collisions, selection and bounded
packet preparation. Existing typed discovery is the baseline to reuse. Extend
the standalone persona-only tests before implementation; follow the revised
readme matrix. No v2 gate has been accepted.

Observed 2026-09-06: standalone persona-skills test has 1 pass, 2 expected red
failures (missing persona defaults and companion binary), 0 errors. Existing
reveal tests: 31 passed. Remaining baseline work: inspect Improvement 29 in detail,
freeze remaining packet/config cases and run the existing init tests before accepting.
These observations cover the old v1 scope only; re-establish the v2 baseline.

Reviewed 2026-09-09: v2 independent discovery cases pass; legacy reveal, artifact
types and init baselines pass. Defaults/companion suite remains intentionally red.
See main.command.reveal.persona-skills.tests.md for exact observations and the
unfrozen association/packet cases. Slice-0 remains pending.
