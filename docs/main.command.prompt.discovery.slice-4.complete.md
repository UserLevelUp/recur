# slice-4: shared-command-adapters
Status: complete

Contract: contract:main.command.prompt.discovery.slice-4:v2.
Acceptance gate: shared-command-adapters.

Core, trait and recur-warp llm prompt use one registry and packet builder. Tests verify app/project/custom capability behavior and JSON parity, including configured and explicit separators.

Observed: prompt discovery suite 376 passed, 0 failed; Cargo 185 passed,
0 failed, 7 ignored doc tests. Full regression closeout belongs to slice-final.
Evidence: main.command.prompt.discovery.verification.md.
