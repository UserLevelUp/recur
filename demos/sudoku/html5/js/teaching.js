/** Live, solution-independent evidence. Recur cascades describe relationships,
 * not proofs. Inputs are accepted values; tentative attempts block deductions;
 * pencil notes are annotations and never premises. Coordinates are 1-based. */
export const cellId = (row, col) => `sudoku.r${row}.c${col}`;
export const position = id => {
  const m = /^sudoku\.r([1-9])\.c([1-9])$/.exec(id);
  if (!m) throw new Error('Invalid cell ID');
  return { row: Number(m[1]), col: Number(m[2]) };
};
export const label = id => { const {row, col} = position(id); return `row ${row}, column ${col}`; };
export function peers(row, col) {
  const result = [];
  for (let r = 1; r <= 9; r++) for (let c = 1; c <= 9; c++) {
    if ((r !== row || c !== col) && (r === row || c === col ||
        (Math.floor((r-1)/3) === Math.floor((row-1)/3) && Math.floor((c-1)/3) === Math.floor((col-1)/3)))) {
      result.push({row:r, col:c, id:cellId(r,c)});
    }
  }
  return result;
}
export function validBoard(board) {
  return Array.isArray(board) && board.length === 9 && board.every(row => Array.isArray(row) &&
    row.length === 9 && row.every(v => Number.isInteger(v) && v >= 0 && v <= 9)) &&
    board.every((row,r) => row.every((v,c) => v === 0 ||
      peers(r+1,c+1).every(p => board[p.row-1][p.col-1] !== v)));
}
export const fingerprint = board => board.map(row => row.join('')).join('/');
export function available(board, row, col) {
  if (board[row-1][col-1]) return [];
  const used = new Set(peers(row,col).map(p => board[p.row-1][p.col-1]));
  return [1,2,3,4,5,6,7,8,9].filter(v => !used.has(v));
}
function validState(state) {
  return typeof state.puzzleId === 'string' && state.puzzleId.length > 0 &&
    Number.isInteger(state.revision) && state.revision >= 0 &&
    Array.isArray(state.tentative) && state.tentative.length === 0 && validBoard(state.board) &&
    state.board.every((row,r) => row.every((v,c) => v || available(state.board,r+1,c+1).length));
}
export function deduction(state, target) {
  if (!validState(state)) return null;
  const {row,col} = position(target);
  const values = available(state.board,row,col);
  if (values.length !== 1) return null;
  const premises = peers(row,col).filter(p => state.board[p.row-1][p.col-1])
    .map(p => ({cell:p.id, value:state.board[p.row-1][p.col-1]}));
  return {schema:'sudoku-deduction-v1', puzzleId:state.puzzleId, revision:state.revision,
    board:fingerprint(state.board), target, technique:'naked-single', premises,
    conclusion:{cell:target, value:values[0]}, highlights:{target, sources:premises.map(p => p.cell)}};
}
export function validateDeduction(record, state) {
  try {
    if (!record || record.technique !== 'naked-single') return false;
    const expected = deduction(state,record.target);
    return expected !== null && JSON.stringify(expected) === JSON.stringify(record);
  } catch { return false; }
}
export function deductions(state) {
  if (!validState(state)) return [];
  const result = [];
  for (let r=1;r<=9;r++) for(let c=1;c<=9;c++) {
    const record = deduction(state,cellId(r,c));
    if(record) result.push(record);
  }
  return result; // deterministic row-major scan, not a human difficulty score
}
export function teachingStep(record, state, level) {
  if (!validateDeduction(record,state)) throw new Error('Stale or invalid deduction');
  const {row,col} = position(record.target);
  const group = Array.from({length:9},(_,i)=>cellId(row,i+1));
  const missing = [1,2,3,4,5,6,7,8,9].filter(v=>!state.board[row-1].includes(v));
  const decisive = record.premises.filter(p=>missing.includes(p.value));
  if(level === 0) return {text:`Look across row ${row}. Which digits are missing?`, sources:group};
  if(level === 1) return {text:`Row ${row} is missing ${missing.join(', ')}. Compare column ${col} and the box.`, sources:group};
  if(level === 2) return {text:decisive.length ? decisive.map(p=>`${label(p.cell)} contains ${p.value}, excluding it here.`).join(' ') :
    'The row alone excludes eight digits. Only one digit remains.', sources:decisive.map(p=>p.cell)};
  return {text:`Naked single: ${label(record.target)} must be ${record.conclusion.value}. Every other digit occurs in a peer.`, sources:record.highlights.sources};
}
