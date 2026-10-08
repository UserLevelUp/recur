artifact.type = lane
publish: main.demo.blackjack-skill-replay.final.attempt.1.result durable worker result
consumer: main.demo.blackjack-skill-replay.coordination scoped rehydration

Intelligence and evidence live in this Eventness receipt. Its notification carries only the trace ID. Producer state and reviewed acceptance remain distinct.

```json
{
  "warp": "main.demo.blackjack-skill-replay",
  "slice": "final",
  "attempt": 1,
  "trace_id": "main.demo.blackjack-skill-replay.final.attempt.1.result",
  "state": "produced",
  "host": "acceptance-codex-high",
  "reasoning": "high",
  "claimed_at_unix": 1791438792,
  "finished_at_unix": 1791439093,
  "agent_seconds": 301,
  "artifacts": [
    {
      "reference": ".recur/blackjack-skill-workers/final/review.json",
      "sha256": "e6cc92708cf7632e2f7a6bdc914c07246033dc550186e75ef4a88171cab079b4",
      "result": {
        "schema": "blackjack-skill-coordinator-review-v1",
        "warp": "main.demo.blackjack-skill-replay",
        "slice": "final",
        "contract": "contract:main.demo.blackjack-skill-replay.final:v1",
        "recommendation": "revise",
        "acceptance": false,
        "summary": "The frozen implementation passes the selected runtime suites and independent accounting/recording probes. One user-visible contract gap remains: strategy and rules versions are available in public data but are not displayed in either the live table or completed replay. Revise that presentation and its tests before coordinator acceptance. Browser saved-download success remains unverified; neither passing tests nor this review accepts a gate.",
        "findings": [
          {
            "id": "F01",
            "priority": "P2",
            "file": "source/demos/blackjack-web/main.blackjack.rivals.js:60",
            "related_files": [
              "source/demos/blackjack-web/main.blackjack.web.js:82",
              "source/demos/blackjack-web/main.blackjack.web.js:148",
              "source/demos/blackjack-web/index.html:66",
              "source/demos/blackjack-web/skill/main.blackjack.skill.browser.test.cjs"
            ],
            "reason": "The contract requires showing strategy/rules versions with meaningful session results. Rival rendering prints only the unversioned policy name, and neither the live statistics view nor renderReplaySnapshot prints rules_id. policy_version is never consumed for display; policy_id is only used to restore reset selections. The reset option labels are also unversioned. Public JSON provenance is correct, but a person viewing results cannot identify the benchmark/rules version without exporting or inspecting JSON.",
            "evidence": [
              "main.demo.blackjack-skill-replay.contract.md, Meaningful session statistics: 'Show sample size and strategy/rules version.'",
              "main.blackjack.rivals.engine.jl public_rival supplies policy_id and policy_version=1; public_state supplies rules_id. Both engine snapshots contain versioned rules identities.",
              "coordinator-browser-probe.cjs reuses the frozen DOM harness with real identity fields supplied. Node exited 1: live_policy_name=true, live_policy_id=false, live_rules_id=false, replay_policy_id=false, replay_rules_id=false. See coordinator-browser-probe.log.",
              "The preserved narrow screenshot was independently inspected: policy names, capability labels and ROI/sample labels are visible, but strategy/rules version labels are absent."
            ],
            "required_change": "Display each effective strategy's versioned identity (or name plus explicit version) and the effective rules version beside live and replay session results. Add browser assertions for those visible labels, preserving detached rendering and existing transport. The probe uses exact IDs as one acceptable representation; equivalent explicit version text also satisfies the obligation."
          }
        ],
        "assessment": {
          "privacy_and_strategy": {
            "result": "No blocking defect found in reviewed scope.",
            "evidence": "Strategy validates exactly eight public keys and rejects Bool integers, invalid rules/policies and invalid action sets. policy_input constructs fresh values from the visible hand, dealer upcard and balance. Hidden-field sentinel tests run through the integrated engine. Both public projections mask the current hole card during play and omit deck/cursor. Completed public reports use an explicit totals/provenance allowlist, with no cards/history/commands."
          },
          "statistics_and_integers": {
            "result": "No blocking defect found in reviewed scope.",
            "evidence": "Both engines call start_round! once per accepted deal; empty rival hands skip settlement. Phase guards reject repeated settlement, and normal transitions operate on copies. The accumulator validates every hand before mutation and checked-adds into a candidate dictionary. Tests cover overflow atomicity, split denominators, final doubled stake, natural pushes and net reconciliation. Independent probes reproduced funded rivals becoming sitting-out, with started=2, active=1, sitting_out=1, settled=1, wager=20 and net=-20 per rival; repeated settle! rejected without mutation. Engine money/stake bounds and finite recordings bound ordinary ledger arithmetic."
          },
          "replay_and_bounds": {
            "result": "No blocking defect found in reviewed scope; exact-limit coverage remains bounded as described in unknowns.",
            "evidence": "Private canonicalization recursively covers engine fields, sorted string keys and ordered arrays; primitive UTF-8 SHA-256 golden vectors passed. Recording retains realized full permutations and uses isolated transitions before committing. Playback validates exact schemas, identities, types, revisions, legal transitions and every state hash, then returns a detached settled game. Commands/deals/bytes are bounded; deal admission reserves command/revision/byte capacity before starting another round. Independent 200-deal/400-command probe exported and replayed equal private state; the 201st deal failed atomically while leaving a settled exportable game. Direct recorder.game mutation is detected. Hashes establish consistency, not a signature or authenticity."
          },
          "reset_and_http_isolation": {
            "result": "No blocking defect found in reviewed scope.",
            "evidence": "Actual frozen integration tests switch 0/1/2/0 rivals, retain v2 solo/v3 rivals, pass old revision+1 into fresh recorders, reject stale/wrong-version commands and replay exports after reset. GET replay/report requires a completed session and existing cookie. POST replay has a 1 MiB body guard, accepts exactly {replay: envelope}, and returns only a completed public snapshot. It does not replace or revise live sessions; independent cookies and tampered replay rejection are tested. Browser playback uses a separate dialog and does not assign live state or revision."
          },
          "demo_selection": {
            "result": "Verified.",
            "evidence": "Frozen runtests.selection.jl maps blackjack-web to its optional wrapper; new suites are included under the selected demo's test entry. The selected suite reported core skipped and blackjack-web only; default dry-run reported core selected and no demos. No unrelated demo suite was run."
          },
          "browser": {
            "result": "Revise visible version provenance; retain download uncertainty.",
            "evidence": "All 14 frozen Node tests passed for schema-selected transport, policy selection, detached/inert rendering, ROI null/zero semantics, export Blob construction, upload errors and concurrent isolated playback. Parent browser-smoke.json reports actual zero/one/two-rival play, split/double, privacy disclosure, HTTP-exported replay upload, restored live balance and a 390px layout. The referenced narrow screenshot was independently inspected and is consistent with the layout/label claims. These are distinct evidence sources: the Node harness does not prove native browser download completion, and the historical screenshot does not prove current interactions."
          },
          "parallel_workers_and_static_boundaries": {
            "result": "Reviewed as bounded evidence, not automatic acceptance.",
            "evidence": "Worker reports distinguish author-run tests from companion integration. Statistics author reports no execution because its shell was denied; this review independently ran the frozen hardened module through the demo suite. Replay worker adapters were not treated as integrated v2/v3 proof; actual engine/HTTP tests were run here. Prepared workflow and interfaces match frozen source copies. Lang check is sound-within-coverage with no graph findings, whole_source_validated=false and execution=not-run. Plan is needs-target because preferences leave target unspecified. No preferences, source boundaries, provider assignments or gates were changed."
          }
        },
        "tested": [
          {
            "command": "C:/Users/marcn/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/bin/node.exe validate.cjs",
            "exit_code": 0,
            "result": "coordinator report shape valid; parent substantive review still required. This assigned verification checks report shape only, not acceptance."
          },
          {
            "command": "recur lang check workflow.recur --scope final -d . --json",
            "exit_code": 0,
            "result": "sound-within-coverage; no graph findings; whole_source_validated=false; execution=not-run",
            "log": "coordinator-lang-check.json"
          },
          {
            "command": "recur-lang plan workflow.recur --scope final -d . --json",
            "exit_code": 0,
            "result": "needs-target; target unspecified; static advice only",
            "log": "coordinator-lang-plan.json"
          },
          {
            "command": "C:/Users/marcn/.julia/juliaup/julia-1.12.7+0.x64.w64.mingw32/bin/julia.exe --startup-file=no -O0 -C generic --project=C:/src/recur/demos/web-evidence-lab source/julia-tests/runtests.jl --demo blackjack-web",
            "environment": {
              "RECUR_BIN": "C:/Users/marcn/.cargo/bin/recur.exe"
            },
            "exit_code": 0,
            "result": "19868/19868 assertions passed; core skipped; only blackjack-web selected. Includes strategy/statistics/replay, original solo/split/rival, HTTP/session and Lang/source-freshness checks. The displayed SGR001 cycles are expected injected fault fixtures, not suite failures.",
            "log": "coordinator-demo.log"
          },
          {
            "command": "C:/Users/marcn/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/bin/node.exe --test source/demos/blackjack-web/main.blackjack.rivals.test.cjs source/demos/blackjack-web/skill/main.blackjack.skill.browser.test.cjs",
            "exit_code": 0,
            "result": "14 tests passed; 0 failed",
            "log": "coordinator-browser-node.log"
          },
          {
            "command": "C:/Users/marcn/.julia/juliaup/julia-1.12.7+0.x64.w64.mingw32/bin/julia.exe --startup-file=no --project=C:/src/recur/demos/web-evidence-lab source/julia-tests/runtests.jl --dry-run",
            "exit_code": 0,
            "result": "Core tests selected; demo tests none",
            "log": "coordinator-default-selection.log"
          },
          {
            "command": "C:/Users/marcn/.julia/juliaup/julia-1.12.7+0.x64.w64.mingw32/bin/julia.exe --startup-file=no -O0 -C generic --project=C:/src/recur/demos/web-evidence-lab coordinator-probes.jl",
            "exit_code": 0,
            "result": "24/24 assertions passed: real 200-deal limit, exact replay, atomic 201st rejection, sit-out reconciliation, once-only settlement and public report net totals",
            "log": "coordinator-probes.log"
          },
          {
            "command": "C:/Users/marcn/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/bin/node.exe coordinator-audit.cjs",
            "exit_code": 0,
            "result": "All 45 manifest entries match frozen files; 51 source files fingerprinted; interfaces.md and workflow.recur match their frozen source counterparts. Manifest hash also matches the integration report's source_manifest_sha256.",
            "log": "coordinator-source-audit.json"
          },
          {
            "command": "C:/Users/marcn/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/bin/node.exe coordinator-browser-probe.cjs",
            "exit_code": 1,
            "result": "Confirmed F01: policy names render, versioned policy/rules identities do not render in live or replay results",
            "log": "coordinator-browser-probe.log"
          },
          {
            "command": "recur warp show main.demo.blackjack-skill-replay -d C:/src/recur/warps --json; recur warp slices main.demo.blackjack-skill-replay -d C:/src/recur/warps --json",
            "exit_code": 0,
            "result": "Read-only recovery: final ready and pending; contract final:v1; coordinator-acceptance pending. Earlier slices have recorded completion; those declarations were not substituted for source/runtime review.",
            "logs": [
              "coordinator-warp-show.json",
              "coordinator-warp-slices.json"
            ]
          }
        ],
        "observed_failures": [
          "Initial Warp list/show/slices used C:/src/recur/warps/blackjack-skill, which contains the workflow but not the map: list found zero maps and show/slices failed. Read-only recovery at C:/src/recur/warps succeeded.",
          "Initial Julia default-selection dry-run omitted --project and raised EACCES while resolving C:/Users/marcn. The combined shell call did not retain that Julia exit separately. Repeating with the supplied installed project succeeded; no permission or runtime security setting was changed.",
          "Two auxiliary rg calls used wildcard characters in positional Windows paths and emitted path errors. Explicit paths or rg -g filters supplied the necessary reads.",
          "The independent browser provenance probe failed as expected for the confirmed F01 defect. No implementation was edited to make it pass."
        ],
        "unknowns": [
          "Browser download event remains uncertain: parent browser-smoke.json says the UI reported downloaded, but its download event timed out and no saved browser file was independently located. HTTP bodies and Node Blob/click behavior passed; neither proves a saved file. This review did not rerun native browser download or locate a saved export.",
          "Browser interactions and live-http-smoke are supplied historical observations. This review inspected the narrow screenshot and reran frozen Node/Julia HTTP tests, but did not independently repeat the reported real browser interaction sequence. Referenced screenshots live outside the frozen evidence directory.",
          "The replay suite checks primitive canonical golden vectors and actual engine round trips. Fixed cross-implementation private-engine hash fixtures and exhaustive canonical-type coverage are not established by the frozen tests. Worker supplementary test reports describe broader cases, but their extra-tests source/raw execution logs are not supplied here.",
          "The independent probe exercised the real 200-deal boundary, not 2000 legal accepted commands or a valid envelope of exactly 1 MiB. The frozen command-capacity test seeds private step lists; near-limit rejection and bounded-writer source review do not constitute exhaustive boundary execution.",
          "Provider history, the three-worker overlap, unsupported Gemini client, and shell-denied Copilot execution are described in supplied reports/deficiencies. Raw provider attempt logs and timing traces are not included here, so provider access, retry classification and comparative speed/cost are not independently established. No provider was invoked or retried by this reviewer.",
          "Historical red-result.json reports tests executed against unimplemented stubs; raw red logs are absent. Later statistics hardening changed that test fingerprint. Current passing behavior was checked directly; historical red-first provenance is not independently reconstructed.",
          "Lang planning remains needs-target; static graph soundness excludes function bodies, runtime behavior and receipt validation. A WorkReceipt declaration is not evidence that a reviewed gate has been accepted.",
          "Sessions/recorders are process-local and playback presents a final settled snapshot. Hashes prove consistency, not authenticity; a replay author can construct a different internally consistent recording. Unequal human/rival actions, shared deck, seat order, wagers and bankroll prevent interpreting profit or ROI as a fair skill rating."
        ],
        "preservation": {
          "source_manifest_sha256": "17314d9c1bad172c106d84ee6dd33c472f77b1ac10811b388c7af3aa05f451ff",
          "source_edited": false,
          "prepared_boundaries_edited": false,
          "agents_dispatched": false,
          "committed": false,
          "pushed": false,
          "gates_accepted": false,
          "authored": "review.json plus private coordinator probes, query captures, fingerprints and test logs in this workspace only"
        }
      }
    }
  ],
  "acceptance": "separate reviewed gate; worker result does not imply acceptance",
  "raw_execution_references": {
    "stdout": "\\\\?\\C:\\src\\recur\\.recur\\dispatch\\fnv1a64-a7b70ede52e7954d\\fnv1a64-7c324d8022a76537.1.agent.stdout.txt",
    "stderr": "\\\\?\\C:\\src\\recur\\.recur\\dispatch\\fnv1a64-a7b70ede52e7954d\\fnv1a64-7c324d8022a76537.1.agent.stderr.txt"
  }
}
```
