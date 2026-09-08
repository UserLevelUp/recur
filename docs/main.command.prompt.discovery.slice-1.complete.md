# slice-1: registry-resolution
Status: complete

Contract: contract:main.command.prompt.discovery.slice-1:v2.
Acceptance gate: registry-resolution.

Shared app catalogs, typed project overrides, opt-out, exact source fingerprints and containment are implemented. Registry unit tests cover duplicate provider IDs, malformed definitions, source size/UTF-8 and Windows junction escapes.

Observed: prompt discovery suite 376 passed, 0 failed; Cargo 185 passed,
0 failed, 7 ignored doc tests. Full regression closeout belongs to slice-final.
Evidence: main.command.prompt.discovery.verification.md.
