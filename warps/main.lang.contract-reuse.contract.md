# Reusable Lang contracts: contract-first follow-up

defines: main.lang.contract-reuse boundary for reusable WIR results and state
consumes: demo.blackjack.improvements.contracts repeated and nested state shapes

Status: pending contract/test design; no grammar change or acceptance claimed.
The blackjack experiment needed configuration data in both round and session
outputs. WIR input aliases already work; general output reuse is a separate
proposal. Equal field shape must not silently imply shared identity.

Slice 0 must freeze explicit syntax, versioning, canonical identity, cycle
handling, optional/default semantics and compatibility. Demonstrate current
behavior and publish runnable expected-red cases before implementation.

Required examples: round output reused by session, state plus action, custom
target type, contradictory field types, cyclic aliases, missing fields, explicit
opt-outs, old WIR1 sources unchanged and source spans preserved. Include a
positive control plus a deliberate field-shape mismatch from blackjack.

Final acceptance requires reviewed contracts, current tests, bounded changes,
query/binding documentation and regression evidence. This is independent of
advanced imports and dynamic scatter/gather; do not bundle those semantics here.
