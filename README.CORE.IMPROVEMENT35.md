<!-- recur-trace-v1
defines: main.improvement.35
consumer: main.improvement.34
consumer: main.command.trace_id
consumer: main.command.warp
-->
# Recur Improvement 35: Production Hardening, Cross-Platform Evidence Portability, and Linux/Cloud Distribution

Date: 2026-11-12
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

Recent implementation and cross-project dogfooding demonstrated four concrete properties
that distinguish Recur from generic Markdown spec templates, single-vendor agent
harnesses, and AST-only code indexers:

1. **Transcript-free multi-LLM verification (`C:/src/recur`)**:
   - In `main.command.trace-id.role-classification` (`a6563e4`), one model family
     implemented five slices against a locked `warp-map.json` contract and recorded
     scoped FNV-1a evidence, while a second independent model family audited the
     checkout cold—without shared chat history—using `recur warp show` and
     `recur warp evidence`.
   - That independent audit caught a genuine cross-platform Git filter mismatch
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
   - **Closed-world static queries vs opinionated transitions** (`recur lang` vs `recur-lang`):
     `recur lang` remains a pure, read-only query over `i(...)`/`o(...)` contracts,
     joins, and scopes, while all planning, scaffolding, and checked state transitions
     stay isolated in `recur-lang`.
3. **Cross-domain coverage beyond source ASTs (`C:/src/kicad_ra`)**:
   - Hierarchical identifiers and `--declared` revision pins connect mechanical CAD
     exports, KiCad schematics/PCBs, sourcing registers, Python generators, tests, and
     handoff documents in one query surface where language-specific LSPs cannot see.
4. **Strict read/write separation (`recur` sensor vs `recur-*` actuators)**:
   - Core `recur` commands (`files`, `tree`, `trace-id`, `warp`, `reveal`, `lang`) do
     not mutate repository state, spawn background workers, or call external networks,
     making them safe inside untrusted agent sandboxes and read-only CI containers.

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
   - Exclude non-source tool/cache directories (`.tools`, `__pycache__`, `.pytest_cache`,
     `.mypy_cache`, `.ruff_cache`, `.tox`, `.nox`) by default in bounded discovery and
     inspection walkers (`src/trace_relationships.rs`, `src/trace_inspection.rs`), add
     repeatable `--exclude-dir <DIR>`, and sort directory entries deterministically so
     `50,000`-entry / `32 MiB` budgets are never starved by local vendor caches.
2. **`main.command.tree.hierarchy-depth`** (`warps/main.command.tree.hierarchy-depth.warp-map.json`):
   - Repair `recur tree <base> --sep <sep> --depth <N>` depth counting relative to the
     matched base prefix and clarify directory-path vs dot-hierarchy invocation on
     existing brownfield layouts.
3. **`main.command.init.cold-start-orientation`** (`warps/main.command.init.cold-start-orientation.warp-map.json`):
   - Make `recur init` (and `--dry-run` preview) surface discovered lanes, inferred
     separators, `.gitattributes` portability hints, and the immediate three-step
     `trace-id` / `tree` / `reveal` orientation commands without mutating existing user
     files unexpectedly.

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
    verify `.verification.evidence.json` manifests with portability preflight enabled by
    default, refusing to seal non-portable text fingerprints unless `--allow-unpinned-eol`
    is explicitly passed.

---

## main.improvement.35.linux_and_ci_distribution

defines: main.improvement.35.linux_and_ci_distribution

### Track 3: Linux Packaging, Static Binaries, and Zero-Trust CI Gates

Today `choco/` provides Windows Chocolatey packaging, whereas cloud CI runners,
containers, and developer servers predominantly run Linux (`Ubuntu`, `Debian`, `RHEL`,
`Alpine`). Because Recur is written in Rust with no runtime interpreter dependencies,
it is ideally suited for sub-second container and VM installation.

Hardening requirements:

1. **Reproducible multi-platform release artifacts**:
   - Build and test static Linux release tarballs (`x86_64-unknown-linux-musl` and
     `aarch64-unknown-linux-musl`), macOS archives (`x86_64-apple-darwin`,
     `aarch64-apple-darwin`), and Windows archives (`x86_64-pc-windows-msvc`) containing
     `recur` plus all official companions (`recur-warp`, `recur-trace-id`, `recur-reveal`,
     `recur-lang`, `recur-git`, `recur-watch`).
   - Emit a signed/pinned `SHA256SUMS` manifest (and optional `recur-trace-v1` release
     receipt) with every release tag.
2. **Native Linux & developer package channels**:
   - Add `[package.metadata.binstall]` to `Cargo.toml` so `cargo binstall recur`
     installs prebuilt binaries in seconds without compiling from source.
   - Add `packaging/linux/` recipes for `.deb` (`apt` / Debian / Ubuntu), `.rpm`
     (`dnf` / Fedora / RHEL / Azure Linux), Alpine `apk` / minimal scratch or Chainguard-compatible
     OCI layer, and a Homebrew formula (`brew install`).
   - Run the full Rust + Julia verification matrix on Linux in CI alongside Windows so
     path-separator (`/` vs `\`), case-sensitivity, and `.gitattributes` invariants are
     continuously enforced on real Linux filesystems.
3. **Turnkey GitHub Action & Cloud CI Gate (`setup-recur`)**:
   - Provide a minimal composite action / shell installer for GitHub Actions, GCP Cloud
     Build, and Azure Pipelines that installs `recur` in `< 2s` and runs deterministic,
     zero-LLM-cost PR checks:
     ```bash
     recur warp list --json -d .
     recur warp show <warp-id> --json -d .
     recur trace-id "<scope>.**" --scope "**" --declared --json -d .
     ```
   - Fail CI cleanly (non-zero exit or structured JSON assertion) when a required Warp
     slice is `stale`/`blocked` or a `--declared` revision pin (`expected` vs `actual`
     or `sha256`) fails.

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

---

## Recommended Execution Order

| Rank | Item | Surface | Why this order |
| ---: | --- | --- | --- |
| 1 | `main.improvement.35.saved_run_classifier_freshness` | Core `recur trace-id` | Closes post-upgrade `--reuse-if-fresh` stale-cache gap immediately after `a6563e4` |
| 2 | `main.command.trace-id.scan-coverage` | Core `recur` (planned Warp) | Prevents `.tools` / `__pycache__` from starving bounded discovery on real repos |
| 3 | `main.command.tree.hierarchy-depth` | Core `recur` (planned Warp) | Makes `recur tree` predictable on brownfield directory layouts |
| 4 | `main.command.init.cold-start-orientation` | `recur init` (planned Warp) | Turns Day-1 onboarding into a guided 60-second experience |
| 5 | `main.improvement.35.checkout_portability` | `recur warp` + `recur-warp` | Eliminates `core.autocrlf` fresh-clone staleness before Linux CI rollout |
| 6 | `main.improvement.35.settled_ux_and_hygiene` | `recur warp` + `recur-watch` | Removes false alarm warnings on completed Warps and smooths upgrades |
| 7 | `main.improvement.35.linux_and_ci_distribution` | CI, `Cargo.toml`, `packaging/` | Unlocks Linux workstations, containers, GitHub Actions, GCP, and Azure |
| 8 | `main.improvement.35.cross_server_contracts` | Conventions + companion sync | Extends `--declared` & Warp receipts across repos/servers once Linux CI is live |

---

## Proposed Verification Matrix (For Future Warps)

Each hardening track above must ship with its own Warp contract, red baseline, and
self-contained Rust/Julia/CI tests before any completion claim is made:

| Track | Required self-contained verification |
| --- | --- |
| Saved-run classifier freshness (Rank 1) | Seed a legacy `manifest.toml` (missing `classifier_revision` or with prior revision) + poisoned `latest.json`; assert `--check-run` reports `stale` (`"classifier revision changed"`), `--reuse-if-fresh` recomputes clean results rather than serving poisoned payload, and same-revision `--reuse-if-fresh` still hits cache |
| Brownfield Warps (Ranks 2–4) | Existing contracts in `warps/main.command.{trace-id.scan-coverage,tree.hierarchy-depth,init.cold-start-orientation}.contract.md` |
| Checkout portability (Rank 5) | Temporary Git repo fixture testing mixed `CRLF`/`LF` and unpinned `.gitattributes` under both `git clone -c core.autocrlf=true` and `git clone -c core.autocrlf=false`; verify raw FNV-1a hashes stay unchanged and unpinned text inputs are flagged |
| Settled UX & hygiene (Rank 6) | Completed Warp fixture with `"current_slice": "slice-0"` reports settled completion without misleading warnings; unknown slice names still warn |
| Linux & CI packaging (Rank 7) | Linux CI workflow building `musl` binaries, verifying `SHA256SUMS`, running `cargo test --locked` + Julia suites on Ubuntu, and gating a fixture PR via `recur warp show` + `recur trace-id --declared` |
| Cross-server contracts (Rank 8) | Two-repo `mktempdir` fixture where Repo A exports a `R12` declaration/digest bundle and Repo B verifies matching (`R12`) vs drifted (`R11` or tampered bytes) pins offline with zero network calls in core `recur` |

## References

- `README.CORE.EVENTNESS.md` — Eventness lifecycle and scoped completion model
- `README.CORE.IMPROVEMENT30.md` — Recur Lang WIR1/CIR1 and companion boundary
- `README.CORE.IMPROVEMENT34.md` — Explicit declarations, revision pins, and workflow clarity
- `docs/main.command.trace-id.readme.md` — Three-step `trace-id` workflow and precision ladder
- `docs/main.command.warp.evidence-refresh.readme.md` — Immutable same-contract evidence refresh
- `warps/main.command.warp.discovery-performance.verification.md` — Completed Warp #1 evidence (`2ad068c`)
- `warps/main.command.trace-id.role-classification.verification.md` — Completed Warp #2 evidence (`a6563e4`)
