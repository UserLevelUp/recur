# Slice-1 specification and expected red observations

2026-09-12. No loader implementation existed at this observation.
`main.lang.evidence.recur` declares exact contracts, repeated local `a` under
associate/assess, alias assess.i(b)=associate.o(b), and intended E0/dE/Ef.
Lang check exits 0, sound only within WIR1 coverage, execution not-run.

Requirement-to-case matrix (exact IDs/results in the linked observation JSON):

| Requirement | Stable cases |
| --- | --- |
| identity/association | identity.*, association.*, inputs.missing |
| history and requirements | history.*, cases.* |
| freshness of spec/implementation/test/config/runner/behavior/result | freshness.* |
| nonzero executed passing counts, no skips | counts.* |
| native receipt and recorded complete are not proof | native-ack.*, identity.valid |
| canonical containment, traversal, directories, missing references | containment.* |
| bounded bytes/files/associations/attempts/cases | limits.* |
| query purity | every case compares packet and file bytes; purity.inert-commands |
| checked evidence versus accepted slice | identity.valid, acceptance.explicit |
| legacy compatibility | red mode invokes existing API with an argument-vector fixture |
| HTTP/browser presentation and process reentry | slice-3/final gates, before respective implementation |

Final red command: Julia 1.12.7 WindowsApps launcher with --startup-file=no
-C generic --compile=min --project=demos/web-evidence-lab,
LANG_EVIDENCE_RED=1, executing demos/web-evidence-lab/main.lang.evidence.test.jl.
Exit 1: 105 passing assertions, 51 expected failing verdict assertions, zero
setup errors or broken/skipped cases. All 52 fixture cases executed through the
real legacy MainLangAPI.respond seam. It returned observed_evidence=[] and the
test translates that established E0 to absent. This is a compatibility baseline,
not a fabricated execution of a nonexistent loader.

Logs and exact case observations: runtime-evidence/slice-1-red-final.log and
runtime-evidence/slice-1-red-final.observations.json. Earlier runs remain:
initial parse error and Windows file-symlink privilege error are setup failures,
not behavioral red. A directory junction now exercises canonical outside-root
resolution using the installed runtime; no privilege changes. WIR1 initially
required the desired Ef to be declared as a state event; corrected before this
final observation. Desired state remains declaration, never observed completion.

SHA256 at final red:

- Test: b420eeb0e95150d17f2e3842d6ea71b863c46607acee3ca66d6f584df1fc68ef
- Lang spec: cbaadcdfca7f954fac666e18838643c5761af2f58e6ca4e1dc07ad337d98caf8
- Association contract: bd55b3e5e652e1f4b3da88f58870afa5f5bc2763b86bfb9a2bb8171d75887e06

Acceptance is declared tests-first observation only, never a failed passing gate.
Next slice-2 implements the frozen loader and runs the unchanged assertions green.

produces: main.lang.runtime-evidence.tests observed tests-first cases
