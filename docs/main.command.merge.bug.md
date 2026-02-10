# Bug: Three separators crash merge

## Summary
Using three separators with merge causes a crash. This blocks multi-domain merges beyond two separators.

## Repro
```
recur.exe tree main --sep "." --sep "_" --sep "-" --show-sep
```

## Expected
Command completes and merges across three separator domains.

## Actual
Process crashes when the third separator is supplied.

## Notes
- Appeared during Phase 3 validation.
- Two separators work as expected.
