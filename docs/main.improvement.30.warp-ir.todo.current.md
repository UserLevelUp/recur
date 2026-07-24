# Improvement 30: Warp IR v1

Status: `todo.current`
Parent: `README.CORE.IMPROVEMENT30.md`
Slice: `1 / Freeze the coordination IR`
Date: 2026-07-24

## Manual Warp

```text
E0(main.improvement.30.warp-ir.todo.current)
  -> dE(freeze one versioned canonical Warp model)
  -> Ef(main.improvement.30.warp-ir.contract.complete)
```

This lifecycle is advanced manually. No command is trusted to declare the
slice complete.

## Current Eventness

The first `recur-lang warp` increment parses a useful bounded contract, but its
model is implicit inside `src/recur_lang_main.rs`. Plan rendering, receipt
validation, and mutation therefore depend directly on private parser details.
There is no versioned IR schema, source-span contract, or stable diagnostic
code surface for later pure queries and coordinator projections to share.

## Goldilocks dE

Freeze `recur-lang-warp-ir-v1` for the existing Recur Lang 0.1 Warp subset:

- language version and class identity;
- selected scope identity;
- exact input, function, and output canonical identities;
- flow mode and compact expression;
- declared Eventness edges;
- `E0`, `dE`, and `Ef`;
- source content hash;
- source spans for the scope, function, flow, event block, and Warp;
- stable diagnostic codes for rejected source.

The companion must consume this IR instead of reparsing those facts itself.
Text and JSON plans must remain projections over the same model.

## Acceptance Criteria

- The IR has an explicit `recur-lang-warp-ir-v1` schema identifier.
- Canonical identities are fully qualified and deterministic.
- Source spans use a documented byte and line convention.
- Equivalent line endings produce their own honest source hashes and spans.
- Duplicate/missing scope, function, flow, event, or Warp declarations return
  stable diagnostic codes.
- A flow/slice mismatch and an undeclared Ef state return stable diagnostic
  codes.
- The existing dry-run, ACK, NAK, stale-receipt, and bounded-root behavior
  continues to pass through the shared IR.
- The full Rust and focused Julia Recur Lang suites remain green.

## Non-Goals

- no systems, subsystems, imports, lanes, joins, waits, or feedback;
- no 0.2 watch-coordination fixture parsing;
- no live grid or coordinator loop;
- no receipt schema expansion;
- no target-language execution;
- no automatic Eventness completion.

## Frozen Contract

Schema:

```text
recur-lang-warp-ir-v1
```

Canonical JSON shape:

```json
{
  "schema": "recur-lang-warp-ir-v1",
  "language_version": "0.1",
  "class_name": "Demo",
  "source": "demo.recur",
  "source_hash": "fnv1a64:...",
  "scope": {
    "name": "verify",
    "span": {},
    "function": {
      "symbol": "f",
      "identity": "verify.f",
      "familiar_name": "Verify the artifact",
      "worker": "external.verify",
      "input": {
        "symbol": "b",
        "role": "input",
        "local_identity": "verify.i(b)",
        "canonical_identity": "source.o(b)",
        "fields": [
          {
            "name": "artifact",
            "type_name": "Text"
          }
        ]
      },
      "output": {
        "symbol": "c",
        "role": "output",
        "local_identity": "verify.o(c)",
        "canonical_identity": "verify.o(c)",
        "fields": []
      },
      "span": {}
    },
    "flow": {
      "mode": "sync",
      "expression": "i(b) -> f(b) -> o(c)",
      "span": {}
    },
    "event_span": {},
    "events": [
      {
        "edge": "state",
        "identifier": "demo.verify.complete",
        "span": {}
      }
    ],
    "warp": {
      "current": "demo.verify.todo.current",
      "slice": "verify.f",
      "desired": "demo.verify.complete",
      "span": {}
    }
  }
}
```

The abbreviated empty `fields` and `events` arrays above describe shape, not
cardinality. Runtime IR preserves every declared field and Eventness edge.

### Span convention

Every span uses:

```text
start_byte = zero-based inclusive UTF-8 byte offset
end_byte   = zero-based exclusive UTF-8 byte offset
start_line = one-based line containing start_byte
end_line   = one-based line containing the final byte
```

The source hash covers the original UTF-8 bytes. Line-ending changes therefore
produce an honestly different hash and may change byte offsets.

### Stable diagnostics

| Code | Meaning |
|---|---|
| `RLIR001` | Missing or duplicate Recur version/class declaration |
| `RLIR002` | Missing, duplicate, or unclosed selected scope |
| `RLIR003` | Missing, duplicate, or contract-invalid compact function |
| `RLIR004` | Missing or duplicate compact body flow |
| `RLIR005` | Body flow does not match the function contract |
| `RLIR006` | Missing, duplicate, or unclosed Eventness block |
| `RLIR007` | Missing or duplicate Warp declaration |
| `RLIR008` | `dE` does not reference the selected function identity |
| `RLIR009` | `E0` and `Ef` are identical |
| `RLIR010` | `Ef` is not a declared `state` Eventness edge |
| `RLIR011` | Invalid, duplicate, or unresolved bundle contract |

Diagnostics render as `[CODE] message` and may include a source span. Consumers
must branch on `code`, not parse the human message.

### Receipt binding

`recur-lang-warp-receipt-v1`, dry-run plans, and durable Warp status records
carry:

```toml
ir_schema = "recur-lang-warp-ir-v1"
```

The source hash binds evidence to exact bytes; the IR schema binds it to the
model that interpreted those bytes.

## Manual Completion Rule

After code, contract tests, documentation, and regressions agree, rename this
artifact to:

```text
docs/main.improvement.30.warp-ir.contract.complete.md
```

The completed record must preserve the final schema, diagnostic list, test
evidence, commit, and remaining limitations.

## Discovery

```powershell
recur tree "main.improvement.30.warp-ir" -d docs/
recur files "main.improvement.30.warp-ir.**" -d docs/
recur trace-id "recur.lang.warp.ir.v1" --scope "**" --ext ".md" -d .
```

## Trace-Id Lines

```text
defines: recur.lang.warp.ir.v1 first versioned canonical model for one Recur Lang 0.1 Warp
consumes: main.improvement.30.slice.1 coordination IR freeze direction
produces: main.improvement.30.warp-ir.contract stable parser projection and diagnostic boundary
triggers: main.improvement.30.static-graph future graph analysis over versioned IR
```
