# slice-5: trait-reveal-references
Status: complete

Contract: contract:main.command.prompt.discovery.slice-5:v2.
Acceptance gate: trait-reveal-references.

Trait explanation lists effective prompt metadata. Reveal resolves prompt.ids to available, missing or unregistered references while retaining raw fields and existing skill pointers. Neither reference surface embeds prompt bodies.

Observed: prompt discovery suite 376 passed, 0 failed; Cargo 185 passed,
0 failed, 7 ignored doc tests. Full regression closeout belongs to slice-final.
Evidence: main.command.prompt.discovery.verification.md.
