# Split analysis: historical pre-implementation observation

Superseded for current status by main.blackjack.web.split.verification.complete.md. Statements below describe the analysis-time baseline; the preserved logs and fingerprints have not been rewritten to claim current acceptance.

defines: demo.blackjack.web.split.analysis reviewed design, not implemented game behavior
consumer: demo.blackjack.web.split.design proposed two-hand policy
consumer: demo.blackjack.web.split.tests acceptance matrix
publish: demo.blackjack.web.split.analysis.result observed model checks and runtime gaps
register: demo.blackjack.web.split.implementation pending implementation Warp

Observed 2026-10-02 on `recur-lang`. The report concerns the local blackjack
website. The existing game/server/UI and their accepted source review remain
unchanged. Both new Lang files are isolated under `split/` as proposals.

## What was actually run

- Normal model suite: **12 passed**, exit 0. Both proposed WIR/CIR fragments
  validate within declared coverage; their execution field is `not-run`.
- Injected dependency fault: SGR001 reports
  `dealer -> settlement -> hands -> turns -> dealer`, including when scoped to
  the view lane. This proves the checker sees that declared cycle, not that it
  extracts hidden calls from Julia.
- `--runtime-gap` mode: model assertions pass, then **2 expected failures**,
  exit 1. The current pair fixture has no Split action and the current response
  has no `hands` field. This intentionally red mode is outside the normal runner.
- Trace query: recorded role sites link the proposed contract, design, test plan
  and observation. The trace JSON is a query result, not runtime acceptance.
- Existing website regression after adding these isolated proposals: **8,702
  passed**, exit 0. No gameplay code was changed.
- Visual report/source viewer: all 12 allowlisted routes returned 200; every
  raw evidence response matched its actual project file. Browser navigation to
  the design file displayed its path, SHA-256 and trace-ID header correctly.

The 22-row runtime acceptance matrix is a plan. It has not been implemented or
passed. The new Warp `main.demo.blackjack-web.split` has no accepted layers;
its implementation and final acceptance are still pending.

## What using Lang changed

1. Expanding the output bundle exposed the single-hand API and ledger assumptions.
2. Separating advance/dealer/settle scopes exposed the premature dealer-run risk.
3. Annotating hand origin exposed the two-card-21 natural-payout trap.
4. Separating hand result from round result exposed mixed outcomes and statistics.
5. Mapping review prerequisites made the prohibited backwards dependency testable.

These findings came from comparing source to the reference as an author review.
Lang did not autonomously infer the rules, discover every call or prove the game.

## Reproduce

From `C:\src\recur`:

```powershell
recur lang check main.blackjack.web.split.contract.recur -d demos/blackjack-web/split --json
recur lang check main.blackjack.web.split.dependencies.recur -d demos/blackjack-web/split --json
recur lang show main.blackjack.web.split.contract.recur --scope split.f --expand -d demos/blackjack-web/split
recur trace-id 'demo.blackjack.web.split.**' --scope 'main.blackjack.web.split.**' -d demos/blackjack-web/split --format full
julia --startup-file=no -O0 -C generic --project=demos/web-evidence-lab demos/blackjack-web/split/main.blackjack.web.split.model.test.jl
# Intentionally fails until Split is implemented:
julia --startup-file=no -O0 -C generic --project=demos/web-evidence-lab demos/blackjack-web/split/main.blackjack.web.split.model.test.jl --runtime-gap
```

On this Windows host use Juliaup's executable at
`C:\Users\marcn\AppData\Local\Microsoft\WindowsApps\julia.exe`.
Raw observations and scoped fingerprints are under `split/observations`.
