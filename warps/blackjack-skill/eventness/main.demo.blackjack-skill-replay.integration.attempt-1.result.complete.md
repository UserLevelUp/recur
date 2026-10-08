artifact.type = lane
publish: main.demo.blackjack-skill-replay.integration.attempt.1.result durable worker result
consumer: main.demo.blackjack-skill-replay.coordination scoped rehydration

Intelligence and evidence live in this Eventness receipt. Its notification carries only the trace ID. Producer state and reviewed acceptance remain distinct.

```json
{
  "warp": "main.demo.blackjack-skill-replay",
  "slice": "integration",
  "attempt": 1,
  "trace_id": "main.demo.blackjack-skill-replay.integration.attempt.1.result",
  "state": "produced",
  "host": "blackjack-skill-verifier",
  "reasoning": "host-default",
  "claimed_at_unix": 1791438600,
  "finished_at_unix": 1791438623,
  "agent_seconds": 22,
  "artifacts": [
    {
      "reference": ".recur/blackjack-skill-workers/integration/report.json",
      "sha256": "460b608c10170c3958c2f86c2df2bb22bc8558a51f56835442f6f3566937086c",
      "result": {
        "schema": "blackjack-skill-integration-report-v1",
        "warp": "main.demo.blackjack-skill-replay",
        "status": "passed",
        "observed_at": "2026-10-08T05:50:23.612Z",
        "tests": [
          {
            "name": "optional-blackjack-web",
            "program": "C:/Users/marcn/.julia/juliaup/julia-1.12.7+0.x64.w64.mingw32/bin/julia.exe",
            "args": [
              "--startup-file=no",
              "-O0",
              "-C",
              "generic",
              "--project=C:/src/recur/demos/web-evidence-lab",
              "C:/src/recur/julia-tests/runtests.jl",
              "--demo",
              "blackjack-web"
            ],
            "exit_code": 0,
            "elapsed_ms": 22204,
            "log": "optional-blackjack-web.log"
          },
          {
            "name": "browser-node",
            "program": "C:/Users/marcn/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/bin/node.exe",
            "args": [
              "--test",
              "C:/src/recur/demos/blackjack-web/main.blackjack.rivals.test.cjs",
              "C:/src/recur/demos/blackjack-web/skill/main.blackjack.skill.browser.test.cjs"
            ],
            "exit_code": 0,
            "elapsed_ms": 118,
            "log": "browser-node.log"
          },
          {
            "name": "default-selection",
            "program": "C:/Users/marcn/.julia/juliaup/julia-1.12.7+0.x64.w64.mingw32/bin/julia.exe",
            "args": [
              "--startup-file=no",
              "C:/src/recur/julia-tests/runtests.jl",
              "--dry-run"
            ],
            "exit_code": 0,
            "elapsed_ms": 383,
            "log": "default-selection.log"
          },
          {
            "name": "lang-check",
            "program": "C:/Users/marcn/.cargo/bin/recur.exe",
            "args": [
              "lang",
              "check",
              "warps/blackjack-skill/main.blackjack.skill.flow.recur",
              "-d",
              "C:/src/recur",
              "--json"
            ],
            "exit_code": 0,
            "elapsed_ms": 53,
            "log": "lang-check.log"
          },
          {
            "name": "lang-plan",
            "program": "C:/Users/marcn/.cargo/bin/recur-lang.exe",
            "args": [
              "plan",
              "warps/blackjack-skill/main.blackjack.skill.flow.recur",
              "-d",
              "C:/src/recur",
              "--json"
            ],
            "exit_code": 0,
            "elapsed_ms": 56,
            "log": "lang-plan.log"
          }
        ],
        "lang_execution": "not-run",
        "browser_observation": "separate parent browser-smoke.json",
        "source_manifest_sha256": "17314d9c1bad172c106d84ee6dd33c472f77b1ac10811b388c7af3aa05f451ff",
        "acceptance": false
      }
    }
  ],
  "acceptance": "separate reviewed gate; worker result does not imply acceptance",
  "raw_execution_references": {
    "stdout": "\\\\?\\C:\\src\\recur\\.recur\\dispatch\\fnv1a64-a7b70ede52e7954d\\fnv1a64-08eb5f91d02be9cd.1.agent.stdout.txt",
    "stderr": "\\\\?\\C:\\src\\recur\\.recur\\dispatch\\fnv1a64-a7b70ede52e7954d\\fnv1a64-08eb5f91d02be9cd.1.agent.stderr.txt"
  }
}
```
