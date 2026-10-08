# Binding correspondence: reference-first follow-up

defines: main.lang.binding-correspondence source-bound human or LLM conformance review
consumes: demo.blackjack.dependencies.review concrete helper-reference experiment

Status: pending general protocol design. The user's intended boundary is an
ideal Lang reference that an LLM can compare with implementation source. This
increment does not require a Julia parser, code execution or runtime scheduler.

The blackjack example now records 19 helper contracts, direct calls, allowed
call directions, source fingerprints, intentional state loops and unresolved
callback effects. Its tests show that changed code invalidates the observation,
and that an implementation-only call becomes a detectable cycle once the
reviewer maps it into the reference. Core does not perform that mapping.

Slice 0 must freeze a review packet and observation schema: selected files,
source and reference fingerprints, function/binding mapping, argument and result
symbols, allowed dependencies, actual reviewed edges, mismatch explanations,
unknown/dynamic targets, termination notes, reviewer identity and scope limits.
A source change must make the observation stale. Simply refreshing hashes must
not create acceptance, and model edits must not silently legitimize faulty code.

Freeze red tests for missing private helpers, a hidden upward call, a cyclic
mapping, changed source, missing result fields, unknown callback targets and
intentional state feedback. Include the current blackjack default-call reference
as a positive control. Distinguish author self-review from independent review.

Final acceptance requires a reviewed protocol, bounded companion support if
needed, current test evidence and Eventness artifacts that preserve uncertainty.
General static Julia analysis, arbitrary binding execution and runtime effect
checking remain separate optional proposals, not prerequisites for useful
reference-based verification.
