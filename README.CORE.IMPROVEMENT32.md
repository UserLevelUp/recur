# Recur Improvement 32: Agent Discovery and Hierarchical Specialist Coordination

Date: 2026-09-07
Status: Closed / superseded by the reveal route (2026-09-07)

## Closure decision

The user closed this standalone agent improvement because agent and skill
artifacts can both be discovered and presented through the shared reveal route.
Do not schedule a separate `recur agent` implementation or Warp from this proposal.

Preserve hierarchical agent/persona/skill artifacts, scoped `.recur/` context,
bounded visibility and parent-facing reports as design input for later reveal
work. Core recognition remains read-only; runtime execution and opinionated
coordination remain separate companion concerns, not new responsibilities of
`recur reveal`.

This closure does not implement typed reveal, dispatch or coordination. The
proposal below is retained as historical rationale, not a pending implementation
assignment. See [the closed Eventness record](docs/main.improvement.32.closed.md)
and [the reveal direction](README.CORE.IMPROVEMENT29.md).

## Intent

Allow core Recur to query agent-related records while a separate `recur-agent`
application owns opinionated implementation work, runtime integration and agent
lifecycle actions. Support a solution coordinator, project managers and deeper
specialists whose local `.recur/` contexts reflect their responsibilities.

This is a future improvement document, not an active implementation or a frozen
CLI/schema contract. Choose a Warp and its tests after the current Warp and Lang
work is complete. Do not interpret either proposal's existence as authorization
to launch agents or implement a new runtime now.

## Ownership boundary

| Surface | Responsibility |
|---|---|
| `recur agent` | List, inspect, filter and select agent profiles, assignments, scope, reports and recorded evidence |
| `recur-agent` | Configure adapters/profiles, bind skills, dispatch authorized work and manage supported lifecycle actions |
| Hierarchical skill artifacts / reveal | Discover skill guidance through the reveal route; Improvement 31 is closed as a standalone proposal |
| Warp and Recur Lang | Supply acceptance and coordination contracts instead of duplicating them in the agent layer |

Core must not call a model, spawn a worker, interpret skill instructions as commands
or impose an agent persona. It can describe an opinionated profile without adopting
its opinions. Runtime permissions and enforcement belong to the acting application.

An agent profile, an assignment and a running instance are different records.
Likewise, a persona, skill package, model and harness/runtime adapter are distinct.
Keep their identities and relationships explicit.

### Shared typed artifact discovery

Retain the design from the now-closed Improvement 31: artifacts themselves have hierarchical names,
such as `agent.backend.recur.md`, `persona.skippy.recur.md` and
`skill.recur.expert.recur.md`. They can be found through ordinary `recur tree`
queries without separate discovery stubs or external runtime naming conventions.

The proposed shared reveal enhancement distinguishes their declared types and
presents appropriate metadata/content. Core recognizes and describes an artifact;
the appropriate companion owns its opinionated definitions and actions. A persona
does not become a running agent because reveal recognizes it. Type semantics and
full-body presentation remain to be specified and implemented, not current behavior.

## Root coordinator and project-local specialists

Illustrative layout; exact storage schemas remain undecided:

```text
solution/
  .recur/                    # Solution coordinator's context and child summaries
  backend/
    .recur/                  # Backend manager's local policy and assignments
    payments/
      .recur/                # Specialist skills, working detail and evidence
  frontend/
    .recur/                  # Frontend manager's local context
```

Each manager should normally inspect its children's public work contracts,
summaries, blockers and acceptance evidence. Detailed experiments, implementation
chatter and intermediate notes stay in the child's context until needed. A parent
can request expansion rather than receiving every descendant's working state.

Here, "public" means exposed to the parent workflow, not published on the internet
or committed to Git. These `.recur/` folders are typically hidden and Git-ignored;
other machines or runtimes need an explicit transfer or shared-access mechanism.

## Three separate visibility boundaries

1. **Directory scope:** the project or section from which records may be selected.
2. **Hierarchy depth:** how far a query expands descendants from its selected base.
3. **Reporting boundary:** the child artifacts intended for the parent to consume.

A depth limit reduces incidental detail but is not a sandbox or a reporting
protocol. A specialist's deeply nested blocker may need to appear in a shallow
parent report. Define explicit escalation/summary records so depth filtering does
not silently turn missing detail into a claim that everything is healthy.

Keep query visibility distinct from execution read/write scope. Neither directory
placement nor an ancestor's profile automatically grants authority to act in a
child project. Ancestor configuration lookup must not silently broaden evidence
collection or imply recursive inheritance.

## Profiles and the meaning of "tuned for"

A listing could show which runtime or agent families have configured adapters,
which local profiles reference them, and the specialization supplied by prompts,
skills and context preferences. It should distinguish:

- **Configured:** a definition or binding exists.
- **Available:** required local adapter/material can be resolved.
- **Tested:** evidence records validation of a particular adapter/configuration.
- **Reported runtime state:** an instance's last observation, with freshness or
  uncertainty made visible rather than inferred from its profile.

"Tuned for" should identify actual adaptations and their evidence. A named profile
does not establish that a vendor executable is installed, a service is connected,
an agent is running or its assigned work has passed acceptance.

## Illustrative queries and actions

Command shapes below are proposals, not commands available today:

```powershell
recur agent
recur agent explain backend.manager
recur agent list --scope backend --json
recur agent list -d backend/.recur --json
```

Queries should support selecting related profiles/tasks and inspecting child
summaries without dispatching work. Choose depth syntax during contract design;
reuse existing traversal concepts where their semantics match.

The companion may eventually initialize profiles, bind skills, dispatch a task,
resume an interrupted instance or request cancellation. Its first scope may be
an adapter to an existing agent runtime rather than a new model/tool loop.
Determine that from the completed Warp/Lang boundaries and concrete use cases.

## Assignment, report and acceptance

A future assignment needs an identity, parent/child relationship, intent,
input/output contract, selected skills, allowed scope and stopping conditions.
A result needs a matching assignment/attempt identity, reported outcome and
evidence references. Design these records around existing Warp/Lang contracts.

Child completion can mean "ready for parent review." It does not establish that
integration passed or that the parent's overall goal is complete. Preserve the
distinction between a report, declared evidence and checked acceptance.

The acting companion needs explicit behavior for acknowledgment, concurrent
ownership, interrupted work, retries, cancellation, stale reports and duplicate
delivery. Files can carry those records, but filename conventions alone do not
provide the runtime protocol. Avoid a second incompatible receipt mechanism.

## Integration without making MCP mandatory

A local coordinator can use scoped files, Recur queries, companion commands and
runtime adapters to coordinate specialists. Skills can be read directly by a
skill-capable runtime. MCP is optional: a later adapter may expose Recur queries
or connect the acting companion to external tools. Neither core discovery nor
local specialist coordination should depend on MCP or a specific model provider.

Reuse the shared prompt catalog for any future `agent.*` defaults instead of
hardcoding agent policy into the query engine. Defining such defaults, provider
calls and automatic skill selection remains later companion work.

## Decisions and acceptance candidates for a later Warp

- Freeze the profile/assignment/instance distinction and relationships to skill,
  reveal, Warp and Lang records before introducing new storage schemas.
- Demonstrate a coordinator, two project managers and a specialist with different
  local skill bindings. A shallow query exposes child summaries and escalated
  blockers while detailed child notes remain outside that selection.
- Define inheritance, duplicate profile IDs, nearest-project lookup and explicit
  cross-project access; show provenance rather than silently merging policy.
- Query configured, unavailable, tested and stale-reported cases without launching
  agents, contacting providers or treating declarations as live observations.
- Verify core queries produce no writes, dispatch, cancellation or skill execution.
- Test companion lifecycle behavior against a deterministic worker or adapter,
  including duplicate attempts, interruption, cancellation and failed gates.
- Prove that child acceptance does not automatically accept the parent.
- Decide the first adapter through local use; do not promise universal runtime
  support or create an MCP dependency merely to expose these records.

These are future test-design inputs. No tests, active Warp, slices or acceptance
receipts are created by this improvement proposal.

## Related proposals and discovery

- [Improvement 31: closed skill proposal / reveal route](README.CORE.IMPROVEMENT31.md)
- [Improvement 27: Warp](README.CORE.IMPROVEMENT27.md)
- [Improvement 29: orientation](README.CORE.IMPROVEMENT29.md)
- [Improvement 30: Recur Lang](README.CORE.IMPROVEMENT30.md)
- [Prompt discovery](docs/main.command.prompt.discovery.readme.md)
- [Closed decision record](docs/main.improvement.32.closed.md)

defines: recur.agent.discovery proposed pure agent and assignment query surface
defines: recur.agent.coordination proposed opinionated agent companion
consumes: recur.skill.discovery skill identities and scoped bindings
consumes: recur.prompt.discovery shared capability prompt registry
