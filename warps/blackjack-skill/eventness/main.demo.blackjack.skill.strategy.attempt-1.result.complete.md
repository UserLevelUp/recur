artifact.type = lane
publish: demo.blackjack.skill.strategy.attempt.1.result durable worker result
consumer: demo.blackjack.skill.coordination scoped rehydration

Intelligence and evidence live in this Eventness receipt. Its notification carries only the trace ID. Producer state and reviewed acceptance remain distinct. Earlier hyphenated Warp-slug signatures are preserved as legacy metadata; current signatures are trace-queryable.

```json
{
  "warp": "main.demo.blackjack-skill-replay",
  "slice": "strategy",
  "attempt": 1,
  "trace_id": "demo.blackjack.skill.strategy.attempt.1.result",
  "legacy_signature": "main.demo.blackjack-skill-replay.strategy.attempt.1.result",
  "state": "produced",
  "host": "blackjack-skill-codex",
  "reasoning": "high",
  "claimed_at_unix": 1791437560,
  "finished_at_unix": 1791437690,
  "agent_seconds": 129,
  "artifacts": [
    {
      "reference": ".recur/blackjack-skill-workers/strategy/report.json",
      "sha256": "57c746311b81442f23971e395671bc0d787a88ffe20502517f4d3ade4f4a811c",
      "result": {
        "status": "implemented",
        "warp": "main.demo.blackjack-skill-replay",
        "slice": "strategy",
        "contract": "contract:main.demo.blackjack-skill-replay.strategy:v1",
        "files_changed": [
          "main.blackjack.skill.strategy.jl",
          "extra-tests.jl",
          "report.json"
        ],
        "changes": [
          "Implemented detached NamedTuple records for the two versioned hit/stand benchmark policies.",
          "Validated exact public-information keys, integer ranges excluding Bool, actual booleans, rules identity, and nonempty unique legal actions before choosing an action.",
          "Implemented deterministic stand17 and dealer-aware decisions with suit-independent card ranks, natural/bust preference, and legal-action fallback.",
          "Added supplementary tests without changing frozen tests, interfaces, workflow, or reference engines/rules."
        ],
        "tests_run": [
          {
            "program": "C:/Users/marcn/.julia/juliaup/julia-1.12.7+0.x64.w64.mingw32/bin/julia.exe",
            "args": [
              "--startup-file=no",
              "-O0",
              "-C",
              "generic",
              "--project=C:/src/recur/demos/web-evidence-lab",
              "main.blackjack.skill.strategy.test.jl"
            ],
            "executed_by": "strategy implementation agent",
            "exit_code": 0,
            "passed": 2949,
            "failed": 0
          },
          {
            "program": "C:/Users/marcn/.julia/juliaup/julia-1.12.7+0.x64.w64.mingw32/bin/julia.exe",
            "args": [
              "--startup-file=no",
              "-O0",
              "-C",
              "generic",
              "--project=C:/src/recur/demos/web-evidence-lab",
              "extra-tests.jl"
            ],
            "executed_by": "strategy implementation agent",
            "exit_code": 0,
            "passed": 554,
            "failed": 0,
            "testsets": {
              "Detached policy records": 4,
              "Strict public input and no mutation": 110,
              "Golden decision table across suits": 440
            }
          }
        ],
        "static_advice": {
          "commands_run_once": [
            "C:/src/recur/target/debug/recur.exe lang check workflow.recur -d . --json",
            "C:/src/recur/target/debug/recur-lang.exe plan workflow.recur -d . --json",
            "C:/src/recur/target/debug/recur.exe lang show workflow.recur --scope strategy -d . --json"
          ],
          "exit_codes": [
            0,
            0,
            0
          ],
          "source_hash": "fnv1a64:85d2b2c7bf799dbf",
          "check": "sound-within-coverage",
          "findings": [],
          "plan": "needs-target",
          "plan_note": "Project preferences have target unspecified; this assignment explicitly supplies Julia. No preferences or prepared Lang boundaries were changed.",
          "boundary_review": "Strategy remains a standalone module with local validation and decision helpers. It imports no engine, replay, statistics, or private state. The prepared strategy receipt edge into integration remains unchanged.",
          "limitations": "Static checks exclude function bodies, whole-document grammar, runtime behavior and receipt validation. They did not execute Julia or accept any gate."
        },
        "frozen_sha256": {
          "interfaces.md": "b2cfd94ee042b740223f5ca6a1013bb43fea00e711711748172b930f9c546382",
          "workflow.recur": "c048d6714ca83d1e9fb6040efa56b599683bde7c6ee11223a0cff3da364eae6a",
          "main.blackjack.skill.strategy.test.jl": "958978859b82347b21a74424a4d44f48d8bee2c74b44da7b27ff63f1b165eaca"
        },
        "failures": [],
        "remaining_concerns": [
          "Lang planning target remains unspecified in existing project preferences.",
          "Engine, HTTP, browser and replay integration are outside this slice and were not tested here.",
          "The strategy-tests gate remains for coordinator review; no acceptance was marked."
        ],
        "unresolved_implementation_questions": [],
        "acceptance_marked": false,
        "committed": false,
        "pushed": false
      }
    }
  ],
  "acceptance": "separate reviewed gate; worker result does not imply acceptance",
  "raw_execution_references": {
    "stdout": "\\\\?\\C:\\src\\recur\\.recur\\dispatch\\fnv1a64-a7b70ede52e7954d\\fnv1a64-2f2724f046220c64.1.agent.stdout.txt",
    "stderr": "\\\\?\\C:\\src\\recur\\.recur\\dispatch\\fnv1a64-a7b70ede52e7954d\\fnv1a64-2f2724f046220c64.1.agent.stderr.txt"
  }
}
```
