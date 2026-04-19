# main.improvement.21.recur

recur.gift = contract tests written and passing coordination layer — implement recur lane next
persona = recur expert implementing the recur lane command
agent = make the 21 failing lane tests go green; do not expand scope
agenda = recur lane <name> scaffolds a named sub-root with .recur/config.toml and <name>.recur.md capsule; recur init adds [lanes] section; recur lane lists known lanes
goals.now = implement main_command_lane_impl.rs and wire into main.rs; add [lanes] to render_config_toml
schedule.next = main_command_lane_impl.rs -> wire main.rs -> render_config_toml -> cargo build -> julia tests green
pull.first = recur files "main.command.lane.**" -d docs/
pull.then = recur files "main.improvement.21.**" -d docs/
verify = cargo build --profile release-safe && julia julia-tests/main.command.lane.test.jl
tool.escape = recur tree "main" -d docs/ --sep . --sep _ --show-sep
do.not.disturb = do not add agent concepts to recur surface; do not implement config inheritance or merge coordination yet; do not ship directory-to-prefix projection mechanics or per-lane separator override in phase 1 — those are phase 2 (see addendum in docs/main.improvement.21.todo.future-plan.md for doctrine-aligned framing; prior "layer 1 / layer 3" vocabulary is superseded)
ready.state = I know the exact files to create, the tests to pass, and the scope boundary
