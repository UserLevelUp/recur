# Per-slice intelligence ticks

defines: main.recur.warp.intelligence.tick advisory per-slice reasoning adjustment at Warp creation

Each newly created Warp slice has `intelligence_tick`: `-1` requests one step
down, `0` holds the starting level, and `1` requests one step up. The reference
is the Warp's starting reasoning level, not the previously executed slice.
This keeps independent and parallel slices from accumulating adjustments.

The companion owns the policy. `recur-warp init` adds missing editable defaults
to `.recur/config.toml`, preserving existing settings and comments:

```toml
[warp.intelligence]
baseline = "host-current"
default_tick = 0
```

Set `baseline` to a host reasoning level such as `medium`, or leave `host-current`
to use the level active when the Warp begins. Creation records this reference as
`intelligence_baseline` in the map. It does not inspect the host or resolve a
symbolic reference into a measured level.

The operator or agent records the actual starting level in the work context and maps
ticks onto the chosen host's available reasoning levels. At a host limit, report
that no further step is available. A tick is guidance: it does not change models,
grant authority, run an agent or establish acceptance. Selecting a different
model remains a separate choice.

`recur-warp create` previews the map and accepts repeatable overrides:

```powershell
recur-warp create demo.blackjack.review --goal 'Review blackjack bindings' --slice-intelligence slice-0=-1 --slice-intelligence slice-final=1 --json
```

Add `--confirm` to publish the reviewed map. Every slice uses configured
`default_tick` (initially `0`) unless
its configured JSON template supplies a tick or a CLI override selects one.
Overrides must name existing slices; duplicates, malformed values and ticks
outside -1 through 1 fail in the companion before publication. Core queries expose
recorded slice metadata without choosing or enforcing reasoning policy.
Existing maps may omit the field;
querying them does not migrate or infer a requirement.

For more detailed requirements, put the rationale and problem-specific trace
relationships in the slice's Eventness artifacts. Those artifacts can consume
this definition and define a more specific review requirement. For blackjack,
binding correspondence or cross-contract compatibility may justify a tick up;
mechanical metadata changes may justify a tick down. These are judgment calls,
not a calculated complexity score or acceptance gate.

consumes: recur.reveal.artifact-types discovery of related work-context artifacts
