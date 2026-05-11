# main.improvement.26.test.todo.criteria-coverage

Status: `todo`
Date: 2026-05-06

## Purpose

Track the synthetic test work needed to prove Improvement 26 without relying on
private or domain-specific data.

## Canonical Proposal

- `README.CORE.IMPROVEMENT26.md`

## Test Goal

Create fixture-driven tests that prove version-eventness can stay generic while
using `.recur/config.toml` to supply domain-specific artifact semantics.

## Required Coverage

- Detect versioning potential for operational, repeated, consequential
  artifacts.
- Load artifact identity fields, tracked fields, state words, transition rules,
  risk class, and privacy policy from `.recur/config.toml`.
- Preserve current artifacts into version lanes without deleting older versions.
- Update and query a version manifest.
- Answer precise history questions with evidence from versions, manifests, and
  trace records.
- Warn when a user or agent is viewing an old version while a newer current
  artifact exists.
- Require confirmation for ambiguous referents such as `this one` when a
  high-risk transition is requested.
- Require operator authorization before promoting proposed artifacts to
  approved artifacts.
- Keep private-root fixture behavior from leaking into public output.
- Exercise at least two synthetic domains so the tests prove the generic engine
  rather than one hardcoded topic.

## Suggested Fixture Shape

```text
fixtures/improvement26/.recur/config.toml
fixtures/improvement26/care.subject.routine.proposed.current.csv
fixtures/improvement26/care.subject.routine.proposed.version.manifest.current.md
fixtures/improvement26/care.subject.routine.proposed.version.a1.csv
fixtures/improvement26/care.subject.routine.proposed.version.a2.csv
fixtures/improvement26/project.release.manifest.proposed.current.toml
fixtures/improvement26/project.release.manifest.proposed.version.manifest.current.md
```

## Candidate Commands Under Test

```powershell
recur-watch scan care.subject.routine -d fixtures/improvement26
recur version status care.subject.routine -d fixtures/improvement26
recur version policy care.subject.routine -d fixtures/improvement26
recur version schema care.subject.routine -d fixtures/improvement26
recur version query care.subject.routine --question "when did item-a become discontinued" -d fixtures/improvement26
recur-version save care.subject.routine.proposed.current.csv --slug item-a-discontinued -d fixtures/improvement26
recur-trace lineage care.subject.routine -d fixtures/improvement26
```

## Privacy Rule

Fixtures must use only synthetic values such as `care.subject`, `item-a`,
`medication-a`, `provider-a`, `amount-a`, and `time-a`. Do not copy real names,
contacts, medication names, dose/timing details, private CSV rows, screenshots,
or private-root logs.
