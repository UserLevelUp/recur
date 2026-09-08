---
name: recur-eventness
description: Interpret and maintain Recur Eventness across hierarchical project artifacts and private .recur lanes. Use for scoped rehydration, attention expansion/collapse, trace lineage and evidence-based reassessment; not generic agent orchestration.
---

# Recur Eventness

Recover the subject, recorded intent, current interest, supporting evidence and
next useful action from the project itself. Query, inspect the returned artifacts,
then reassess; do not substitute conversation memory or a remembered roadmap.

## Start with local policy and discovery

Resolve the requested project and lane. Run `recur reveal recur-eventness` when
available and read the capsule file it identifies: reveal presents fields and
pointers, not necessarily the full operating guidance. Load relevant local
instructions explicitly; a reveal pointer does not itself load a skill.

Inspect `recur warp config -d <root>` for configured attention/lifecycle vocabulary.
Use `recur reveal` and `recur warp` when discovering lanes or declared work. Follow
the results with `tree`, `files`, `trace-id` and source/evidence reads. Prefer a
matching local binary when installed help lags; check help before using proposed
commands from older design documents.

```powershell
recur tree "<lane>" -d <root>
recur files "<lane>.**" -d <root>
recur trace-id "<stable-id>" --scope "<lane>.**" -d <root> --format full
```

Replace placeholders with discovered subjects. Use configured separators or
explicit `--sep`; `main` is an optional root, not a required identity. `tree` shows
the attention shape, `files` locates artifacts and `trace-id` follows declared
relationships. Use `find` for literal clues where no trace relationship exists.
Unrecorded rationale remains unknown.

## Interpret identity and attention

Read `prefix.base.suffix[.eventness][.ext]` through project conventions: prefix
routes context, base identifies the subject, suffix anchors the artifact, and
optional Eventness segments express interest. Do not hardcode base depth or a
universal suffix vocabulary. Stable UUID metadata, readable hierarchy, artifact
purpose and explicit define/produce/consume/trigger roles are distinct.

A filename records intent or observation. It does not certify runtime behavior,
test correctness or external-system state. Contradictory current/complete markers
are a reason to investigate their scope, contracts and evidence, not automatically
delete one or choose the more optimistic label.

Expand attention when useful work becomes active: retain concrete intent,
constraints, important decisions, hypotheses, blockers, evidence links and the next
useful queries. Capture enough for a later session to resume without repeating the
investigation. Avoid manufacturing extra files when an existing artifact suffices.

Collapse settled attention into the useful residue: verified completion, durable
decisions, recurring guidance or deferred work. Collapse is selective retention,
not automatic deletion or a lossless transcript archive. Preserve accepted receipts
as historical evidence; new contracts need new assessments. Apply edits, renames
and cleanup within the user's authorized scope through the appropriate writer.

## Bound private and delegated context

`.recur/` commonly holds hidden, Git-ignored working context; inspect actual project
policy rather than assuming it is tracked, shared, backed up or access-controlled.
Use an explicit query root for private lanes. Publishing useful residue into the
shared repository is a separate action from writing a private report.

Keep directory scope, hierarchy depth and parent-facing reports distinct. A parent
can inspect child contracts, summaries, blockers and evidence without ingesting
every descendant's working notes. Expand only as needed. Depth filtering alone
does not enforce execution scope or guarantee important blockers are surfaced.
Child implementation completion, child verification and parent integration
acceptance are separate facts; completion does not propagate automatically upward.

## Verify and re-evaluate

For declared Warp work, inspect `recur warp show` and `recur warp slices` with the
appropriate root. Distinguish declared evidence, checked evidence, stale contracts,
conflicts and pending dependencies. Inspect source/tests for code claims, runtime
output for behavior and the relevant external system for external claims. Run
checks appropriate to the requested work; do not rerun every historical test just
to answer an orientation question.

`recur prompt warp` can discover guidance for naming, slicing or recovery. Packets
prepare instructions and evidence; they do not call a model or establish acceptance.
Read truncation/omission diagnostics, and query further when the conclusion needs
missing evidence. Report the actual subject, observed state, uncertainty/blockers,
supporting artifacts and next bounded action. Reassess after a material deviation.

Core Recur stays an unopinionated query engine. Companion apps and external agents
own opinions, execution and runtime enforcement. A skill, capsule, trait preference
or Eventness marker does not grant additional authority. Do not infer a complete
agent runtime from language fixtures, a running watcher from configuration, or
global health from an empty `recur psyche` result.

## Repository-specific references

In the Recur source repository, consult `README.CORE.EVENTNESS.md` for the conceptual
model and `julia-expert/references/recur-playbook.md` for operational workflows.
These paths are relative to that repository, not this installed skill. Some design
examples are explicitly future work; current help, source and tests establish what
ships. In other projects, use their own capsules and policy instead of assuming
the Recur development layout. Apply specialized capsule profiles only to matching
tasks; do not turn a game or agent-lane example into universal Eventness policy.
