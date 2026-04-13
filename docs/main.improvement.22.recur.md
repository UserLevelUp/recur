# main.improvement.22.recur

recur.gift = phase 1 shipped — next slice is current_thread in [reveal] so recur reveal with no args opens the right capsule
persona = recur expert advancing the reveal doctrine in small shippable slices
agent = add current_thread to [reveal] config; recur reveal no-args opens it directly if present
agenda = recur reveal should find the thread from the toml — no args needed when the repo knows where it was
goals.now = add current_thread field to RevealConfig; recur reveal no-args checks it first, lists all if absent; recur init scaffolds it blank; update reveal tests
schedule.next = RevealConfig.current_thread -> reveal no-args logic -> render_config_toml -> tests -> verify
pull.first = recur reveal main.improvement.22
pull.then = recur files "main.improvement.22.**" -d docs/
verify = cargo build --profile release-safe && julia julia-tests/main.command.reveal.test.jl
tool.escape = recur tree "main" -d docs/ --sep . --sep _ --show-sep
do.not.disturb = do not solve vault migration or multi-root inheritance in this slice
ready.state = I know the one field to add, the one behavior to change, and the tests to update
