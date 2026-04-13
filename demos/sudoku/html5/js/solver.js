/**
 * solver.js — Sudoku reasoning engine for the browser.
 *
 * This module teaches. Every function explains its reasoning so the
 * player learns the strategy, not just the answer.
 *
 * Strategies are presented in learning order:
 *   1. Naked single    — only one candidate left in a cell
 *   2. Hidden single   — only one cell in a group can hold a value
 *   3. Pointing pair   — candidates in a box confined to one row/col
 *   4. Naked pair      — two cells share the same two candidates
 *   5. X-Wing          — rectangle pattern eliminates candidates
 *   6. Box/line reduce — candidates in a row/col confined to one box
 *   7. Swordfish        — 3-row/col extension of X-Wing
 *
 * The independence principle: once you learn these strategies,
 * you don't need this module. You see the constraints yourself.
 */

/**
 * Compute which 3x3 box a cell belongs to (1-indexed).
 */
export function boxOf(row, col) {
  return Math.floor((row - 1) / 3) * 3 + Math.floor((col - 1) / 3) + 1;
}

/**
 * Get all peer cells of (row, col) — same row, column, or box, excluding self.
 * Returns array of { row, col } objects.
 */
export function peers(row, col) {
  const seen = new Set();
  const result = [];
  const add = (r, c) => {
    const key = `${r},${c}`;
    if ((r !== row || c !== col) && !seen.has(key)) {
      seen.add(key);
      result.push({ row: r, col: c });
    }
  };

  // Row peers
  for (let c = 1; c <= 9; c++) add(row, c);
  // Column peers
  for (let r = 1; r <= 9; r++) add(r, col);
  // Box peers
  const br = Math.floor((row - 1) / 3) * 3 + 1;
  const bc = Math.floor((col - 1) / 3) * 3 + 1;
  for (let r = br; r < br + 3; r++)
    for (let c = bc; c < bc + 3; c++) add(r, c);

  return result;
}

/**
 * Given the current grid state, compute candidates for a cell.
 * grid is a 9x9 array (grid[row-1][col-1], 0 = empty).
 * Returns array of possible values (1-9).
 */
export function candidates(grid, row, col) {
  if (grid[row - 1][col - 1] !== 0) return [];
  const taken = new Set();
  for (const p of peers(row, col)) {
    const v = grid[p.row - 1][p.col - 1];
    if (v !== 0) taken.add(v);
  }
  return [1,2,3,4,5,6,7,8,9].filter(v => !taken.has(v));
}

/**
 * Explain why a value CANNOT go in a cell.
 * Returns an array of conflict explanations, each with:
 *   { type: 'row'|'col'|'box', value, conflictCell: {row, col}, message }
 * Empty array means the value is valid by Sudoku rules.
 */
export function explainConflicts(grid, row, col, value) {
  const conflicts = [];

  // Check row
  for (let c = 1; c <= 9; c++) {
    if (c !== col && grid[row - 1][c - 1] === value) {
      conflicts.push({
        type: 'row',
        value,
        conflictCell: { row, col: c },
        message: `Row ${row} already has ${value} at column ${c}.`,
        law: `Sudoku Law: each row contains exactly one of each digit 1-9.`,
      });
    }
  }

  // Check column
  for (let r = 1; r <= 9; r++) {
    if (r !== row && grid[r - 1][col - 1] === value) {
      conflicts.push({
        type: 'col',
        value,
        conflictCell: { row: r, col },
        message: `Column ${col} already has ${value} at row ${r}.`,
        law: `Sudoku Law: each column contains exactly one of each digit 1-9.`,
      });
    }
  }

  // Check box
  const br = Math.floor((row - 1) / 3) * 3 + 1;
  const bc = Math.floor((col - 1) / 3) * 3 + 1;
  const box = boxOf(row, col);
  for (let r = br; r < br + 3; r++) {
    for (let c = bc; c < bc + 3; c++) {
      if ((r !== row || c !== col) && grid[r - 1][c - 1] === value) {
        conflicts.push({
          type: 'box',
          value,
          conflictCell: { row: r, col: c },
          message: `Box ${box} already has ${value} at r${r}.c${c}.`,
          law: `Sudoku Law: each 3x3 box contains exactly one of each digit 1-9.`,
        });
      }
    }
  }

  return conflicts;
}

// ── Strategy detection ─────────────────────────────────────────────

/**
 * Detect if a value is a Hidden Single for a cell.
 * A hidden single: value can only go in ONE cell within a row, col, or box.
 * Returns { found: true, group, groupNum, value } or { found: false }.
 */
export function findHiddenSingle(grid, row, col) {
  const cands = candidates(grid, row, col);
  if (cands.length <= 1) return { found: false };

  for (const val of cands) {
    // Check row: can val go anywhere else in this row?
    let rowCount = 0;
    for (let c = 1; c <= 9; c++) {
      if (grid[row - 1][c - 1] === 0 && candidates(grid, row, c).includes(val)) rowCount++;
    }
    if (rowCount === 1) return { found: true, group: 'row', groupNum: row, value: val };

    // Check column
    let colCount = 0;
    for (let r = 1; r <= 9; r++) {
      if (grid[r - 1][col - 1] === 0 && candidates(grid, r, col).includes(val)) colCount++;
    }
    if (colCount === 1) return { found: true, group: 'column', groupNum: col, value: val };

    // Check box
    const br = Math.floor((row - 1) / 3) * 3 + 1;
    const bc = Math.floor((col - 1) / 3) * 3 + 1;
    let boxCount = 0;
    for (let r = br; r < br + 3; r++) {
      for (let c = bc; c < bc + 3; c++) {
        if (grid[r - 1][c - 1] === 0 && candidates(grid, r, c).includes(val)) boxCount++;
      }
    }
    if (boxCount === 1) return { found: true, group: 'box', groupNum: boxOf(row, col), value: val };
  }

  return { found: false };
}

/**
 * Detect Pointing Pairs/Triples relevant to this cell.
 * When a candidate in a box is confined to one row or column,
 * it can be eliminated from that row/column outside the box.
 * Returns { found, value, boxNum, line, lineType, eliminates[] } or { found: false }.
 */
export function findPointingPair(grid, row, col) {
  const cands = candidates(grid, row, col);
  if (cands.length <= 1) return { found: false };

  const box = boxOf(row, col);
  const br = Math.floor((row - 1) / 3) * 3 + 1;
  const bc = Math.floor((col - 1) / 3) * 3 + 1;

  for (const val of cands) {
    // Find all cells in this box that can hold val
    const boxCells = [];
    for (let r = br; r < br + 3; r++) {
      for (let c = bc; c < bc + 3; c++) {
        if (grid[r - 1][c - 1] === 0 && candidates(grid, r, c).includes(val)) {
          boxCells.push({ row: r, col: c });
        }
      }
    }

    if (boxCells.length < 2) continue;

    // All in same row?
    const allSameRow = boxCells.every(c => c.row === boxCells[0].row);
    if (allSameRow) {
      const lineRow = boxCells[0].row;
      const eliminates = [];
      for (let c = 1; c <= 9; c++) {
        if (c >= bc && c < bc + 3) continue; // skip cells in this box
        if (grid[lineRow - 1][c - 1] === 0 && candidates(grid, lineRow, c).includes(val)) {
          eliminates.push({ row: lineRow, col: c });
        }
      }
      if (eliminates.length > 0) {
        return { found: true, value: val, boxNum: box, line: lineRow, lineType: 'row', cells: boxCells, eliminates };
      }
    }

    // All in same column?
    const allSameCol = boxCells.every(c => c.col === boxCells[0].col);
    if (allSameCol) {
      const lineCol = boxCells[0].col;
      const eliminates = [];
      for (let r = 1; r <= 9; r++) {
        if (r >= br && r < br + 3) continue;
        if (grid[r - 1][lineCol - 1] === 0 && candidates(grid, r, lineCol).includes(val)) {
          eliminates.push({ row: r, col: lineCol });
        }
      }
      if (eliminates.length > 0) {
        return { found: true, value: val, boxNum: box, line: lineCol, lineType: 'column', cells: boxCells, eliminates };
      }
    }
  }

  return { found: false };
}

/**
 * Detect Naked Pairs relevant to this cell.
 * Two cells in a group share the same two candidates — those values
 * are locked to those cells and eliminated from other peers.
 * Returns { found, pair, values, group, groupNum, eliminates[] } or { found: false }.
 */
export function findNakedPair(grid, row, col) {
  const cands = candidates(grid, row, col);
  if (cands.length !== 2) return { found: false };

  const groups = [
    { type: 'row', cells: rowCells(row) },
    { type: 'column', cells: colCells(col) },
    { type: 'box', cells: boxCells(row, col) },
  ];

  for (const { type, cells } of groups) {
    for (const cell of cells) {
      if (cell.row === row && cell.col === col) continue;
      const peerCands = candidates(grid, cell.row, cell.col);
      if (peerCands.length === 2 && peerCands[0] === cands[0] && peerCands[1] === cands[1]) {
        // Found a naked pair — check if it eliminates anything
        const eliminates = [];
        for (const other of cells) {
          if ((other.row === row && other.col === col) || (other.row === cell.row && other.col === cell.col)) continue;
          const otherCands = candidates(grid, other.row, other.col);
          if (otherCands.some(v => cands.includes(v))) {
            eliminates.push({ row: other.row, col: other.col, values: otherCands.filter(v => cands.includes(v)) });
          }
        }
        if (eliminates.length > 0) {
          const num = type === 'row' ? row : type === 'column' ? col : boxOf(row, col);
          return { found: true, pair: [{ row, col }, cell], values: cands, group: type, groupNum: num, eliminates };
        }
      }
    }
  }

  return { found: false };
}

/**
 * Detect X-Wing pattern relevant to this cell.
 * A candidate appears in exactly 2 cells in 2 different rows,
 * and those cells share the same 2 columns — eliminates from those columns.
 * Returns { found, value, rows, cols, eliminates[] } or { found: false }.
 */
export function findXWing(grid, row, col) {
  const cands = candidates(grid, row, col);

  for (const val of cands) {
    // Row-based X-Wing: find rows where val appears in exactly 2 columns
    const rowPairs = [];
    for (let r = 1; r <= 9; r++) {
      const cols = [];
      for (let c = 1; c <= 9; c++) {
        if (grid[r - 1][c - 1] === 0 && candidates(grid, r, c).includes(val)) {
          cols.push(c);
        }
      }
      if (cols.length === 2) rowPairs.push({ row: r, cols });
    }

    // Look for two rows with the same column pair
    for (let i = 0; i < rowPairs.length; i++) {
      for (let j = i + 1; j < rowPairs.length; j++) {
        if (rowPairs[i].cols[0] === rowPairs[j].cols[0] && rowPairs[i].cols[1] === rowPairs[j].cols[1]) {
          // Check if this X-Wing is relevant to our cell (cell is in one of the columns)
          const [c1, c2] = rowPairs[i].cols;
          const r1 = rowPairs[i].row, r2 = rowPairs[j].row;

          if (col !== c1 && col !== c2) continue;

          // Eliminates val from the two columns in other rows
          const eliminates = [];
          for (const cc of [c1, c2]) {
            for (let r = 1; r <= 9; r++) {
              if (r === r1 || r === r2) continue;
              if (grid[r - 1][cc - 1] === 0 && candidates(grid, r, cc).includes(val)) {
                eliminates.push({ row: r, col: cc });
              }
            }
          }

          if (eliminates.length > 0) {
            return { found: true, value: val, rows: [r1, r2], cols: [c1, c2], eliminates };
          }
        }
      }
    }
  }

  return { found: false };
}

/**
 * Detect Box/Line Reduction relevant to this cell.
 * When a candidate in a row/col is confined to one box,
 * it can be eliminated from that box outside the row/col.
 * (Reverse of Pointing Pair — the line "reduces" the box.)
 * Returns { found, value, lineType, lineNum, boxNum, cells[], eliminates[] } or { found: false }.
 */
export function findBoxLineReduction(grid, row, col) {
  const cands = candidates(grid, row, col);
  if (cands.length <= 1) return { found: false };

  const box = boxOf(row, col);
  const br = Math.floor((row - 1) / 3) * 3 + 1;
  const bc = Math.floor((col - 1) / 3) * 3 + 1;

  for (const val of cands) {
    // Check row: where can val go in this row?
    const rowPositions = [];
    for (let c = 1; c <= 9; c++) {
      if (grid[row - 1][c - 1] === 0 && candidates(grid, row, c).includes(val)) {
        rowPositions.push({ row, col: c });
      }
    }
    // All in same box?
    if (rowPositions.length >= 2 && rowPositions.every(p => boxOf(p.row, p.col) === box)) {
      const eliminates = [];
      for (let r = br; r < br + 3; r++) {
        for (let c = bc; c < bc + 3; c++) {
          if (r === row) continue; // skip cells in the row itself
          if (grid[r - 1][c - 1] === 0 && candidates(grid, r, c).includes(val)) {
            eliminates.push({ row: r, col: c });
          }
        }
      }
      if (eliminates.length > 0) {
        return { found: true, value: val, lineType: 'row', lineNum: row, boxNum: box, cells: rowPositions, eliminates };
      }
    }

    // Check column: where can val go in this column?
    const colPositions = [];
    for (let r = 1; r <= 9; r++) {
      if (grid[r - 1][col - 1] === 0 && candidates(grid, r, col).includes(val)) {
        colPositions.push({ row: r, col });
      }
    }
    if (colPositions.length >= 2 && colPositions.every(p => boxOf(p.row, p.col) === box)) {
      const eliminates = [];
      for (let r = br; r < br + 3; r++) {
        for (let c = bc; c < bc + 3; c++) {
          if (c === col) continue;
          if (grid[r - 1][c - 1] === 0 && candidates(grid, r, c).includes(val)) {
            eliminates.push({ row: r, col: c });
          }
        }
      }
      if (eliminates.length > 0) {
        return { found: true, value: val, lineType: 'column', lineNum: col, boxNum: box, cells: colPositions, eliminates };
      }
    }
  }

  return { found: false };
}

/**
 * Detect Swordfish pattern relevant to this cell.
 * A 3-row/col extension of X-Wing: a candidate appears in exactly 2-3 cells
 * in 3 rows, and those cells share the same 3 columns. Eliminates from those
 * columns in all other rows (and vice versa for column-based Swordfish).
 * Returns { found, value, rows, cols, cells[], eliminates[] } or { found: false }.
 */
export function findSwordfish(grid, row, col) {
  const cands = candidates(grid, row, col);

  for (const val of cands) {
    // Row-based Swordfish: find rows where val appears in 2-3 columns
    const rowData = [];
    for (let r = 1; r <= 9; r++) {
      const cols = [];
      for (let c = 1; c <= 9; c++) {
        if (grid[r - 1][c - 1] === 0 && candidates(grid, r, c).includes(val)) {
          cols.push(c);
        }
      }
      if (cols.length >= 2 && cols.length <= 3) rowData.push({ row: r, cols });
    }

    // Try all combinations of 3 rows
    for (let i = 0; i < rowData.length; i++) {
      for (let j = i + 1; j < rowData.length; j++) {
        for (let k = j + 1; k < rowData.length; k++) {
          const unionCols = new Set([...rowData[i].cols, ...rowData[j].cols, ...rowData[k].cols]);
          if (unionCols.size !== 3) continue;

          const sfRows = [rowData[i].row, rowData[j].row, rowData[k].row];
          const sfCols = [...unionCols].sort((a, b) => a - b);

          // Is this cell in one of the swordfish columns?
          if (!sfCols.includes(col)) continue;

          // Collect the swordfish cells and elimination targets
          const cells = [];
          for (const rd of [rowData[i], rowData[j], rowData[k]]) {
            for (const c of rd.cols) cells.push({ row: rd.row, col: c });
          }

          const eliminates = [];
          for (const c of sfCols) {
            for (let r = 1; r <= 9; r++) {
              if (sfRows.includes(r)) continue;
              if (grid[r - 1][c - 1] === 0 && candidates(grid, r, c).includes(val)) {
                eliminates.push({ row: r, col: c });
              }
            }
          }

          if (eliminates.length > 0) {
            return { found: true, value: val, rows: sfRows, cols: sfCols, cells, eliminates };
          }
        }
      }
    }
  }

  return { found: false };
}

/** Helper: get all empty cells in a row */
function rowCells(row) {
  const cells = [];
  for (let c = 1; c <= 9; c++) cells.push({ row, col: c });
  return cells;
}

/** Helper: get all empty cells in a column */
function colCells(col) {
  const cells = [];
  for (let r = 1; r <= 9; r++) cells.push({ row: r, col });
  return cells;
}

/** Helper: get all cells in a box */
function boxCells(row, col) {
  const cells = [];
  const br = Math.floor((row - 1) / 3) * 3 + 1;
  const bc = Math.floor((col - 1) / 3) * 3 + 1;
  for (let r = br; r < br + 3; r++)
    for (let c = bc; c < bc + 3; c++) cells.push({ row: r, col: c });
  return cells;
}

// ── Progressive hint system ───────────────────────────────────────

function cellIdentifier(row, col) {
  return `sudoku.r${row}.c${col}`;
}

function bestFocusGroup(grid, row, col) {
  const rowFilled = countFilled(grid, 'row', row);
  const colFilled = countFilled(grid, 'col', col);
  const boxFilled = countFilled(grid, 'box', row, col);
  const boxNum = boxOf(row, col);

  const groups = [
    { type: 'row', groupNum: row, filled: rowFilled, label: `row ${row}` },
    { type: 'column', groupNum: col, filled: colFilled, label: `column ${col}` },
    { type: 'box', groupNum: boxNum, filled: boxFilled, label: `box ${boxNum}` },
  ];

  groups.sort((a, b) => b.filled - a.filled || a.label.localeCompare(b.label));
  return {
    ...groups[0],
    pressure: rowFilled + colFilled + boxFilled,
    rowFilled,
    colFilled,
    boxFilled,
  };
}

function compareEyeballEntries(a, b) {
  return (
    a.priority - b.priority ||
    a.candidateCount - b.candidateCount ||
    b.impact - a.impact ||
    b.pressure - a.pressure ||
    a.row - b.row ||
    a.col - b.col
  );
}

function buildTechniqueEntry(grid, row, col) {
  const id = cellIdentifier(row, col);
  const cands = candidates(grid, row, col);
  if (cands.length === 0) return null;

  const focus = bestFocusGroup(grid, row, col);

  if (cands.length === 1) {
    return {
      kind: 'naked-single',
      signature: `naked-single:${id}`,
      priority: 0,
      candidateCount: cands.length,
      impact: 9,
      pressure: focus.pressure,
      row,
      col,
      id,
      title: 'Naked Single',
      focusLabel: `Scan ${focus.label}, then return to ${id}.`,
      preview: 'Only one candidate survives in this cell after elimination.',
      whyThisFirst: `${id} is already reduced to one legal value, so it is the fastest stable move on the board.`,
      whatToNotice: `Check the row, column, and box around ${id}. Every digit except one is already blocked.`,
      whyNotElsewhere: 'Other unsolved cells still have multiple candidates or depend on a larger shared pattern.',
      nextEscalation: 'If it still feels hazy, use the first two hint levels to see the eliminations without revealing the answer.',
    };
  }

  const hs = findHiddenSingle(grid, row, col);
  if (hs.found) {
    const groupLabel = hs.group === 'column' ? `column ${hs.groupNum}` : `${hs.group} ${hs.groupNum}`;
    return {
      kind: 'hidden-single',
      signature: `hidden-single:${hs.group}:${hs.groupNum}:${hs.value}`,
      priority: 1,
      candidateCount: cands.length,
      impact: 8,
      pressure: focus.pressure,
      row,
      col,
      id,
      title: 'Hidden Single',
      focusLabel: `Scan ${groupLabel}, then check ${id}.`,
      preview: `One missing digit only fits in this cell within ${groupLabel}.`,
      whyThisFirst: `${groupLabel} has a bottleneck: one digit has only one legal landing spot, and ${id} is that spot.`,
      whatToNotice: `List the missing digits in ${groupLabel} and see which one has nowhere else to go.`,
      whyNotElsewhere: `Cells outside ${groupLabel} are not forced by a single-group bottleneck yet.`,
      nextEscalation: 'Press H for the group-based hint if you want the pattern highlighted without jumping straight to the final digit.',
    };
  }

  const pp = findPointingPair(grid, row, col);
  if (pp.found) {
    return {
      kind: 'pointing-pair',
      signature: `pointing-pair:${pp.boxNum}:${pp.lineType}:${pp.line}:${pp.value}`,
      priority: 2,
      candidateCount: cands.length,
      impact: pp.eliminates.length + pp.cells.length,
      pressure: focus.pressure,
      row,
      col,
      id,
      title: 'Pointing Pair',
      focusLabel: `Look inside box ${pp.boxNum}, then follow ${pp.lineType} ${pp.line}.`,
      preview: `A box pattern is squeezing one digit into a single ${pp.lineType}.`,
      whyThisFirst: `When a box confines a digit to one ${pp.lineType}, the rest of that ${pp.lineType} becomes easier to clean up.`,
      whatToNotice: `Inside box ${pp.boxNum}, find the candidate cells that line up on the same ${pp.lineType}.`,
      whyNotElsewhere: 'Other areas do not currently create as many immediate eliminations from one clean pattern.',
      nextEscalation: 'Use a strategy hint if you want the pattern cells highlighted while still hiding the final placement.',
    };
  }

  const blr = findBoxLineReduction(grid, row, col);
  if (blr.found) {
    return {
      kind: 'box-line-reduction',
      signature: `box-line-reduction:${blr.boxNum}:${blr.lineType}:${blr.lineNum}:${blr.value}`,
      priority: 3,
      candidateCount: cands.length,
      impact: blr.eliminates.length + blr.cells.length,
      pressure: focus.pressure,
      row,
      col,
      id,
      title: 'Box/Line Reduction',
      focusLabel: `Scan ${blr.lineType} ${blr.lineNum}, then compare it against box ${blr.boxNum}.`,
      preview: 'A row-or-column pattern is squeezing a box from the outside.',
      whyThisFirst: `This line already limits a digit to one box, so the rest of that box can be pruned quickly.`,
      whatToNotice: `Track one candidate across ${blr.lineType} ${blr.lineNum} and see that all of its legal spots stay inside box ${blr.boxNum}.`,
      whyNotElsewhere: 'Other groups are not offering the same outside-in reduction yet.',
      nextEscalation: 'If the reduction is still slippery, press H for the highlighted strategy cells before asking for the answer.',
    };
  }

  const np = findNakedPair(grid, row, col);
  if (np.found) {
    return {
      kind: 'naked-pair',
      signature: `naked-pair:${np.group}:${np.groupNum}:${np.values.join(',')}`,
      priority: 4,
      candidateCount: cands.length,
      impact: np.eliminates.length + np.pair.length,
      pressure: focus.pressure,
      row,
      col,
      id,
      title: 'Naked Pair',
      focusLabel: `Scan ${np.group} ${np.groupNum} for a shared two-cell pattern.`,
      preview: 'Two cells share the same pair, which squeezes the rest of the group.',
      whyThisFirst: `A locked pair removes ambiguity from several peers at once, making it a strong training pattern.`,
      whatToNotice: `Find two cells in ${np.group} ${np.groupNum} that show the same two-candidate set.`,
      whyNotElsewhere: `Other groups do not currently have a pair that produces as many safe eliminations.`,
      nextEscalation: 'Use a strategy hint if you want the pair highlighted without exposing the final placement.',
    };
  }

  const xw = findXWing(grid, row, col);
  if (xw.found) {
    return {
      kind: 'x-wing',
      signature: `x-wing:${xw.value}:${xw.rows.join(',')}:${xw.cols.join(',')}`,
      priority: 5,
      candidateCount: cands.length,
      impact: xw.eliminates.length + 4,
      pressure: focus.pressure,
      row,
      col,
      id,
      title: 'X-Wing',
      focusLabel: `Compare rows ${xw.rows.join(' and ')} across columns ${xw.cols.join(' and ')}.`,
      preview: 'A rectangle pattern is creating stable eliminations across two lines.',
      whyThisFirst: 'This is a rarer but very clean pattern: once the rectangle locks in, the outside columns become easier to prune.',
      whatToNotice: 'Look for two rows that place the same candidate in the same two columns.',
      whyNotElsewhere: 'Most other unsolved cells are still waiting on simpler singles or pair-based structure.',
      nextEscalation: 'If you want help seeing the rectangle, press H for the strategy overlay before revealing anything final.',
    };
  }

  const sf = findSwordfish(grid, row, col);
  if (sf.found) {
    return {
      kind: 'swordfish',
      signature: `swordfish:${sf.value}:${sf.rows.join(',')}:${sf.cols.join(',')}`,
      priority: 6,
      candidateCount: cands.length,
      impact: sf.eliminates.length + sf.cells.length,
      pressure: focus.pressure,
      row,
      col,
      id,
      title: 'Swordfish',
      focusLabel: `Track three rows against columns ${sf.cols.join(', ')}.`,
      preview: 'A larger line pattern is starting to lock one candidate into a three-by-three sweep.',
      whyThisFirst: 'This is a high-value pattern when the simpler techniques are exhausted, because it clears many cells at once.',
      whatToNotice: 'Find three rows whose candidate positions compress into the same three columns.',
      whyNotElsewhere: 'Nothing simpler is currently resolving this branch of the puzzle cleanly.',
      nextEscalation: 'Use the strategy hint to highlight the sweep if you want help seeing the structure before the answer.',
    };
  }

  const candidatePreview = cands.length === 2
    ? 'Two candidates remain here, making it one of the tighter cells on the board.'
    : `${cands.length} candidates remain here, but the surrounding groups are already highly constrained.`;

  return {
    kind: 'constrained-scan',
    signature: `constrained-scan:${id}`,
    priority: 7,
    candidateCount: cands.length,
    impact: 1,
    pressure: focus.pressure,
    row,
    col,
    id,
    title: 'Constrained Scan',
    focusLabel: `Start with ${focus.label}, then inspect ${id}.`,
    preview: candidatePreview,
    whyThisFirst: `${focus.label} is already dense with information, so this cell is under stronger pressure than most of the board.`,
    whatToNotice: `Count the missing digits in ${focus.label} and compare them against this cell's row, column, and box.`,
    whyNotElsewhere: 'Other cells currently have looser candidate sets or would require a deeper chain to justify a move.',
    nextEscalation: 'Use the early hint levels for elimination and candidates if you want a nudge without surrendering the answer.',
  };
}

export function eyeballOrder(grid, selectedRow = null, selectedCol = null, limit = 3) {
  const selectedId = Number.isInteger(selectedRow) && Number.isInteger(selectedCol)
    ? cellIdentifier(selectedRow, selectedCol)
    : null;

  const rawEntries = [];
  for (let row = 1; row <= 9; row++) {
    for (let col = 1; col <= 9; col++) {
      if (grid[row - 1][col - 1] !== 0) continue;
      const entry = buildTechniqueEntry(grid, row, col);
      if (entry) rawEntries.push(entry);
    }
  }

  rawEntries.sort(compareEyeballEntries);

  const bySignature = new Map();
  for (const entry of rawEntries) {
    const existing = bySignature.get(entry.signature);
    if (!existing) {
      bySignature.set(entry.signature, entry);
      continue;
    }
    if (entry.id === selectedId && existing.id !== selectedId) {
      bySignature.set(entry.signature, entry);
    }
  }

  const uniqueEntries = [...bySignature.values()].sort(compareEyeballEntries);
  const items = uniqueEntries.slice(0, limit).map((entry, index) => ({
    ...entry,
    rank: index + 1,
    selected: entry.id === selectedId,
  }));

  const selectedSummary = selectedId
    ? uniqueEntries.find(entry => entry.id === selectedId) ?? rawEntries.find(entry => entry.id === selectedId) ?? null
    : null;
  const selectedRank = selectedSummary
    ? uniqueEntries.findIndex(entry => entry.id === selectedSummary.id) + 1
    : null;

  return {
    items,
    selectedId,
    selectedRank: selectedRank > 0 ? selectedRank : null,
    selectedInTop: items.some(item => item.id === selectedId),
    selectedSummary: selectedSummary
      ? {
          ...selectedSummary,
          rank: selectedRank > 0 ? selectedRank : null,
        }
      : null,
  };
}

/**
 * Generate a progressive hint for a cell.
 * Each call with a higher `level` reveals more:
 *   0 — which group (row/col/box) is most constrained (Nudge)
 *   1 — elimination: list values ruled out and why
 *   2 — candidates remaining + naked single check
 *   3 — advanced strategy (hidden single, pointing pair, naked pair, X-Wing,
 *       box/line reduction, swordfish)
 *   4 — the answer with full reasoning chain
 *
 * Returns { level, title, lines[], strategy, cell: {row, col}, overlayCells }
 *
 * overlayCells maps to eventness roles:
 *   strategy[]   — pattern source cells (produce/publish — amber)
 *   eliminates[] — affected peer cells (consume/subscribe — red outline)
 *   peers[]      — generic constraint peers (level 0-1 fallback)
 */
export function hint(grid, solution, row, col, level = 0) {
  const id = `sudoku.r${row}.c${col}`;
  const answer = solution.get(id);
  const cands = candidates(grid, row, col);
  const eyeball = eyeballOrder(grid, row, col);
  const withEyeball = payload => ({ ...payload, eyeball });

  if (grid[row - 1][col - 1] !== 0) {
    return withEyeball({ level: -1, title: 'Already filled', lines: ['This cell already has a value.'], strategy: null, cell: { row, col }, overlayCells: null });
  }

  // Compute generic peers for level 0-1 overlays
  const genericPeers = _computeGenericPeers(grid, row, col);

  if (level === 0) {
    // Hint 0: Nudge — point to the most constrained group
    const rowFilled = countFilled(grid, 'row', row);
    const colFilled = countFilled(grid, 'col', col);
    const boxFilled = countFilled(grid, 'box', row, col);

    let best, filled, bestPeers;
    if (rowFilled >= colFilled && rowFilled >= boxFilled) {
      best = `row ${row}`;
      filled = rowFilled;
      bestPeers = genericPeers.filledRow;
    } else if (colFilled >= boxFilled) {
      best = `column ${col}`;
      filled = colFilled;
      bestPeers = genericPeers.filledCol;
    } else {
      best = `box ${boxOf(row, col)}`;
      filled = boxFilled;
      bestPeers = genericPeers.filledBox;
    }

    return withEyeball({
      level: 0,
      title: 'Look at the constraints',
      lines: [
        `Focus on ${best} — it already has ${filled} of 9 values filled.`,
        `What values are missing?`,
      ],
      strategy: 'Scanning — look at the most filled group first.',
      cell: { row, col },
      overlayCells: { strategy: bestPeers, eliminates: [], peers: genericPeers.allFilled },
    });
  }

  if (level === 1) {
    // Hint 1: Elimination — show which values are ruled out
    const eliminated = [];
    const conflictCells = [];

    for (let v = 1; v <= 9; v++) {
      if (cands.includes(v)) continue;
      const conflicts = explainConflicts(grid, row, col, v);
      if (conflicts.length > 0) {
        eliminated.push(`${v} is ruled out — ${conflicts[0].message}`);
        conflictCells.push(conflicts[0].conflictCell);
      }
    }

    return withEyeball({
      level: 1,
      title: 'Elimination',
      lines: [
        `For ${id}, these values are impossible:`,
        ...eliminated,
      ],
      strategy: 'Elimination — cross off values that conflict with peers in the same row, column, or box.',
      cell: { row, col },
      overlayCells: { strategy: conflictCells, eliminates: [], peers: genericPeers.allFilled },
    });
  }

  if (level === 2) {
    // Hint 2: Candidates remaining
    if (cands.length === 1) {
      return withEyeball({
        level: 2,
        title: 'Naked Single',
        lines: [
          `After elimination, ${id} can only be: ${cands[0]}`,
          `Only one candidate remains — this is a Naked Single!`,
          `When all but one value is eliminated by row, column, and box constraints,`,
          `the remaining value must go here. No further reasoning needed.`,
        ],
        strategy: 'Naked Single — the simplest strategy. When elimination leaves exactly one candidate, place it.',
        cell: { row, col },
        overlayCells: { strategy: genericPeers.allFilled, eliminates: [], peers: [] },
      });
    }

    return withEyeball({
      level: 2,
      title: 'Candidates',
      lines: [
        `After elimination, ${id} can be: ${cands.join(', ')}`,
        `${cands.length} candidates remain — basic elimination isn't enough.`,
        `Press H again for an advanced strategy to narrow it down.`,
      ],
      strategy: 'When multiple candidates remain, you need a more advanced technique. Press H to see which one applies.',
      cell: { row, col },
      overlayCells: { strategy: genericPeers.allFilled, eliminates: genericPeers.allEmpty, peers: [] },
    });
  }

  if (level === 3) {
    // Hint 3: Advanced strategy detection
    // Try strategies in order of complexity

    // 1. Hidden Single
    const hs = findHiddenSingle(grid, row, col);
    if (hs.found) {
      // Highlight: all filled cells in the decisive group (strategy), empty non-target cells (eliminates)
      const groupCells = _groupFilledCells(grid, hs.group, hs.groupNum, row, col);
      return withEyeball({
        level: 3,
        title: 'Hidden Single',
        lines: [
          `${id} has candidates: ${cands.join(', ')}`,
          `But look at ${hs.group} ${hs.groupNum}:`,
          `${hs.value} can ONLY go in this cell within ${hs.group} ${hs.groupNum}.`,
          `Even though this cell has ${cands.length} candidates, ${hs.value} has nowhere else to go.`,
          `Therefore ${id} = ${hs.value}.`,
        ],
        strategy: `Hidden Single — a value has only one possible cell in a ${hs.group}. Even if the cell has multiple candidates, this value is "hidden" as the only option for its group.`,
        cell: { row, col },
        overlayCells: { strategy: groupCells.filled, eliminates: groupCells.empty, peers: [] },
      });
    }

    // 2. Pointing Pair
    const pp = findPointingPair(grid, row, col);
    if (pp.found) {
      const cellList = pp.cells.map(c => `r${c.row}.c${c.col}`).join(', ');
      const elimList = pp.eliminates.map(c => `r${c.row}.c${c.col}`).join(', ');
      return withEyeball({
        level: 3,
        title: 'Pointing Pair',
        lines: [
          `In box ${pp.boxNum}, the value ${pp.value} can only go in ${pp.lineType} ${pp.line}:`,
          `  Cells: ${cellList}`,
          `Since ${pp.value} must be in one of these cells, it can't appear`,
          `elsewhere in ${pp.lineType} ${pp.line}.`,
          `This eliminates ${pp.value} from: ${elimList}`,
          `Which narrows candidates for this cell.`,
        ],
        strategy: `Pointing Pair — when a candidate in a box is confined to one ${pp.lineType}, it can be eliminated from that ${pp.lineType} outside the box. The box "points" to where the value must go.`,
        cell: { row, col },
        overlayCells: { strategy: pp.cells, eliminates: pp.eliminates, peers: [] },
      });
    }

    // 3. Naked Pair
    const np = findNakedPair(grid, row, col);
    if (np.found) {
      const pairCells = np.pair.map(c => `r${c.row}.c${c.col}`).join(' and ');
      const elimList = np.eliminates.map(e => `r${e.row}.c${e.col} (remove ${e.values.join(',')})`).join(', ');
      return withEyeball({
        level: 3,
        title: 'Naked Pair',
        lines: [
          `In ${np.group} ${np.groupNum}, cells ${pairCells} both have only: {${np.values.join(', ')}}`,
          `These two values are "locked" to these two cells.`,
          `No other cell in the ${np.group} can contain ${np.values.join(' or ')}.`,
          `Eliminates from: ${elimList}`,
        ],
        strategy: `Naked Pair — when two cells in a group share the same two candidates, those values are locked. Other cells in the group can eliminate both values.`,
        cell: { row, col },
        overlayCells: { strategy: np.pair, eliminates: np.eliminates.map(e => ({ row: e.row, col: e.col })), peers: [] },
      });
    }

    // 4. X-Wing
    const xw = findXWing(grid, row, col);
    if (xw.found) {
      // The 4 corners of the X-Wing rectangle
      const corners = [];
      for (const r of xw.rows) for (const c of xw.cols) corners.push({ row: r, col: c });
      return withEyeball({
        level: 3,
        title: 'X-Wing',
        lines: [
          `Value ${xw.value} forms an X-Wing pattern:`,
          `  Rows ${xw.rows.join(', ')} each have ${xw.value} in exactly columns ${xw.cols.join(', ')}.`,
          `These four cells form a rectangle. The value must occupy two diagonal corners.`,
          `Therefore ${xw.value} can be eliminated from columns ${xw.cols.join(', ')} in all other rows.`,
          `Eliminates from: ${xw.eliminates.map(c => `r${c.row}.c${c.col}`).join(', ')}`,
        ],
        strategy: `X-Wing — when a candidate appears in exactly 2 cells in 2 rows, and those cells align in the same 2 columns, the candidate is eliminated from those columns in other rows. The four cells form an "X" pattern.`,
        cell: { row, col },
        overlayCells: { strategy: corners, eliminates: xw.eliminates, peers: [] },
      });
    }

    // 5. Box/Line Reduction
    const blr = findBoxLineReduction(grid, row, col);
    if (blr.found) {
      const cellList = blr.cells.map(c => `r${c.row}.c${c.col}`).join(', ');
      const elimList = blr.eliminates.map(c => `r${c.row}.c${c.col}`).join(', ');
      return withEyeball({
        level: 3,
        title: 'Box/Line Reduction',
        lines: [
          `In ${blr.lineType} ${blr.lineNum}, the value ${blr.value} is confined to box ${blr.boxNum}:`,
          `  Cells: ${cellList}`,
          `Since ${blr.value} must be in one of these cells within the ${blr.lineType},`,
          `it can be eliminated from other cells in box ${blr.boxNum}.`,
          `Eliminates from: ${elimList}`,
        ],
        strategy: `Box/Line Reduction — when a candidate in a ${blr.lineType} is confined to one box, it can be eliminated from that box outside the ${blr.lineType}. The reverse of Pointing Pair: the line "reduces" the box.`,
        cell: { row, col },
        overlayCells: { strategy: blr.cells, eliminates: blr.eliminates, peers: [] },
      });
    }

    // 6. Swordfish
    const sf = findSwordfish(grid, row, col);
    if (sf.found) {
      return withEyeball({
        level: 3,
        title: 'Swordfish',
        lines: [
          `Value ${sf.value} forms a Swordfish pattern:`,
          `  Rows ${sf.rows.join(', ')} contain ${sf.value} only in columns ${sf.cols.join(', ')}.`,
          `These ${sf.cells.length} cells form a 3×3 grid pattern.`,
          `The value is locked to these rows/columns and can be eliminated elsewhere.`,
          `Eliminates from: ${sf.eliminates.map(c => `r${c.row}.c${c.col}`).join(', ')}`,
        ],
        strategy: `Swordfish — the 3-row extension of X-Wing. When a candidate appears in 2-3 cells across 3 rows, and those cells span exactly 3 columns, the candidate is eliminated from those columns in all other rows.`,
        cell: { row, col },
        overlayCells: { strategy: sf.cells, eliminates: sf.eliminates, peers: [] },
      });
    }

    // No advanced strategy found — teach backtracking as last resort
    return withEyeball({
      level: 3,
      title: 'Advanced Reasoning',
      lines: [
        `${id} has candidates: ${cands.join(', ')}`,
        `No single-step strategy resolves this cell directly.`,
        `Try solving easier cells first — their placements will`,
        `create new constraints that narrow this cell's candidates.`,
        `If stuck, try backtracking: assume a candidate, follow the`,
        `implications, and if you hit a contradiction, eliminate it.`,
      ],
      strategy: 'Backtracking — pick a candidate, trace its consequences. If any peer ends up with zero candidates, that value is impossible here. This is the "last resort" algorithm — slow but always works.',
      cell: { row, col },
      overlayCells: { strategy: [], eliminates: [], peers: genericPeers.allEmpty },
    });
  }

  // Hint 4+: Full answer with reasoning chain
  const chain = buildReasoningChain(grid, row, col, answer, cands);
  return withEyeball({
    level: 4,
    title: 'Solution',
    lines: chain,
    strategy: 'Full reasoning chain — each step follows from Sudoku constraints.',
    cell: { row, col },
    overlayCells: { strategy: genericPeers.allFilled, eliminates: [], peers: [] },
  });
}

/**
 * Compute generic peer cells for overlay (used by levels 0-1).
 * Separates filled vs empty peers in row, column, and box.
 */
function _computeGenericPeers(grid, row, col) {
  const allPeers = peers(row, col);
  const allFilled = allPeers.filter(p => grid[p.row - 1][p.col - 1] !== 0);
  const allEmpty = allPeers.filter(p => grid[p.row - 1][p.col - 1] === 0);
  const filledRow = allFilled.filter(p => p.row === row);
  const filledCol = allFilled.filter(p => p.col === col);
  const br = Math.floor((row - 1) / 3) * 3 + 1;
  const bc = Math.floor((col - 1) / 3) * 3 + 1;
  const filledBox = allFilled.filter(p => p.row >= br && p.row < br + 3 && p.col >= bc && p.col < bc + 3);
  return { allFilled, allEmpty, filledRow, filledCol, filledBox };
}

/**
 * Get filled and empty cells in a specific group (for Hidden Single overlay).
 */
function _groupFilledCells(grid, group, groupNum, targetRow, targetCol) {
  const filled = [];
  const empty = [];
  if (group === 'row') {
    for (let c = 1; c <= 9; c++) {
      if (c === targetCol) continue;
      if (grid[groupNum - 1][c - 1] !== 0) filled.push({ row: groupNum, col: c });
      else empty.push({ row: groupNum, col: c });
    }
  } else if (group === 'column') {
    for (let r = 1; r <= 9; r++) {
      if (r === targetRow) continue;
      if (grid[r - 1][groupNum - 1] !== 0) filled.push({ row: r, col: groupNum });
      else empty.push({ row: r, col: groupNum });
    }
  } else {
    const br = Math.floor((targetRow - 1) / 3) * 3 + 1;
    const bc = Math.floor((targetCol - 1) / 3) * 3 + 1;
    for (let r = br; r < br + 3; r++) {
      for (let c = bc; c < bc + 3; c++) {
        if (r === targetRow && c === targetCol) continue;
        if (grid[r - 1][c - 1] !== 0) filled.push({ row: r, col: c });
        else empty.push({ row: r, col: c });
      }
    }
  }
  return { filled, empty };
}

/**
 * Build a full reasoning chain explaining why `answer` is correct.
 */
function buildReasoningChain(grid, row, col, answer, cands) {
  const id = `sudoku.r${row}.c${col}`;
  const lines = [`${id} = ${answer}`, ''];

  // Show what's in each group
  const rowVals = groupValues(grid, 'row', row);
  const colVals = groupValues(grid, 'col', col);
  const boxVals = groupValues(grid, 'box', row, col);

  lines.push(`Row ${row} contains: ${rowVals.join(', ') || '(empty)'}`);
  lines.push(`Column ${col} contains: ${colVals.join(', ') || '(empty)'}`);
  lines.push(`Box ${boxOf(row, col)} contains: ${boxVals.join(', ') || '(empty)'}`);
  lines.push('');

  const allTaken = new Set([...rowVals, ...colVals, ...boxVals]);
  const missing = [1,2,3,4,5,6,7,8,9].filter(v => !allTaken.has(v));
  lines.push(`Combined elimination leaves: ${missing.join(', ')}`);

  if (missing.length === 1) {
    lines.push(`Naked Single: only ${answer} is possible.`);
  } else {
    lines.push(`The answer is ${answer}.`);
  }

  return lines;
}

function countFilled(grid, type, rowOrRow, col) {
  let count = 0;
  if (type === 'row') {
    for (let c = 0; c < 9; c++) if (grid[rowOrRow - 1][c] !== 0) count++;
  } else if (type === 'col') {
    for (let r = 0; r < 9; r++) if (grid[r][rowOrRow - 1] !== 0) count++;
  } else {
    const br = Math.floor((rowOrRow - 1) / 3) * 3;
    const bc = Math.floor((col - 1) / 3) * 3;
    for (let r = br; r < br + 3; r++)
      for (let c = bc; c < bc + 3; c++)
        if (grid[r][c] !== 0) count++;
  }
  return count;
}

function groupValues(grid, type, rowOrRow, col) {
  const vals = [];
  if (type === 'row') {
    for (let c = 0; c < 9; c++) { const v = grid[rowOrRow - 1][c]; if (v) vals.push(v); }
  } else if (type === 'col') {
    for (let r = 0; r < 9; r++) { const v = grid[r][rowOrRow - 1]; if (v) vals.push(v); }
  } else {
    const br = Math.floor((rowOrRow - 1) / 3) * 3;
    const bc = Math.floor((col - 1) / 3) * 3;
    for (let r = br; r < br + 3; r++)
      for (let c = bc; c < bc + 3; c++) { const v = grid[r][c]; if (v) vals.push(v); }
  }
  return vals.sort((a, b) => a - b);
}
