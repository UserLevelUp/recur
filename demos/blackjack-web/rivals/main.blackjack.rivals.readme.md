# Optional rival seats

defines: demo.blackjack.rivals playable benchmark seats
defines: demo.blackjack.rivals.policy.decision visible hand policy
defines: demo.blackjack.rivals.table.deal shared finite deck and seat order
defines: demo.blackjack.rivals.table.advance automatic rivals then dealer
defines: demo.blackjack.rivals.table.settlement independent outcomes and one settlement
defines: demo.blackjack.rivals.table.projection public detached snapshots
defines: demo.blackjack.rivals.table.ledger conserved wallets, escrow and shared house
defines: demo.blackjack.rivals.table.revision copied guarded transitions
defines: demo.blackjack.rivals.http.revision session mode and revision guards
defines: demo.blackjack.rivals.browser.seats safe accessible seat presentation
consumes: demo.blackjack.rivals.coordination asynchronous CLI worker verification

Open the local Julia website and choose **New table → Computer rivals → One rival
or Two rivals**. Solo remains the default. Changing seats starts a fresh session
and is available only between rounds. You retain Hit, Stand, Double and Split;
the rivals act automatically after your hands finish, then Ada plays once.

All seats share one shuffled deck and dealer. Each rival has a separate bankroll,
stake and results. V1 policy is visibly labeled **Stand on 17 benchmark**: hit
below 17, otherwise stand, with no Split or Double. Rivals use their own visible
hand score; no policy receives the private deck or dealer hole card. Net chips
compare observed results and do not establish a skill rating or optimal strategy.

The optional mode uses public schema v3 and protocol 3; solo retains the exact
v2 engine/schema. HTTP handler selects the current protocol, rejects stale/replayed
actions, and keeps cookie sessions isolated. Normal browser reload preserves state.

Warp: `main.demo.blackjack-computer-player`. Engine and browser modules were
produced by two actual concurrent Codex CLI workers in disjoint workspaces.
`recur-watch dispatch` tracked completion and worker-run checks; root coordinator
reviewed and accepted each gate before integrating the modules. A dependent
CLI-only Node verification job ran the existing website suite, then its worker
independently reran HTTP and JavaScript checks. No model invocation was needed
for that verification slice. Polling and gate review remain distinct steps.

Trace a bounded branch:

```powershell
recur trace-id 'demo.blackjack.rivals.**' --scope 'main.blackjack.rivals.**' -d demos/blackjack-web --json
recur warp show main.demo.blackjack-computer-player -d . --json
node demos/blackjack-web/rivals/main.blackjack.rivals.verify.cjs
```

The check launcher names this desktop's Node/Julia binaries intentionally; update
those paths for another platform. Its earlier PowerShell helper and failed host
attempt logs are retained for diagnosis of the local unsigned-script restriction.
New source review and verification observations are separate from original Split
receipts. Lang is optional and the existing fragment models do not prove rival mode.
