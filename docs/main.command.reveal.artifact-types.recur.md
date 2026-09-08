# Reveal artifact types
warp.id = main.command.reveal.artifact-types
warp.root = docs
observed.state = complete
pull.first = read docs/main.command.reveal.artifact-types.verification.md; recur warp slices main.command.reveal.artifact-types
verify = cargo test --locked; run the full Julia runner using the verified runtime configuration recorded in the verification file
do.not.disturb = Type recognition does not activate personas, load skills or execute agents; acceptance requires observed evidence.
prompt.ids = warp.naming, warp.slicing
