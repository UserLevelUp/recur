# Browser worker report

Warp: `main.demo.blackjack-computer-player`  
Slice: `browser`  
Contract: `contract:demo.blackjack.rivals.browser:v1`

## Changes

- Implemented `main.blackjack.rivals.js` with browser and CommonJS exports for `render`, `accepts`, and `protocolVersion`.
- Added named semantic rival sections, headings, card lists, supplied score flags, bankroll, escrow, net, policy, status and outcomes. All snapshot content uses DOM text/attribute methods. Public card IDs are decoded into rank/suit labels solely for presentation.
- Zero rivals clears and hides the container; later snapshots restore it. Rendering replaces previous panels and never mutates snapshots or computes scores, actions or payouts.
- Preserved both original supplied tests and added six tests for exports, version selection, replacement/hiding, accessibility, immutable authoritative values, safe text, empty hands, hidden scores and card labels.
- Included the three required trace roles. No prepared Lang source, canonical application file, coordinator configuration, historical evidence or acceptance artifact was changed.

## Constraint recovery

Read local `task.md`, the frozen Warp contract, the Recur expert skill and Warp playbook guidance, and the existing browser projection boundary and card representation. No applicable ancestor `AGENTS.md` was present in the checked workspace ancestry.

Inspected `recur warp --help`, then queried `recur warp show main.demo.blackjack-computer-player -d C:/src/recur --json`. It reported the matching browser contract ready/pending and browser-tests evidence absent. These CLI queries recover coordination context; they do not implement the view or establish acceptance.

## Observed verification

Run from the assigned browser workspace on 2026-10-06:

```text
C:/Users/marcn/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/bin/node.exe --test main.blackjack.rivals.test.cjs
```

Baseline: exit 1, 0 passed / 2 failed, because the placeholder lacked `accepts` and `render`.

After implementation: exit 0, 8 passed / 0 failed / 0 skipped. Tests execute the JavaScript against the supplied fake DOM, with separate VM loading checks for browser and Node exports. This is observed runtime evidence for the tested fragment, not real-browser or integration acceptance.

An initial read-only `rg` lookup used an incorrect demo directory and Windows wildcard arguments and failed; corrected file discovery located `demos/blackjack-web` and the relevant sources. No unresolved test failures remain.

## Integration limits and questions

`accepts` recognizes exactly schemas v2 and v3. `protocolVersion` derives 2 or 3 from the schema and throws TypeError for unknown/missing schemas. It is a selector, not a validator: the integrator retains responsibility for rejecting inconsistent protocol fields, validating snapshots and enforcing revision ordering.

No blocking contract questions remain for this slice. Canonical asset inclusion, chooser, HTTP mode switching, gameplay/privacy regressions, actual browser play with one/two rivals and narrow-viewport review remain unverified here and belong to integration/review. No commits, pushes, dependency installation, acceptance marking or publication were performed.
