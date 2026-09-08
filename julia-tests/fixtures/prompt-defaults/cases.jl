# Evidence markers intentionally describe decisions, not implementation details.
# Rubrics are for evaluating future model responses; packet tests do not score reasoning.
const DEFAULT_PROMPT_CASES = [
    (id="reuse-existing", prompt="warp.naming", scope="main.lang", intent="Add retry support around await without creating a duplicate feature.",
     files=Dict("main.lang.retry-await.readme.md" => "defines: recur.lang.retry.await\nRetry around await already belongs to main.lang.retry-await.\n"),
     evidence=["recur.lang.retry.await"],
     rubric="Prefer reuse of main.lang.retry-await; cite its definition and distinguish extending it from a new sibling."),
    (id="ambiguous-parent", prompt="warp.naming", scope="main.command", intent="Add recovery support; the caller has not said which subsystem.",
     files=Dict("main.command.git.readme.md" => "defines: recur.git.recovery\nGit recovery repairs interrupted branch operations.\n",
                "main.command.warp.readme.md" => "defines: recur.warp.recovery\nWarp recovery resumes work from receipts.\n"),
     evidence=["recur.git.recovery", "recur.warp.recovery"],
     rubric="Present both plausible parents with evidence and request the missing subsystem; do not invent certainty."),
    (id="project-root-and-identity", prompt="warp.naming", scope="product.checkout", intent="Name the retry work under this project's existing checkout hierarchy.",
     files=Dict("product.checkout.readme.md" => "defines: product.checkout\nThis project's root is product, not main.\n",
                "product.checkout.retry.todo.current.md" => "consumes: product.checkout\nStable identity: 01990000-0000-7000-8000-000000000001\nCurrent attention: retry work awaiting implementation.\n"),
     evidence=["product.checkout", "01990000-0000-7000-8000-000000000001"],
     rubric="Reuse product.checkout.retry; do not force main. Keep UUID identity, hierarchy, todo purpose and current attention distinct."),
    (id="vertical-slices", prompt="warp.slicing", scope="main.command.export", intent="Plan an export command with useful, independently verifiable slices.",
     files=Dict("main.command.export.readme.md" => "defines: recur.export\nEXPORT_REQUIREMENTS: export filtered records as UTF-8 CSV, preserve Unicode, reject invalid filters before writing output.\nEXPORT_GATES: compare CSV records, exercise invalid filters, prove failed validation leaves destination unchanged.\n"),
     evidence=["EXPORT_REQUIREMENTS", "EXPORT_GATES", "Unicode"],
     rubric="Propose vertical slices with dependencies and observable acceptance gates, including Unicode and no output on invalid input. Do not mark proposed tests as passed."),
    (id="stale-attention", prompt="warp.recovery", scope="main.command.resume", intent="Determine the next step from actual Warp evidence after interruption.",
     files=Dict("main.command.resume.slice-0.todo.current.md" => "STALE_ATTENTION: this note still points at slice-0; consult map and receipts.\n",
                "main.command.resume.readme.md" => "defines: recur.resume\nRECOVERY_REQUIREMENTS: slice-0 is baseline; slice-1 implements the behavior after baseline acceptance.\n",
                "main.command.resume.warp-map.json" => """
                {"schema":"warp-bubble-map-v1","warp_id":"main.command.resume","required_slices":[
                  {"slice_id":"slice-0","contract_hash":"baseline-v1","depends_on":[],"evidence_gates":["tests"]},
                  {"slice_id":"slice-1","contract_hash":"implementation-v1","depends_on":["slice-0"],"evidence_gates":["tests"]}]}
                """,
                "main.command.resume.slice-0.accepted.warp-layer.json" => """
                {"schema":"warp-slice-layer-v1","warp_id":"main.command.resume","slice_id":"slice-0","contract_hash":"baseline-v1","attempt_id":"baseline","result_state":"accepted","result_hash":"baseline-result","evidence":{"tests":["baseline passed"]}}
                """),
     evidence=["baseline-v1", "implementation-v1", "baseline-result"],
     rubric="Use the accepted baseline receipt and dependency map to identify slice-1 as next. Explain stale attention; do not rerun accepted work or claim pending implementation passed.")
]
