/**
 * game.js — Game loop orchestrator.
 *
 * Wires grid events to solver logic, cascade lookups, and hint display.
 * Tracks state: grid, remaining cells, hint levels per cell.
 *
 * Wrong answers are allowed — the panel explains why they're wrong.
 * Hints are progressive — each H press reveals more reasoning.
 * Hint overlays can be toggled on/off on the grid.
 */

import { loadPuzzle, buildMask, buildGrid, cellId } from './puzzle.js';
import { Grid } from './grid.js';
import { CascadePanel } from './cascade.js';
import { explainConflicts, hint as getHint, candidates } from './solver.js';

const DATA_DIR = 'data/easy-001';

async function init() {
  const { solution, cascades } = await loadPuzzle(DATA_DIR);

  const params = new URLSearchParams(window.location.search);
  const difficulty = params.get('difficulty') ?? 'medium';

  const mask = buildMask(solution, difficulty);
  const remaining = new Set(mask);

  // The live grid state — solver works against this, not the solution
  const grid = buildGrid(solution, mask);

  // Track hint level per cell (progressive hints)
  const hintLevels = new Map();

  // Track which cells have wrong values placed
  const wrongCells = new Set();

  // Hint overlay state
  let lastHintData = null;

  // Wire difficulty buttons
  document.querySelectorAll('.difficulty-btn').forEach(btn => {
    btn.addEventListener('click', () => {
      window.location.search = `?difficulty=${btn.dataset.difficulty}`;
    });
    if (btn.dataset.difficulty === difficulty) {
      btn.classList.add('active');
    }
  });

  // Wire "New Puzzle" button
  const newPuzzleBtn = document.getElementById('new-puzzle-btn');
  const generateStatus = document.getElementById('generate-status');
  if (newPuzzleBtn) {
    newPuzzleBtn.addEventListener('click', async () => {
      newPuzzleBtn.disabled = true;
      newPuzzleBtn.textContent = 'Generating...';
      generateStatus.classList.remove('hidden');
      generateStatus.innerHTML = 'Julia is generating a new solution + calling <code>recur trace-id</code> 81 times...';

      try {
        const resp = await fetch('/api/generate', { method: 'POST' });
        const data = await resp.json();

        if (data.status === 'ok') {
          generateStatus.innerHTML = `New puzzle generated in ${data.elapsed_ms}ms. Reloading...`;
          setTimeout(() => {
            window.location.search = `?difficulty=${difficulty}`;
          }, 500);
        } else {
          generateStatus.innerHTML = `Error: ${data.message}`;
          newPuzzleBtn.disabled = false;
          newPuzzleBtn.textContent = 'New Puzzle';
        }
      } catch (err) {
        generateStatus.innerHTML = `Server error — is serve.jl running? ${err.message}`;
        newPuzzleBtn.disabled = false;
        newPuzzleBtn.textContent = 'New Puzzle';
      }
    });
  }

  // Hint overlay now auto-activates on H key and clears on cell deselect

  // Build grid UI
  const gridContainer = document.getElementById('grid-container');
  const gridUI = new Grid(gridContainer, solution, mask);

  // Build cascade panel
  const cascadeContainer = document.getElementById('cascade-container');
  const panel = new CascadePanel(cascadeContainer);

  const updateStats = () => {
    document.getElementById('stat-remaining').textContent = remaining.size;
    document.getElementById('stat-placed').textContent = mask.size - remaining.size;
    document.getElementById('stat-total').textContent = mask.size;
  };
  updateStats();

  // Auto-fill pencil marks — only on explicit request (it's cheating!)
  const autoFillPencilMarks = () => {
    const candidateMap = new Map();
    for (const id of remaining) {
      const m = id.match(/sudoku\.r(\d+)\.c(\d+)/);
      if (!m) continue;
      candidateMap.set(id, candidates(grid, parseInt(m[1]), parseInt(m[2])));
    }
    gridUI.updatePencilMarks(candidateMap);
    updatePencilPanel();
  };

  // ── Cell selection ─────────────────────────────────────────────

  gridUI.onCellSelected = (row, col) => {
    gridUI.clearHighlights();
    gridUI.highlightCrosshatch(row, col);
    lastHintData = null;
    labelCascade?.classList.remove('panel-label-active');
    // Show candidates in panel when selecting an empty cell
    const id = cellId(row, col);
    if (remaining.has(id) && !wrongCells.has(id)) {
      const cands = candidates(grid, row, col);
      if (cands.length > 0) {
        panel.renderCellInfo(row, col, cands);
      }
    }
  };

  gridUI.onCellDeselected = () => {
    gridUI.clearHighlights();
    lastHintData = null;
    labelCascade?.classList.remove('panel-label-active');
    panel._renderEmpty();
  };

  // ── Digit placement ────────────────────────────────────────────

  gridUI.onDigitPlaced = (row, col, value) => {
    const id = cellId(row, col);

    // If cell has a wrong value, clear it first
    if (wrongCells.has(id)) {
      gridUI.clearCell(row, col);
      wrongCells.delete(id);
      grid[row - 1][col - 1] = 0;
    }

    // Already correctly placed
    if (!remaining.has(id)) return;

    // Check against Sudoku rules (not solution — explain the conflict)
    const conflicts = explainConflicts(grid, row, col, value);

    if (solution.get(id) !== value) {
      // Wrong answer — show it, explain why
      gridUI.showValue(row, col, value, false);
      wrongCells.add(id);
      // Don't update the solver grid with wrong value

      if (conflicts.length > 0) {
        // Has Sudoku rule violations — explain them
        panel.renderConflict(row, col, value, conflicts);
        gridUI.highlightConflicts(conflicts.map(c => c.conflictCell));
      } else {
        // Value doesn't violate visible rules but isn't the solution
      // Show candidates to help the player reason about it
      const cands = candidates(grid, row, col);
      const otherCands = cands.filter(c => c !== value);
      panel.renderConflict(row, col, value, [{
        type: 'logic',
        value,
        conflictCell: { row, col },
        message: `${value} doesn't conflict with any visible constraint yet, but it leads to a contradiction deeper in the puzzle.`,
        law: `This cell has ${cands.length} candidate${cands.length !== 1 ? 's' : ''}: ${cands.join(', ')}. Try elimination — check where each candidate can go in the row, column, or box.`,
      }]);
      }
      return;
    }

    // ── Correct placement ──────────────────────────────────────
    gridUI.showValue(row, col, value, true);
    grid[row - 1][col - 1] = value;
    remaining.delete(id);
    hintLevels.delete(id);
    updateStats();

    // Show cascade from pre-generated JSON
    const entry = cascades.get(id);
    if (entry) {
      panel.renderCascade(entry);
      labelCascade?.classList.add('panel-label-active');

      // Highlight peers from consume array
      const peerIds = entry.cascade.consume
        .map(item => extractCellId(item.line))
        .filter(Boolean);
      gridUI.highlightPeers(peerIds);
    }

    // Win check
    if (remaining.size === 0) {
      setTimeout(() => {
        panel.showWin();
        gridUI.clearHighlights();
        document.getElementById('win-banner').classList.remove('hidden');
      }, 400);
    }
  };

  // ── Pencil ins panel ───────────────────────────────────────────

  const pencilContainer = document.getElementById('pencil-container');

  function updatePencilPanel() {
    const entries = [];
    for (const [id, marks] of gridUI.manualMarks) {
      if (marks.size === 0) continue;
      if (!remaining.has(id)) continue; // skip solved cells
      const sorted = [...marks].sort((a, b) => a - b);
      entries.push({ id, marks: sorted });
    }

    if (entries.length === 0) {
      pencilContainer.innerHTML = `
        <div class="pencil-panel-empty">
          No pencil marks yet. Press <kbd>P</kbd> to enter pencil mode,
          then type digits to annotate cells.
          <div class="pencil-autofill">
            <button id="pencil-autofill-btn" class="autofill-btn">Auto-fill all candidates</button>
            <div class="autofill-warning">This is the easy way out, Joe.</div>
          </div>
        </div>
      `;
    } else {
      const lines = entries.map(e =>
        `<div class="pencil-entry"><span class="pencil-cell">${esc(e.id)}</span><span class="pencil-values">${e.marks.join(', ')}</span></div>`
      ).join('');

      pencilContainer.innerHTML = `
        <div class="pencil-panel">
          <div class="pencil-count">${entries.length} cell${entries.length !== 1 ? 's' : ''} annotated</div>
          <div class="pencil-entries">${lines}</div>
          <div class="pencil-autofill">
            <button id="pencil-autofill-btn" class="autofill-btn">Auto-fill all candidates</button>
            <div class="autofill-warning">Replaces your marks with computed candidates.</div>
          </div>
        </div>
      `;
    }

    // Wire auto-fill button
    const btn = document.getElementById('pencil-autofill-btn');
    if (btn) btn.addEventListener('click', autoFillPencilMarks);
  }

  function esc(str) {
    return String(str).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
  }

  updatePencilPanel();

  gridUI.onPencilMarkChanged = () => {
    updatePencilPanel();
  };

  // ── Pencil mode (P key) ────────────────────────────────────────

  const pencilIndicator = document.getElementById('pencil-mode');
  const labelPencil = document.getElementById('label-pencil');
  const labelCascade = document.getElementById('label-cascade');

  document.addEventListener('keydown', (e) => {
    if (e.key !== 'p' && e.key !== 'P') return;
    if (e.key === 'P' && e.shiftKey) return; // ignore Shift+P
    gridUI.pencilMode = !gridUI.pencilMode;
    if (pencilIndicator) {
      pencilIndicator.classList.toggle('active', gridUI.pencilMode);
      pencilIndicator.textContent = gridUI.pencilMode ? 'Pencil Mode ON' : 'Pencil Mode';
    }
    labelPencil?.classList.toggle('panel-label-active', gridUI.pencilMode);
  });

  // ── Hint system (H key) ────────────────────────────────────────

  document.addEventListener('keydown', (e) => {
    if (e.key !== 'h' && e.key !== 'H') return;
    if (!gridUI.selected) return;

    const m = gridUI.selected.match(/sudoku\.r(\d+)\.c(\d+)/);
    if (!m) return;
    const row = parseInt(m[1]);
    const col = parseInt(m[2]);
    const id = gridUI.selected;

    // If cell has wrong value, clear it first
    if (wrongCells.has(id)) {
      gridUI.clearCell(row, col);
      wrongCells.delete(id);
      grid[row - 1][col - 1] = 0;
    }

    if (!remaining.has(id)) return;

    // Progressive: each H press increments the hint level
    const currentLevel = hintLevels.get(id) ?? 0;
    const hintData = getHint(grid, solution, row, col, currentLevel);
    hintLevels.set(id, Math.min(currentLevel + 1, 4));

    panel.renderHint(hintData);
    lastHintData = hintData;
    labelCascade?.classList.add('panel-label-active');

    // Always show overlay on the grid when a hint is displayed
    applyHintOverlay(hintData);
  });

  /**
   * Apply hint overlay to the grid — highlight relevant cells
   * using strategy-specific overlay data from solver.js.
   *
   * Maps to eventness roles:
   *   strategy[]   → amber (produce/publish — the pattern source)
   *   eliminates[] → red dashed (consume/subscribe — affected peers)
   *   peers[]      → green (generic fallback for early levels)
   */
  function applyHintOverlay(hintData) {
    gridUI.clearHighlights();
    const overlay = hintData.overlayCells;
    if (!overlay) return;

    // Strategy cells (amber) — the discovered pattern
    if (overlay.strategy && overlay.strategy.length > 0) {
      gridUI.highlightStrategy(overlay.strategy);
    }

    // Elimination targets (red dashed) — cells affected by the pattern
    if (overlay.eliminates && overlay.eliminates.length > 0) {
      gridUI.highlightEliminations(overlay.eliminates);
    }

    // Generic peers (green) — fallback for early levels
    if (overlay.peers && overlay.peers.length > 0) {
      gridUI.highlightPeers(overlay.peers.map(p => cellId(p.row, p.col)));
    }
  }

  // ── Cell info display (on CascadePanel) ────────────────────────
  // Monkey-patch a simple cell info renderer onto the panel
  panel.renderCellInfo = function(row, col, cands) {
    const id = `sudoku.r${row}.c${col}`;
    this.container.innerHTML = `
      <div class="panel-section panel-info">
        <div class="cascade-header">
          <span class="cascade-cell">${id}</span>
          <span class="hint-level-label">Selected</span>
        </div>
        <div class="cascade-divider"></div>
        <div class="hint-lines">
          <div class="hint-line">Candidates: ${cands.join(', ')}</div>
          <div class="hint-line">${cands.length === 1 ? 'Naked Single — only one value possible!' : `${cands.length} possible values remain.`}</div>
        </div>
        <div class="hint-tip">
          Type a digit to place, or press <kbd>H</kbd> for a hint.
        </div>
      </div>
    `;
  };
}

function extractCellId(line) {
  const m = line.match(/^(sudoku\.r\d+\.c\d+)/);
  return m ? m[1] : null;
}

init().catch(err => {
  console.error('Failed to initialise game:', err);
  document.getElementById('grid-container').innerHTML =
    `<p class="load-error">Failed to load puzzle data. Try serving from a local web server.</p>`;
});
