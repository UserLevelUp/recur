/** Julia authors packages and Eventness relationships; this browser owns live teaching. */
import {loadPuzzle, buildMask, buildGrid} from './puzzle.js';
import {Grid} from './grid.js';
import {CascadePanel} from './cascade.js';
import {explainConflicts} from './solver.js';
import {cellId, position, label, fingerprint, available, deduction, deductions, teachingStep, validateDeduction} from './teaching.js';

async function init() {
  const difficulty = new URLSearchParams(location.search).get('difficulty') ?? 'medium';
  const loaded = await loadPuzzle('data/easy-001', difficulty);
  const {solution, cascades} = loaded;
  const mask = loaded.mask ?? buildMask(solution,difficulty);
  const board = buildGrid(solution,mask), remaining = new Set(mask), tentative = new Map();
  const puzzleId = loaded.puzzleId ?? 'legacy:' + fingerprint(board);
  let revision = 0, active = null, level = -1, automatic = false;
  const gridUI = new Grid(document.getElementById('grid-container'),solution,mask);
  const panel = new CascadePanel(document.getElementById('cascade-container'));
  const content = document.getElementById('teaching-content');
  const next = document.getElementById('next-hint'), jump = document.getElementById('jump-hint');
  const state = () => ({puzzleId,revision,board,tentative:[...tentative]});
  const selected = () => remaining.has(gridUI.selected) ? gridUI.selected : null;
  const message = text => { content.textContent=text; };
  const resetTeaching = text => {
    active=null; level=-1; next.hidden=true; jump.hidden=true;
    gridUI.clearHighlights();
    if(text) message(text);
  };
  const refresh = () => {
    document.getElementById('stat-remaining').textContent=remaining.size;
    document.getElementById('stat-placed').textContent=mask.size-remaining.size;
    document.getElementById('stat-total').textContent=mask.size;
    gridUI.computedMarks.clear();
    if(automatic && !tentative.size) for(const id of remaining) {
      const {row,col}=position(id); gridUI.computedMarks.set(id,new Set(available(board,row,col)));
    }
    gridUI.renderMarks();
    const container=document.getElementById('pencil-container');
    container.replaceChildren();
    const entries=[...remaining].map(id=>({id,manual:gridUI.manualMarks.has(id),
      marks:gridUI.manualMarks.get(id) ?? gridUI.computedMarks.get(id) ?? new Set()})).filter(e=>e.marks.size);
    const count=document.createElement('div'); count.className='pencil-count';
    count.textContent=entries.length+' cells annotated'; container.append(count);
    const list=document.createElement('div'); list.className='pencil-entries';
    for(const e of entries) {
      const line=document.createElement('div'); line.className='pencil-entry'; line.dataset.cell=e.id;
      line.textContent=label(e.id)+': '+[...e.marks].sort().join(', ')+(e.manual?' (manual)':' (computed)');
      list.append(line);
    }
    container.append(list);
    const button=document.createElement('button'); button.id='pencil-autofill-btn';
    button.textContent='Auto-fill all candidates'; button.disabled=tentative.size>0;
    button.onclick=()=>{gridUI.manualMarks.clear(); automatic=true; refresh();};
    container.append(button);
    const warning=document.createElement('p');
    warning.textContent='Replaces manual marks. Computed candidates then refresh after moves; editing a cell creates manual notes.';
    container.append(warning);
  };
  const mutate = () => {revision++; resetTeaching('Board changed. Request a fresh teaching step.'); refresh();};
  document.getElementById('puzzle-grade').textContent=loaded.grade ??
    'Legacy puzzle: clue-density preset only; uniqueness and technique difficulty are unverified. Generate a validated puzzle.';

  const showStep = () => {
    if(!active || !validateDeduction(active,state())) {resetTeaching('That explanation is no longer current. Request a fresh hint.'); return;}
    level=Math.min(level+1,3);
    const step=teachingStep(active,state(),level);
    message('Teaching '+label(active.target)+' — step '+(level+1)+' of 4. '+step.text);
    gridUI.clearHighlights();
    gridUI.highlightPeers(step.sources);
    gridUI.highlightStrategy([position(active.target)]);
    next.hidden=false;
    next.textContent=level===3 ? 'Repeat explanation' : 'Next teaching step';
    jump.hidden=gridUI.selected===active.target;
  };
  const help = () => {
    if(tentative.size) {resetTeaching('Clear the tentative attempt before asking for a deduction. It is not an accepted premise.'); return;}
    const id=selected();
    if(!id) {resetTeaching('Select an empty cell first, or find an easier move.'); return;}
    if(active?.target===id && validateDeduction(active,state())) {showStep(); return;}
    resetTeaching();
    active=deduction(state(),id);
    if(!active) {message('No naked-single proof for '+label(id)+' on this board. Try Find an easier move, Show candidates, or Show solution.'); return;}
    showStep();
  };
  document.getElementById('help-cell').onclick=help;
  next.onclick=showStep;
  document.getElementById('easier-move').onclick=()=>{
    resetTeaching();
    if(tentative.size) {message('Clear tentative attempts first.'); return;}
    active=deductions(state())[0] ?? null;
    if(!active) {message('No supported naked single found. Advanced techniques are not proved by this teaching mode.'); return;}
    showStep(); // highlights, but does not change the player's selection
  };
  jump.onclick=()=>{
    if(!active || !validateDeduction(active,state())) {resetTeaching('Suggestion expired.'); return;}
    const record=active, currentLevel=level, {row,col}=position(record.target);
    if(gridUI.selected!==record.target) gridUI._selectCell(record.target,row,col,gridUI.cells.get(record.target));
    active=record; level=currentLevel-1; showStep();
  };
  document.getElementById('show-candidates').onclick=()=>{
    const id=selected(); resetTeaching();
    if(!id || tentative.size) {message('Select an empty cell and clear tentative attempts first.'); return;}
    const {row,col}=position(id);
    message('Candidates for '+label(id)+': '+available(board,row,col).join(', ')+'. These follow from accepted values, not pencil notes.');
  };
  document.getElementById('show-solution').onclick=()=>{
    const id=selected(); resetTeaching();
    message(id ? 'Stored solution for '+label(id)+': '+solution.get(id)+'. Answer disclosure, not a logical proof.' : 'Select an empty cell first.');
  };
  gridUI.onCellSelected=(row,col)=>resetTeaching('Selected '+label(cellId(row,col))+'. Ask for help when ready.');
  gridUI.onCellDeselected=()=>resetTeaching('Select an empty cell or find an easier move.');
  gridUI.onCellCleared=(row,col)=>{tentative.delete(cellId(row,col)); mutate();};
  gridUI.onPencilMarkChanged=()=>{refresh();};
  gridUI.onDigitPlaced=(row,col,value)=>{
    const id=cellId(row,col);
    if(!remaining.has(id)) return;
    if(tentative.has(id)) gridUI.clearCell(row,col);
    if(solution.get(id)!==value) {
      tentative.set(id,value); gridUI.showValue(row,col,value,false); mutate();
      const conflicts=explainConflicts(board,row,col,value);
      if(conflicts.length) {
        panel.renderConflict(row,col,value,conflicts);
        gridUI.highlightConflicts(conflicts.map(c=>c.conflictCell));
        message('Tentative attempt conflicts with the highlighted accepted values. Backspace clears it.');
      } else message('This differs from the stored solution, but no visible rule conflict was found. No deeper contradiction has been proved. Backspace clears it.');
      return;
    }
    board[row-1][col-1]=value; remaining.delete(id);
    gridUI.showValue(row,col,value,true); mutate();
    const entry=cascades.get(id);
    if(entry) panel.renderCascade(entry);
    message('Value accepted. The cascade below is pre-generated relationship context, not a live deduction proof.');
    if(!remaining.size) {
      panel.showWin(); document.getElementById('win-banner').classList.remove('hidden');
      message('Puzzle complete. Every cell is filled.'); gridUI.clearHighlights();
    }
  };
  document.addEventListener('keydown',e=>{
    if(e.ctrlKey || e.altKey || e.metaKey || /INPUT|TEXTAREA|SELECT/.test(e.target.tagName)) return;
    if(e.key.toLowerCase()==='h') {e.preventDefault(); help();}
    if(e.key.toLowerCase()==='p') {
      gridUI.pencilMode=!gridUI.pencilMode;
      const indicator=document.getElementById('pencil-mode');
      indicator.textContent=gridUI.pencilMode?'Pencil Mode ON':'Pencil Mode';
      indicator.classList.toggle('active',gridUI.pencilMode);
    }
  });
  document.querySelectorAll('.difficulty-btn').forEach(btn=>{
    btn.classList.toggle('active',btn.dataset.difficulty===difficulty);
    btn.onclick=()=>{resetTeaching('Changing puzzle preset…'); location.search='?difficulty='+btn.dataset.difficulty;};
  });
  document.getElementById('new-puzzle-btn').onclick=async()=>{
    const btn=document.getElementById('new-puzzle-btn'), status=document.getElementById('generate-status');
    btn.disabled=true; status.classList.remove('hidden');
    status.textContent='Julia is validating playable puzzles and generating Recur relationships…';
    resetTeaching('Generating a new puzzle…');
    try {
      const response=await fetch('/api/generate',{method:'POST'}), data=await response.json();
      if(!response.ok || data.status!=='ok') throw new Error(data.message ?? 'Generation failed');
      location.reload();
    } catch(error) {
      status.textContent='Generation failed; current puzzle retained. '+error.message;
      btn.disabled=false;
    }
  };
  refresh();
}
init().catch(error=>{
  console.error('Failed to initialise game:',error);
  document.getElementById('grid-container').textContent='Failed to load puzzle: '+error.message;
});
