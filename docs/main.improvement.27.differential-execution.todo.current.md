# Improvement 27 differential execution

Status: `todo.current`
Date: 2026-07-15

## Warp signature

```text
E_k       = observable eventness state at slice k
E_target  = constrained intended state
ΔE_k      = E_target - E_k
slice_k   = smallest verified change that reduces a named residual in ΔE_k
```

Each slice is query-first, test-verified, checkpointed with `recur-git`, and
committed separately. A companion executable remains deferred until its core
query contract has proven useful.

## Slice trajectory

1. `E0 → E1`: lane and Psyche structural contracts green.
2. `E1 → E2`: Improvement 26 generic governance fixtures.
3. `E2 → E3`: trace-id saved-run freshness policy and tests.
4. `E3 → E4`: frozen `warp-status-v1` fixture contract.
5. `E4 → E5`: read-only `recur warp status`.
6. `E5 → E6`: read-only `recur warp explain` and `recur warp next`.
7. `E6 → E7`: document the confirmation-gated `recur-warp` automation boundary.

## Slice 1 receipt

`main.improvement.21.phase1.complete` records the lane/Psyche delta and its
full-suite receipt. The next residual is generic governance evidence rather
than warp scoring.

## Slice 2 target

```text
E1 = governance semantics are described but not yet a committed generic fixture
Δ  = config-defined artifact policy, history queries, preserved snapshots, and
     explicit ACK/NAK receipts
E2 = a domain-neutral version-eventness contract with pure-query and companion
     writer boundaries
```

## Slice 3 target

```text
E2 = cached trace evidence can be interpreted ambiguously
Δ  = explicit retention, freshness, and consumer-trust policy
E3 = only fresh saved runs may supply derived evidence to a later warp query
```

## Slice 4 target

```text
E3 = warp has a concept but no executable evidence envelope
Δ  = frozen JSON fields plus synthetic optimum/sub-optimum/blocked cases
E4 = scorer implementation can be judged against stable, domain-neutral fixtures
```
