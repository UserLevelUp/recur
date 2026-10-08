/* consumer: demo.blackjack.web.view authoritative snapshots
   publish: demo.blackjack.web.browser accessible presentation and user requests
   No scoring, shuffling, payout logic or hidden card data belongs in this module. */
'use strict';
const el = id => document.getElementById(id);
let state = null, busy = false, offline = false, lastCardKey = '', lastRevision = -1;
let downloadBusy = false, playbackBusy = false;
const replayLimit = 1024 * 1024;
const fmt = n => Number(n).toLocaleString('en-US');
const signed = n => (n > 0 ? '+' : n < 0 ? '−' : '') + fmt(Math.abs(n));
const names = {blackjack:'Blackjack', win:'You won', loss:'Dealer won', push:'Push', mixed:'Mixed results'};
function acceptSnapshot(data) {
  if(!window.BlackjackRivalsView.accepts(data.schema))throw new Error('Table version changed. Reload this page to continue.');
  state=data;
}
function error(message, reconnect=false) {
  el('error-banner').hidden = !message;
  el('error-message').textContent = message;
  el('retry').hidden = !reconnect;
}
function cardNode(id, placeholder=false) {
  const card = document.createElement('div'); card.className = 'card';
  if (placeholder) {card.classList.add('ghost');card.textContent='♠';card.setAttribute('aria-label','Waiting for a card');return card;}
  if (id === null) {card.classList.add('back');card.setAttribute('aria-label','Face-down dealer card');return card;}
  const rank=(id-1)%13, suit=Math.floor((id-1)/13);
  const ranks=['A','2','3','4','5','6','7','8','9','10','J','Q','K'];
  const suits=['♠','♥','♣','♦'], suitNames=['spades','hearts','clubs','diamonds'];
  if (suit===1||suit===3) card.classList.add('red');
  card.setAttribute('aria-label',`${ranks[rank]} of ${suitNames[suit]}`);
  for (const cls of ['card-corner','card-corner bottom']) {
    const corner=document.createElement('span');corner.className=cls;corner.textContent=ranks[rank];
    const symbol=document.createElement('small');symbol.textContent=suits[suit];corner.append(symbol);card.append(corner);
  }
  const pip=document.createElement('span');pip.textContent=suits[suit];pip.setAttribute('aria-hidden','true');card.append(pip);
  return card;
}
function paintCards(target,cards,dealer=false) {
  el(target).classList.toggle('many',cards.length>5);
  el(target).classList.toggle('crowded',cards.length>8);
  const nodes=cards.length ? cards.map(c=>cardNode(c)) : [cardNode(null,!dealer),cardNode(null,!dealer)];
  el(target).replaceChildren(...nodes);
}
function scoreText(score) {return !score ? '—' : score.bust ? `${score.total} · BUST` : score.natural ? '21 · BJ' : `${score.total}${score.soft?' · SOFT':''}`;}
function historyRows(rows) {
  if (!rows.length) {
    const box=document.createElement('div');box.className='empty-history';
    const icon=document.createElement('span');icon.textContent='♧';
    const text=document.createElement('p');text.textContent='Your story starts with the first hand.';box.append(icon,text);el('history').replaceChildren(box);return;
  }
  el('history').replaceChildren(...rows.map(row=>{
    const item=document.createElement('div');item.className=`history-item ${row.result}`;
    const icon=document.createElement('span');icon.className='history-icon';icon.textContent=row.profit>0?'↗':row.profit<0?'↘':'−';
    const label=document.createElement('div');const title=document.createElement('strong');title.textContent=names[row.result];
    const sub=document.createElement('small');sub.textContent=`Round ${row.round} · Bet ${row.stake}`;label.append(title,sub);
    for(const hand of row.hands){const detail=document.createElement('small');detail.textContent=`Hand ${hand.id}: ${names[hand.result]} (${signed(hand.profit)})`;label.append(detail);}
    const delta=document.createElement('span');delta.className='history-delta';delta.textContent=signed(row.profit);item.append(icon,label,delta);return item;
  }));
}
function chosenBet() {return Number(el('bet').value);}
function updateBet() {
  const bet=chosenBet(), valid=Number.isInteger(bet)&&bet>=2&&bet%2===0&&bet<=Math.min(500,state?.max_bet??0);
  el('deal').disabled=busy||offline||!state?.allowed.includes('deal')||!valid;
  document.querySelectorAll('[data-bet]').forEach(button=>{
    button.classList.toggle('selected',Number(button.dataset.bet)===bet);
    button.disabled=busy||offline||!state||state.phase==='playing'||Number(button.dataset.bet)>state.max_bet;
  });
  el('bet').setAttribute('aria-invalid',String(!valid&&!!state&&state.phase!=='playing'));
  if(state&&state.phase!=='playing') el('bet-help').textContent=valid?'Even bets from 2 to 500 · Blackjack pays 3:2':state.max_bet<2?'Not enough chips to deal. Start a new table.':`Choose an even bet from 2 to ${fmt(state.max_bet)} chips.`;
}
// consumer: demo.blackjack.web.render snapshot, with public fields only
function render() {
  if(!state) {paintCards('dealer-cards',[],true);updateSessionTools();return;}
  // consumer: demo.blackjack.rivals.table.projection; publish: demo.blackjack.rivals.browser.seats
  window.BlackjackRivalsView.render(el('rival-seats'),state.rivals||[]);
  const rivalCount=state.rival_count||0;
  el('table-mode').textContent=rivalCount?`${rivalCount} ${rivalCount===1?'RIVAL':'RIVALS'} · BLACKJACK`:'SOLO · BLACKJACK';
  el('table-subtitle').textContent=rivalCount?`You and ${rivalCount} ${rivalCount===1?'rival':'rivals'}, one dealer, one shared deck.`:'Just you, the dealer, and the next card.';
  const playing=state.phase==='playing', settled=state.phase==='settled';
  el('balance').textContent=fmt(state.balance);
  el('net').textContent=`${signed(state.net)} this session${playing?' · bet in play':''}`;
  for(const stat of ['wins','losses','pushes']) el(stat).textContent=state.stats[stat];
  el('session-statistics').textContent=window.BlackjackRivalsView.statisticsLabel(state.session_statistics);
  el('rules-provenance').textContent=`Rules: ${state.rules_id || 'unavailable'} · Engine: ${state.engine_id || 'unavailable'}`;
  el('comparison-label').textContent=state.comparison_label || (rivalCount ? 'Practice results only: rivals hit/stand; the human may split/double. Shared deck, seat order, stakes and bankroll affect results; no skill rating.' : 'Solo practice; profit is descriptive, not a skill rating.');
  updateSessionTools();
  el('round-count').textContent=`${state.stats.rounds} ${state.stats.rounds===1?'round':'rounds'} · ${state.stats.hands} ${state.stats.hands===1?'hand':'hands'}`;
  el('hand-number').textContent=playing?`ROUND ${state.round_id} · HAND ${state.active_hand_id}`:settled?`ROUND ${state.round_id} · COMPLETE`:'READY WHEN YOU ARE';
  el('session-status').textContent=offline?'OFFLINE':busy?'DEALING':'AT THE TABLE';
  const cardKey=JSON.stringify([state.hands,state.dealer,state.active_hand_id]);
  if(cardKey!==lastCardKey){paintCards('dealer-cards',state.dealer,true);paintHands();lastCardKey=cardKey;}
  el('dealer-score').textContent=playing?'?':scoreText(state.dealer_score);
  el('bet-controls').hidden=playing;el('action-controls').hidden=!playing;
  el('reset-open').disabled=busy||offline||!state.allowed.includes('reset');
  el('bet').disabled=playing||busy||offline;
  el('bet').max=state.max_bet;
  for(const action of ['hit','stand','double','split']) el(action).disabled=busy||offline||!state.allowed.includes(action);
  el('split-help').textContent=state.split.allowed?`Split this pair for ${fmt(state.split.extra_stake)} more chips. Play each hand in turn.`:state.split.reason;
  el('split-help').hidden=!playing;
  for(const id of ['bet-minus','bet-plus']) el(id).disabled=playing||busy||offline||state.max_bet<2;
  el('deal').firstChild.textContent=settled?'Deal next round ':'Deal me in ';
  el('result-profit').textContent=settled?`${signed(state.profit)}`:'';
  el('result-profit').classList.toggle('negative',state.profit<0);
  const title=playing?`Your move · Hand ${state.active_hand_id}.`:settled?state.result==='blackjack'?'A little natural talent.':state.profit>0?'Nicely played.':state.profit===0?'Evenly matched.':'This round goes to Ada.':'Pull up a chair.';
  const detail=playing?`Bet ${fmt(state.hands.find(h=>h.id===state.active_hand_id).stake)} · ${state.hands.length===2?'Play the highlighted hand. Ada waits for both.':'Take a card, stand, or choose an available action.'}`:settled?`${names[state.result]} · ${signed(state.profit)} chips this round. All wagers settled.`:'Choose your chips and let the cards fall.';
  el('status-title').textContent=title;el('status-detail').textContent=detail;
  el('status-symbol').textContent=settled?(state.profit>0?'✧':state.profit<0?'♠':'='):'♠';
  if(playing) el('bet-help').textContent='H to hit · S to stand · D to double · P to split';
  if(state.revision!==lastRevision){historyRows(state.history);if(settled&&state.profit>0){el('felt').classList.remove('flash');requestAnimationFrame(()=>el('felt').classList.add('flash'));}lastRevision=state.revision;}
  updateBet();
}
function updateSessionTools() {
  const downloadable = !!state && state.phase === 'settled' && !busy && !offline && !downloadBusy;
  el('report-download').disabled = !downloadable;
  el('replay-download-open').disabled = !downloadable;
  el('replay-download-confirm').disabled = !downloadable;
  el('replay-file').disabled = !state || offline || playbackBusy;
  el('export-help').textContent = state?.phase === 'settled' ? 'Public report contains totals and provenance. PRIVATE replay reveals full deck history.' : 'Finish a round to download settled results.';
}
function toolsStatus(message) { el('session-tools-status').textContent = message; }
function updatePolicyChoices() {
  const count = Number(el('rival-count').value);
  for (const index of [1,2]) {
    el(`rival-policy-row-${index}`).hidden = index > count;
    el(`rival-policy-${index}`).disabled = index > count;
  }
}
function selectedPolicies(count) {
  return Array.from({length:count},(_,index)=>el(`rival-policy-${index+1}`).value);
}
async function downloadSession(kind) {
  if (downloadBusy || busy || offline || state?.phase !== 'settled') return;
  downloadBusy = true; updateSessionTools(); toolsStatus('Preparing download…');
  try {
    const response = await fetch(kind === 'report' ? '/api/session-report' : '/api/replay', {cache:'no-store'});
    const data = await response.json();
    if (!response.ok) throw new Error(data.error || 'Download unavailable.');
    const schema = kind === 'report' ? 'blackjack-session-report-v1' : 'blackjack-private-replay-v1';
    if (data.schema !== schema) throw new Error('Unsupported download format.');
    const url = URL.createObjectURL(new Blob([JSON.stringify(data,null,2)],{type:'application/json'}));
    const link = document.createElement('a');
    link.href = url; link.download = kind === 'report' ? 'blackjack-session-report.json' : 'blackjack-PRIVATE-replay.json';
    document.body.append(link);
    try { link.click(); } finally { link.remove(); URL.revokeObjectURL(url); }
    toolsStatus(kind === 'report' ? 'Public settled session report downloaded.' : 'PRIVATE replay downloaded. This file reveals full deck history.');
  } catch (e) { toolsStatus(e.message); }
  finally { downloadBusy = false; updateSessionTools(); }
}
// Replay public snapshots never enter acceptSnapshot, render, revision or history.
function renderReplaySnapshot(snapshot) {
  if (!snapshot || !window.BlackjackRivalsView.accepts(snapshot.schema) || snapshot.phase !== 'settled') throw new Error('Replay did not return a supported completed snapshot.');
  const box = el('replay-result');
  const node = (tag,text) => { const item=document.createElement(tag); item.textContent=text; return item; };
  const human = node('section',''); human.className='replay-seat'; human.setAttribute('aria-label','Replay human seat');
  human.append(node('h3','Human'),node('p',`Bankroll: ${fmt(snapshot.balance)} chips · Net: ${signed(snapshot.net)} · Capabilities: hit, stand, double, split`),node('p',window.BlackjackRivalsView.statisticsLabel(snapshot.session_statistics)));
  for (const hand of snapshot.hands || []) {
    human.append(node('p',`Hand ${hand.id} · ${scoreText(hand.score)} · Bet ${hand.stake} · Outcome ${hand.result} · Profit ${signed(hand.profit)}`));
    const cards=node('div',''); cards.className='cards'; cards.setAttribute('aria-label',`Replay human hand ${hand.id} cards`); cards.append(...(hand.cards || []).map(c=>cardNode(c))); human.append(cards);
  }
  const dealer=node('section',''); dealer.className='replay-seat'; dealer.setAttribute('aria-label','Replay dealer'); dealer.append(node('h3',`Dealer · ${scoreText(snapshot.dealer_score)}`));
  const cards=node('div',''); cards.className='cards'; cards.setAttribute('aria-label','Replay dealer cards'); cards.append(...(snapshot.dealer || []).map(c=>cardNode(c))); dealer.append(cards);
  const rivals=node('section',''); rivals.className='rival-seats'; rivals.setAttribute('aria-label','Replay computer rivals'); window.BlackjackRivalsView.render(rivals,snapshot.rivals || []);
  box.replaceChildren(node('p',`Rules: ${snapshot.rules_id || 'unavailable'} · Engine: ${snapshot.engine_id || 'unavailable'}`),node('p',snapshot.comparison_label || 'Practice results only; unequal capabilities, shared deck and seat order affect comparisons. No skill rating.'),human,dealer,rivals);
}
async function uploadReplay(file) {
  if (!file || playbackBusy || !state || offline) return;
  playbackBusy=true; updateSessionTools(); toolsStatus('Checking private replay…');
  try {
    if (file.size > replayLimit) throw new Error('Replay file must be 1 MiB or smaller.');
    let replay;
    try { replay=JSON.parse(await file.text()); } catch { throw new Error('Choose a valid replay JSON file.'); }
    if (!replay || typeof replay !== 'object' || Array.isArray(replay)) throw new Error('Replay envelope must be a JSON object.');
    const body=JSON.stringify({replay});
    if (new TextEncoder().encode(body).byteLength > replayLimit) throw new Error('Replay request must fit within 1 MiB, including its JSON wrapper.');
    const response=await fetch('/api/replay',{method:'POST',headers:{'Content-Type':'application/json'},body});
    const data=await response.json();
    if (!response.ok) throw new Error(data.error || 'Replay could not be checked.');
    if (data.schema !== 'blackjack-replay-result-v1') throw new Error('Unsupported replay response format.');
    renderReplaySnapshot(data.state); el('replay-result-dialog').showModal(); toolsStatus('Completed replay opened in an isolated, read-only view.');
  } catch(e) { toolsStatus(e.message); }
  finally { playbackBusy=false; el('replay-file').value=''; updateSessionTools(); }
}
// publish: demo.blackjack.web.split.browser two independent hands, one active control group
function paintHands() {
  const hands=state.hands.length?state.hands:[{id:1,cards:[],score:null,stake:0,status:'waiting',result:''}];
  el('player-hands').classList.toggle('split-hands',hands.length===2);
  el('player-hands').replaceChildren(...hands.map(hand=>{
    const panel=document.createElement('section');panel.className='player-hand';
    panel.setAttribute('aria-label',`Your hand ${hand.id}`);
    const current=hand.id===state.active_hand_id;
    panel.classList.toggle('active-hand',current);
    if(current)panel.setAttribute('aria-current','true');
    const heading=document.createElement('div');heading.className='player-heading';
    const label=document.createElement('span');label.className='participant';label.textContent=`HAND ${hand.id}`;
    const score=document.createElement('span');score.className='score-badge';score.textContent=scoreText(hand.score);
    heading.append(label,score);
    const cards=document.createElement('div');cards.className='cards';cards.setAttribute('aria-label',`Hand ${hand.id} cards`);
    cards.classList.toggle('many',hand.cards.length>5);cards.classList.toggle('crowded',hand.cards.length>8);
    cards.replaceChildren(...(hand.cards.length?hand.cards.map(c=>cardNode(c)):[cardNode(null,true),cardNode(null,true)]));
    const detail=document.createElement('p');detail.className='hand-detail';
    detail.textContent=hand.stake?`BET ${hand.stake} · ${hand.result?`${names[hand.result]} ${signed(hand.profit)}`:current?'YOUR TURN':hand.status==='waiting'?'UP NEXT':hand.status.toUpperCase()}`:'YOUR NEXT HAND';
    panel.append(heading,cards,detail);return panel;
  }));
}
async function load() {
  if(busy)return;busy=true;render();
  try {
    const response=await fetch('/api/state',{cache:'no-store'});const data=await response.json();
    if(!response.ok)throw new Error(data.error||'The table is unavailable.');
    acceptSnapshot(data);offline=false;error('');
  } catch(e){offline=true;error(e.message.includes('version')?e.message:'Cannot reach your table. Check that the Julia server is running, then reconnect.',true);}
  finally{busy=false;render();}
}
// consumer: demo.blackjack.web.send user choice; publish: demo.blackjack.web.http command
async function command(action,extra={}) {
  if(busy||offline||!state||!state.allowed.includes(action))return;
  busy=true;error('');render();
  try {
    const hand=['hit','stand','double','split'].includes(action)?{hand_id:state.active_hand_id}:{};
    const response=await fetch('/api/action',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({version:window.BlackjackRivalsView.protocolVersion(state),action,revision:state.revision,...hand,...extra})});
    const data=await response.json();
    if(response.status===409){
      if(data.state){acceptSnapshot(data.state);error('Your table changed in another tab. The current hand is restored.');}
      else{offline=true;error(data.error,true);}
    } else if(!response.ok){if(response.status===426)offline=true;error(data.error||'That action could not be completed.');}
    else {acceptSnapshot(data);offline=false;}
  } catch(e){offline=true;error(e.message.includes('version')?e.message:'Connection interrupted. Reconnect to see whether your action completed; your bet will not be sent twice.',true);}
  finally{busy=false;render();}
}
for(const action of ['hit','stand','double','split']) el(action).addEventListener('click',()=>command(action));
el('deal').addEventListener('click',()=>command('deal',{bet:chosenBet()}));
el('bet').addEventListener('input',updateBet);
for(const [id,delta] of [['bet-minus',-10],['bet-plus',10]])el(id).addEventListener('click',()=>{el('bet').value=Math.max(2,Math.min(state.max_bet,(Math.round(chosenBet()/2)||1)*2+delta));updateBet();});
document.querySelectorAll('[data-bet]').forEach(button=>button.addEventListener('click',()=>{el('bet').value=button.dataset.bet;updateBet();}));
for(const id of ['rules-open','rules-bottom'])el(id).addEventListener('click',()=>el('rules-dialog').showModal());
el('reset-open').addEventListener('click',()=>{
  el('rival-count').value=String(state?.rival_count||0);
  for(const index of [1,2]) el(`rival-policy-${index}`).value=state?.rivals?.[index-1]?.policy_id || 'stand17-v1';
  updatePolicyChoices(); el('reset-dialog').showModal();
});
el('rival-count').addEventListener('change',updatePolicyChoices);
document.querySelectorAll('[data-close]').forEach(button=>button.addEventListener('click',()=>el(button.dataset.close).close()));
el('reset-confirm').addEventListener('click',()=>{const rival_count=Number(el('rival-count').value);el('reset-dialog').close();command('reset',{money:Number(el('starting-money').value),rival_count,rival_policies:selectedPolicies(rival_count)});});
el('report-download').addEventListener('click',()=>downloadSession('report'));
el('replay-download-open').addEventListener('click',()=>{if(!el('replay-download-open').disabled)el('private-replay-dialog').showModal();});
el('replay-download-confirm').addEventListener('click',()=>{el('private-replay-dialog').close();downloadSession('replay');});
el('replay-file').addEventListener('change',()=>uploadReplay(el('replay-file').files?.[0]));
el('retry').addEventListener('click',load);
document.addEventListener('keydown',event=>{
  if(event.repeat||event.ctrlKey||event.altKey||event.metaKey||document.querySelector('dialog[open]')||['INPUT','SELECT','TEXTAREA'].includes(document.activeElement?.tagName))return;
  const action={h:'hit',s:'stand',d:'double',p:'split'}[event.key.toLowerCase()];
  if(action&&state?.allowed.includes(action)){event.preventDefault();command(action);}
});
document.addEventListener('visibilitychange',()=>{if(!document.hidden)load();});
render();load();
