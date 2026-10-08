# Core workflow skill registration

defines: main.command.reveal.core-skills.init additive skill registry initialization at selected project levels

Core workflow specialties all use Reveal artifact type `skill`. The default
registry contains `recur-expert`, `recur-warp`, `recur-lang`, `recur-watch`,
`recur-trace-id` and `recur-demo-tests`. Names and trace namespaces distinguish
their purposes; no subtype mechanism is introduced.

`recur init` includes these records in fresh generated configuration.
On existing configuration it uses the same additive local registration path,
preserving project settings and explicit opt-outs rather than regenerating them.
`recur-reveal init` adds missing association tables and missing core entries
within a populated skill registry. Existing records, paths, associations and
comments remain authoritative. An explicitly empty skill registry opts out;
an empty individual record also remains untouched. Persona lists are preserved,
so registering six skills does not load all six into every persona's context.

```powershell
recur-reveal init --dry-run -d . --json
recur-reveal init -d . --json
recur-reveal init --local --dry-run -d demos/blackjack-web --json
```

By default init targets the nearest existing `.recur/config.toml`. `--local`
instead targets the explicit directory's own config, allowing nested project
or demo levels without modifying their parent. Repeated initialization is a
no-op after missing defaults are registered. Dry-run writes nothing and shows
the chosen path, scope and full proposed configuration.

Paths resolve relative to their config's project root. For example an existing
record can point to `guidance/team/recur-warp/SKILL.md` and keeps that path.
Scoped Reveal queries/context packets retain their canonical root boundary;
they do not automatically inherit or import parent registries. A parent or a
child may register the same skill name against its own local body.

Init writes configuration only. It does not copy repository skills into another
project, install global Codex skills, activate a persona or invoke an agent.
Missing bodies remain unresolved in discovery/packet diagnostics until supplied.
This repository supplies the five focused skill bodies and typed capsules;
their Codex installation is separate from companion initialization.
