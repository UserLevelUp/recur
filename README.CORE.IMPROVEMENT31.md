# Recur Improvement 31: Skill Discovery and Opinionated Skill Management

Date: 2026-09-07
Status: Closed / superseded by the reveal route (2026-09-07)

## Closure decision

The user closed this standalone skill improvement in favor of hierarchical skill
artifacts discovered through `recur tree` and the reveal route. Do not schedule
a separate `recur skill` implementation or Warp from this proposal.

Carry the useful design forward: skill/persona/agent artifacts are their own
discovery records; typed presentation belongs to reveal, while opinionated
definitions and actions remain outside the core query engine. Any companion
authoring work is a later design decision, not implemented by this closure.

This is a superseded design decision, not implementation completion or acceptance.
The proposal below is retained as historical rationale. See
[the closed Eventness record](docs/main.improvement.31.closed.md) and
[the reveal direction](README.CORE.IMPROVEMENT29.md).

## Intent

Make skill records discoverable, inspectable and selectable through Recur's
hierarchical query model. Put opinionated skill configuration and implementation
work in a separate `recur-skill` companion. Core Recur remains useful without an
agent, a model provider or MCP.

This document captures direction, not an executable contract. `recur skill` and
`recur-skill` are proposed surfaces. Decide their implementation and any Warp
bubble later; this proposal creates no active slice or acceptance receipt.

## Ownership boundary

| Surface | Responsibility |
|---|---|
| `recur skill` | Read skill records; list, filter, select and explain identities, sources, bindings and recorded evidence |
| `recur-skill` | Apply opinionated policy to author, configure, register, install or update skill packages where implemented |
| Agent or acting companion | Load selected instructions and perform the work they describe |

A query may return a skill's instructions or references without following them.
Selection does not mean activation, installation, permission or execution. A skill
is procedural guidance and supporting material, not an agent process.

The companion may eventually use an LLM for authoring or refinement. Provider
selection, invocation, filesystem mutation and publication belong to that acting
surface, not the query engine. No model connection is required for discovery.

## Hierarchical local context

### Accepted design direction: the artifact is the discovery record

Store skill definitions under the project's `.recur/recur-skill/` using hierarchical
filenames. Do not require a directory of identically named `SKILL.md` files or a
second catalog file merely to make the actual instructions discoverable.

```text
.recur/recur-skill/
  skill.recur.expert.recur.md
  skill.recur.eventness.recur.md
```

The same artifact supplies its instructions and discoverable identity. Existing
generic commands can discover suitably named files once those files exist:

```powershell
recur tree skill -d .recur/recur-skill
recur tree skill.recur -d .recur/recur-skill
recur reveal skill.recur.expert -d .recur/recur-skill
```

The default `.recur.md` entry suffix makes these artifacts eligible for current
reveal discovery. Current reveal displays capsule fields, not the complete Markdown
body, and may resolve discovery at the nearest project root. Do not promise that
these commands already provide typed, full-body or strictly subtree-bounded reveal.

Extend shared reveal discovery to distinguish skills, personas, agents and other
declared artifact types. `skill.*`, `persona.*` and `agent.*` are the intended
hierarchical conventions. Freeze type interpretation, any explicit type field,
conflict handling, unknown-type behavior, output schemas and body presentation in
the later contract. Recognizing a type never activates it.

`recur-skill` owns bundled opinionated defaults and their project-local versions
or overrides. Bundled defaults need tracked package sources; ignored `.recur/`
holds the local working artifacts. Other runtimes' naming requirements must not
dictate Recur's canonical storage. Optional exports are separate companion work.

This refines the earlier package/binding discussion below. No files are migrated
and no new reveal behavior is implemented by recording this design direction.

A solution can hold common skill bindings in its root `.recur/` directory, while
projects and sections hold specialized records in their own `.recur/` folders.
For example, a backend manager can select a payment-review skill without loading
frontend instructions or every specialist's working notes.

These folders are conventionally hidden and often Git-ignored. Their contents
remain local unless explicitly shared or published. Hiddenness is not access
control, and naming alone does not determine inheritance or trust.

The eventual query contract must make effective source selection explainable:

- Requested project/directory and hierarchical subject.
- Skill identity, description, source path and content fingerprint.
- Local binding versus any explicitly allowed ancestor or app-provided binding.
- Related prompt IDs, task subjects and agent profiles, where declared.
- Availability, missing references and validation evidence as separate facts.

Do not assume recursive inheritance or silent merging. Define duplicate IDs,
whole-definition overrides, explicit empty bindings and missing-source behavior
when the contract is frozen. Reuse existing source-containment and fingerprint
mechanisms where appropriate.

## Illustrative query surface

These examples express desired use cases; subcommands and flags are not frozen:

```powershell
recur skill
recur skill explain backend.review
recur skill list --scope backend --json
recur skill list -d backend/.recur --json
```

Filtering and selection should be deterministic queries over recorded facts.
An LLM choosing which skill best fits an intent is an optional companion behavior,
with its recommendation and supporting evidence distinguished from configuration.

Potential companion operations include initialization, authoring and updating a
project skill binding. Final verbs, preview behavior, write boundaries and runtime
adapters remain design decisions for the later Warp.

## Reuse existing capabilities

- `recur prompt` already resolves app defaults and project overrides. Skill records
  should reference this shared registry rather than introduce a second prompt engine.
- Reveal currently presents capsule fields and prompt references. Its proposed
  persona/skill packet work owns orientation and binding resolution; coordinate
  with it rather than create incompatible registries.
- Improvement 32 owns agent profiles, assignments and specialist coordination.
  It consumes skill identities and resolved sources from this improvement.
- Warp owns contracts and acceptance evidence; Recur Lang owns formal coordination
  relationships. A skill definition should not independently redefine either.

## Decisions to make before implementation

1. Define the contents and supporting-resource references of the canonical
   hierarchical skill artifact, plus any later migration from existing skill files.
2. What is the relationship between reveal skill bindings and the skill catalog?
3. How are hierarchy depth, directory scope and explicit ancestor lookup expressed?
4. Which metadata can be inspected without loading bodies, and how are body/resource
   reads bounded and omissions reported?
5. Which companion actions are in the first implementation, and what local state
   do they write for subsequent pure queries?

MCP can later expose these interfaces, but local files, CLI calls and library
interfaces are sufficient foundations. MCP is not a prerequisite.

## Candidate acceptance scenarios for a later Warp

- List a root catalog and a project-specific catalog without writing files.
- Explain why a particular source/binding was selected; expose missing or ambiguous
  records without silently substituting different instructions.
- Inspect skill metadata separately from bodies and supporting resources.
- Select by hierarchy, task relationship or declared agent binding without invoking
  a model or activating the skill.
- Preserve explicit project overrides and keep private material within the selected
  query boundary; report depth and byte/file omissions.
- Show that an available skill is not necessarily loaded, executed or evaluated.
- Verify companion mutations separately, including preservation of unrelated local
  configuration and the distinction between private storage and publication.

These are requirements to refine into tests later, not assertions of passing tests.

## Related proposals and discovery

- [Improvement 32: agents](README.CORE.IMPROVEMENT32.md)
- [Improvement 29: orientation](README.CORE.IMPROVEMENT29.md)
- [Improvement 30: Recur Lang](README.CORE.IMPROVEMENT30.md)
- [Prompt discovery](docs/main.command.prompt.discovery.readme.md)
- [Reveal persona/skill proposal](docs/main.command.reveal.persona-skills.readme.md)
- [Closed decision record](docs/main.improvement.31.closed.md)

defines: recur.skill.discovery proposed pure skill query surface
defines: recur.skill.management proposed opinionated skill companion
consumes: recur.prompt.discovery shared prompt definitions and bounded context
