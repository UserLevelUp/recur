Title
Improve recur trace UX for hierarchical repos: graceful failures + onboarding guidance (phased)

Problem
In this repo, hierarchical file naming (e.g., LevelController.CreateWizard3.*) trains users to think in “groups”. Users naturally try:
•	recur trace "LevelController.CreateWizard3" --scope "LevelController.CreateWizard3.*"
But trace expects a method symbol, not a “group/path”, and the failure modes are confusing:
•	unresolved or empty results
•	sometimes long-running/canceled resolution for “group-like” inputs
•	overloads (GET/POST actions) require --pick, but the user isn’t guided
This is a UX mismatch between:
•	how the repo is organized (hierarchical grouping)
•	how trace resolves symbols (method names only)
Goals
1.	Make trace fail gracefully and instructively when given group-like inputs.
2.	Make overload ambiguity discoverable and easy to resolve with --pick.
3.	Provide “fix the user” guidance (docs/examples) that matches real repo workflow.
4.	(Optional) Add Razor-aware “trace-like” behavior for CreateWizard3.AiTab.cshtml partial composition.
Non-goals
•	Full Roslyn semantic call graph (keep heuristic/regex-based if that’s the current design).
•	Tracing runtime dispatch/reflection.
---
Phase 1 — Better errors + guidance (low risk)
Changes
•	If <FUNCTION> looks group-like (contains . segments and matches many files or ends in */**), detect and print:
•	“This looks like a hierarchical group/scope, not a method.”
•	Suggested commands:
•	recur tree "<input>" --count
•	recur files "<input>.*" --ext ".cs"
•	recur find --scope "<input>.*" "<MethodName>(" --ext ".cs"
•	If symbol resolves to 0 matches, include:
•	“Hint: use recur find to locate the method name first…”
•	Add guardrail timeout / max-candidate early stop with explicit stop reason (not silent cancel).
Acceptance criteria
•	For trace invoked with LevelController.CreateWizard3.Ai.cs, output explains what went wrong and gives next-step commands.
•	No “mystery hang”; stop reasons are explicit.
---
Phase 2 — Overload/multiple definition UX (medium risk)
Changes
When multiple matches exist for a method name:
•	Print a numbered list of candidates (file + line + short signature if possible)
•	Tell user to re-run with --pick N
•	If --pick is missing, exit non-zero with a clear message
Acceptance criteria
•	recur trace "CreateWizard3" --scope "LevelController.CreateWizard3.*" prints the two action overloads and suggests --pick 1/2.
---
Phase 3 — “Fix the user”: docs + help examples (very low risk)
Changes
•	Update recur trace --help examples to reflect common patterns:
•	show --pick usage (overloads)
•	show tree/files vs trace distinction
•	Add a short doc page (or README section): “Tracing in hierarchical repos”
•	recommended workflow:
1.	tree to find files
2.	Find(string, int) to locate method signature
3.	trace with --pick
•	include an example for controller GET/POST overloads
Acceptance criteria
•	New user can follow docs to get a working trace in < 2 minutes.
---
Phase 4 (Optional) — Razor “trace” mode (higher effort)
Motivation
Users also want “trace-like” navigation between:
•	CreateWizard3.Tab.cshtml → CreateWizard3.Tab.Ai.cshtml → CreateWizard3.AiTab.cshtml etc.
Changes
Add a separate mode or heuristic:
•	recur trace --ext ".cshtml" --mode razor "CreateWizard3.Tab"
•	Edges inferred from:
•	Html.PartialAsync("Name")
•	Html.RenderPartial(...)
•	@await Html.PartialAsync(...)
Acceptance criteria
•	trace can print a partial-include tree for CreateWizard3 tab partials, without pretending it’s a C# call graph.
---
Notes / Examples from this repo
•	CreateWizard3(Guid?, Guid?) has overloads (GET + POST), so --pick is needed.
•	Hierarchical grouping is heavily used (LevelController.CreateWizard3.*), so group-like trace inputs are common and should be guided.