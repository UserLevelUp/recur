# main.command.flatten.separator-token.investigation

Date: 2026-03-01

## Question

Can multi-character separators improve deep structured pipelines:

- `flatten -> filter -> merge -> (future) unflatten`

especially for complex JSON/XML with deep hierarchies?

## Current State (verified)

1. `tree/files/merge` now accept token separators (example: `--sep "__"`).
2. `flatten` still behaves as single-char separator at execution time.
3. `unflatten` is still contract-only (not implemented in CLI).

### Verification example

```bash
recur --sep "__" flatten nested.json --format json --json
```

Observed output path used `_`, not `__` (`a_b` vs expected `a__b`).

## Performance Spike (local machine, tmp fixture)

Dataset: `wide3000.json` (~116 KB, 9000 flattened entries)

- `flatten_dot`: avg ~144 ms
- `flatten_uscore`: avg ~143 ms
- `flatten_dunder_arg`: avg ~147 ms (currently same effective behavior as `_` in flatten)
- `flatten_uscore_filter_root_k2`: avg ~63 ms

Merge timing on prebuilt flattened JSON:

- `merge_uscore_flat_single`: avg ~337 ms
- `merge_dunder_flat_single`: avg ~343 ms
- `merge_mixed_two_sources`: avg ~502 ms

Interpretation:

1. Separator token length itself is not the dominant cost in this scale.
2. Early filtering is high leverage (roughly 2x faster in this spike).
3. Merge cost scales with source count and input size more than separator style.

## Correctness Spike: Collision Risk

Single-char separators can collapse distinct structures when keys contain separator characters.

Example:

- Single `_` path domain collapses to one branch: `root_a_b_c`
- Token `__` preserves distinctions:
  - `root__a_b__c`  -> `root -> a_b -> c`
  - `root__a__b_c`  -> `root -> a -> b_c`

This is a strong argument for token-aware `flatten` and future `unflatten`.

## Design Implications for Future Unflatten

1. `flatten` and `unflatten` should share the same separator token semantics.
2. Unflatten should support collision diagnostics (detect ambiguous round-trips).
3. Path token escaping strategy is required for keys containing the separator token.
4. Benchmarks should include:
   - deep-only trees
   - wide-only trees
   - mixed deep+wide real fixtures
   - filter-before-merge vs merge-before-filter

## Recommended Next Step

Add token separator support to flatten internals first, then implement unflatten against the same token contract so round-trip invariants can be tested end-to-end.
