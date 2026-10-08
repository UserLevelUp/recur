---
name: recur-lang
description: Inspect Recur Lang contracts and static graphs, prepare source-bound implementation plans and connect runtime evidence. Use for WIR1/CIR1 work; Lang remains optional for ordinary coding.
---

# Recur Lang

Resolve the project root and exact source. Check current executable help before
using a command from a design document. Core `recur lang` is a pure query;
`recur-lang` owns opinionated advice and lifecycle writes. In the Recur source
repository consult `docs/main.command.lang.query.readme.md`, and for planning
`docs/main.command.lang.plan.readme.md`. External projects use their local
specifications rather than assuming these repository paths exist.

```powershell
recur lang -d ROOT
recur lang show SOURCE --scope SYMBOL -d ROOT --json
recur lang report SOURCE --scope SYMBOL -d ROOT --json
recur lang check SOURCE --scope SYMBOL -d ROOT --json
recur-lang plan SOURCE --scope SYMBOL -d ROOT --json
```

Inspect WIR1 input/output aliases and canonical contracts, or CIR1 projected
messages, joins, waits and feedback edges. Language scopes are exact authored
identities, separate from filename hierarchy separators. A scoped CIR report
retains whole-flow graph findings: do not hide a cycle outside the visible lane.
Read coverage exclusions and diagnostics, not just the success exit code.
`check` success establishes soundness within coverage, not runtime behavior,
whole-source correctness or code-to-contract correspondence.

For complex work, use contracts to identify behavioral cases and dependencies
before implementing bindings. Lang is optional; preserve the user's choice
to use direct code or CLI tasks. `recur-lang plan` emits advice and source/config
fingerprints; it does not scaffold code, call an LLM, run bindings/tests or accept
Warp gates. Work-item order is not an execution schedule. Initialize preferences
with `recur-lang init` only when the task needs them; preview missing defaults
and preserve explicit settings, including the chosen implementation target.

Map the implementation's helpers and callbacks back to the declared graph.
Opaque type names do not prove domain rules, conservation, privacy or callback
behavior. Test the declared semantics against precise expected results, and
record unknown/custom bindings honestly. When using the checked-transition
workflow, read `docs/main.command.lang.checked-transition.readme.md` and help
first; preserve historical receipts and distinguish producer claims, current
fingerprints and accepted evidence.

Use hierarchical trace IDs to connect formal contracts with code, tests and
review artifacts outside the parser's boundary. Test a chosen demo explicitly
with `julia julia-tests/runtests.jl --demo NAME` in this repository; do not run
other demos merely because their artifacts appear in a language inventory.
