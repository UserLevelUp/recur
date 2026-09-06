/**
 * puzzle.js — Load and index puzzle data.
 *
 * Parses sudoku.solution.txt and sudoku.cascades.json.
 * Generates difficulty masks from the solution (proper Sudoku masks,
 * not arbitrary geometric patterns).
 * No rendering here — pure data.
 */

import { candidates } from './solver.js';

export async function loadPuzzle(dataDir, difficulty='medium') {
  const response = await fetch(`${dataDir}/sudoku.playable.json`, {cache:'no-store'});
  if (response.ok) return parsePlayable(await response.json(), difficulty);
  if (response.status !== 404) throw new Error('Playable package unavailable');
  const [solutionText, cascadesJson] = await Promise.all([
    fetch(`${dataDir}/sudoku.solution.txt`).then(r => r.text()),
    fetch(`${dataDir}/sudoku.cascades.json`).then(r => r.json()),
  ]);

  const solution = parseSolution(solutionText);
  const cascades = indexCascades(cascadesJson);

  return { solution, cascades };
}

import {validBoard, fingerprint} from './teaching.js';

export function parsePlayable(pkg, difficulty) {
  if(pkg.schema !== 'sudoku-playable-v1' || typeof pkg.puzzle_id !== 'string') throw new Error('Unsupported playable package');
  const solution=parseSolution(pkg.solution_text), full=buildGrid(solution,new Set());
  if(solution.size!==81 || !validBoard(full) || full.flat().includes(0)) throw new Error('Invalid solution');
  const preset=pkg.presets?.[difficulty];
  if(!preset || !validBoard(preset.givens)) throw new Error('Invalid playable givens');
  const mask=new Set();
  for(let r=1;r<=9;r++) for(let c=1;c<=9;c++) {
    const value=preset.givens[r-1][c-1];
    if(value===0) mask.add(cellId(r,c));
    else if(value!==full[r-1][c-1]) throw new Error('Givens disagree with solution');
  }
  if(mask.size!==preset.gaps || !['naked-single-solvable','ungraded'].includes(preset.grade?.label) ||
      preset.grade.rubric!=='naked-singles-v1') throw new Error('Invalid grading metadata');
  if(!Array.isArray(pkg.cascades) || pkg.cascades.length!==81 ||
      new Set(pkg.cascades.map(e=>e.cell)).size!==81 ||
      pkg.cascades.some(e=>solution.get(e.cell)!==e.value || !e.cascade)) throw new Error('Invalid cascade package');
  return {solution,cascades:indexCascades(pkg.cascades),mask,
    puzzleId:pkg.puzzle_id+':'+difficulty+':'+fingerprint(preset.givens),
    grade:`${mask.size} gaps; Julia-validated unique puzzle. Technique grade: ${preset.grade.label}. Ungraded does not mean hard.`};
}

/**
 * Parse sudoku.solution.txt into a Map: "sudoku.r3.c5" -> 7
 */
function parseSolution(text) {
  const solution = new Map();
  for (const line of text.split('\n')) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith('#')) continue;
    const m = trimmed.match(/^(sudoku\.r\d+\.c\d+)\s*=\s*(\d+)$/);
    if (m) solution.set(m[1], parseInt(m[2], 10));
  }
  return solution;
}

/**
 * Index cascades array by cell id for O(1) lookup.
 */
function indexCascades(arr) {
  const index = new Map();
  for (const entry of arr) {
    index.set(entry.cell, entry);
  }
  return index;
}

/**
 * Build a mask Set from the solution — cells the player must fill.
 * Legacy-only seeded clue-density mask. No uniqueness guarantee.
 *
 * Candidate-count heuristics do not establish technique difficulty.
 * New Julia packages provide validated givens; this fallback preserves old data.
 *
 * The mask is symmetric (180° rotational) when possible, giving
 * the puzzle a professional look.
 */
export function buildMask(solution, difficulty = 'medium') {
  const target = { easy: 25, medium: 35, hard: 45 }[difficulty] ?? 35;
  const maxCandidates = { easy: 1, medium: 4, hard: 9 }[difficulty] ?? 4;

  // Build the full grid from solution
  const grid = Array.from({ length: 9 }, () => Array(9).fill(0));
  for (const [id, val] of solution) {
    const m = id.match(/sudoku\.r(\d+)\.c(\d+)/);
    if (m) grid[parseInt(m[1]) - 1][parseInt(m[2]) - 1] = val;
  }

  // Build candidate pairs (cell + its 180° rotation partner)
  // This creates symmetric puzzles that look professional
  const pairs = [];
  const used = new Set();
  for (let r = 1; r <= 9; r++) {
    for (let c = 1; c <= 9; c++) {
      const key = `${r},${c}`;
      if (used.has(key)) continue;
      const mr = 10 - r, mc = 10 - c;
      const mirrorKey = `${mr},${mc}`;
      used.add(key);
      used.add(mirrorKey);
      if (r === mr && c === mc) {
        pairs.push([{ row: r, col: c }]);  // center cell, no mirror
      } else {
        pairs.push([{ row: r, col: c }, { row: mr, col: mc }]);
      }
    }
  }

  // Deterministic shuffle seeded by solution checksum
  let seed = 0;
  for (const [, v] of solution) seed = (seed * 31 + v) & 0x7fffffff;
  const rng = () => { seed = (seed * 1103515245 + 12345) & 0x7fffffff; return seed / 0x7fffffff; };
  for (let i = pairs.length - 1; i > 0; i--) {
    const j = Math.floor(rng() * (i + 1));
    [pairs[i], pairs[j]] = [pairs[j], pairs[i]];
  }

  const mask = new Set();
  const testGrid = grid.map(r => [...r]);

  for (const pair of pairs) {
    if (mask.size >= target) break;

    // Try removing all cells in the pair
    const saved = pair.map(({ row, col }) => testGrid[row - 1][col - 1]);
    for (const { row, col } of pair) {
      testGrid[row - 1][col - 1] = 0;
    }

    // Validate: each cell in the pair must have valid candidates
    let valid = true;
    for (const { row, col } of pair) {
      const cands = candidates(testGrid, row, col);
      if (cands.length === 0 || cands.length > maxCandidates) {
        valid = false;
        break;
      }
    }

    if (!valid) {
      // Restore
      pair.forEach(({ row, col }, i) => { testGrid[row - 1][col - 1] = saved[i]; });
      continue;
    }

    for (const { row, col } of pair) {
      mask.add(`sudoku.r${row}.c${col}`);
    }
  }

  return mask;
}

/**
 * Build a 9x9 grid array from solution and mask.
 * grid[row-1][col-1] = value or 0 for empty.
 */
export function buildGrid(solution, mask) {
  const grid = Array.from({ length: 9 }, () => Array(9).fill(0));
  for (const [id, val] of solution) {
    const m = id.match(/sudoku\.r(\d+)\.c(\d+)/);
    if (!m) continue;
    const r = parseInt(m[1]) - 1;
    const c = parseInt(m[2]) - 1;
    grid[r][c] = mask.has(id) ? 0 : val;
  }
  return grid;
}

export function cellId(row, col) {
  return `sudoku.r${row}.c${col}`;
}
