import {deduction, deductions, validateDeduction, teachingStep, validBoard, fingerprint} from '../js/teaching.js';

// Transcribed from the user's 19-blank screenshot; not their mutable easy-001 file.
export const screenshotBoard = [
  [9,8,2,5,4,1,3,7,6], [1,6,7,8,9,3,0,2,0], [4,3,5,7,2,6,8,1,9],
  [5,1,4,9,8,2,7,6,3], [3,7,8,0,1,0,0,9,0], [2,9,6,4,3,7,1,8,5],
  [0,5,3,0,7,9,0,4,1], [0,2,0,0,6,0,9,5,0], [6,0,9,1,0,8,0,0,0]];
export function runTests() {
  let count=0;
  const check = (value,message) => {if(!value) throw new Error(message); count++;};
  const state = {puzzleId:'transcribed-screenshot-v1', revision:0, board:structuredClone(screenshotBoard), tentative:[]};
  check(validBoard(state.board),'valid fixture');
  check(state.board.flat().filter(v=>!v).length===19,'19 blanks');
  const before=fingerprint(state.board), record=deduction(state,'sudoku.r2.c9');
  check(record.conclusion.value===4,'r2c9=4');
  check(deductions(state).length===12,'12 naked singles');
  for(const d of deductions(state)) {
    check(validateDeduction(d,state),'valid evidence');
    const excluded=new Set(d.premises.map(p=>p.value));
    check(excluded.size===8 && !excluded.has(d.conclusion.value),'all alternatives excluded');
  }
  check(deduction(state,'sudoku.r8.c9')===null,'no false single');
  check(!validateDeduction(record,{...state,revision:1}),'revision invalidation');
  check(!validateDeduction(record,{...state,puzzleId:'other'}),'puzzle invalidation');
  check(!validateDeduction(record,{...state,tentative:[{cell:'sudoku.r8.c9',value:7}]}),'tentative blocks');
  for(const mutation of [d=>d.conclusion.value=5,d=>d.premises.pop(),d=>d.technique='magic',d=>d.highlights.sources=[]]) {
    const changed=structuredClone(record); mutation(changed);
    check(!validateDeduction(changed,state),'tampered record rejected');
  }
  const changed=structuredClone(state); changed.board[1][8]=4;
  check(!validateDeduction(record,changed),'fingerprint invalidation');
  changed.board[1][8]=1; check(!validBoard(changed.board),'duplicate rejected');
  check(deductions(changed).length===0,'invalid board no deductions');
  check(deductions({...state,board:Array.from({length:9},()=>Array(9).fill(0))}).length===0,'ambiguous empty board has no singles');
  check(!validateDeduction(record,{...state,revision:-1}),'invalid revision');
  check(!validBoard([[1]]),'malformed board');
  check(teachingStep(record,state,0).text==='Look across row 2. Which digits are missing?','no initial spoiler');
  check(teachingStep(record,state,1).text.includes('4, 5'),'missing row digits');
  check(teachingStep(record,state,2).text.includes('row 6, column 9 contains 5'),'decisive peer');
  check(teachingStep(record,state,3).text.includes('must be 4'),'requested conclusion');
  check(fingerprint(state.board)===before,'queries do not mutate board');
  return count;
}
