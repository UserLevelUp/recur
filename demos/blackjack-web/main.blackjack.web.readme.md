# The Green Room: a playable blackjack website

defines: demo.blackjack.web single-player contract-first website
consumer: demo.blackjack.cards shared Julia rules from blackjack-lab

A second, separate demo: you choose the bet, Hit, Stand, Double Down or Split against
Ada, the computer dealer. The responsive page includes card faces, hidden dealer
card, bankroll, win/loss/push counts, the last 20 rounds, rules and a fresh-table
dialog. All chips are simulated. No graphics downloads or Python are required.

## Play

Optional computer rivals: choose **New table → Computer rivals → One rival or
Two rivals**. They share Ada and the shuffled deck, have independent bankrolls,
and act automatically using a visible Stand on 17 benchmark. Solo remains the
default. See rivals/main.blackjack.rivals.readme.md and Warp
`main.demo.blackjack-computer-player`; net results are not yet a skill rating.

From the repository root in PowerShell:

```powershell
.\demos\blackjack-web\main.blackjack.web.start.ps1
```

Open **http://127.0.0.1:8791**. Keep the server running; Ctrl+C stops it.
The launcher prefers Juliaup on Windows. `-Port 8792` selects another port.
The browser needs JavaScript. H/S/D/P shortcuts work during your turn except while
editing a field or using a dialog. Enter/Space activate focused buttons normally.

Portable direct command (Julia 1.12 with the existing demo dependencies):

```text
julia --startup-file=no -O0 -C generic --project=demos/web-evidence-lab demos/blackjack-web/main.blackjack.web.server.jl --port 8791
```

On a fresh checkout instantiate the existing pinned environment first:
`julia --project=demos/web-evidence-lab -e 'using Pkg; Pkg.instantiate()'`.
The app reuses its HTTP/JSON3 packages and Julia standard libraries. It does not
run the other demo server or change its routes.

Julia is authoritative: the browser never receives the deck or active hole card,
and never computes outcomes. The server uses separate cookie sessions, a revision
guard and a locked atomic state replacement. Page refresh preserves your hand.
Restarting the process or two hours idle clears the session; history is in memory.
The local server is bound to loopback and is not a public deployment.

## Lang-first progression

| Stage | Reference | Implementation / verification |
| --- | --- | --- |
| 01 table | `main.blackjack.web.01.table.recur` | Private game creation and exact public projection |
| 02 play | `main.blackjack.web.02.play.recur` | Pure copied transitions, escrow and settlement; deterministic plus randomized Julia cases |
| 03 HTTP | `main.blackjack.web.03.http.recur` | Cookie isolation, stale actions, malformed input, hidden cards and actual loopback requests |
| 04 browser | `main.blackjack.web.04.browser.recur` | Responsive HTML/CSS, presentation-only JavaScript; assets plus real browser play-through |
| 05 review | `main.blackjack.web.05.dependencies.recur` | CIR review prerequisites, intentional cycle fault and source-bound author review |

The WIR descriptions and failing tests preceded engine, server and browser asset
implementation. Runtime fields refined the initial reference: the output-shape
test caught omitted fields and the binding test caught a namespace mismatch.
The CIR review reference was added after the implementation existed, before
the correspondence tests and final acceptance. It is retrospective review order,
not a claim that the website is automatically generated or executed by Lang.

`main.blackjack.web.review.md` maps helpers, symbols, effects, loops and logical
bindings. Its JSON observation binds reviewed source hashes. Changed code requires
renewed review, not mechanical rehashing. Static success alone cannot prove that
Julia/JavaScript has no undeclared cycles or semantic bugs.

## Query and test

```powershell
recur tree main.blackjack.web -d demos/blackjack-web
recur lang -d demos/blackjack-web
recur lang show main.blackjack.web.01.table.recur --scope view.f --expand -d demos/blackjack-web
recur lang check main.blackjack.web.05.dependencies.recur -d demos/blackjack-web --json
recur-lang plan main.blackjack.web.02.play.recur -d demos/blackjack-web
recur trace-id 'demo.blackjack.web.**' --scope 'main.blackjack.web.**' -d demos/blackjack-web --format full
julia --startup-file=no -O0 -C generic --project=demos/web-evidence-lab demos/blackjack-web/main.blackjack.web.test.jl
```

Use Juliaup 1.12.7 on the development Windows host. As in the first demo, only the
assertion module uses minimal compilation/inference settings to avoid the known
Windows Julia compiler fault; game/server code remains ordinary Julia. Set
`RECUR_BIN` if testing a specific executable. The full suite includes a wrapper.
`BLACKJACK_WEB_STAGE=engine` runs only the engine during development; remove this
environment variable for full acceptance.

See `main.blackjack.web.requirements.md` for exact game rules,
`main.blackjack.web.verification.complete.md` for observed results and
`main.blackjack.web.improvements.todo.md` for remaining scope. Warp identity:
`main.demo.blackjack-web` under the repository's `warps` directory.

## Split v2

One equal-rank split creates two independently wagered hands sharing one dealer. Play the highlighted hand first. Split aces get one card each; split 21 pays 1:1. See split/main.blackjack.web.split.review.md for implementation bindings and split/main.blackjack.web.split.verification.complete.md for current acceptance. The original v1 review and verification are historical. Restart the server and reload the page when upgrading from v1. Warp: main.demo.blackjack-web.split.
