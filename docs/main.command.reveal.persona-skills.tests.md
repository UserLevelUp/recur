# Reveal v2 test relevance audit

Reviewed 2026-09-09 on `recur-lang`, starting at `4e99976`.
This records baseline observations, not slice-0 acceptance.

Implementation follow-up: the suite is now integrated in the normal runner.
The remaining cases listed below describe the baseline review, not current gaps.
See `main.command.reveal.persona-skills.verification.md` for final results and
`main.command.reveal.persona-skills.contract.md` for the frozen interface.

## Coverage that exists

`julia-tests/main.command.reveal.persona-skills.test.jl` now exercises independent
agent, persona and skill discovery with project-local fixtures. The same short
name exists across all three types. Cases include type-filtered selection,
same-type ambiguity, exact identity, wrong-type selection, a standalone skill
with a non-SKILL.md body pointer, legacy untyped metadata, an explicit nested root
and unchanged file content. The body sentinel test establishes that body content
does not appear in discovery output; it does not instrument filesystem reads.

`RECUR_BIN` selects the actual core candidate. `RECUR_REVEAL_BIN` optionally
selects the companion, otherwise it is sought beside that core binary. This
avoids accidentally testing an older same-version build in another profile.

Existing persona defaults and missing-skill packet assertions are retained as
v1 compatibility requirements, not mistaken for complete v2 coverage. The
default-config checks now also require agent and skill tables. No assertion
has been converted to `@test_broken`. The suite remains standalone and red.

## Observed results

Executable: `target/lang-final/choco-smoke/tools/recur.exe` (the previously
verified 0.2.8 Windows candidate). Julia 1.12.7, with
`--startup-file=no --compiled-modules=no --compile=min -O0` and
`--project=demos/web-evidence-lab`.

| Suite | Result |
| --- | --- |
| New v2 independent discovery cases | 34 passed |
| Defaults/companion standalone cases | 1 passed, 4 failed, 0 errors |
| Existing reveal | 31 passed |
| Existing artifact types | 123 passed |
| Existing init | 33 passed |

The four expected failures are absent `reveal.agents`, `reveal.skills`,
`reveal.personas` defaults and the absent `recur-reveal` executable. Conditional
packet assertions cannot run until that executable exists; they are not passing
evidence. Legacy suites were included in separate Julia modules in a single
process to keep their shared setup globals isolated. No full regression or Cargo
run was needed for this test/documentation-only review.

## Required before v2 acceptance

| Area | Missing coverage / decision |
| --- | --- |
| Association identity | Freeze how config records bind to typed capsules and how explicit references identify targets; test cross-type name collisions without implicit lookup |
| Association resolution | Agent-to-persona, agent-to-skill and persona-to-skill; absent/ambiguous/wrong-type targets, conflicting declarations and provenance |
| Defaults and retrofit | Fresh versus existing project, explicit empty associations, custom profiles, comments, repeatability and partial-write recovery |
| Packet selection | Freeze agent/persona disambiguation and the v2 CLI/schema before asserting guessed fields |
| Packet contents | Stable ordering, shared-skill deduplication with all association sources retained, missing required references, malformed bodies and source fingerprints |
| Read bounds | Outside-root paths and symlinks for body resolution, collection limits and explicit omissions; no implicit global or remote lookup |
| Execution boundary | No activation, installation or evaluation of referenced instructions; query and packet operations leave project assets unchanged |

The existing classifier's root/type tests do not automatically establish the
future body resolver's safety. Likewise, presence of a config table or companion
binary establishes no resolver behavior. Extend executable red coverage as each
interface is frozen, then implement against it. Keep the current Warp at 0/6.

defines: recur.reveal.persona-skills.tests v2 relevance audit and observed baseline
consumes: recur.reveal.persona-skills independent artifact discovery contract
