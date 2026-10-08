artifact.type = lane
publish: main.demo.blackjack-skill-replay.tests.attempt.1.result durable worker result
consumer: main.demo.blackjack-skill-replay.coordination scoped rehydration

Intelligence and evidence live in this Eventness receipt. Its notification carries only the trace ID. Producer state and reviewed acceptance remain distinct.

```json
{
  "warp": "main.demo.blackjack-skill-replay",
  "slice": "tests",
  "attempt": 1,
  "trace_id": "main.demo.blackjack-skill-replay.tests.attempt.1.result",
  "state": "produced",
  "host": "blackjack-skill-red-check",
  "reasoning": "host-default",
  "claimed_at_unix": 1791437418,
  "finished_at_unix": 1791437422,
  "agent_seconds": 3,
  "artifacts": [
    {
      "reference": ".recur/blackjack-skill-workers/tests/red-result.json",
      "sha256": "4bfe8f9329dd7c19cb9f3fc8b3d06f5996d7363f5a9f8ca5ed6807fa6a8cd7d5",
      "result": {
        "status": "observed-red",
        "method": "prepared unimplemented API stubs, actual execution before implementation",
        "results": [
          {
            "slice": "strategy",
            "exit": 1,
            "test_sha256": "958978859b82347b21a74424a4d44f48d8bee2c74b44da7b27ff63f1b165eaca",
            "source_sha256": "eed6f5b7e464ca53abeb5a89287a6513f5ed2e66c77c9e3f8db7c56a6340e6bb"
          },
          {
            "slice": "replay",
            "exit": 1,
            "test_sha256": "8f4bdeb41b24cd4e1af01b147486d41305c08fdbeec961e2b650e93a1d646396",
            "source_sha256": "eab3f0721925988d94eaeddf263c139991783caf27907e9dfd8d7a1e8c6f8ace"
          },
          {
            "slice": "statistics",
            "exit": 1,
            "test_sha256": "a908896456c1d7318798d0eff3e2b69aab988083362dc5c725ef04c45d06f324",
            "source_sha256": "45bc9bf060379f28ea1864e7b0c2e3e48418c6670806f8e750e071c6801c55f7"
          }
        ]
      }
    }
  ],
  "acceptance": "separate reviewed gate; worker result does not imply acceptance",
  "raw_execution_references": {
    "stdout": "\\\\?\\C:\\src\\recur\\.recur\\dispatch\\fnv1a64-a7b70ede52e7954d\\fnv1a64-1f85be484dfda322.1.agent.stdout.txt",
    "stderr": "\\\\?\\C:\\src\\recur\\.recur\\dispatch\\fnv1a64-a7b70ede52e7954d\\fnv1a64-1f85be484dfda322.1.agent.stderr.txt"
  }
}
```
