artifact.type = lane
publish: demo.blackjack.skill.contracts.attempt.1.result durable worker result
consumer: demo.blackjack.skill.coordination scoped rehydration

Intelligence and evidence live in this Eventness receipt. Its notification carries only the trace ID. Producer state and reviewed acceptance remain distinct. Earlier hyphenated Warp-slug signatures are preserved as legacy metadata; current signatures are trace-queryable.

```json
{
  "warp": "main.demo.blackjack-skill-replay",
  "slice": "contracts",
  "attempt": 1,
  "trace_id": "demo.blackjack.skill.contracts.attempt.1.result",
  "legacy_signature": "main.demo.blackjack-skill-replay.contracts.attempt.1.result",
  "state": "produced",
  "host": "acceptance-codex-high",
  "reasoning": "high",
  "claimed_at_unix": 1791437138,
  "finished_at_unix": 1791437304,
  "agent_seconds": 165,
  "artifacts": [
    {
      "reference": ".recur/blackjack-skill-workers/contracts/contract-review.json",
      "sha256": "eb419f3bd271b621b7d2d5e3c4da2ea4651c2f5cd9065ae75b9e4f3807532672",
      "result": {
        "status": "reviewed",
        "recommendation": "revise",
        "warp_id": "main.demo.blackjack-skill-replay",
        "slice_id": "contracts",
        "contract_id": "contract:main.demo.blackjack-skill-replay.contracts:v1",
        "gate": "contract-review",
        "acceptance_marked": false,
        "summary": "The ownership boundaries and restricted action parity are appropriate, but the v1 interfaces are not yet sufficiently frozen for independently written replay, statistics and integration consumers to interoperate without additional agreements. Revise the schemas and fixture expectations through the coordinator; this review changes neither interfaces nor implementation.",
        "independent_implementation": {
          "strategy": "The deterministic decision table and strict visible-input boundary are implementable independently. Freeze policy-record serialization and golden input/output cases before integration.",
          "statistics": "The accumulator can be implemented independently after fixing the hand-record representation, natural-count semantics and exact summary fields. The engine must supply scored settlement records and guarantee once-only calls.",
          "replay": "Not fully integration-ready: exact envelope/step schemas, canonical bytes, supported engine adapter and reset revision semantics are missing. A worker can build an internally consistent implementation, but that alone does not freeze an interoperable v1 protocol.",
          "integration": "Feasible with coordinator-owned engine and HTTP changes after the identified agreements. The supplied baseline is only the rivals engine; v2 solo compatibility and HTTP/browser behavior have not been demonstrated."
        },
        "evidence": {
          "method": "Static source review of the supplied contract, interfaces and baseline; read-only CLI recovery of the selected Warp map and gates. No Julia engine, HTTP, browser or Lang runtime behavior was executed.",
          "input_sha256": {
            "task.md": "c89e1abc7ef5f92823328302bb08bb8fd7f2ba54e07c7456c9f63da9f0eb4f16",
            "interfaces.md": "63db1bb4902d5eedfc356faa4ce4153a6713d019b68ca20ba7a280e56b765a69",
            "contract.md": "0c0d3b3536f02f700a5b4e7844b741fbe71296d8ce32bd23cfa1404244eb309b",
            "baseline-engine.jl": "791713b1cd1fa91697559f94392b9156e452e21a33dc667c89859635636302ca",
            "validate.cjs": "c74426f73c6c345fba1841f9eb8066adeef33228fc987691585514003118ffe7"
          },
          "recovered_constraints": [
            "No applicable AGENTS.md was found at the workspace or checked ancestors.",
            "Installed recur help was inspected before read-only warp list/show/slices and reveal queries.",
            "The selected map identifies contracts as ready but pending, with contract-review absent; tests depend on contracts, implementation slices depend on tests, and integration depends on strategy/replay/statistics.",
            "Only contract-review.json is authored. Prepared interfaces and any Lang boundaries remain unchanged. Static soundness is not runtime evidence.",
            "Provider retry instructions were recovered from docs/main.command.warp.dispatch.retry.recur.md. No provider invocation, retry, host change, dispatch or acceptance operation was performed."
          ],
          "limitations": [
            "validate.cjs checks status, recommendation and whether findings is an array; it neither validates finding substance nor executes gameplay.",
            "No golden fixture files or red-first execution results are supplied in this contracts workspace. This does not establish that fixtures are absent elsewhere.",
            "Rules.deck and Rules.score implementations, the v2 solo engine and HTTP/browser adapters are not part of the supplied baseline file; their runtime behavior is unverified.",
            "One auxiliary read used the workspace instead of the repository root and produced path-not-found diagnostics; the relevant retry document was subsequently read successfully from the repository root. No source was changed by these reads."
          ]
        },
        "findings": [
          {
            "id": "C01",
            "severity": "blocking",
            "topic": "Replay wire format and canonical hashing",
            "sources": [
              "interfaces.md:37-55",
              "contract.md:42-48",
              "baseline-engine.jl:12-51",
              "baseline-engine.jl:174-175"
            ],
            "finding": "Playback must reject non-exact envelope and step keys, but no envelope or step key sets or field types are specified. Engine ID values, identity lookup, policy version representation, revision placement and hash encoding are also unspecified. Sorting map keys alone does not define canonical bytes for the Table/Rival/Hand structs, named tuples in history, vectors, nothing and nested dictionaries.",
            "impact": "Separate replay producers, validators and HTTP consumers can disagree while each follows the prose; hashes may change across serialization choices even for identical states.",
            "required_resolution": "Freeze exact required/optional keys and types, supported engine/rules/strategy identity values and validation, command/deck placement, initial and post-state revision/hash fields, UTF-8 canonical JSON and SHA-256 representation. Define struct/named-tuple conversion, array order, null, integer/Bool encoding and unsupported value/key rejection. Supply a complete envelope and canonical state/hash vector for each supported engine. Hashes check deterministic consistency, not authenticity of an untrusted exported file."
          },
          {
            "id": "C02",
            "severity": "blocking",
            "topic": "Engine adapter and legacy solo compatibility",
            "sources": [
              "interfaces.md:38-40",
              "interfaces.md:47-48",
              "interfaces.md:57-58",
              "baseline-engine.jl:63-67",
              "baseline-engine.jl:118-126",
              "baseline-engine.jl:268-270",
              "baseline-engine.jl:284-288"
            ],
            "finding": "Replay mandates engine.newgame(money,count), but the only supplied engine is BlackjackRivals, whose zero-rival tables still publish v3 and require v3 commands. Its constructor has no policies keyword, and its policy is an unversioned constant rather than per-seat effective identities. The comment about the HTTP integrator selecting v2 does not establish the solo constructor or adapter contract.",
            "impact": "Using BlackjackRivals with count zero cannot by itself satisfy legacy solo v2. Replay cannot discover effective policy identities or pass explicit policies to this baseline without coordinated engine changes.",
            "required_resolution": "Freeze the supplied-engine protocol: constructors/adapters for v2 and v3, transition with order, phase/revision access, engine identity and effective policy metadata. Assign engine adaptation to integration and provide a solo fixture or adapter stub before claiming independent replay integration. Preserve distinct v2/v3 public schemas and defaults."
          },
          {
            "id": "C03",
            "severity": "blocking",
            "topic": "Reset revision origin",
            "sources": [
              "interfaces.md:38-45",
              "interfaces.md:64-65",
              "baseline-engine.jl:67",
              "baseline-engine.jl:271-272",
              "baseline-engine.jl:284-288"
            ],
            "finding": "Baseline reset creates a new game at old revision + 1. A fresh recorder built through newgame starts at revision zero and cannot record reset. The interfaces do not state whether reset also resets the public revision or how an inherited initial revision is recorded and reconstructed.",
            "impact": "After resetting a nonzero-revision session, accepted commands and the recorded initial hash can disagree with playback, or a fresh recorder can break the existing stale-request convention.",
            "required_resolution": "Choose and document one reset/initial-revision protocol shared by server and replay, including engine switches. Add a reset-after-several-actions fixture followed by deal, settlement and export/playback, plus rejection of requests from the preceding session without mutation."
          },
          {
            "id": "C04",
            "severity": "blocking",
            "topic": "Statistics transport and counting semantics",
            "sources": [
              "interfaces.md:24-35",
              "baseline-engine.jl:12-20",
              "baseline-engine.jl:56-58",
              "baseline-engine.jl:141-162"
            ],
            "finding": "Settlement hands are described by field names but not by an access convention or container type. Baseline Hand has result/stake/profit but no natural or bust fields; integration must derive these through handscore. Summary does not freeze the ROI field name, label value or complete output schema. The text also does not explicitly resolve a natural push, or contradictory records such as blackjack with natural=false.",
            "impact": "A Dict-based worker, a property-based worker and an engine passing raw Hand values can fail to interoperate. Counting naturals only when result is blackjack would miss a natural pushed against a dealer natural; summing both signals would double-count blackjack.",
            "required_resolution": "Specify a concrete settlement DTO and summary schema. Define naturals once per qualifying hand, including natural pushes, and decide validation of inconsistent result/natural/bust fields. Explicitly exclude Bool from integer stake/profit, define supported integer bounds and require complete prevalidation before mutation. Freeze sample_hands, sample_rounds and ROI labels. Keep payout computation in the engine."
          },
          {
            "id": "C05",
            "severity": "integration_obligation",
            "topic": "Once-only statistics and conservation",
            "sources": [
              "interfaces.md:27-35",
              "baseline-engine.jl:150-175",
              "baseline-engine.jl:226-251",
              "baseline-engine.jl:255-263",
              "baseline-engine.jl:290-310"
            ],
            "finding": "The baseline phase guard in settle! and deepcopy in transition provide useful transaction boundaries. settle_seat! is not independently idempotent, and simply adding the new accumulator alongside its existing counter increments would count twice. Dealer naturals can settle during deal!, before an ordinary turn. Empty rival hands currently skip settlement entirely.",
            "required_resolution": "Replace legacy counter updates rather than supplementing them. Call start_round! exactly once for every seat after an accepted deal, marking unfunded rivals sitting out, before either immediate or later settlement. Call settle_round! once per nonempty seat inside the guarded transition. Preserve started=active+sitting_out, rounds=settled_rounds for played rounds, hands as the split-hand denominator, resolved_wager as final stakes including doubles, and net_chips=balance+escrow-starting. No counts change on failed commands, stale requests or deck exhaustion. Repeated summary/export must be read-only. Test both per-seat reconciliation and whole-table conservation."
          },
          {
            "id": "C06",
            "severity": "blocking",
            "topic": "Policy selection and public capability metadata",
            "sources": [
              "interfaces.md:6-22",
              "interfaces.md:66-68",
              "contract.md:26-34",
              "baseline-engine.jl:10",
              "baseline-engine.jl:103-120",
              "baseline-engine.jl:182-195",
              "baseline-engine.jl:276-279"
            ],
            "finding": "Restricted hit/stand rivals and human split/double are explicitly selected and fit the baseline. However, policy-record version/capabilities types and rival_policies request shape are not frozen. The current reset whitelist rejects rival_policies, and public_rival exposes only a policy string. The baseline's shared-deck order is human, rivals in array order, then dealer during each deal pass; turns finish human hands, rivals, dealer.",
            "required_resolution": "Freeze policy records and the mapping from reset rival_policies to constructor policies, including ordering, omitted/default behavior, length matching count, unknown IDs and the zero-rival case. Keep policy string compatibility while adding detached version/capability metadata. Define public labels for human/rival action differences, seat order, shared card consumption, stake and bankroll effects; raw profit is descriptive, not a fair skill score. Preserve the dealer-aware strategy as a named benchmark without an optimality claim."
          },
          {
            "id": "C07",
            "severity": "blocking",
            "topic": "Public report and private replay boundaries",
            "sources": [
              "interfaces.md:49-68",
              "contract.md:50-68",
              "baseline-engine.jl:123-133",
              "baseline-engine.jl:174-176"
            ],
            "finding": "The endpoint responsibilities correctly keep replay server-owned and imported playback detached, but documented public report fields and completion eligibility are not enumerated. Baseline public_state intentionally reveals the dealer after settlement and retains completed-round dealer cards in history, while interfaces.md says live /api/state never includes a hole card without qualifying the round/phase.",
            "required_resolution": "Freeze a report allowlist with schema, per-seat totals, denominators, versions/provenance and capability/context labels; define completed-session eligibility and reject a fresh betting session. Clarify that current hidden cards remain secret during play while completed-round dealer visibility follows the intended public protocol. Replay GET must require settled state and the requesting cookie; POST must return only a detached completed public snapshot and must not replace or increment a live session. Keep raw decks/cursors and private envelopes out of public projections even after settlement. Label replay downloads as private and reports as explicitly exported in-memory-session data, without a durability promise."
          },
          {
            "id": "C08",
            "severity": "blocking",
            "topic": "Replay bounds and atomic rejection",
            "sources": [
              "interfaces.md:40-55",
              "baseline-engine.jl:136-138",
              "baseline-engine.jl:226-228",
              "baseline-engine.jl:268-310"
            ],
            "finding": "Complete 52-card permutations and inclusive 2000-command/200-deal/1MiB limits are good constraints. Their exact enforcement stages are not specified: whether record! also enforces them, whether a non-deal order is rejected, and what reject-before-work means for envelope encoding versus engine execution. The baseline only delegates deck validation to Rules.deck, whose implementation is not supplied here.",
            "required_resolution": "Define limits for recording and playback, canonical UTF-8 byte accounting, raw HTTP body preparse bounds, and shape/count/deck/hash-format preflight before engine transitions. Reject noninteger/Bool/out-of-range/duplicate/missing cards and define non-deal deck behavior. Validate the candidate recorder state, including post-state hashing and size, before committing game/commands/decks. Freeze behavior when a recording reaches a limit mid-round. Test exact limits and one-over rejection with unchanged recorder/envelope/live-session snapshots; an exhaustion failure must not append an accepted command or deck."
          },
          {
            "id": "C09",
            "severity": "integration_obligation",
            "topic": "Visible strategy inputs and detachment",
            "sources": [
              "interfaces.md:10-22",
              "interfaces.md:40-53",
              "baseline-engine.jl:56-58",
              "baseline-engine.jl:184-186",
              "baseline-engine.jl:290-311"
            ],
            "finding": "The strict eight-key strategy input can be constructed without exposing table, deck, cursor or dealer hole state. Independent tests of decision alone do not establish that integration actually constructs only this projection. Similarly, detached exports do not ensure recorded caller commands/decks remain stable if record! stores aliases. Recorder.game is externally accessible by contract.",
            "required_resolution": "Build only the documented visible values at the engine policy call site and test hidden-information sentinels there. Copy accepted commands, realized decks and policy parameters into recorder ownership. Define whether callers may mutate Recorder.game or a returned game; either forbid this contractually or defend consistency before subsequent recording/export. Mutating policy records, input dictionaries, exported envelopes or playback outputs must not change stored recordings or live sessions."
          },
          {
            "id": "C10",
            "severity": "blocking",
            "topic": "Golden fixtures and evidence sufficiency",
            "sources": [
              "contract.md:15-16",
              "contract.md:70-94",
              "interfaces.md:70",
              "validate.cjs:1"
            ],
            "finding": "This workspace supplies prose and a review-shape validator, not frozen golden fixtures or executed red-first results. The contract requires schema/fixture freezing before implementation gates. A successful validator run establishes only that this review artifact has the expected top-level shape.",
            "required_resolution": "Attach versioned golden cases to the frozen interface agreement and have the assigned tests slice record failing baseline execution before implementation acceptance. Cover both engine versions and all boundaries listed in fixture_expectations. Later integration must run selected blackjack-web tests and browser evidence for zero/one/two rivals and narrow viewport. Keep the default runner core-only, select blackjack-lab separately only if its rules change, and do not infer runtime or gate acceptance from Lang/static checks or produced worker status."
          }
        ],
        "fixture_expectations": [
          "Strategy: stand17 hard 16 hits and 17 stands; dealer-aware hard 12 stands against 4 but hits against 3; hard 13..16 stand against 2..6 and hit against 7; soft 17 hits, soft 18 stands against 8 and hits against 9/10/A, soft 19 stands. Exercise face/ace ranks across suits, natural/bust precedence and hit-only/stand-only fallback. Reject missing/extra keys, invalid types/ranges, Bool integers, duplicate/empty actions, unknown rules/policies; no input mutation.",
          "Statistics: a new summary has null ROI; an active round with two split hands, stake 10 each and profits +10/-10, gives one settled round, two hands, wager 20, net 0 and ROI 0. A doubled hand counts its final stake once. A natural push counts one natural and one push, with zero profit. A sitting-out start increments started/sitting_out but empty settlement adds no hands/settled rounds. Invalid second hand leaves every counter unchanged; detached summary and repeated reads do not mutate.",
          "Replay: fixed full decks for v2 solo and v3 one/two rivals with differing policies; initial and every accepted post-state hash must match through natural settlement, split, double, multiple rounds and sit-outs. Randomly generated deals export their realized permutations, and playback consumes those without RNG dependence. Mutating a private hole/deck/cursor field changes state_hash even when the public snapshot is unchanged. Hash all stats and history, not only the capped current public projection.",
          "Replay rejection: exact schemas, unknown engine/rules/policy/replay versions, invalid funds/count/policies, stale/Bool revisions, illegal commands/hand IDs, reset, malformed full decks, tampered initial/intermediate/final hashes and nonsettled endings. Test 2000/200/1MiB boundaries and one-over inputs. Verify full snapshots before/after rejection, caller-input mutation after recording, export detachment and live-session isolation.",
          "Integration: natural peek and ordinary finish settle each participating seat once; split aces and human split/double preserve original rules; rivals remain hit/stand and finish in stable seat order. Insufficient funds/sit-outs, bounded deck exhaustion and house reserve failure leave rejected transitions unchanged and conserve wallets/bank/escrow. Per-seat net statistics reconcile after each settlement.",
          "HTTP/browser: reset switches between v2 solo and v3 rivals with the chosen revision origin; stale/reset-invalid requests do not mutate. Two cookie sessions cannot obtain each other's private replay. During active play, state/report/replay endpoints obey privacy and phase guards. Imported completed playback returns only public detached data and leaves existing session revision/state unchanged. Completed reports include exact public provenance/denominators/capability labels; UI covers policy selection, download/playback, zero/one/two rivals and narrow viewport."
        ],
        "unresolved_questions": [
          "What exact envelope, step, policy-record, statistics-summary and public-report schemas constitute frozen v1?",
          "Which engine adapter supplies v2 solo construction, engine IDs and effective per-seat policy identities?",
          "Does reset preserve a monotonic public revision, and how is that origin represented in a fresh recording?",
          "Which settlement DTO and natural-consistency rules must every caller use?",
          "Which canonical hash vectors and red-first fixtures will the coordinator bind to the revised agreement?",
          "How do recording limits, public completed-round dealer visibility and completed-session report eligibility resolve at their boundaries?"
        ]
      }
    }
  ],
  "acceptance": "separate reviewed gate; worker result does not imply acceptance",
  "raw_execution_references": {
    "stdout": "\\\\?\\C:\\src\\recur\\.recur\\dispatch\\fnv1a64-a7b70ede52e7954d\\fnv1a64-fe2a717d57e66770.1.agent.stdout.txt",
    "stderr": "\\\\?\\C:\\src\\recur\\.recur\\dispatch\\fnv1a64-a7b70ede52e7954d\\fnv1a64-fe2a717d57e66770.1.agent.stderr.txt"
  }
}
```
