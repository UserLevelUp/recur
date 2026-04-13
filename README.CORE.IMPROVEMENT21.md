RECUR IMPROVEMENT 21
Directory Projection / Namespace Mapping
========================================
Date: April 9, 2026
Status: Proposal / future direction
Author: Proposed from real Sudoku demo + eventness usage


INTENT
------
This document proposes a future improvement for one of recur's current weak
spots:

recur understands hierarchical names well, but it is weaker when the real
semantic hierarchy lives partly in directory structure rather than only in the
basename.

Today recur is strong at:
- basename hierarchy
- separator-aware parsing
- multi-separator merge across naming conventions
- visible eventness in file names

Today recur is weaker at:
- treating directory segments as part of the logical namespace
- projecting path roots into stable hierarchy prefixes
- expressing "this directory is the base of a semantic lane"
- expressing "this extension or parent directory implies a suffix or role"


SUMMARY
-------
Improvement 21 proposes an optional projection layer that can map physical
paths into logical recur namespaces.

In short:

- directories can contribute a prefix
- a root can define the base namespace
- extensions or path roots can imply suffix/role information
- docs, code, tests, and demos can project into one auditable namespace

This is not a replacement for separator merge.
It is the next layer above separator merge.

Separator merge answers:
"How do I unify the same logical entity when different files use different
separators?"

Improvement 21 asks:
"How do I unify the same logical entity when part of the hierarchy lives in
directories, part in basenames, and part in suffix conventions?"


THE PROBLEM
-----------
Real repo usage keeps bumping into this.

Example:

```text
demos/sudoku/html5/js/solver.js
demos/sudoku/html5/js/game.js
demos/sudoku/html5/js/cascade.js
demos/sudoku/html5/css/game.css
demos/sudoku/html5/index.html
demos/sudoku/julia/Generator.jl
demos/sudoku/julia/Recur.jl
```

Those files clearly have semantic structure:

- `demos`
- `sudoku`
- `html5`
- `js`
- `solver`

But current recur mostly sees the basename hierarchy well only if the basename
already carries the structure.

That means the user has to manually invent doc-side identifiers like:

```text
demos.sudoku.html5.js.solver.eyeball.order.current
```

even though the physical file path already knows most of that story.

The same issue appears across:

- `docs/`
- `src/`
- `julia-tests/`
- `demos/`

Current recur can often *merge* these once names already exist.
What is missing is a good way to *project* physical structure into names.


SUDOKU EXAMPLE
--------------
Desired projections:

```text
demos/sudoku/html5/js/solver.js
  -> demos.sudoku.html5.js.solver

demos/sudoku/html5/js/game.js
  -> demos.sudoku.html5.js.game

demos/sudoku/html5/css/game.css
  -> demos.sudoku.html5.css.game

demos/sudoku/html5/index.html
  -> demos.sudoku.html5.index

demos/sudoku/julia/Recur.jl
  -> demos.sudoku.julia.Recur
```

That would let eventness, trace-id, and cross-domain discovery use the actual
demo structure instead of forcing all semantic projection into docs-only names.


CORE IDEA
---------
Add an optional mapping/projection layer, likely config-driven.

Possible config file:

```toml
# .recur/map.toml

[[projection]]
root = "demos/sudoku/html5/js"
prefix = "demos.sudoku.html5.js"
separator = "."
strip_extension = true

[[projection]]
root = "demos/sudoku/html5/css"
prefix = "demos.sudoku.html5.css"
separator = "."
strip_extension = true

[[projection]]
root = "demos/sudoku/html5"
prefix = "demos.sudoku.html5"
separator = "."
include_filenames = ["index.html"]
strip_extension = true

[[projection]]
root = "demos/sudoku/julia"
prefix = "demos.sudoku.julia"
separator = "."
strip_extension = true
```

This would allow recur to project physical paths into logical identifiers
without renaming the actual files on disk.


WHAT THIS COULD ENABLE
----------------------
1. Directory-as-prefix projection

```text
src/main_command_trace_id_impl.rs
  -> src.main_command_trace_id_impl
```

or, with separator normalization:

```text
src.main.command.trace-id.impl
```

2. Path-root-as-base

Example:

```text
demos/sudoku/html5/js/solver.js
```

could automatically gain base:

```text
demos.sudoku.html5.js
```

3. Role/suffix projection from extension or root

Examples:

- `.jl` implies Julia code lane
- `.js` implies browser logic lane
- `.css` implies styling lane
- `.html` implies page/layout lane

This does not need to be magical.
It just needs to be representable.

4. Reverse mapping

Given:

```text
demos.sudoku.html5.js.solver
```

recur could answer:

```text
-> demos/sudoku/html5/js/solver.js
```

5. Better cross-domain traceability

Docs, tests, source, and demos could all project into one namespace instead of
requiring hand-maintained name alignment everywhere.


WHY THIS MATTERS
----------------
This is not cosmetic.

It would improve:

- eventness quality
- trace-id usefulness
- discoverability across code + docs + tests + demos
- onboarding for humans and AI
- realism of demo-aligned identifiers

Right now recur knows names better than places.
Improvement 21 teaches recur that places can also become names.


RELATIONSHIP TO EXISTING IMPROVEMENTS
-------------------------------------
This fits naturally with:

- multi-separator merge
- trace-id
- recur-map / cross-namespace mapping ideas

It is especially close to the thinking in `recur-map`:

- separator merge unifies naming conventions
- recur-map unifies namespaces
- Improvement 21 would provide the projection primitive that makes those
  namespace mappings less manual

In other words:

- separator merge = same thing, different separator
- Improvement 21 = same thing, different directory root + separator + suffix


POSSIBLE COMMAND SHAPES
-----------------------
This could stay config-only at first.

But likely commands would help.

Examples:

```bash
recur map project
recur map "demos.sudoku.html5.js.solver"
recur map "demos/sudoku/html5/js/solver.js" --reverse
recur files "demos.sudoku.html5.js.**" --projected
recur tree "demos.sudoku.html5" --projected
```

The exact command surface is not the important part yet.
The important part is the projection model.


MVP
---
The smallest useful version would be:

1. config-driven projections
2. path -> logical identifier projection
3. identifier -> path reverse lookup
4. one inspection command to preview the projected namespace

That alone would be enough to prove the idea on the Sudoku demo and the
docs/src/julia-tests cross-domain layout in this repo.


NON-GOALS
---------
Improvement 21 is not trying to:

1. Replace current file naming conventions
2. Force every project to use directory projection
3. Remove separator merge
4. Require extensions to become semantic roles automatically
5. Solve all namespace mapping in one giant feature drop


WHY THIS IS FUTURE WORK
-----------------------
This is powerful, but it touches recur's core mental model.

It affects:

- path normalization
- logical identifier generation
- reverse lookup
- merge semantics
- trace-id scope expectations
- possibly config shape

That means it should be treated as a real future improvement, not a quick hack.


WORKING NAME
------------
Improvement 21: Directory Projection / Namespace Mapping

Other reasonable names:

- Path Projection
- Namespace Projection
- Directory-to-Namespace Mapping
- Recur Map Projection


BOTTOM LINE
-----------
Current recur is good at semantic basenames.

Improvement 21 would make recur much better at projects where semantic
structure is split across:

- directories
- basenames
- separators
- suffix conventions

That is increasingly common in real repos.

This would let recur reason more naturally about the actual shape of a project
instead of making users manually restate directory meaning in separate names.
