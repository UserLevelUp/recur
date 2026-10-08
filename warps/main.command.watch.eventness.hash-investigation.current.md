# Eventness Watch binary evidence investigation

artifact.type = lane
defines: main.command.watch.eventness.evidence.binary.identity
consumes: main.command.watch.eventness.verification
publish: main.command.watch.eventness.evidence.binary.investigation.ready
warp.id = main.command.watch.eventness
warp.uuid = 01a11a3f-14c9-76f0-a4f6-62956194944b
branch = fix/watch-evidence-binary-hashes
status = diagnosed; fixes pending; promotion held

The branch starts at `2edda9caf5ce6fa8edcd90ba3cf3571cc39dbcb3` on
`recur-lang`. Existing uncommitted implementation and evidence were carried
into this branch. `recur-lang`, `main`, and `a.0.2.8` have not moved; nothing
from this investigation has been committed or pushed.

## Observed failure

The promotion rerun passed 305 Rust tests and 172 selected Julia checks.
The combined Node checks passed 45 and failed four. All four failures were
the acceptance suite comparing the current executable SHA-256 with the exact
binary identity recorded in the v6 native, integration, Rust, and Julia runs.
The Watch behavior checks did not fail.

Recorded v6 `recur.exe` SHA-256:
`a8f5515347712ddfa8547ad2f0788c16c2f9e554fc2e6d4a8df72160ba6d4ff9`.
Promotion rerun SHA-256:
`1089a09d53d9f52903edbe1b6c951b462c770fd5300c992361acdb1dce47f5d5`.
Both Rust logs show Cargo recompiling Recur. At the original failure, the
source checks passed. Subsequently, the verification runner and acceptance
suite were edited to support a fresh evidence prefix; those two edits now
make the old source evidence stale. This later source drift is distinct from
the initial binary mismatch.

## Reproduced cause

Changing only `src/main.rs` modification time, without changing its bytes,
forced two consecutive identical commands:

`cargo build --profile release-safe --bin recur`

The builds produced different whole-executable hashes but identical `.text`,
`.data`, `.pdata`, and `.reloc` sections. A PE parser checked every differing
byte. Exactly 20 bytes differed: one COFF timestamp byte, three debug-directory
timestamp bytes, and the 16-byte CodeView PDB GUID. There were no differences
outside those metadata fields.

The first build hash was
`152fdde3e4559060390556430759a04bddb8f45c5975506beae0b33fe6b4de7a`;
the second was
`e52aaf0e46efb418b2e414af4c6b121ffe6dfa805372294ad943875df381db02`.
Both `.text` hashes were
`47259c6541769feb37c0641d0df9591a771ae3aae44bd8ebb472e8f17346d068`.

`cargo test --profile release-safe --no-run --lib --bins --tests -v` also
rebuilt the normal CLI. Its `.text` hash matched the promotion executable,
while differing from the plain `cargo build` variant. Cargo fingerprint
records show the test dependency graph enabling Serde's additional `alloc`
feature and using different dependency artifacts. The Rust compiler and
release-safe profile identities matched. Build and test artifacts therefore
must not be assumed interchangeable even with identical repository sources.

The original v6 executable was not retained. These experiments establish a
same-source rebuild mechanism for the mismatch, but cannot prove every byte
of the historical v6 difference. The original SHA-256 check correctly
reported a different artifact; it was not evidence that Watch behavior failed.

## Fix sequence on this branch

1. Retain the exact tested executables in a separate immutable evidence bundle,
   outside Cargo's mutable output directory. Record toolchain, target, features,
   build commands, executable identities, and `build.rs` among the source inputs.
2. Run build-producing checks first. Bind native and Julia checks to the chosen
   binary bundle, and detect any binary change during those checks. Keep exact
   SHA-256 integrity checks; do not normalize away metadata to accept old evidence.
3. Make evidence output prefixes explicit and reject overwriting an existing
   bundle. The current prefix-support edits are preparatory changes, not a fix
   validated by this investigation.
4. Produce and review fresh evidence, preserving all v6 observations and accepted
   layers. Resolve the separate refresh-scope deficiency before publication:
   the predecessor manifests contain 265 source inputs, while refresh's current
   reader permits 128 unique canonical files. Do not narrow replacement evidence
   or silently increase the existing default to evade this check.
5. Test replacement and mid-run mutation negative controls, inspect live Warp
   gates, then commit, push, and fast-forward only after the fixes pass review.

No refresh records were published, no accepted layer was rewritten, and no
refresh budget was changed during this investigation. No demo suite was selected.

Private `recur-git checkpoint --snapshot` capture:
`.recur/dispatch-acceptance/hash-investigation/before-branch.snapshot.txt`.
Private retained executable copies and executable-byte comparison scripts:
`.recur/dispatch-acceptance/hash-investigation/`.
Shared experiment observations:
`warps/watch-eventness/observations/hash-investigation-20261008/`.
