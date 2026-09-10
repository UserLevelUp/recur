# Greeting fixtures — current

Contract: `contract:main.lang.dogfood.slice-1:v1`.
Gate: `greeting-boundary-fixtures`.

Start with `demos/web-evidence-lab/main.greeting.recur` and the slice-1 section
of `main.lang.dogfood.contract.md`. Derive a table of independent expected
responses before making implementation changes. Preserve the 33 existing
server assertions and the 33 inspector assertions observed at baseline.

Next action: add boundary fixtures for missing/whitespace names, 1/40/41 Julia
characters, multibyte names, locale fallback and transport behavior. Run them
against the existing server first. Record any mismatch as a specification or
behavior question before changing an established contract.

The new inspector route/page is planned, not implemented. Its contract and red
tests belong to slice-2, after this boundary is established.

consumes: main.lang.dogfood.baseline observed starting behavior
produces: main.lang.dogfood.greeting-fixtures boundary acceptance examples
