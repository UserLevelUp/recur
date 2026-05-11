RECUR IMPROVEMENT 26
Recur Watch and Recur-Trace: Versioning Potential, Lineage, and Operator Approval
=================================================================================
Date: May 3, 2026
Status: Proposal / future direction
Author: Captured from a 2026-05-03 design proposal
  (Joe Bishop / Skippy coordinator - private-routine governance pattern)

INTENT
------
Keep `recur` hierarchically pure while adding companion tooling that can notice
when an artifact has become important enough to deserve versioning,
traceability, approval state, or human/operator review.

`recur` should remain the tree. It reveals what exists through dot-path
hierarchy and eventness naming.

`recur-watch` and `recur-trace` would sit beside it as interpretation and
governance tools.

This proposal deliberately keeps responsibility separate from discovery:
the tools may detect, propose, preserve, compare, and summarize, but they must
not promote consequential artifacts to approved status without explicit
operator authorization.

SUMMARY
-------
Improvement 26 proposes three companion surfaces:

- `recur-watch` - artifact watcher and versioning-potential detector.
- `recur-version` / `recur version` - config-driven artifact versioning,
  manifests, and version policy.
- `recur-trace` - lineage, provenance, control, and approval-process tracer.

The governing split is:

- `recur` shows **what exists**.
- `recur trace` analyzes **technical origin and call graphs** inside the
  existing command family.
- `recur-watch` detects **whether an artifact deserves a version or approval
  lane**.
- `recur-version` preserves **current/version history and manifests according
  to project policy**.
- `recur-trace` explains **why this artifact state exists, who controls it,
  and what approvals or promotions led here**.

The recommendation is to build `recur-watch` first as a practical proposal
generator, build `recur-version` next to make approved version lanes cheap and
repeatable, then build `recur-trace` as the lineage and responsibility layer.

TOOL BOUNDARIES
---------------

### `recur`

Pure hierarchy and eventness visibility.

Primary questions:

- What exists?
- Where is it in the hierarchy?
- What state does its filename reveal?
- What children, siblings, and related files are present?

Example:

```powershell
recur tree care.subject.routine -d fixtures/private --ascii --count
```

### `recur trace`

Function or code-origin tracing inside the existing `recur` command family.

Primary questions:

- Where does this function originate?
- Who calls it?
- What does it call?
- What dependency or call-chain complexity exists?

This should remain close to technical origin and call graph analysis.

### `recur-watch`

Artifact watcher and versioning-potential detector.

Primary questions:

- Is this artifact changing enough to deserve a version lane?
- Is it operational, repeated, consequential, or approval-dependent?
- Does it need proposed/approved separation?
- Should a human or AI operator be asked to approve a new eventness structure?

Example:

```powershell
recur-watch scan care.subject.routine -d fixtures/private
```

Possible output:

```text
Subject: care.subject.routine
Watch finding: versioning-potential HIGH

Detected:
- current CSV artifact
- proposed/approved lifecycle
- repeated operational use
- incremental changes
- external approval or verification dependency

Suggested lanes:
- care.subject.routine.proposed.version
- care.subject.routine.approved.version
- care.subject.routine.approval.pending

Operator action:
Approve creation of version-eventness lane? yes/no
```

### `recur-trace`

Lineage, control, provenance, and approval-process tracing.

Primary questions:

- Who or what is in control of this artifact?
- Why does this state exist?
- What is the trace-id process?
- What approvals, denials, blocks, or promotions led here?
- Where did responsibility cross from agent suggestion to human/operator
  authorization?

Example:

```powershell
recur-trace care.subject.routine -d fixtures/private
```

Possible output:

```text
Trace: care.subject.routine
Trace-id: care.subject.routine.trace.2026-05-03.001
Artifact: care.subject.routine.proposed.current.csv
Format: CSV
Control state: proposed, human/operator approval required
Versioning potential: HIGH
Promotion threshold: proposed -> approved requires explicit operator confirmation

Lineage:
001 initial proposed routine created
002 proposed routine expanded with care rows
003 proposed routine revised after caregiver update
004 version lane recommended by watcher

Responsibility boundary:
Agent may organize, preserve, compare, summarize, and propose.
Human/operator remains responsible for approval, promotion, and real-world action.
```

NAME COLLISION AND SURFACE NOTE
-------------------------------
This repo contains the watcher capability, but the active subscription loop was
extracted from the core `recur` binary into the companion executable
`recur-watch`.

This proposal intentionally spells the companion as `recur-watch`, with a
hyphen, to mark a separate executable or companion layer.

The repaired command boundary is:

```text
recur watch   = pure watcher-state query; list/status/explain ACK/NAK records
recur-watch   = active watcher runner; stream/poll/write watcher eventness
```

If the team later chooses a plugin-style surface for the governance layer, the
equivalent shape could become:

```powershell
recur watch scan <subject> [-d root]
recur watch propose <subject> --versioning [-d root]
```

That decision is left open. The important boundary is behavioral, not spelling:
active event subscription remains the job of `recur-watch`; pure watcher-state
inspection belongs under `recur watch`; versioning-potential and approval-lane
proposal belong to the governance layer unless explicitly promoted later.

### `recur-version` / `recur version`

Artifact versioning and manifest management.

The preferred boundary is:

- `recur-version` is the operational companion that writes: save snapshots,
  choose the next version, update manifests, enforce configured policy, and
  optionally call `recur-git checkpoint`.
- `recur version` is the core read/query surface that keeps `recur` pure: show
  status, inspect policy/schema, query history, explain diffs, and report what
  the TOML-defined semantics mean.

If the team later decides to keep everything inside one binary, this same split
should still be preserved behaviorally: write operations remain explicitly
versioning-oriented, while `recur` continues to reveal and query rather than
silently mutate consequential artifacts.

Primary questions:

- What is the current artifact?
- What is the latest preserved version?
- What version identifier comes next?
- What changed, why, and under whose authority?
- Should this artifact be saved as a proposed version, approved version, or
  restart/discontinue event?
- Which versioning policy from `.recur/config.toml` applies?

Example:

```powershell
recur version status care.subject.routine -d fixtures/private
recur-version save care.subject.routine.proposed.current.csv --slug evening-items-added -d fixtures/private
recur version manifest care.subject.routine -d fixtures/private
recur version diff care.subject.routine --from b1 --to b2 -d fixtures/private
```

Possible output:

```text
Subject: care.subject.routine
Current artifact: care.subject.routine.proposed.current.csv
Format: CSV
Lifecycle: proposed
Latest version: b2
Next version: b3
Manifest: care.subject.routine.proposed.version.manifest.current.md
Policy: clinical-private

Version event required fields:
- slug
- reason
- operator
- checkpoint

Privacy rule:
Private root detected. Do not stage or commit artifact contents to public git.
```

`recur-version` is not a replacement for Git. Git versions repository state.
`recur-version` versions a named artifact inside its Recur eventness lifecycle:
current, proposed, approved, discontinued, restarted, superseded, or complete.
`recur version` should be able to query and explain those records without
turning core `recur` into an opinionated workflow engine.

VERSIONING-POTENTIAL CRITERIA
-----------------------------
An artifact should be proposed for version-eventness when several of these are
true:

- It is operational rather than purely descriptive.
- It is used repeatedly.
- It changes incrementally.
- It affects legal, medical, financial, safety, caregiving, or coordination
  decisions.
- It has proposed, approved, denied, blocked, or superseded states.
- It has external dependencies such as clinician, insurer, court, agency,
  vendor, or family approval.
- It benefits from rollback, comparison, provenance, or audit history.
- It has a structured format such as CSV, JSON, YAML, TOML, forms, or generated
  reports.

When criteria are met, the tool should propose a version-eventness lane instead
of silently creating one.

OPERATOR MODEL
--------------
The operator can be a human or a delegated AI/persona acting under human
authority.

The operator coordinates the process, but the system should preserve a clear
autonomy boundary:

- Agents may detect structure.
- Agents may prepare proposals.
- Agents may preserve versions.
- Agents may summarize risks and change arcs.
- Agents may not promote consequential artifacts to approved status without
  explicit authorization.
- The human remains the final authority for real-world action.

This lets the tool act as a semi-autonomous coordinator without obscuring
responsibility.

ROUTINE-LEVEL EVENTNESS
-----------------------
The larger routine schedule can be versioned separately from individual items
inside it.

Suggested structure:

```text
care.subject.routine.proposed.current.csv
care.subject.routine.proposed.version.manifest.current.md
care.subject.routine.proposed.version.2026-05-03.007.feeding-6-12-6-draft.csv

care.subject.routine.approved.current.csv
care.subject.routine.approved.version.manifest.current.md
care.subject.routine.approved.version.2026-05-10.001.provider-confirmed-baseline.csv
```

The `recur` view should make the approval shape visible:

```powershell
recur tree care.subject.routine -d fixtures/private --ascii --count
```

Conceptual output:

```text
care.subject.routine
|
+-- approved
|   +-- current.csv
|   +-- version
|       +-- manifest.current.md
|       +-- 2026-05-10.001.provider-confirmed-baseline.csv
|
+-- proposed
    +-- current.csv
    +-- notes.current.md
    +-- version
        +-- manifest.current.md
        +-- 2026-05-02.001.initial-csv.csv
        +-- 2026-05-02.002.dressing-rows.csv
        +-- 2026-05-02.003.turning-repositioning.csv
        +-- 2026-05-02.004.respiratory-care.csv
        +-- 2026-05-02.005.timing-spacing.csv
        +-- 2026-05-03.006.feeding-draft.csv
```

The important signal is that most work may remain proposed while only a small,
clean subset becomes approved.

CONFIG-DRIVEN VERSION EVENTNESS
-------------------------------
The non-obvious versioning feature is not the copied file. The copied file is
only the storage artifact.

The useful unit is a version event:

```text
what changed
why it changed
who or what authorized it
which current artifact it preserves
which lifecycle branch it belongs to
whether promotion or restart requires operator approval
whether the artifact is private and must stay out of public git
```

This is where a dedicated `recur-version` or `recur version` command becomes
useful. It lets versioning policy live in `.recur/config.toml` instead of in a
human's memory.

Example config sketch:

```toml
[versioning]
enabled = true
default_strategy = "letter-number"
preserve_current = true
require_manifest = true
checkpoint_after_save = true

[versioning.sequence.letter-number]
letters = "abcdefghijklmnopqrstuvwxyz"
numbers = "1..9"
after = "next-letter"

[versioning.patterns]
current_copy = "{subject}.current.{version}.{ext}"
event_record = "{subject}.version.{date}.{ordinal}.{slug}.{ext}"
manifest = "{subject}.version.manifest.current.md"

[versioning.roots.private]
dir = "fixtures/private/"
allow_git_commit = false
require_privacy_warning = true

[versioning.policy.synthetic_clinical]
require_slug = true
require_reason = true
require_operator = true
require_checkpoint = true
warn_on_old_version_view = true
require_confirmation_for = [
  "discontinue",
  "restart",
  "dose_change",
  "route_change",
  "timing_change",
  "approval_promotion"
]
```

With that policy, the tool can remember details humans forget:

- latest version is `b2`, so the next version is `b3`
- a manifest must be updated after saving
- private artifacts should not be staged into public Git
- a checkpoint should run after saving
- a high-risk transition needs a reason and operator
- edits against stale versions should warn before applying

Example stale-version warning:

```text
Warning: you are viewing care.subject.routine.proposed.current.a7.csv.
Current artifact: care.subject.routine.proposed.current.csv.
Latest preserved version: b2.

Apply this change to current, to the old version, or cancel?
```

Example ambiguity warning:

```text
High-risk version event detected: discontinue.
Request used an ambiguous referent: "this one".

Candidate rows:
1. medication-a
2. medication-b
3. combined medication-a / medication-c row

Choose the exact item before saving version b3.
```

The manifest is the readable control surface:

```text
Artifact: care.subject.routine.proposed.current.csv
Format: CSV
Lifecycle: proposed
Privacy: private fixture
Latest version: b2
Approval: not approved

Versions:
a7 - nurse-reported schedule pattern
a8 - safety review row
a9 - discontinued-state first pass
b1 - corrected discontinued item
b2 - added evening items
```

This makes `recur tree` and `recur files` reveal more than a pile of backup
copies. They reveal that a living artifact has a governed change story.

QUERYABLE VERSION HISTORY
-------------------------
To become more than backup discipline, version eventness should be queryable.

The revolutionary feature is a precise answer over the full artifact history:

```text
When did this item become discontinued?
What version first introduced this field?
Which version was approved by an operator?
What changed between proposed and approved?
Who or what supplied the reason for this transition?
Which versions touched route, amount, timing, status, or approval?
```

The query engine should remain generic. It should not know one domain's nouns
as built-in concepts. Instead, `.recur/config.toml` teaches it which columns,
states, transitions, and risk words matter for a project.

Example generic config sketch:

```toml
[versioning.query]
enabled = true
history_sources = ["manifest", "versions", "trace"]
answer_style = "precise"
include_evidence = true

[versioning.query.artifact.csv]
identity_columns = ["TaskOrItem", "Route"]
tracked_columns = ["Time", "DoseOrAmount", "Route", "Status", "Notes"]
state_column = "Status"
note_columns = ["Notes"]

[versioning.query.states]
proposed = ["DRAFT", "PROPOSED"]
reported = ["REPORTED", "VERBAL"]
approved = ["APPROVED", "CONFIRMED"]
blocked = ["BLOCKED", "HELD"]
discontinued = ["DISCONTINUED", "OUT CURRENTLY"]
restart_candidate = ["MAY RESTART", "RESTART CANDIDATE"]

[versioning.query.transitions]
high_risk = [
  "approved",
  "discontinued",
  "restart_candidate",
  "dose_change",
  "route_change",
  "timing_change"
]
```

The same mechanism can support care schedules, legal forms, invoices,
equipment checklists, research datasets, release manifests, or incident logs.
The tool sees artifact, version, field, state, transition, and evidence. The
project config supplies the vocabulary.

Possible command shape:

```powershell
recur version query care.subject.routine --question "when did item-a become discontinued" -d fixtures/private
recur version history care.subject.routine --item item-a -d fixtures/private
recur version explain care.subject.routine --from b1 --to b2 -d fixtures/private
```

Possible precise answer:

```text
Question: when did item-a become discontinued?

Answer:
item-a first appears with discontinued state in version b1.

Evidence:
- artifact: care.subject.routine.proposed.current.csv
- version: b1
- manifest entry: corrected discontinued item
- changed field: Status
- previous observed state: DRAFT UNVERIFIED
- new observed state: DISCONTINUED - OUT CURRENTLY
- lifecycle branch: proposed
- approval state: not approved
```

This turns the version lane into a local, file-based history index. The answer
is not an LLM guess over loose notes. It is a query over preserved versions,
manifests, trace records, and config-defined field semantics.

GENERIC ENGINE, DOMAIN CONFIGURATION
------------------------------------
The motivating schedule is only one use case. The feature should not become a
caregiving, medical, legal, finance, or release-management tool by default.

The reusable pattern is:

```text
recur core     = hierarchy, files, eventness, and generic artifact discovery
config.toml    = domain vocabulary and policy for this workspace
persona/agent  = reads config and applies the local semantics carefully
version layer  = preserves, queries, and explains artifact history
trace layer    = records control, authorization, and responsibility
```

Specificity should live in `.recur/config.toml` as declarative semantics. A
persona should learn the local versioning vocabulary from config before making
or proposing changes.

Example generic configuration model:

```toml
[artifact."care.subject.routine"]
kind = "structured-routine"
format = "csv"
risk_class = "synthetic-clinical-fixture"
privacy_root = "fixtures/private/"
persona = "care_schedule_expert"

[artifact."care.subject.routine".fields]
identity = ["TaskOrItem", "Route"]
tracked = ["Time", "DoseOrAmount", "Route", "Status", "Notes"]
state = "Status"
notes = ["Notes"]

[artifact."care.subject.routine".states]
proposed = ["DRAFT", "PROPOSED"]
reported = ["REPORTED", "VERBAL"]
approved = ["APPROVED", "CONFIRMED"]
blocked = ["BLOCKED", "HELD"]
discontinued = ["DISCONTINUED", "OUT CURRENTLY"]
restart_candidate = ["MAY RESTART", "RESTART CANDIDATE"]

[artifact."care.subject.routine".versioning]
strategy = "letter-number"
manifest_required = true
queryable = true
operator_required_for = ["approved", "discontinued", "restart_candidate"]
```

The same engine could be configured for another domain without changing code:

```toml
[artifact."project.release.manifest"]
kind = "release-manifest"
format = "toml"
risk_class = "release"
persona = "release_coordinator"

[artifact."project.release.manifest".states]
proposed = ["candidate", "rc"]
approved = ["signed", "approved"]
blocked = ["failed", "blocked"]
superseded = ["replaced", "obsolete"]
```

Persona exposure should be explicit. A revealable persona or agent should be
able to ask the version layer what local rules apply:

```powershell
recur version policy care.subject.routine -d fixtures/private
recur version schema care.subject.routine -d fixtures/private
```

Possible output:

```text
Subject: care.subject.routine
Artifact kind: structured-routine
Format: CSV
Risk class: synthetic-clinical-fixture
Privacy root: fixtures/private/
Persona: care_schedule_expert

Identity fields: TaskOrItem, Route
Tracked fields: Time, DoseOrAmount, Route, Status, Notes
State field: Status
High-risk transitions: approved, discontinued, restart_candidate
Operator required: yes for high-risk transitions
Queryable history: enabled
```

This keeps Recur generic while making personas more capable. The persona does
not need hardcoded knowledge of a domain. It reads the workspace's declared
artifact semantics, then uses `recur version` and `recur-trace` to preserve and
explain changes under those rules.

INDIVIDUAL-ITEM APPROVAL EVENTNESS
----------------------------------
Some items inside a larger schedule may need their own approval chain. The
model should support creating a child lane for an individual item when it
becomes clinically, financially, legally, or logistically interesting.

Suggested abstract structure:

```text
care.subject.item.<item>.proposed.current.md
care.subject.item.<item>.provider.approval.pending.md
care.subject.item.<item>.insurance.approval.pending.md
care.subject.item.<item>.approved.current.md
care.subject.item.<item>.denied.current.md
care.subject.item.<item>.version.manifest.current.md
```

Potential `recur` view:

```powershell
recur tree care.subject.item.<item> -d fixtures/private --ascii --count
```

Conceptual output:

```text
care.subject.item.example
|
+-- proposed.current.md
+-- provider.approval.pending.md
+-- insurance.approval.pending.md
+-- risk-review.current.md
+-- administration.blocked.md
+-- version
    +-- manifest.current.md
    +-- 2026-05-03.001.initial-request.md
    +-- 2026-05-03.002.provider-question-opened.md
    +-- 2026-05-03.003.insurance-review-started.md
```

This makes it visible that an item is not merely present in a schedule. It has
a lifecycle, approvals, blockers, responsible parties, and a traceable chain.

PRIVACY-PRESERVING TEST FIXTURES
--------------------------------
The motivating edge case may come from private, consequential care data, but
tests and public examples must not preserve personal information.

Fixture data should keep the structure and risk shape while replacing all real
identity, contact, and clinical specifics with synthetic values.

Use synthetic subjects:

```text
care.subject.routine.proposed.current.csv
care.subject.routine.proposed.version.manifest.current.md
care.subject.routine.proposed.version.2026-05-03.007.feeding-6-12-6-draft.csv

care.subject.item.medication-a.proposed.current.md
care.subject.item.medication-a.provider.approval.pending.md
care.subject.item.medication-a.version.manifest.current.md
```

Do not use:

- real person names
- real email addresses
- real phone numbers
- real addresses
- real account, provider, insurer, or agency identifiers
- medication names copied from a real private schedule
- dose/timing combinations copied from a real private schedule
- screenshots, CSV rows, or logs from private roots

Synthetic fixture replacements should be boring and obviously fake:

```text
person: subject-a
email: subject-a@example.invalid
phone: 555-0100
item: medication-a, medication-b, supply-a
organization: provider-a, insurer-a
route: route-a
amount: amount-a
schedule: time-a, time-b, time-c
```

The fixture still needs to exercise the real feature pressure:

- a structured CSV artifact
- repeated incremental edits
- proposed and approved lifecycle branches
- version history with a manifest
- ambiguous referent correction, such as "this one" matching multiple rows
- discontinued / restart-candidate state
- operator confirmation before promotion
- private-root handling that prevents accidental public output

Example anonymized versioning-potential output:

```text
Subject: care.subject.routine
Artifact: care.subject.routine.proposed.current.csv
Format: CSV
Risk class: synthetic-clinical-fixture
Versioning potential: HIGH
Privacy class: anonymized fixture

Findings:
- structured CSV artifact detected
- proposed/approved lifecycle detected
- 7 version records detected
- ambiguous medication-state edit fixture detected
- operator approval required before proposed -> approved promotion

Suggested lanes:
- care.subject.routine.proposed.version
- care.subject.routine.approved.version
- care.subject.routine.approval.pending
- care.subject.item.<item>.approval.<state>
```

The point of the fixture is not realism. The point is to prove that Recur can
notice when a changing artifact has become accountable, versioned, and
approval-bound without leaking the private facts that inspired the pattern.

TEST TODO: CRITERIA COVERAGE
----------------------------
Improvement 26 should include tests that exercise every versioning-potential,
version-policy, query-history, privacy, and operator-boundary criterion. These
tests should use only synthetic `care.subject.*` fixtures.

Required test areas:

- Versioning-potential detection
  - structured artifact detected, such as CSV or JSON
  - repeated operational artifact detected
  - incremental version history detected
  - proposed / approved lifecycle detected
  - external approval dependency detected
  - rollback, comparison, or audit-history benefit detected

- Version-event policy
  - next version selected correctly for configured strategy
  - manifest required when configured
  - reason, slug, operator, and checkpoint required when configured
  - private-root artifact refuses public git staging when configured
  - stale-version warning appears when editing an old version

- Queryable history
  - query finds when an item first entered a state
  - query identifies which version changed a tracked field
  - query distinguishes proposed state from approved state
  - query cites manifest/version/trace evidence
  - query semantics come from `.recur/config.toml`, not hardcoded domain words

- Generic domain configuration
  - artifact identity, tracked fields, and state words load from config
  - two different fixture domains use the same generic version engine
  - persona exposure command returns the configured policy and schema
  - missing required config produces an actionable error
  - hardcoded domain vocabulary is not required for tests to pass

- Ambiguity and high-risk transitions
  - ambiguous referent such as "this one" produces candidate rows
  - discontinue transition requires confirmation
  - restart transition requires confirmation
  - route, amount, timing, or status change can be configured as high risk
  - proposed -> approved promotion requires operator authorization

- Individual-item approval eventness
  - item-level proposed lane detected
  - provider / organization approval pending lane detected
  - denied / blocked / approved states remain queryable
  - item-level version manifest links back to larger routine artifact

- Privacy-preserving fixtures
  - no real names, contacts, or private-root records appear in fixtures
  - synthetic item names are used, such as `medication-a` and `supply-a`
  - synthetic contact values use reserved/example forms
  - test output does not leak private roots or private artifact contents

- Command behavior
  - `recur-watch scan` reports versioning potential without modifying files
  - `recur-watch propose` creates a proposal rather than applying changes
  - `recur version save` preserves current and updates manifest
  - `recur version query` returns precise evidence-backed answers
  - `recur-trace lineage` summarizes control, authorization, and responsibility

The tests should prove the generic engine, not one domain. Domain specificity
belongs in fixture `.recur/config.toml` values for identity columns, tracked
columns, state words, transition rules, risk class, and privacy behavior.

TRACE-ID PROCESS
----------------
A trace-id should name the decision lineage rather than merely the file.

Suggested pattern:

```text
<subject>.trace.<yyyy-mm-dd>.<nnn>.<slug>
```

Example:

```text
care.subject.routine.trace.2026-05-03.001.versioning-potential
```

Trace records should capture:

- Subject
- Artifact path
- Format
- Current lifecycle state
- Versioning-potential score
- Criteria met
- Proposed eventness changes
- Operator decision
- Promotion threshold
- Responsibility boundary

PROPOSED COMMANDS
-----------------

```powershell
recur-watch scan <subject> [-d root]
recur-watch propose <subject> --versioning [-d root]
recur-watch apply <subject> --proposal <id> [-d root]

recur-version next <artifact> [-d root]
recur-version save <artifact> --slug <slug> [-d root]
recur-version manifest update <subject> [-d root]
recur-version promote <subject>.proposed --to approved [-d root]

recur version status <subject> [-d root]
recur version manifest <subject> [-d root]
recur version diff <subject> --from <version> --to <version> [-d root]
recur version policy <subject> [-d root]
recur version schema <subject> [-d root]
recur version query <subject> --question <question> [-d root]
recur version history <subject> --item <item> [-d root]
recur version explain <subject> --from <version> --to <version> [-d root]

recur-trace <subject> [-d root]
recur-trace open <trace-id> [-d root]
recur-trace lineage <subject> [-d root]
recur-trace responsibility <subject> [-d root]
```

NON-GOALS
---------
- Do not make `recur` itself opinionated.
- Do not require a database.
- Do not promote proposed artifacts to approved automatically.
- Do not encode medical, legal, or financial decisions as tool authority.
- Do not replace git; use `recur-git` for checkpointing and repository-aware
  preservation.

OPEN QUESTIONS FOR THE RECUR TEAM
---------------------------------
- Should `recur-watch` be a separate executable or a `recur` plugin?
- Should versioning live in core as `recur version`, or as a companion
  executable named `recur-version`?
- Should `.recur/config.toml` define version strategies, privacy roots,
  required manifest fields, and confirmation-required transition kinds?
- Should `.recur/config.toml` also define query semantics for structured
  artifacts, such as identity columns, tracked columns, state values, and
  high-risk transitions?
- Should personas consume artifact semantics through `recur version policy`,
  `recur version schema`, direct TOML reads, or all three?
- Should a persona be allowed to modify artifact semantics, or only read them
  unless an operator approves the config change?
- Should version manifests be Markdown-first, JSON-first, or dual-output?
- Should query answers cite manifest entries, trace records, version files, or
  all of them?
- Should `recur version save` automatically call `recur-git checkpoint` when
  configured?
- Should `recur-trace` write trace files, or only read existing trace/eventness
  records?
- Should versioning-potential be rule-based, LLM-assisted, or both?
- Should proposal files have a standard suffix such as `.proposal.current.md`?
- Should operator approval be recorded as a file rename, a manifest entry, or
  both?
- How should private roots and public roots prevent accidental disclosure during
  trace output?

RECOMMENDATION
--------------
Build `recur-watch` first as a practical proposal generator.

Then build `recur-version` / `recur version` so approved version lanes,
manifests, privacy policy, and next-version selection become repeatable instead
of memory-based. Include queryable history early enough that versions can answer
precise questions over full artifact history, with project-specific semantics
defined in `.recur/config.toml`.

Then build `recur-trace` as the lineage and responsibility layer.

Keep `recur` pure: the tree remains the truth surface, while companion tools
notice when the tree is asking for a new branch.
