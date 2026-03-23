RECUR IMPROVEMENT 19
Enterprise Lane Productization for Human + AI Teams
===================================================
Date: March 21, 2026
Status: Proposal / future direction
Author: Reframed from real-world eventness usage on the nate.* project


INTENT
------
This document is about a possible future product direction for recur.

It is NOT claiming that recur lacks lanes today.

Recur already supports a practical lane system through:
- hierarchical file names
- config-driven directory/separator lanes in `.recur/config.toml`
- eventness suffixes such as `.todo`, `.current`, `.reference`,
  `.trigger.event`, and `.complete`
- cross-lane discovery through `recur files`, `recur tree`, `recur stats`,
  and related commands

That current model is already useful. It works in this repo, and it is
enough for many single-repo and small-to-medium team workflows.

Improvement 19 asks a different question:

If recur's existing eventness conventions keep proving useful, which parts
should become first-class product features so the same model can scale to
larger organizations, multi-repo programs, and mixed human + AI teams?


SUMMARY
-------
Current recur:
- already has lanes as a convention
- already has visible workflow state in the tree
- already supports query-first coordination
- already encourages better git hygiene by keeping work scoped and legible

Improvement 19 proposes:
- formal lane primitives
- optional enforcement around state transitions
- optional owner and review metadata
- optional git/worktree/reporting integrations
- optional federation across repos

The goal is not to replace the current model.
The goal is to preserve the current model while making it easier to adopt,
query, and govern at larger scale.


WHAT EXISTS TODAY
-----------------
Recur today is already more than a raw search tool.

In practice, teams can already do all of the following:

1. Represent work as hierarchical files
   Example:
   `main.command.trace-id.test.todo.current.md`

2. Represent workflow state by suffix
   Examples:
   `*.todo.md`
   `*.current.md`
   `*.reference.md`
   `*.trigger.event.md`
   `*.complete.md`

3. Mirror work across lanes
   Examples:
   - source lane in `src/`
   - docs/eventness lane in `docs/`
   - verification lane in `julia-tests/`

4. Discover status without opening a project board
   Examples:
   `recur files "**.current" -d docs/`
   `recur files "**.complete" -d docs/`
   `recur tree "main" --sep . --sep _ --show-sep`

5. Onboard humans or AI quickly
   A tree view plus an expert/playbook file is often enough to resume work.

6. Use lanes to keep git work smaller and clearer
   A lane qualifies work into a manageable chunk with a clearer boundary for
   a focused branch, worktree, review, or handoff. That tends to produce
   smaller, more impactful merges and makes normal git usage more deliberate.

This is the baseline. Improvement 19 should be understood as a
productization layer on top of this baseline, not as a replacement for it.


WHY THIS EXISTS
---------------
The nate.* project showed that eventness scales surprisingly well when the
team is disciplined:

1. Per-objective lanes scale better than one giant tracker file.
2. The tree exposes status at a glance.
3. Reports can branch by audience using the same hierarchy.
4. Expand/collapse is a natural workflow.
5. AI agents can onboard fast from tree + expert context.

That evidence is strong enough to justify a proposal.
It is not, by itself, a reason to build the full enterprise stack now.


THE QUESTION IMPROVEMENT 19 IS TRYING TO ANSWER
-----------------------------------------------
What should recur add if we want the current lane/eventness model to work
cleanly for:

- larger teams
- multiple roles
- multiple AI agents
- multiple repositories
- stricter workflow governance

In other words:

Current lanes are real.
Improvement 19 is about making them easier to standardize, automate, and
query at enterprise scale.


NON-GOALS
---------
Improvement 19 is not trying to:

1. Replace the current convention-based workflow for this repo.
2. Make recur useless unless enterprise features are added.
3. Turn recur into a generic PM suite by default.
4. Force state machines or ownership models on exploratory projects.
5. Deprioritize recur's core strengths in search, discovery, and analysis.


WHAT "ENTERPRISE" MEANS HERE
----------------------------
In this document, "enterprise" does not just mean "large company."
It means coordination pressure that exceeds what manual naming discipline
comfortably handles.

Examples:
- dozens of simultaneous lanes
- explicit ownership and review routing
- multiple specialist teams
- AI agents claiming and handing off work
- cross-repo dependencies
- reporting for different audiences

If a project does not feel these pressures, the existing recur model is
probably enough.


THE PROPOSAL
============

1. FIRST-CLASS LANE PRIMITIVES
------------------------------
Today a lane is a convention:

  project.steps.auth-rewrite.todo.md

Proposal:

  recur lane create auth-rewrite --owner @sarah --state todo
  recur lane move auth-rewrite --state current
  recur lane list --state current
  recur lane list --owner @sarah

Important clarification:
This does not replace files.
It keeps files as the source of truth and gives recur a first-class
understanding of the convention.

Potential config shape:

  [lanes]
  pattern = "{project}.{area}.{objective}.{state}"
  states = ["todo", "current", "blocked", "review", "complete"]
  state_position = "last"

Value:
- less manual ceremony
- fewer naming mistakes
- easier querying by state or owner
- clearer onboarding for new humans and AI agents


2. ROLE- AND AREA-AWARE ORGANIZATION
------------------------------------
Large teams usually need more than one flat work queue.

Example hierarchy:

  project.frontend.auth-ui.current.md
  project.backend.auth-service.current.md
  project.ml.fraud-model.training.current.md
  project.ai.code-review.auth-pr-42.current.md
  project.infra.deploy-pipeline.todo.md

Potential value:
- role-aware views
- cleaner ownership boundaries
- easier reporting by functional area

This is not required for recur to work.
It is only useful when teams need those slices.


3. OPTIONAL STATE MACHINES
--------------------------
Today state changes are just file renames.
That is often enough.

Proposal:
allow projects to opt into transition rules.

Example:

  [lanes.transitions]
  todo = ["current", "blocked", "cancelled"]
  current = ["review", "blocked", "complete"]
  blocked = ["current", "cancelled"]
  review = ["current", "complete"]
  complete = []

Potential value:
- review gates
- fewer accidental skips
- clearer workflow expectations

Important clarification:
This should be optional.
Strict governance is useful for some teams and harmful for others.


4. HUMAN + AI COORDINATION
--------------------------
This is the strongest long-term reason to keep Improvement 19 around.

If humans and AI agents are both active contributors, a shared lane model
could help with:

- claim / handoff
- review routing
- temporary sub-lane expansion
- visibility into who owns what

Example:

  recur lane claim auth-tests --owner @ai-test-agent
  recur lane move auth-tests --state review --reviewer @sarah

Important clarification:
This is a coordination layer, not an intelligence layer.
Recur would track state and routing. It would not replace the agent itself.


5. OPTIONAL GIT / WORKTREE / PR INTEGRATION
-------------------------------------------
A lane can remain just a file.
Some teams may want stronger coupling to git.

Important clarification:
eventness lanes already help git today even without automation.
A lane qualifies work into a manageable chunk with a natural boundary for a
focused branch, worktree, review, or handoff. That usually means smaller,
more impactful merges, less tangled conflicts, and a clearer review history.

Possible future ideas:
- create a branch when a lane becomes current
- create a PR when a lane enters review
- attach worktree automation for AI agents
- show likely merge order for dependent lanes

This is potentially useful, but it is far from core recur value today.
The immediate value is lane discipline itself: if each lane stays tight, then
each merge can stay quicker, more efficient, and less painful. Automation
would only deepen that relationship.


6. OPTIONAL CROSS-REPO FEDERATION
---------------------------------
Single-repo workflows are already well served by current eventness lanes.

Federation only becomes relevant when:
- one initiative spans several repos
- one repo is blocked on another
- leadership wants one cross-repo view

Possible future idea:

  recur tree --federated

This is a real enterprise need, but also a high-complexity feature.
It should stay clearly in the future bucket until the local lane model is
proven further.


7. OPTIONAL REPORTING LAYERS
----------------------------
The hierarchy already behaves like a dashboard.
Some teams may eventually want recur-generated reports in text, JSON, or HTML.

That would be useful for:
- leadership summaries
- team-specific slices
- historical snapshots

Again: helpful, but not necessary for recur to provide value today.


8. EXPERT FILE GENERATION
-------------------------
This is one of the more promising medium-term ideas.

Projects already benefit from expert/playbook files.
A future recur could generate a project expert file from config and lane
state so humans and AI agents can onboard consistently.

This fits recur's philosophy better than heavyweight orchestration because
it turns structure into readable guidance without hiding the filesystem.


WHAT IS ACTUALLY NEW HERE
-------------------------
The important distinction is:

Existing recur already has:
- lane-like structure
- eventness state
- cross-lane discovery
- human-readable status in the tree

Improvement 19 would add:
- first-class commands for lane lifecycle
- formal metadata and querying
- optional policy enforcement
- optional automation and federation

That is the real intent of this improvement.


WHAT SHOULD HAPPEN NOW VS LATER
-------------------------------
Improvement 19 is too large to treat as one immediate roadmap item.

If pursued at all, it should be split into three buckets:

Near-term / realistic:
- clarify lane terminology in docs
- formalize what a lane is
- add lane linting / templates
- consider lightweight owner metadata
- consider first-class listing/filtering over existing conventions

Mid-term / only if demand appears:
- optional state transitions
- optional expert-file generation
- lightweight claim / handoff commands

Far-term / only with strong evidence:
- git and worktree orchestration
- PR automation
- cross-repo federation
- dashboards and full reporting stack


DECISION CRITERIA
-----------------
Improvement 19 becomes worth building when the current model shows repeated,
costly friction in real work.

Signals that would justify productization:

1. Teams repeatedly reinvent the same lane conventions.
2. Humans or AI agents frequently misname or misplace lane files.
3. Ownership is hard to query from the tree alone.
4. Review routing becomes a coordination bottleneck.
5. Cross-repo dependency visibility becomes painful.

If those signals are absent, the current convention-based system is likely
the right level of abstraction.


RISKS
-----
1. Overbuilding
   Risk:
   recur drifts from "queryable hierarchy" into a heavyweight PM platform.

   Mitigation:
   keep files as the source of truth; make enterprise features optional.

2. Governance creep
   Risk:
   strict workflows make recur worse for exploratory or solo work.

   Mitigation:
   default to conventions first, enforcement second.

3. Complexity before evidence
   Risk:
   federation, git automation, and dashboards arrive before demand is clear.

   Mitigation:
   treat them as later phases, not implied commitments.

4. AI-specific assumptions
   Risk:
   the design overfits one style of agent workflow.

   Mitigation:
   keep the lane model generic; treat AI as one kind of owner, not the center.


RECOMMENDED READING OF THIS DOCUMENT
------------------------------------
Read Improvement 19 as:

"If recur's existing lane/eventness model keeps working, here is how it
might eventually be formalized for larger organizations."

Do NOT read it as:

"Current recur lanes are insufficient or invalid."


CORE PRINCIPLE
--------------
The filesystem is already hierarchical.
File names already encode meaning.
Git already preserves history.

Recur's current power comes from making that visible and queryable.

Improvement 19 only makes sense if it preserves that simplicity:
- files stay primary
- the tree stays legible
- conventions remain usable even without automation
- first-class features reduce friction instead of replacing the model


BOTTOM LINE
-----------
Improvement 19 is useful as a clear future-direction document.
It is not a statement that recur is missing core lane capability today.

Current recur already has real lanes.
Improvement 19 is about whether, when, and how to formalize them for
enterprise-scale coordination while also making lane-to-git workflows
clearer and easier to operate.


