# recur version

Status: `readme` (permanent)
Date: 2026-05-11

`recur version` is the pure query surface for artifact version-eventness.
It reads current artifacts, manifests, and `.recur/config.toml` policy without
writing files.

`recur-version` is the companion writer. It saves current artifact snapshots,
updates manifests, and writes ACK/NAK status records under `.recur/version/`.

## Core Split

```text
recur version   = inspect policy, schema, manifests, status, and history
recur-version   = save snapshots and update manifests
```

## Query Commands

```powershell
recur version status care.subject.routine -d fixtures/improvement26
recur version manifest care.subject.routine -d fixtures/improvement26
recur version policy care.subject.routine -d fixtures/improvement26
recur version schema care.subject.routine -d fixtures/improvement26
recur version query care.subject.routine --question "when did item-a become discontinued" -d fixtures/improvement26
```

## Writer Commands

```powershell
recur-version next care.subject.routine.proposed.current.csv -d fixtures/improvement26
recur-version save care.subject.routine.proposed.current.csv --slug item-a-discontinued -d fixtures/improvement26
```

## Artifact Shape

Current artifacts use:

```text
<subject>.<lifecycle>.current.<ext>
```

Version manifests use:

```text
<subject>.<lifecycle>.version.manifest.current.md
```

Saved snapshots use:

```text
<subject>.<lifecycle>.version.<version>.<slug>.<ext>
```

## Config Shape

Domain semantics stay in `.recur/config.toml`, not in code:

```toml
[artifact."care.subject.routine"]
kind = "structured-routine"
format = "csv"
risk_class = "synthetic-clinical-fixture"
persona = "care_schedule_expert"

[artifact."care.subject.routine".fields]
identity = ["TaskOrItem", "Route"]
tracked = ["Time", "DoseOrAmount", "Route", "Status", "Notes"]
state = "Status"
notes = ["Notes"]

[artifact."care.subject.routine".states]
proposed = ["DRAFT", "PROPOSED"]
discontinued = ["DISCONTINUED", "OUT CURRENTLY"]

[artifact."care.subject.routine".versioning]
strategy = "letter-number"
manifest_required = true
queryable = true
operator_required_for = ["approved", "discontinued", "restart_candidate"]
```

## References

- `README.CORE.IMPROVEMENT26.md`
- `docs/main.improvement.26.todo.future-plan.md`
- `julia-tests/main.command.version.test.jl`
- `src/main_command_version_impl.rs`
- `src/recur_version_main.rs`
