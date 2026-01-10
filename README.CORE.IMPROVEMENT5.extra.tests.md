Goal
recur trace should help a developer answer: “If I start from this symbol, what does it call (directly and indirectly), and where is it defined?” quickly and deterministically, across a hierarchical codebase.
Test strategy (what to test, in order)
1) Contract tests: CLI argument behavior
Purpose: ensure the command is discoverable and failures are actionable.
•	recur trace --help prints:
•	required args (symbol/query + --scope)
•	defaults (depth default, ext default)
•	at least 3 runnable examples
•	Missing args produce errors that say what to add:
•	missing scope → “Provide --scope”
•	missing symbol → “Provide <SYMBOL>”
•	--scope accepts the same patterns as find/tree (e.g., LevelController.CreateWizard3.*) and errors if empty.
Acceptance: a user can copy/paste examples to get a first success within 30 seconds.
2) Symbol resolution tests (core correctness)
Purpose: trace must connect to real symbols, not only arbitrary strings.
Test with known C# symbols that exist:
•	Select a “root” method: LevelController.CreateWizard3(...) action.
•	Select an internal method: ApplyTemplate(...).
•	Select a service method: DynamicGameComponentService.DeleteGameComponentAsync(...).
Run:
•	recur trace "CreateWizard3" --scope "LevelController.CreateWizard3.*" --ext ".cs" --depth 2
•	recur trace "ApplyTemplate" --scope "LevelController.CreateWizard3.*" --ext ".cs" --depth 3
•	recur trace "DeleteGameComponentAsync" --scope "DynamicGameComponentService.*" --ext ".cs" --depth 3
Acceptance: output contains:
•	definition locations (file + line)
•	direct callees with file+line
•	transitive callees up to depth
3) Overload/partial class behavior tests
This repo uses “hierarchical partial files” heavily, so trace must handle that.
Test cases:
•	Same method name appears in multiple partials (should disambiguate).
•	Partial class: LevelController spread across LevelController.CreateWizard3.*.cs
Desired behavior:
•	If exactly one match in scope → trace it.
•	If multiple matches → print a disambiguation list:
•	LevelController.CreateWizard3(...) in ...Main.cs:123
•	LevelController.CreateWizard3(...) in ...Another.cs:55 And require --pick or fully-qualified signature.
4) Boundary tests (what it should not claim)
Razor partial calls are string-based, so don’t pretend you can trace them unless you implement it.
Test:
•	try tracing "CreateWizard3.Tab" within AddComponent.cshtml
Acceptance:
•	trace should either:
•	explicitly say “No symbols found; note .cshtml uses string references; use recur find”
•	OR, if you implement Razor support, output should be correct (see below)
5) “Callers vs trace” consistency tests
If you now have callers/callees, validate consistency:
•	recur callees "ApplyTemplate" ... should be identical to depth=1 of trace.
•	recur callers "ApplyTemplate" ... should be consistent with reverse edges.
Acceptance: same sets, same file/line locations (modulo formatting).
Desired functionality (spec-level requirements)
A) Output should be actionable
For every node in the trace, print:
•	symbol name
•	file path (workspace-relative)
•	line number
•	optional snippet (1 line)
•	call type (method invocation / ctor / extension / interface call)
B) Be explicit about “strings vs symbols”
If a user traces something that only exists as UI route string ("/Level/ApplyAiContent") but not as a method symbol:
•	return “0 symbols found”
•	include suggestion:
•	recur find --scope "LevelController.*" "ApplyAiContent" --ext ".cs"
C) Hierarchical naming integration (best fit for this repo)
Allow:
•	--scope "LevelController.CreateWizard3.*" to be the default mental model. Support shortcuts:
•	--scope-alias cw3=LevelController.CreateWizard3.*
D) Deterministic depth handling
•	Depth 1 = direct callees only
•	Depth N = expand N levels
•	Print counts per level (direct/transitive), and stop reasons (depth reached, external dependency, unresolved symbol).
E) Filtering & noise controls
Tests should validate:
•	--ext ".cs" works (exclude About.md, clear.copilot.cache.jl, etc.)
•	optionally --exclude "obj/**", bin/**
•	optionally --include-private / --public-only (if supported)
Practical test suite you can keep in the repo
Add a small doc file like docs/recur.trace.testing.md containing:
•	5–10 “golden commands” (copy/paste)
•	expected output characteristics (not full output, just assertions)
•	known limitations (Razor, dynamic dispatch, reflection)
That’s enough to keep trace from regressing and makes it easier for users to express “what’s missing” in a way you can implement.