# Lang verification Warp

defines: recur.lang.verification.plan tests-first implementation slices
consumes: recur.lang.work.remaining.verification remaining requirements

Read the [contract](main.lang.verification.contract.md),
[demo guide](../demos/lang-verification/main.lang.verification.readme.md) and
[complete todo ledger](../demos/lang-verification/main.lang.verification.ledger.json).

```powershell
recur warp show main.lang.verification -d .
recur warp slices main.lang.verification -d .
recur reveal main.lang.verification -d .
```

The initial task is tests and planning. Product fixes remain in separate slices;
the red demo suite stays outside the normal regression runner until green.
Existing init and runtime-evidence Warps retain their contracts and history.
