artifact.type = lane
publish: main.demo.blackjack-skill-replay.statistics.attempt.1.result durable worker result
consumer: main.demo.blackjack-skill-replay.coordination scoped rehydration

Intelligence and evidence live in this Eventness receipt. Its notification carries only the trace ID. Producer state and reviewed acceptance remain distinct.

```json
{
  "warp": "main.demo.blackjack-skill-replay",
  "slice": "statistics",
  "attempt": 1,
  "trace_id": "main.demo.blackjack-skill-replay.statistics.attempt.1.result",
  "state": "produced",
  "host": "blackjack-skill-copilot",
  "reasoning": "medium",
  "claimed_at_unix": 1791437693,
  "finished_at_unix": 1791437739,
  "agent_seconds": 45,
  "artifacts": [
    {
      "reference": ".recur/blackjack-skill-workers/statistics/report.json",
      "sha256": "a3cab6ff03f96af7b7703ede0599c674910f7da22358d2095375bce98780a6c4",
      "result": {
        "status": "implemented",
        "files_changed": [
          "main.blackjack.skill.statistics.jl",
          "report.json"
        ],
        "tests_run": [],
        "recur_lang": "not run: shell execution was denied (no interactive approval); no static advice obtained",
        "remaining_concerns": [
          "Frozen test main.blackjack.skill.statistics.test.jl has NOT been run; the shell was unavailable. The companion/host must run the assigned Julia verification command.",
          "Code was only desk-checked against the frozen tests and interfaces.md, not executed.",
          "Hand profit is validated only as an integer; payout math/conservation remain engine-owned."
        ]
      }
    }
  ],
  "acceptance": "separate reviewed gate; worker result does not imply acceptance",
  "raw_execution_references": {
    "stdout": "\\\\?\\C:\\src\\recur\\.recur\\dispatch\\fnv1a64-a7b70ede52e7954d\\fnv1a64-4e4f410661615f4a.1.agent.stdout.txt",
    "stderr": "\\\\?\\C:\\src\\recur\\.recur\\dispatch\\fnv1a64-a7b70ede52e7954d\\fnv1a64-4e4f410661615f4a.1.agent.stderr.txt"
  }
}
```
