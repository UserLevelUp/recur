# Warp: discoverable capability prompts

Status: planned. This bubble freezes contracts and red-first tests; it does not
implement the commands below or call an LLM provider.

Goal: let users and agents discover available prompts, inspect their instructions,
and prepare evidence for deciding where work belongs in an existing hierarchy.
Use the shared registry for every capability, including custom traits. Warp is
the first companion adapter, not a separate registry or prompt engine.

Revision v2 incorporates opinionated app-provided defaults. Pending Slice 1–5
and final contracts advance to v2; the accepted Slice 0 v1 baseline remains an
unchanged historical receipt, with an additional red baseline for this revision.

## Command contract

```powershell
recur prompt                                      # List all registered prompts
recur prompt warp                                 # List prompts for a capability
recur prompt warp.naming                          # Inspect one prompt
recur prompt warp.naming --intent "Add naming assistance" --scope main.command.warp --json
recur trait prompt warp                           # Alias for capability listing
recur-warp llm prompt                              # Alias for recur prompt warp
recur-warp llm prompt warp.naming --intent "Add naming assistance" --scope main.command.warp --json
```

Every form accepts `-d` and `--json`. Exact registered prompt IDs select a prompt;
otherwise a single-segment selector is a capability filter. An unknown qualified
prompt ID is an error. IDs use at least two nonempty dot-separated portable
segments; the first segment names the capability. This includes custom traits.
No selector lists everything. Unknown/empty capabilities list zero entries,
successfully, with human text `No prompts available for this capability.`
An empty global registry prints `No prompts available.` and succeeds.
Supplying `--intent` requires an exact prompt selection.

Listing and inspection prepare data. This bubble does not implement `llm suggest`,
network/provider calls, automatic name application, artifact creation/renaming,
Git operations, skill installation, or execution of instructions embedded in files.
Those are separate follow-on contracts. No new recur-trait executable is required.

## Registry and source resolution

The nearest ancestor `.recur/config.toml` supplies an optional registry:

```toml
[prompts.registry."warp.naming"]
capability = "warp"
description = "Find where new work belongs in the hierarchy"
path = "prompts/warp.naming.md"
inputs = ["intent"]
context = ["hierarchy", "files", "trace-id", "eventness", "warp"]
```

Opinionated companion apps supply immutable, preconfigured prompt catalogs through
the shared implementation. Ship `warp.naming`, `warp.slicing` and `warp.recovery`
with provider `recur-warp`; core discovery reads the same bundled definitions
without launching executables. Prompt availability describes access to those
definitions, not whether a companion executable is installed on PATH. Other apps
can contribute catalogs through the same interface without separate prompt engines.

Resolution precedence is **explicit project override, then app-provided default**.
No configuration, or an explicit empty project registry, still exposes app defaults
without writing anything. A same-ID project entry replaces the app entry as a whole;
invalid/missing overrides stay visibly invalid/missing instead of silently falling
back. Packaged originals are never changed by project configuration. Built-in
catalog IDs must be unique across providers; ambiguous provider claims are errors.

For an explicitly project-only registry, `[prompts] app_defaults = false` disables
app catalogs. This boolean defaults to true and follows nearest-project policy.
An empty resulting catalog reports the empty messages above. Do not invent prompts
merely from a trait's presence or require the persona-skills bubble to be complete.
An entry's capability must match its ID's first segment. Validate field types,
duplicate IDs, unknown context kinds, nonempty descriptions/paths and invalid IDs.
Context kinds are a fixed allowlist, never shell command strings.

Project prompt paths are relative to the config's project root. Refuse absolute paths, parent
traversal and symlink escapes. Resolve a source once within that boundary; do not
fall back to global registries or the network. A missing source remains visible in
list output as `status: missing`; inspecting/assembling it returns a blocked error.
Missing files must be distinguishable from malformed configuration and unsafe paths.
UTF-8 prompt bodies are opaque instructions for a consuming agent, never executed.

## Output schemas

- `recur-prompt-list-v1`: `schema`, `entries`. Entries sorted by `prompt_id`, each
  with `prompt_id`, `capability`, `description`, project-relative `path`, `status`,
  `inputs`, `context`, `origin` (`app` or `project`), and `provider` (`recur-warp`
  for its bundled entries, null for project entries). App paths use logical IDs
  such as `builtin:recur-warp/warp.naming`; they are not filesystem paths.
  Status is `available` or `missing`; listing does not embed bodies.
- `recur-prompt-show-v1`: `schema`, `prompt_id`, `capability`, `description`,
  `instructions`, `inputs`, `context`, `source: {path, fingerprint, origin, provider}`. Fingerprint is
  `sha256:<lowercase-hex>` of the exact source bytes. Inspection needs no intent.
- `recur-prompt-packet-v1`: `schema`, `prompt_id`, `intent`, `scope`, `prompt`
  (the show object), and `context: {items, truncated, diagnostics}`. Items include
  `kind`, `path`, `fingerprint`, `data`; diagnostics explain missing/limited context.
  Preserve the user's intent literally. No timestamps or random IDs in these outputs.
- `recur-prompt-error-v1`: `schema`, `state: blocked`, `code`, `message`.
  Invalid registry, unsafe paths, missing sources, unknown qualified prompt IDs,
  invalid input and invalid budgets exit nonzero with this JSON on stdout.
  Distinct codes: `invalid_registry`, `unsafe_path`, `missing_source`,
  `unknown_prompt`, `invalid_input`, `invalid_budget`.

Equivalent core/trait/companion requests return equal JSON. Human output escapes
control characters in names, descriptions and diagnostics. CLI argument syntax
errors can retain clap's existing stderr behavior.

## Context and naming contract

The packet builder reuses existing query implementations and policy. Collect the
selected hierarchy/files, explicit trace-id roles, Eventness settings, and Warp
maps/projections requested by the entry. `--scope` bounds subject selection; its
default is `**` beneath `-d`. Config/source lookup may find the nearest project,
but evidence collection must never widen beyond the requested `-d` subtree.
Private/hidden evidence follows existing explicit query/discovery rules.

Use `--max-files` (default 32) and `--max-bytes` (default 65536) to bound evidence
collection. Positive values only. Apply the file budget to distinct evidence
sources and the byte budget to serialized UTF-8 `context.items`; do not emit partial
JSON or exceed the budget. Report truncation/omissions deterministically. Prompt
source is separately limited to 64 KiB. A tiny evidence budget can return an empty
items array with `truncated: true`. Repeated queries over unchanged inputs match.

The naming prompt should ask the consuming LLM to prefer existing parents/subjects,
explain reuse versus a new subject, and return alternatives with evidence paths.
Use project-defined `prefix.base.suffix[.eventness][.ext]` conventions and configured
separators; `main` is a possible root, not a universal required component. Readable
identity, UUID identity, artifact purpose, and current attention remain distinct.
The prompt itself does not establish that any generated name is valid or accepted.

## Trait and reveal integration

`recur trait explain warp --json` adds a `prompts` array of registered entry metadata
without changing existing fields. `recur trait prompt <capability>` uses the shared
listing path and supports custom traits. Registration does not change trait runtime
preferences or grant execution authority.

A reveal capsule may contain `prompt.ids = warp.naming, warp.slicing`. Preserve
that raw field and add a top-level `prompts` array with `prompt_id`, `status`
(`available`, `missing`, `unregistered`) and `path` when registered. Unknown references
remain visible diagnostics and do not turn an otherwise valid reveal into a failure.
Reveal and trait explanation expose references without embedding prompt bodies or
assembling context. Existing skill pointers and reveal schema fields remain intact.

## Slices and acceptance

0. Freeze this contract, observe standalone red tests and passing legacy trait,
   reveal and Warp usability tests; accept only the baseline gate.
1. Typed registry and shared app catalogs, precedence, nearest-project resolution,
   source validation/fingerprints and explicit-empty/missing behavior. Test app
   catalog integrity, opt-out, malformed overrides, path escapes and symlinks.
2. Core `recur prompt` list/show/error schemas and human output. Test deterministic
   ordering, empty filters, ambiguity rules and no-write behavior.
3. Intent/context packet construction using bounded existing queries. Test
   hierarchy placement evidence, policy/separators, trace roles, real Warp context,
   nested-root narrowing, Unicode byte budgets, freshness and truncation.
4. `recur trait prompt` and `recur-warp llm prompt` adapters. Reuse one implementation;
   test app/default/project JSON parity, custom capabilities, empty registries and errors.
5. Trait explain and reveal prompt references. Test mixed available/missing/unknown
   references while retaining legacy fields and without loading source bodies.
Final. Integrate green tests into runtests.jl, run Cargo/full Julia regressions,
   document actual behavior and record receipts. Preserve known-broken cases.

Standalone initial contract: `julia-tests/main.command.prompt.discovery.test.jl`.
Keep it outside runtests.jl while intentionally red; extend the remaining cases
listed above during implementation. Do not weaken existing assertions or hide
missing functionality behind `@test_broken`. No production implementation is part
of this planning request.

defines: recur.prompt.discovery shared capability prompt discovery and bounded context
consumes: recur.trait.capabilities existing capability metadata
consumes: recur.warp.identity.policy safe initialization and readable/UUID identity conventions
