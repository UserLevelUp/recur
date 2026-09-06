# Sudoku teaching demo

Restart `julia demos/sudoku/html5/serve.jl`, then refresh http://localhost:8787.
Existing legacy puzzle files are preserved. Select **New Puzzle** to have Julia
publish a unique playable package, or generate into a chosen directory:

```powershell
julia demos/sudoku/html5/generate.jl C:/path/to/output
```

Select a cell and press H, or use **Help with this cell**. **Find an easier move**
suggests the first supported naked single in row-major order; it highlights that
cell without changing your selection. **Select suggested cell** explicitly moves
focus. Each teaching step reveals more: row, missing digits, excluding peers,
conclusion. Hints never place digits. Show candidates and Show solution are
separate opt-in disclosures; a stored answer is not presented as a proof.

P toggles manual pencil mode. Auto-fill explicitly replaces manual notes and
starts a live computed layer. Editing a cell overrides that layer with manual
notes for that cell. Other moves preserve those notes; filling the cell removes
its notes and refreshes counts. Tentative wrong entries are visible but excluded
from accepted values: clear them with Backspace before requesting deductions.

## Contracts and honest limits

Julia writes `publish`, `subscribe`, and `trigger` relationships; Recur classifies
them. The browser still reads the pre-generated cascade format. Live teaching
records (`sudoku-deduction-v1`) are separate: puzzle ID, revision, board fingerprint,
target, occupied peer premises, conclusion and highlights. Validation recomputes
the complete record; notes and stored solutions are never premises. Invalid,
tentative, stale, mismatched or unsupported inputs cannot produce a teaching proof.

Only naked singles are taught here. Existing advanced solver exports remain for
compatibility, but the teaching UI does not present their results as verified proofs.
Scan order is deterministic, not a universal ranking of human difficulty.

The former Easy/Medium/Hard labels are now **25/35/45 gaps**. Existing URL parameter
values `easy`, `medium`, `hard` remain compatible. Blank count is not difficulty.
Julia grades each actual puzzle by replaying naked singles in deterministic order:
`naked-single-solvable` means the full replay finishes; `ungraded` means it stalls,
not that advanced techniques are required. A separate MRV search counts solutions
up to two. Each removal is kept only if unique; node/removal budgets fail closed.

`sudoku-playable-v1` is additive: solution_text and cascades retain their shapes;
presets supply validated givens, actual gap counts and grading traces. The browser
uses those givens directly and checks structural consistency. It trusts Julia's
uniqueness certification, not arbitrary third-party packages. Legacy packages
remain playable via the old mask, explicitly labeled unverified. A malformed new
package is an error, not a silent fallback.

Generation builds and validates a staging package, then replaces one JSON file
with a same-filesystem rename. Readers get one complete generation; failure leaves
the old package in place. Legacy files are not overwritten. Trace paths in cascade
metadata describe temporary generation inputs, not durable live-board files.
No live deduction export/persistence service is introduced.

## Reproduce verification

```powershell
cargo test --locked
julia julia-tests/runtests.jl
julia julia-tests/runtests.demo.sudoku.teaching.jl
python -m venv "$env:TEMP/recur-sudoku-teaching-tools"
& "$env:TEMP/recur-sudoku-teaching-tools/Scripts/python.exe" -m pip install playwright==1.62.0
& "$env:TEMP/recur-sudoku-teaching-tools/Scripts/python.exe" demos/sudoku/html5/tests/browser_test.py
& "$env:TEMP/recur-sudoku-teaching-tools/Scripts/python.exe" demos/sudoku/html5/tests/api_test.py
```

Browser tests use installed Edge (152.0.4191.62 in the receipt), an isolated profile,
an ephemeral local port and a synthetic screenshot fixture independently checked
for uniqueness in Python. `browser_test.py --channel chrome` also works with an
installed Chrome. `--package <path>/sudoku.playable.json` independently verifies
and loads a real generated package; `--screenshot <path>` records the final view.
API tests launch their own Julia server and temporary output directory. They do
not contact your running server or overwrite its data. `SUDOKU_PORT` and
`SUDOKU_DATA_DIR` are optional server overrides.

The full Julia suite now mirrors the watcher demo into a disposable directory:
its table-reset behavior cannot clear the user's live table. Expected legacy
package/favicon 404s and an intentionally injected generation 500 are reported
separately from unexpected browser errors.
