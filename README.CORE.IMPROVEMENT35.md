<!-- recur-trace-v1
defines: main.improvement.35
defines: main.improvement.35.observed_moat
defines: main.improvement.35.brownfield_onboarding
defines: main.improvement.35.checkout_portability
defines: main.improvement.35.linux_and_ci_distribution
defines: main.improvement.35.cross_server_contracts
defines: main.improvement.35.settled_ux_and_hygiene
defines: main.improvement.35.saved_run_classifier_freshness
consumer: main.improvement.34
consumer: main.command.trace_id
consumer: main.command.warp
defines: main.improvement.35.saved_run_integrity
defines: main.improvement.35.independent_verification
defines: main.improvement.35.evidence_provenance
defines: main.improvement.35.trace_accuracy
defines: main.improvement.35.eventness_history
defines: main.improvement.35.watch_recovery
defines: main.improvement.35.lang_verification
consumer: main.improvement.34.evidence_scope
consumer: main.improvement.34.gate_review
consumer: main.improvement.34.correction_records
-->
# Recur Improvement 35: Production Hardening, Cross-Platform Evidence Portability, and Linux/Cloud Distribution

Date: 2026-11-12
Reviewed: 2026-11-12 against source and contracts at `e469bea`
Status: Proposed hardening roadmap; two prerequisite Warps completed (`2ad068c`, `a6563e4`), three local UX Warps planned; no Linux package, CI action, or cloud-bridge implementation claimed

defines: main.improvement.35
consumes: main.improvement.34 main.command.trace-id main.command.warp main.command.tree main.command.init main.command.lang

## Intent

Consolidate Recur's proven local multi-LLM workflow into a hardened, portable toolset
that adopts cleanly on brownfield repositories, survives cross-platform Git checkouts
without false evidence staleness, installs natively across Linux package managers and
CI runners, and extends contract/artifact verification across repository and server
boundaries (GitHub, GCP, Azure) without turning core `recur` into a stateful daemon,
message broker, or vendor-locked agent harness.

This README records empirical rationale, strict architectural boundaries, and a ranked
hardening sequence. It does not claim that unimplemented tracks already exist or
authorize breaking changes to current CLI/JSON contracts.

---

## main.improvement.35.observed_moat

defines: main.improvement.35.observed_moat

Recent implementation and cross-project dogfooding demonstrated these properties
of Recur. The comparison table below records positioning hypotheses for future
evaluation against current products and configurations.

1. **Separate implementation and verification passes (`C:/src/recur`)**:
   - In `main.command.trace-id.role-classification` (`a6563e4`), one model family
     implemented five slices against a declared `warp-map.json` contract and recorded
     scoped FNV-1a evidence, while a second independent model family audited the
     changes using repository artifacts, fresh test runs, `recur warp show` and
     `recur warp evidence`. This chat retained prior context, and the agents shared
     write access. Protected policy and verifier permissions remain proposed work.
   - That audit caught a cross-platform Git filter mismatch
     (`core.autocrlf=true` vs `core.autocrlf=false`) on `julia-tests/runtests.trace-id.jl`,
     which was then resolved without rewriting accepted layer history via
     `recur-warp refresh` (`warp-evidence-refresh-v1`).
2. **The graduated precision ladder in one namespace**:
   - **Hierarchy & Eventness** (`files`, `tree`, `.todo`/`.current`/`.complete`):
     zero-parse filesystem orientation and state visibility.
   - **Three-step open-world tracing** (`recur trace-id`):
     1. *Find the files* (`recur trace-id "<topic>" -d <root>`).
     2. *See what they say* (`--scope` + `--context` / `--list-ids` / heuristic roles).
     3. *Check what they claim* (`--declared` `recur-trace-v1` revision pins and SHA-256 digests).
   - **Byte-addressed Warp gates** (`recur warp`, `recur warp evidence`, `recur-warp refresh`):
     external test/build/scan outcomes bound to exact input fingerprints that turn
     `stale` automatically when source bytes drift.
   - **Optional formal contracts and planning** (`recur lang`, `recur-lang`):
     `recur lang` remains a pure, read-only query over `i(...)`/`o(...)` contracts,
     joins, and scopes. `recur-lang` owns planning preferences and supported checked
     transitions. Additional scaffolding belongs to future companion work.
3. **Cross-domain coverage beyond source ASTs (`C:/src/kicad_ra`)**:
   - Hierarchical identifiers and `--declared` revision pins connect mechanical CAD
     exports, KiCad schematics/PCBs, sourcing registers, Python generators, tests, and
     handoff documents in one query surface where language-specific LSPs cannot see.
4. **Read-oriented queries and explicit writes**:
   - Ordinary `files`, `tree`, `trace-id`, `warp`, `reveal`, and `lang` queries inspect
     local artifacts. Existing explicit writers include `recur init` and
     `trace-id --save-run`. CI profiles should select supported read-only commands
     and apply filesystem permissions appropriate to that job.

| Competitive surface | Strength | Structural gap Recur fills |
| --- | --- | --- |
| Repo spec templates (`Spec Kit`, `OpenSpec`, `.ank`) | Easy Markdown authoring in Git | Prose specs drift silently; no byte-bound staleness or revision-pin enforcement |
| Vendor agent harnesses (`Claude Code`, `Codex`, `Copilot`, `Antigravity`) | Tight single-vendor UX & subagent loops | Optimized for one vendor's runtime rather than neutral cross-LLM handoffs and receipts |
| AST / LSP code graphs | Exact symbol resolution inside supported languages | Blind to CAD, PCBs, BOMs, JSON receipts, Markdown contracts, and cross-repo artifacts |
| Runtime tracers & workflow engines (`OpenTelemetry`, `Temporal`) | Millisecond live RPC telemetry & retries | Do not govern pre-deployment spec/code/artifact contract lineage in Git |

---

## main.improvement.35.brownfield_onboarding

defines: main.improvement.35.brownfield_onboarding
consumes: main.command.trace-id main.command.tree main.command.init

### Track 1: Complete Brownfield Onboarding & Scan-Hygiene Warps

Two foundational Warps from the cross-functional usability batch are now complete:
- `main.command.warp.discovery-performance` (`2ad068c` — `11.5x` faster full Warp
  inventory and `173x` faster single-Warp lookup via candidate filtering and in-memory
  digest memoization).
- `main.command.trace-id.role-classification` (`a6563e4` — keyword-argument symbol
  guards, `.pyc`/binary skipping, word/CamelCase role boundaries, `define` deduplication,
  and `[role]` labels in `--format full`).

Before expanding external distribution, complete the remaining three planned local Warps
so first-contact behavior on existing non-Recur repositories is fast and predictable:

1. **`main.command.trace-id.scan-coverage`** (`warps/main.command.trace-id.scan-coverage.warp-map.json`):
   - Add repeatable `--exclude-dir <NAME>` across path discovery, declarations,
     inventory, context, and scoped role queries, retaining the contract's existing
     default exclusions and caps. Companion configuration can suggest project-specific
     cache exclusions. The bounded scanner already hides dot-prefixed entries and
     sorts traversal. Explicit exclusions help it reach relevant files within budget.
   - Clarify the planned diagnostics before implementation. The current scanner skips
     a file that exceeds its remaining byte allowance and continues traversal.
     Report omitted file content separately from directories never visited, and emit
     `stopped_at` only for actual termination. Count only eligible directories when
     reporting unreached scope, with a bound on the diagnostic inventory itself.
2. **`main.command.tree.hierarchy-depth`** (`warps/main.command.tree.hierarchy-depth.warp-map.json`):
   - Add opt-in `--levels <N>` / `--tree-depth <N>` relative to the matched base.
     Existing `--depth` continues to bound filesystem traversal. Specify whether
     `--count` reports all matched files when display branches are folded, and test
     configured lanes, separators, JSON, and both depth controls together.
3. **`main.command.init.cold-start-orientation`** (`warps/main.command.init.cold-start-orientation.warp-map.json`):
   - Extend read-only `recur init --analyze` with detected prefixes and orientation
     hints. Define prefix ranking, ties, root-level files, and configured separators.
     Keep write-mode configuration generation under its existing contract. Checkout
     portability diagnostics belong to Track 2.

---

## main.improvement.35.checkout_portability

defines: main.improvement.35.checkout_portability
consumes: main.command.warp

### Track 2: Cross-Platform Evidence Portability & Git Checkout Guardrails

**Observed failure mode (`a6563e4`)**: `recur warp evidence` hashes exact file bytes on
disk (`fnv1a64`). When a fingerprinted text file has mixed `CRLF`/`LF` endings in a
developer's working tree or lacks an explicit `.gitattributes` rule (`text eol=lf`,
`text eol=crlf`, or `-text`), Git's clean/smudge filters check out different bytes on
Linux CI (`core.autocrlf=false`) versus Windows (`core.autocrlf=true`), turning valid
evidence `stale` on a fresh clone.

Hardening requirements:

- **Keep raw-byte FNV-1a verification authoritative in `recur warp evidence`**:
  - Core `recur warp evidence` must continue hashing exact bytes on disk and must never
    silently normalize line endings behind the caller's back, because silent text
    normalization would weaken binary integrity and break existing `fnv1a64:` receipts.
- **Add read-only checkout portability diagnostics**:
  - Add an opt-in check (for example `recur warp evidence <manifest> --check-portability`
    and/or structured warnings in `recur-warp` evidence authoring/refresh workflows)
    that inspects every path in `source.files` when inside a Git worktree:
    1. Detect mixed `CRLF` + bare `LF` line endings in any non-binary text input.
    2. Detect when working-tree bytes differ from what Git would materialize under both
       `core.autocrlf=true` and `core.autocrlf=false` (unpinned `text` / `eol` attributes).
    3. Recommend exact `.gitattributes` rules (`text eol=lf`, `text eol=crlf`, or `-text`)
       before commit.
- **Companion-assisted evidence capture**:
  - Teach `recur-warp` (companion, not core `recur`) a helper subcommand to scaffold or
    verify `.verification.evidence.json` manifests with a configurable portability
    preflight. Offer `warn` and `require` project preferences, plus an explicit,
    recorded exception when a required preflight is intentionally waived. Initialize
    new helper preferences through companion `init`, preserving existing choices.
  - A missing attribute is a risk indicator. Compare actual checkout bytes, including
    result artifacts, and report custom filters or unavailable Git context as unknown.
    Authorize any execution of repository-defined filter programs through the host
    workflow. Core diagnostics can inspect attributes and report a pending comparison.

---

## main.improvement.35.linux_and_ci_distribution

defines: main.improvement.35.linux_and_ci_distribution

### Track 3: Linux Packaging, Static Binaries, and Zero-Trust CI Gates

Today `choco/` provides Windows Chocolatey packaging. Extend distribution to Linux
CI runners, containers, and developer servers. Measure installation time and runtime
dependencies for each selected core/companion bundle; configured test runners and
agent CLIs have their own dependencies.

Hardening requirements:

1. **Reproducible multi-platform release artifacts**:
   - Build and test static Linux release tarballs (`x86_64-unknown-linux-musl` and
     `aarch64-unknown-linux-musl`), macOS archives (`x86_64-apple-darwin`,
     `aarch64-apple-darwin`), and Windows archives (`x86_64-pc-windows-msvc`) containing
     the binaries declared for distribution in `Cargo.toml`. Test the complete selected
     bundle, including core and each shipped companion, on the target platform.
   - Emit a signed/pinned `SHA256SUMS` manifest (and optional `recur-trace-v1` release
     receipt) with every release tag.
2. **Native Linux & developer package channels**:
   - Add `[package.metadata.binstall]` to `Cargo.toml` so `cargo binstall recur`
     installs prebuilt binaries in seconds without compiling from source.
   - Add `packaging/linux/` recipes for `.deb` (`apt` / Debian / Ubuntu), `.rpm`
     (`dnf` / Fedora / RHEL / Azure Linux), Alpine `apk` / minimal scratch or Chainguard-compatible
     OCI layer, and a Homebrew formula (`brew install`).
   - Run core Rust + Julia verification on Linux in CI alongside Windows, with each
     demo suite selected explicitly in its own job, so
     path-separator (`/` vs `\`), case-sensitivity, and `.gitattributes` invariants are
     continuously enforced on real Linux filesystems.
3. **Turnkey GitHub Action & Cloud CI Gate (`setup-recur`)**:
   - Provide a minimal composite action / shell installer for GitHub Actions, GCP Cloud
     Build, and Azure Pipelines that runs deterministic PR checks. Measure installation
     separately for warm caches and cold downloads on named runners. The `< 2s`
     installation target is a performance prediction requiring measurement.
     ```bash
     recur warp list --json -d .
     recur warp show <warp-id> --json -d .
     recur trace-id "<scope>.**" --scope "**" --declared --json -d .
     ```
   - Fail CI cleanly (non-zero exit or structured JSON assertion) when a required Warp
     slice is `stale`/`blocked` or a `--declared` revision pin (`expected` vs `actual`
     or `sha256`) fails.
   - The CI wrapper must also reject missing required slices, conflicting contracts,
     malformed output, incomplete required discovery, and absent required review.
     Validate the expected Warp/slice identities and selected schema before accepting
     a successful command. Required identities come from the verifier's approved policy.
     A declared baseline slice may satisfy its own declared rule while implementation
     slices require checked evidence. Evaluate each required gate separately.

---

## main.improvement.35.cross_server_contracts

defines: main.improvement.35.cross_server_contracts
consumes: main.improvement.34.revision_dependencies main.command.trace-id main.command.warp

### Track 4: Cross-Repository and Server-to-Server Contract Lineage (GitHub, GCP, Azure)

As Recur moves from single-machine multi-LLM development into multi-service and cloud
environments, its scope boundary must remain sharp:

- **Non-goal**: Core `recur` must **not** become an HTTP server, cloud SDK wrapper,
  distributed lock manager, or live RPC span collector (those belong to OpenTelemetry,
  Temporal, Pub/Sub, or Event Grid).
- **Target niche**: **Stateless cross-boundary contract, schema, and artifact lineage**
  between producers and consumers across repositories, build pipelines, and servers.

Proposed architecture preserving offline, read-only core `recur`:

1. **Exportable contract & evidence bundles (`exports/` or `.recur/remotes/<alias>/`)**:
   - A producer repository or build server (e.g., an API schema repo on GitHub, a model
     training job on GCP Vertex/Cloud Run, or a firmware pipeline on Azure) publishes a
     compact, self-contained bundle containing:
     - `recur-trace-v1` `defines` + `artifact` records (`id`, `context`, `revision`,
       `sha256`), and
     - optional checked `warp-external-evidence-v1` / `warp-progress-v1` summary receipts.
2. **Transport stays outside core `recur`**:
   - Standard cloud/OS tools (`gh release download`, `gcloud storage cp`, `az storage blob download`,
     OCI artifact pull, or `recur-watch` / CI fetch scripts) sync remote bundles into a
     local read-only mirror directory inside `-d ROOT` (for example `remotes/billing/`).
3. **Verification uses ordinary local `recur` queries**:
   - Consumer repositories or deployment gates declare `requires: {"id": "service.billing.invoice", "context": "wire", "expected": "R12", ...}`
     and run standard offline `recur trace-id --declared` across local docs + `remotes/**`
     within `-d ROOT` (respecting existing symlink and root-containment guards).
   - Drift between Producer Server A (`R13` or mismatched digest) and Consumer Server B
     (`expected: R12`) becomes immediately visible with exact spans and zero central
     database.
4. **Bound imported bundles and record provenance**:
   - Bind repository identity, exported revision, producer policy, and artifact digests.
     Validate extraction paths, file counts, sizes, duplicate names, and case collisions
     before publishing a mirror. Stage the full bundle before changing its selected
     pointer, so interrupted downloads retain the previous complete view.
   - Consumer configuration chooses trusted issuers and revisions. Namespace aliases
     retain the original trace ID and producer identity. Duplicate `defines` from two
     mirrors remain ambiguous until that explicit context resolves them.
   - Record when a mirror was fetched. Offline queries assess that recorded snapshot,
     with remote freshness unknown until a separately authorized fetch occurs.
     Authenticity follows the independent verification proposal below.

---

## main.improvement.35.settled_ux_and_hygiene

defines: main.improvement.35.settled_ux_and_hygiene
defines: main.improvement.35.saved_run_classifier_freshness
consumes: main.command.warp main.command.trace-id

### Track 5: Saved-Run Classifier Freshness, Settled Warp Diagnostics & Upgrade Hygiene

Three operational and upgrade rough edges surfaced during live multi-agent dogfooding
and independent post-commit audit of `a6563e4`:

1. **Saved `trace-id` runs ignore classifier/semantics upgrades (`main.improvement.35.saved_run_classifier_freshness`)**:
   - **Observed upgrade gap**: In `src/main_command_trace_id_impl.rs` (`TraceIdRunManifest`
     and `evaluate_run_status` around line 1176), `--check-run` and `--reuse-if-fresh`
     validate only `request_fingerprint`, `config_fingerprint`, `file_fingerprint`, and
     payload existence—they do not check `manifest.version` or a `classifier_revision`
     token. Consequently, a run saved under an older binary (containing pre-`a6563e4`
     false-positive `consume`/`trigger` sites in `.recur/trace-id/runs/<name>/latest.json`)
     still reports `"status": "fresh"` after upgrading `recur`, and `--reuse-if-fresh`
     returns the stale cached payload instead of recomputing.
   - **Hardening requirement**: Add an explicit `classifier_revision` field (with
     `#[serde(default)]` so legacy `manifest.toml` files deserialize cleanly without
     parse errors) and check both `manifest.version` and `classifier_revision` inside
     `evaluate_run_status`. When an older saved run lacks the current `classifier_revision`,
     report `"status": "stale"` with reason `"classifier revision changed"` so the first
     `--reuse-if-fresh` query after an upgrade automatically recomputes while preserving
     same-version cache hits (`julia-tests/runtests.trace-id.jl` Phase 4b).
   - Keep this first repair small. Saved payload integrity and concurrent publication
     have a separate proposed identity, `main.improvement.35.saved_run_integrity`.
     Current `save_run_artifacts` writes payload and manifest separately, and freshness
     checks only require payload existence. A follow-up should bind the payload digest,
     query identity, effective trait policy, and classifier revision to one generation.
     Publish an immutable generation before atomically changing its current pointer.
     Detect torn writes, invalid payloads, and concurrent incompatible saves. Specify
     migration for old manifests and safe recomputation for unsupported cache versions.
   - Test broad saved queries against their own output directory. Writing a saved run
     must have a documented effect on subsequent discovery, so generated cache records
     cannot silently contribute recursive role hits or perpetual staleness. Preserve
     explicit inspection of cache files through a deliberate scope.
2. **Settled `current_slice` warning on completed Warps**:
   - Completed bubbles whose initial `warp-map.json` declared `"current_slice": "slice-0"`
     still emit `"Declared current_slice 'slice-0' is no longer ready; selection not inferred."`
     even when `"state": "complete"` and all required slices are covered (also noted in
     Improvement 34's settled-attention item). Suppress or rephrase this into a settled
     status note when `state == "complete"` while keeping `warp-map.json` immutable.
3. **Live `recur-watch` binary lock during Windows upgrades**:
   - Running `cargo install --path . --force` on Windows fails with `os error 5 (Access is denied)`
     when `recur-watch.exe` is actively running as a background daemon, leaving partial
     companion upgrades if operators do not install binaries selectively. Document a clean
     stop/upgrade/restart sequence in `recur-watch` and package installers (`choco/`,
     future Linux `systemd`/container hooks).
   - Record executable paths, digests, protocol support, and the previous installation
     in upgrade diagnostics. Detect a mixed core/companion installation before starting
     a coordinator. Recovery must identify which existing subscriptions can resume and
     preserve their journals. A shared version string alone is insufficient evidence
     that the same build or command set is running.

---

## main.improvement.35.independent_verification

defines: main.improvement.35.independent_verification
consumes: main.improvement.34.gate_review main.improvement.35.evidence_provenance

### Track 6: Independently controlled acceptance

The two completed Warps kept their planned gate rules. Existing `checked` evidence
establishes consistency and current fingerprints within the declared scope. The same
workspace can still give an implementation agent write access to rules, tests, and
receipts. Choosing another model supplies another review perspective. Filesystem and
CI permissions determine who can actually change the decision inputs.

- Make independent review an opt-in companion or CI policy for selected gates. Store
  its approved contract, required tests, verifier executable, and acceptance rules in
  a protected location outside the worker's write authority. Candidate changes to those
  rules require an explicit policy revision reviewed under the existing policy.
- Review the diff from the approved base to the candidate. Summarize changes to gates,
  fixtures, expected outputs, skips, dependencies, configuration, and file exclusions.
  Keep known failing counterexamples in the verifier's regression set. A removed check
  or narrowed source set needs a recorded reason and the required review.
- Bind the result to the tested commit or explicit source-tree digest, full policy
  digest, verifier identity, runner/tool identities, and exact test selection. Store
  acceptance in a separate receipt after testing, avoiding a receipt that tries to
  include its own future commit hash. Recheck the candidate before integration, and
  rerun affected checks if the merge changes the tested source tree.
- Separate artifact collection from execution of candidate code. The protected verifier
  and acceptance writer need their own authority. Run candidate code with permissions
  that prevent it from rewriting the verifier policy or publishing an accepted result.
  This isolation belongs to the execution host or CI. A different worktree alone does
  not establish it.
- For remote or adversarial workflows, define an optional versioned receipt envelope
  using SHA-256 digests and a verifiable signature or trusted CI attestation. Bind issuer,
  subject, policy and result together. Trust roots and revocation policy come from the
  consumer. Existing `fnv1a64:` receipts keep their current byte-integrity semantics.
  Their checksum and producer name alone establish no authenticated identity.
- Add counterexamples for a weakened gate, a forged all-green JSON result, a substituted
  verifier, an old receipt replayed against new source, and an unauthorized signing key.
  Each should fail the opted-in independent acceptance policy. Existing local workflows
  retain their own declared or checked gate requirements.

## main.improvement.35.evidence_provenance

defines: main.improvement.35.evidence_provenance
consumes: main.improvement.34.evidence_scope main.improvement.35.checkout_portability

### Track 7: Reproducible evidence and bounded current-state checks

- Record the executed argv, working directory, exit status, test selection, relevant
  non-secret environment/configuration, tool versions and executable digests. A retained
  binary with the same version label may differ from the executable under review.
  Bind logs and machine-readable test results to the observation. Apply redaction before
  publishing logs, and disclose which fields were omitted.
- Make verification scope inspectable. Include fixtures, generated inputs, imported
  contracts, runtime modules, dependency lockfiles, and scripts that can change the
  claimed behavior. A suite count alone cannot establish this dependency coverage.
  Label selected checks and full regression runs separately, including known skips and
  unselected demos. Each gate states the coverage it requires.
- Add mutations that independently change an implementation, fixture, runner, policy,
  imported contract, or result. Confirm that the relevant evidence becomes stale or
  rejected. Include missing paths, malformed counts, zero executed tests, and failed
  results with matching hashes.
- Record assessment scope and observation time. Audit command-local digest reuse under
  file replacement, deletion, symlink changes, and equal-size edits during one query.
  Define snapshot consistency and an explicit incomplete/changed-input result before
  strengthening its claims. Test that every new invocation sees current bytes and that
  per-gate read limits still apply to cached inputs. Publish trusted results from an
  immutable tested checkout or host-controlled snapshot when hostile writers matter.
- Use versioned sidecars or envelopes for additional provenance. The current external
  evidence/result structures reject unknown fields. Existing schema identifiers,
  byte-digest interpretation, historical receipts, and refresh-chain validation need
  migration tests before any schema evolution.

## main.improvement.35.trace_accuracy

defines: main.improvement.35.trace_accuracy
consumes: main.improvement.34.declarations main.improvement.35.saved_run_integrity

### Track 8: Precise trace matching with visible coverage

- Add a fixture matrix for `topic.ready`, `topic.ready.extra`, `topic.ready_extra`,
  `topic.readyish`, and several IDs on the same line. A line mentioning a producer and
  a different consumer needs an observable role-association limit. Preserve existing
  heuristic behavior through a compatibility path, and use explicit declarations or
  a separately specified precise mode where exact role attribution is required.
- Cover configured keywords, punctuation, CamelCase, case folding, Unicode surrounding
  text, Markdown code examples, Python arguments, C# constants, and multi-line calls.
  Report the matching source span and classification basis where the output contract
  permits it. Keep observed tokens, inferred roles, and structured declarations
  identifiable in downstream evidence.
- Treat scan coverage and displayed-result coverage independently. An empty truncated
  result cannot satisfy a policy requiring absence of a dependency or unique authority.
  Preserve useful partial results with actionable bounds and omitted-scope diagnostics.
- Specify declaration adapters by format and schema before adding new ones. The current
  strict mode reads first-line Markdown trace metadata. JSON keys named `defines` and
  text embedded in source code require their own opt-in adapter contracts.
- Shared matching, separator, and read-budget controls should reuse `[traits.*]` when
  their semantics apply across features. Companion recipes, suggested scope, and
  review policy belong to companion configuration and its additive `init`. Test
  configuration precedence through both the core CLI and companion entry points.

## main.improvement.35.eventness_history

defines: main.improvement.35.eventness_history
consumes: main.improvement.34.correction_records main.improvement.34.evidence_scope

### Track 9: Recoverable Eventness and correction history

- Build on Improvement 34's correction and evidence-scope proposals. An opt-in summary
  should show the predicted outcome, recorded observation, current evidence assessment,
  unresolved findings, and next verification action. Each claim links to its subject,
  trace IDs, slice/attempt identity where applicable, source revision and evidence.
- Represent planned, implemented, tested, reviewed, and integrated claims under project
  vocabulary. A design-only bubble can finish its own contract while a separate test
  bubble remains pending. Parent release policy explicitly declares the dependencies
  required for integration. Preserve the existing meaning of `complete`.
- Keep original claims and receipts addressable. Corrections identify the exact claim
  and digest they affect. Detect competing replacements, missing targets, and cycles in
  bounded supersession traversal. Historical completion and a current open finding can
  both appear in a summary. Timestamps alone cannot resolve contradictory observations.
- Preserve ordinary files, empty artifacts, and multi-file Eventness. Require typed
  metadata only where a chosen structured workflow needs it. Collapse should preserve
  blockers, rationale and evidence pointers needed for a fresh session to resume.
  Show unresolved references and budget exhaustion in compact views.
- Treat commands embedded in capsules, descriptions and Lang bindings as inert content
  during discovery. Execution uses explicitly configured companion argv and host
  authorization. Add tests that an imported capsule cannot redirect acceptance or run
  its instructions merely by being discovered.

## main.improvement.35.watch_recovery

defines: main.improvement.35.watch_recovery
consumes: main.improvement.35.eventness_history main.command.watch.eventness.native

### Track 10: Coordinator restart and reference reconciliation

- Extend the existing topic journal/replay tests through the coordinator. Kill it after
  a publication commits, after it reads the signature, and during downstream work.
  Restart from durable state and prove the same accepted work is not applied twice.
- Persist rich results in Eventness and retain the compact trace-ID notification API.
  Reconcile the referenced publication's digest, optional Warp binding and attempt
  context before acceptance. Reusing one trace ID for a later publication must not
  cause an old notification to accept an unrelated newer result.
- Use journal sequence and publication identity in companion-owned processing records
  for idempotency. Re-query live gates after recovery. Test changed/missing attachments,
  malformed journal tails, duplicate notifications, concurrent drains, exhausted limits,
  and host exits. Keep delivery, produced output, review and slice acceptance visible
  as separate observations. The existing replay protocol is the starting point.

## main.improvement.35.lang_verification

defines: main.improvement.35.lang_verification
consumes: main.improvement.35.evidence_provenance main.improvement.35.eventness_history

### Track 11: Optional Lang precision and runtime correspondence

- Pin a selected `.recur` source, exact function/flow identity, grammar/query revision,
  supported fragment, and configuration in evidence. Extend the existing WIR1 checked
  transition policy and case binding where it already provides these associations.
- Test dependency and wait cycles, self-dependencies, unreachable steps, missing
  producers, and ambiguous joins. A scoped view must retain relevant whole-flow
  findings. Intentional feedback needs an explicit initial input and iteration/exit
  semantics supported by the selected language revision. Until specified and supported,
  report that coverage limit. Static success establishes only the modeled fragment.
- Bind implementation and runner inputs to each required behavioral case. Add a fixture
  whose graph is statically valid but whose implementation returns the wrong output,
  and ensure runtime evidence rejects it. Test missing, repeated, failed and unexpected
  case identities, plus changed callbacks and external bindings.
- Treat imports and richer feedback semantics as separately versioned extensions.
  Specify contained resolution, transitive digests, duplicate symbols, import cycles,
  bounded expansion and unknown syntax before enabling them. Current query-v1 documents
  import resolution as deferred work.
- `recur lang` supplies deterministic inspection and checks. `recur-lang` supplies
  configured planning and transition guidance. Source snippets, trace IDs, ordinary
  coding and optional demo workflows remain valid entry points without a Lang contract.

All new tracks are proposals. Companion `init` should add missing preferences while
preserving explicit settings and opt-outs. Keep existing commands and schemas compatible,
and version additions that require stricter structure. The child Warp planning step
assigns actual dependencies and acceptance criteria to each proposed capability.

---

## Recommended Execution Order

This ranks the work by expected value, not by a strict dependency chain. Protected
verification can proceed alongside the UX work when a suitable host is available.
Lang is optional and does not block ordinary trace-ID, Eventness, or Watch adoption.
Create bounded child contracts before implementation; Track numbers above identify
topics, while ranks below suggest scheduling.

| Rank | Item | Surface | Why this order |
| ---: | --- | --- | --- |
| 1 | `main.improvement.35.saved_run_classifier_freshness` | Core `recur trace-id` | Closes post-upgrade `--reuse-if-fresh` stale-cache gap immediately after `a6563e4` |
| 2 | `main.improvement.35.saved_run_integrity` | Core saved runs | Binds cached payloads to their manifests and publishes consistent generations |
| 3 | `main.command.trace-id.scan-coverage` + `main.improvement.35.trace_accuracy` | Core queries; separate child contracts | Makes omissions visible and tests exact role/ID attribution before broader automation relies on it |
| 4 | `main.command.tree.hierarchy-depth` | Core `recur` (planned Warp) | Adds bounded hierarchy views without changing filesystem depth semantics |
| 5 | `main.command.init.cold-start-orientation` | `recur init` (planned Warp) | Teaches a tested discovery-to-detail path to new users |
| 6 | `main.improvement.35.checkout_portability` | Core diagnostics + companion preflight | Reduces fresh-checkout staleness before CI rollout |
| 7 | `main.improvement.35.evidence_provenance` | Evidence envelopes + runner records | Makes scope, tool identity, and test selection reviewable |
| 8 | `main.improvement.35.independent_verification` | Companion policy + execution host | Protects required checks and acceptance from the implementation worker |
| 9 | `main.improvement.35.eventness_history` | Existing Improvement 34 work + companion conventions | Preserves predictions, corrections, and unresolved findings across handoffs |
| 10 | `main.improvement.35.watch_recovery` | Native Watch + coordinator | Reconciles durable publication identities through restarts |
| 11 | `main.improvement.35.lang_verification` | Optional Lang checks and transitions | Connects static findings with selected runtime cases |
| 12 | Remaining `main.improvement.35.settled_ux_and_hygiene` work | Warp diagnostics + installers | Clarifies settled attention and makes mixed-install recovery explicit |
| 13 | `main.improvement.35.linux_and_ci_distribution` | CI and packaging | Distributes a tested bundle and independently controlled gate policy |
| 14 | `main.improvement.35.cross_server_contracts` | Conventions + companion sync | Extends verified local snapshots across repositories and servers |

---

## Proposed Verification Matrix (For Future Warps)

Each selected implementation track needs a bounded child contract, explicit evidence
scope, and relevant Rust/Julia/CI checks. Capture a failing counterexample where fixing
a defect. Record selected suites and allowed skips. Keep ordinary regressions separate
from optional, explicitly selected demo tests. A planning bubble can finish its own
contract; implementation and release acceptance require their declared verification.

| Track | Required self-contained verification |
| --- | --- |
| Saved-run classifier freshness | Seed legacy and prior-revision manifests with obsolete role results. `--check-run` reports stale and `--reuse-if-fresh` recomputes. Current compatible manifests still hit cache; unsupported versions never return apparently fresh results. |
| Saved-run integrity | Tampered payload, truncated JSON, interrupted manifest/payload publication, and concurrent saves cannot produce a fresh mismatched generation. Exercise broad queries for self-inclusion and define legacy migration. |
| Scan coverage | Existing scan-coverage contract plus byte-oversized files followed by small eligible files, exhausted entry limits, excluded directories, and bounded omission diagnostics. Test each query mode and unchanged defaults. |
| Trace accuracy | Exact ID versus descendants and lookalikes, multiple roles/IDs on one line, punctuation, Unicode, declaration adapters and configured separators. Truncated scans/results cannot establish project-wide absence or uniqueness. |
| Tree and init | Existing hierarchy-depth and cold-start contracts; filesystem depth versus displayed levels, folded counts, non-default separators, prefix ties, and read-only analysis. Companion init preserves settings and opt-outs. |
| Checkout portability | Temporary repositories with pinned and unpinned text, mixed endings, result artifacts, and custom filters. Compare checkout bytes under both autocrlf settings: pinned fixtures match, unstable fixtures are diagnosed, and unknown comparisons are reported. Preserve raw-byte hash semantics. |
| Evidence provenance | Independently mutate source, fixture, runner, imported contract, policy, or result. Exercise missing inputs, malformed counts, zero tests, failed outcomes with matching hashes, and unsupported schema versions. |
| Query snapshot consistency | Replace/delete files or switch links during a query; test equal-size edits, digest-cache reuse, and per-gate limits. Require the documented consistency result and fresh bytes on the next invocation. |
| Independent verification | Weaken a gate, remove a required test, substitute a verifier, forge a green result, replay an old receipt, or use an unauthorized issuer. The protected verifier rejects each; an explicitly approved policy revision can succeed. |
| Eventness history | Preserve original predictions through failure/correction, conflicting replacements, missing targets, and supersession cycles. Collapsed views retain blockers and evidence pointers; empty/multi-file Eventness remains usable. Embedded commands stay inert. |
| Watch recovery | Kill the coordinator at journal commit, notification consumption, and downstream completion boundaries. Replay duplicate notifications and reuse a trace ID with different publication content. No wrong-attempt acceptance or duplicate accepted effect. |
| Optional Lang | Valid graph with failing runtime implementation; missing/repeated/unexpected cases; cycles crossing scoped views; changed source/bindings. Unsupported imports or feedback semantics cannot be mistaken for complete static coverage. Ordinary non-Lang workflows remain usable. |
| Settled UX and upgrades | Complete Warp with an old valid `current_slice` yields a settled note; invalid slice names still warn. Partial binary upgrades, active Windows locks, and restart recovery preserve subscriptions/journals or clearly report incompatibility. |
| Linux and CI packaging | Build selected bundles on target platforms, verify release checksums/issuer, and run core suites with demos in selected jobs. Missing required Warp identities, incomplete discovery, malformed JSON, or an absent review fail opted-in CI policy even if a query exits zero. |
| Cross-server contracts | Two-repository fixture accepts matching revision/digest and rejects tampered or ambiguous imports. Exercise traversal paths, case collisions, interrupted fetches, bundle limits, and stale mirrors. Local queries make no network calls. |

## Implementation References for Child Warps

- [Saved-run implementation](src/main_command_trace_id_impl.rs): manifest freshness and separate payload/manifest writes.
- [Bounded trace discovery](src/trace_relationships.rs) and [inspection](src/trace_inspection.rs): coverage, declarations, ID inventory and excerpts.
- [Warp evidence](src/warp_evidence.rs) and [refresh](src/warp_refresh.rs): current schema constraints, raw-byte inputs and immutable refresh chains.
- [Native Watch Eventness](docs/main.command.watch.eventness.readme.md): existing publication identities, compact notifications and replay semantics.
- [Lang queries](docs/main.command.lang.query.readme.md) and [checked transitions](docs/main.command.lang.checked-transition.readme.md): supported fragments and current runtime policy bindings.
- [Improvement 34](README.CORE.IMPROVEMENT34.md): reuse the pending evidence-scope, gate-review and correction-record work when creating child Warps; avoid competing definitions of the same feature.

## References

- `README.CORE.EVENTNESS.md` — Eventness lifecycle and scoped completion model
- `README.CORE.IMPROVEMENT30.md` — Recur Lang WIR1/CIR1 and companion boundary
- `README.CORE.IMPROVEMENT34.md` — Explicit declarations, revision pins, and workflow clarity
- `docs/main.command.trace-id.readme.md` — Three-step `trace-id` workflow and precision ladder
- `docs/main.command.warp.evidence-refresh.readme.md` — Immutable same-contract evidence refresh
- `warps/main.command.warp.discovery-performance.verification.md` — Completed Warp #1 evidence (`2ad068c`)
- `warps/main.command.trace-id.role-classification.verification.md` — Completed Warp #2 evidence (`a6563e4`)
