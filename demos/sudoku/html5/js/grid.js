/**
 * grid.js — 9x9 Sudoku grid renderer and input handler.
 *
 * Knows about cells and their positions. Does not know about cascades
 * or solving strategies. Fires events that game.js listens to.
 */

export class Grid {
  constructor(container, solution, mask) {
    this.container = container;
    this.solution = solution;   // Map: cellId -> value
    this.mask = mask;           // Set: cellIds the player must fill
    this.cells = new Map();     // cellId -> <td> element
    this.selected = null;       // currently selected cellId
    this.onDigitPlaced = null;  // callback(row, col, value)
    this.onCellSelected = null; // callback(row, col)
    this.onCellDeselected = null; // callback()
    this.onPencilMarkChanged = null; // callback()

    this.pencilMode = false;    // P key toggles pencil mode
    this.manualMarks = new Map(); // cellId -> Set of player-annotated digits
    this.computedMarks = new Map(); // live automatic layer; manual notes override per cell

    this._build();
    this._bindKeyboard();
  }

  _build() {
    const table = document.createElement('table');
    table.className = 'sudoku-grid';

    for (let row = 1; row <= 9; row++) {
      const tr = document.createElement('tr');
      if (row === 4 || row === 7) tr.classList.add('box-top');

      for (let col = 1; col <= 9; col++) {
        const td = document.createElement('td');
        const id = `sudoku.r${row}.c${col}`;
        td.dataset.cell = id;
        td.setAttribute('aria-label', `row ${row}, column ${col}`);

        if (col === 4 || col === 7) td.classList.add('box-left');

        if (this.mask.has(id)) {
          td.classList.add('empty');
          td.tabIndex = 0;

          // Pencil marks — 3×3 mini-grid of candidate digits
          const pm = document.createElement('div');
          pm.className = 'pencil-marks';
          for (let d = 1; d <= 9; d++) {
            const span = document.createElement('span');
            span.className = `pm pm-${d}`;
            pm.appendChild(span);
          }
          td.appendChild(pm);

          // Value display — hidden until a digit is placed
          const val = document.createElement('span');
          val.className = 'cell-value';
          td.appendChild(val);

          td.addEventListener('click', () => this._selectCell(id, row, col, td));
          td.addEventListener('keydown', e => {
            if (e.key === 'Enter' || e.key === ' ') {
              e.preventDefault();
              if (this.selected !== id) this._selectCell(id, row, col, td);
            }
          });
        } else {
          td.classList.add('given');
          td.textContent = this.solution.get(id);
        }

        this.cells.set(id, td);
        tr.appendChild(td);
      }

      table.appendChild(tr);
    }

    this.container.appendChild(table);
  }

  _bindKeyboard() {
    document.addEventListener('keydown', (e) => {
      if (e.ctrlKey || e.altKey || e.metaKey || /INPUT|TEXTAREA|SELECT/.test(e.target.tagName)) return;
      if (!this.selected) return;
      const m = this.selected.match(/sudoku\.r(\d+)\.c(\d+)/);
      if (!m) return;
      this._handleKey(e, parseInt(m[1]), parseInt(m[2]));
    });
  }

  _selectCell(id, row, col, td) {
    if (!td.classList.contains('empty')) return;
    // Toggle: clicking the same cell deselects it
    if (this.selected === id) {
      td.classList.remove('selected');
      this.selected = null;
      this.onCellDeselected?.();
      return;
    }

    if (this.selected && this.cells.has(this.selected)) {
      this.cells.get(this.selected).classList.remove('selected');
    }
    this.selected = id;
    td.classList.add('selected');
    td.focus();
    this.onCellSelected?.(row, col);
  }

  _handleKey(e, row, col) {
    const digit = parseInt(e.key);
    if (digit >= 1 && digit <= 9) {
      e.preventDefault();
      if (this.pencilMode) {
        this._togglePencilMark(row, col, digit);
      } else {
        this.onDigitPlaced?.(row, col, digit);
      }
      return;
    }
    // Backspace/Delete clears an incorrect attempt
    if (e.key === 'Backspace' || e.key === 'Delete') {
      e.preventDefault();
      this.clearCell(row, col);
    }
  }

  /**
   * Show a value in the cell — may be correct or incorrect.
   */
  showValue(row, col, value, correct) {
    const id = `sudoku.r${row}.c${col}`;
    const td = this.cells.get(id);
    if (!td) return;

    // Hide pencil marks, show placed value
    const pm = td.querySelector('.pencil-marks');
    const val = td.querySelector('.cell-value');
    if (pm) pm.style.display = 'none';
    if (val) { val.textContent = value; val.style.display = 'flex'; }
    else td.textContent = value; // fallback for given cells

    td.classList.remove('error', 'placed', 'wrong');
    td.classList.add(correct ? 'placed' : 'wrong');
    if (correct) {
      td.classList.remove('empty');
      td.classList.remove('selected');
      this.manualMarks.delete(id);
      this.computedMarks.delete(id);
      td.tabIndex = -1;
      if (this.selected === id) this.selected = null;
    }
  }

  /**
   * Clear a cell back to empty (for correcting wrong answers).
   */
  clearCell(row, col) {
    const id = `sudoku.r${row}.c${col}`;
    const td = this.cells.get(id);
    if (!td || !td.classList.contains('wrong')) return;

    // Restore pencil marks, hide placed value
    const pm = td.querySelector('.pencil-marks');
    const val = td.querySelector('.cell-value');
    if (val) { val.textContent = ''; val.style.display = 'none'; }
    if (pm) pm.style.display = '';

    td.classList.remove('wrong');
    td.classList.add('empty');
    this.onCellCleared?.(row, col);
    return true;
  }

  /**
   * Toggle a pencil mark on/off for a cell.
   * Manual marks override auto-computed candidates.
   */
  _togglePencilMark(row, col, digit) {
    const id = `sudoku.r${row}.c${col}`;
    const td = this.cells.get(id);
    if (!td || !td.classList.contains('empty') || td.classList.contains('wrong')) return;

    if (!this.manualMarks.has(id)) {
      this.manualMarks.set(id, new Set());
    }
    const marks = this.manualMarks.get(id);

    const pm = td.querySelector('.pencil-marks');
    if (!pm) return;
    const span = pm.children[digit - 1];

    if (marks.has(digit)) {
      // Toggle off
      marks.delete(digit);
      span.textContent = '';
      span.classList.remove('pm-active', 'pm-manual');
    } else {
      // Toggle on
      marks.add(digit);
      span.textContent = digit;
      span.classList.add('pm-active', 'pm-manual');
    }
    this.onPencilMarkChanged?.();
  }

  renderMarks() {
    for (const [id, td] of this.cells) {
      const pm = td.querySelector('.pencil-marks');
      if (!pm || !td.classList.contains('empty')) continue;
      const manual = this.manualMarks.has(id);
      const marks = this.manualMarks.get(id) ?? this.computedMarks.get(id) ?? new Set();
      for (let digit=1; digit<=9; digit++) {
        const span=pm.children[digit-1];
        span.textContent=marks.has(digit) ? digit : '';
        span.classList.toggle('pm-active',marks.has(digit));
        span.classList.toggle('pm-manual',manual && marks.has(digit));
      }
    }
  }

  /**
   * Highlight cells that conflict with a placement.
   */
  highlightConflicts(conflictCells) {
    this._clearClass('conflict-highlight');
    for (const { row, col } of conflictCells) {
      const id = `sudoku.r${row}.c${col}`;
      const td = this.cells.get(id);
      if (td) td.classList.add('conflict-highlight');
    }
  }

  /**
   * Highlight peers notified by a correct placement (from cascade consume).
   */
  highlightPeers(cellIds) {
    this._clearClass('peer-highlight');
    for (const id of cellIds) {
      const td = this.cells.get(id);
      if (td) td.classList.add('peer-highlight');
    }
  }

  /**
   * Highlight strategy pattern cells (the "produce" role — amber).
   * These are the cells that form the discovered pattern.
   */
  highlightStrategy(cells) {
    this._clearClass('strategy-highlight');
    for (const { row, col } of cells) {
      const id = `sudoku.r${row}.c${col}`;
      const td = this.cells.get(id);
      if (td) td.classList.add('strategy-highlight');
    }
  }

  /**
   * Highlight elimination target cells (the "consume" role — red dashed).
   * These are the cells where candidates get eliminated by the strategy.
   */
  highlightEliminations(cells) {
    this._clearClass('elimination-highlight');
    for (const { row, col } of cells) {
      const id = `sudoku.r${row}.c${col}`;
      const td = this.cells.get(id);
      if (td) td.classList.add('elimination-highlight');
    }
  }

  /**
   * Bulk-fill pencil marks from computed candidates (opt-in cheat).
   * Populates manualMarks so the player can then adjust individually.
   * candidateMap: Map<cellId, number[]>
   */
  updatePencilMarks(candidateMap) {
    for (const [id, td] of this.cells) {
      const pm = td.querySelector('.pencil-marks');
      if (!pm || pm.style.display === 'none') continue;

      const cands = candidateMap.get(id) || [];
      this.manualMarks.set(id, new Set(cands));

      for (let d = 1; d <= 9; d++) {
        const span = pm.children[d - 1];
        if (cands.includes(d)) {
          span.textContent = d;
          span.classList.add('pm-active');
        } else {
          span.textContent = '';
          span.classList.remove('pm-active');
        }
      }
    }
    this.onPencilMarkChanged?.();
  }

  /**
   * Show the 3D crosshatch — row, column, and box constraint bands.
   * The selected cell sits at the intersection of all three.
   */
  highlightCrosshatch(row, col) {
    const br = Math.floor((row - 1) / 3) * 3 + 1;
    const bc = Math.floor((col - 1) / 3) * 3 + 1;

    for (let c = 1; c <= 9; c++) {
      if (c !== col) this.cells.get(`sudoku.r${row}.c${c}`)?.classList.add('crosshatch-row');
    }
    for (let r = 1; r <= 9; r++) {
      if (r !== row) this.cells.get(`sudoku.r${r}.c${col}`)?.classList.add('crosshatch-col');
    }
    for (let r = br; r < br + 3; r++) {
      for (let c = bc; c < bc + 3; c++) {
        if (r !== row || c !== col) {
          this.cells.get(`sudoku.r${r}.c${c}`)?.classList.add('crosshatch-box');
        }
      }
    }
  }

  clearHighlights() {
    this._clearClass('peer-highlight');
    this._clearClass('conflict-highlight');
    this._clearClass('strategy-highlight');
    this._clearClass('elimination-highlight');
    this._clearClass('crosshatch-row');
    this._clearClass('crosshatch-col');
    this._clearClass('crosshatch-box');
  }

  _clearClass(cls) {
    for (const td of this.cells.values()) {
      td.classList.remove(cls);
    }
  }
}
