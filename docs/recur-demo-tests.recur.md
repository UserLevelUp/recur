artifact.type = skill
defines: main.recur.skill.demo-tests optional demo selection and scoped verification guidance

skill.name = recur-demo-tests
skill.path = recur-demo-tests/SKILL.md
recur.gift = Test the selected demo with exact routing and retain core versus demo scope in evidence.
pull.first = read recur-demo-tests/SKILL.md; julia julia-tests/runtests.jl --list-demos
pull.then = preview --demo NAME --dry-run; run the selected demo with its dependency environment
verify = selection checks plus observed scoped runtime results; add core explicitly when relevant
skill.loading = Reveal discovers this pointer; explicitly read the skill to load it.
