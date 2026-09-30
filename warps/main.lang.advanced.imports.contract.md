# Subsystems and imports: contract-first Warp

defines: recur.lang.advanced.imports independent advanced feature planning
consumes: recur.lang.work.remaining.verification V30 advanced milestone boundary

Status: deferred contract-first plan; no feature implementation or accepted gate.
Warp: `main.lang.advanced.imports`. This is a process contract for choosing semantics,
not a language grammar or a claim that the feature exists.

## Decisions before executable requirements

Define syntax/version, root containment, cycles, alias identity, subsystem boundary and source spans. Freeze collapse/expand equivalence before implementing grammar.

## Required demonstrations after the contract

Resolve in-root imports; reject escape/missing/ambiguous imports; expose import cycles; preserve exact canonical identities and cross-boundary edges through collapse/expand.

## Ordered gates

1. **slice-contract:** write and review exact inputs/outputs, syntax/version if
   needed, errors, limits, ownership and compatibility. Record unresolved choices.
2. **slice-tests:** implement the demonstrations as Cargo/Julia assertions and
   fixtures with positive controls, record a real baseline, and verify nonzero
   eligible tests. No placeholder passing test stands in for missing behavior.
3. **slice-implementation:** implement only the reviewed contract and make its
   tests pass, preserving affected legacy behavior and pure query boundaries.
4. **slice-final:** demonstrate native behavior, restart/failure where applicable,
   docs/discovery, exact input freshness and separate parent acceptance.

This Warp remains separate from `main.lang.verification` and `main.lang.init`.
Do not promote its implementation slice while either contract or tests is absent.
Its planning presence is not a promise to include it in a.0.2.8.
