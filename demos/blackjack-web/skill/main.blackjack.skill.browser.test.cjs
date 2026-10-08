'use strict';
// publish: demo.blackjack.skill.browser transport and isolated playback acceptance
const test=require('node:test'), assert=require('node:assert/strict');
const fs=require('node:fs'), path=require('node:path'), vm=require('node:vm');
const root=path.resolve(__dirname,'..');
const html=fs.readFileSync(path.join(root,'index.html'),'utf8');
const source=fs.readFileSync(path.join(root,'main.blackjack.web.js'),'utf8');
const rivalSource=fs.readFileSync(path.join(root,'main.blackjack.rivals.js'),'utf8');
class Element {
  constructor(tag='div') {this.tagName=tag.toUpperCase();this.children=[];this.attrs={};this.textContent='';this.className='';this.hidden=false;this.disabled=false;this.value='';this.handlers={};this.dataset={};this.firstChild={textContent:''};this.classes=new Set();this.classList={toggle:(k,on)=>on?this.classes.add(k):this.classes.delete(k),add:k=>this.classes.add(k),remove:k=>this.classes.delete(k)};}
  append(...items){this.children.push(...items);}
  replaceChildren(...items){this.children=[...items];}
  setAttribute(k,v){this.attrs[k]=String(v);}
  addEventListener(k,fn){this.handlers[k]=fn;}
  showModal(){this.open=true;}
  close(){this.open=false;}
  click(){this.clicked=true;return this.handlers.click?.();}
  remove(){this.removed=true;}
  set innerHTML(_){throw Error('Unsafe HTML write');}
}
const texts=n=>[n.textContent,...n.children.flatMap(texts)].join(' ');
const walk=n=>[n,...n.children.flatMap(walk)];
const stats={rounds:1,hands:1,wins:1,losses:0,pushes:0};
const summary={roi:0.5,net_chips:10,resolved_wager:20,sample_rounds:1,sample_hands:1};
function snapshot(phase='settled',count=0) {
  const score={total:20,soft:false,natural:false,bust:false};
  return {schema:count?'blackjack-web-state-v3':'blackjack-web-state-v2',revision:7,phase,
    balance:510,net:10,stats:{...stats},session_statistics:{...summary},round_id:1,active_hand_id:phase==='playing'?1:null,
    hands:[{id:1,cards:[10,23],score,stake:20,status:phase,result:'win',profit:10}],dealer:[9,22],dealer_score:score,
    allowed:phase==='playing'?['hit','stand','double','split']:['deal','reset'],split:{allowed:false,extra_stake:0,reason:'No pair'},
    max_bet:500,profit:10,result:'win',history:[{round:1,stake:20,result:'win',profit:10,hands:[{id:1,result:'win',profit:10}]}],rival_count:count,
    rules_id:'s17-3to2-one-split-v1',engine_id:count?'blackjack-rivals-v3':'blackjack-web-v2',
    comparison_label:'Practice results only: unequal capabilities, shared deck and seat order; no skill rating.',
    rivals:Array.from({length:count},(_,i)=>({id:i+1,name:`Rival ${i+1}`,policy_id:'stand17-v1',policy_version:1,policy:'Stand on 17 benchmark',balance:500,net:0,escrow:0,result:'push',profit:0,hands:[],session_statistics:{...summary}}))};
}
const response=(data,status=200)=>({ok:status>=200&&status<300,status,json:async()=>data});
const tick=()=>new Promise(resolve=>setImmediate(resolve));
async function browser(initial=snapshot()) {
  const ids=new Map([...html.matchAll(/id="([^"]+)"/g)].map(m=>[m[1],new Element()]));
  ids.get('bet').value='20';ids.get('starting-money').value='500';ids.get('rival-count').value='0';
  for(const i of [1,2])ids.get(`rival-policy-${i}`).value='stand17-v1';
  const body=new Element('body'),created=[],calls=[],urls=[],revoked=[];
  const document={body,hidden:false,activeElement:body,getElementById:id=>{assert.ok(ids.has(id),id);return ids.get(id);},
    createElement:tag=>{const e=new Element(tag);created.push(e);return e;},querySelectorAll:()=>[],querySelector:()=>null,addEventListener:()=>{}};
  const context=vm.createContext({window:{},document,Blob,TextEncoder,URL:{createObjectURL:b=>{urls.push(b);return 'blob:download';},revokeObjectURL:u=>revoked.push(u)},requestAnimationFrame:fn=>fn()});
  let handler=async()=>response(initial);
  context.fetch=async(url,options={})=>{calls.push({url,...options});return handler(url,options);};
  vm.runInContext(rivalSource,context);vm.runInContext(source,context);await tick();
  return {context,ids,calls,created,urls,revoked,run:code=>vm.runInContext(code,context),handle:fn=>{handler=fn;}};
}
function upload(b,content,size=Buffer.byteLength(content)) {b.context.uploadFile={size,text:async()=>content};return b.run('uploadReplay(uploadFile)');}

test('each reset sends exactly its selected rival policies and keeps schema protocol selection',async()=>{
  const b=await browser(snapshot('settled',2));
  b.ids.get('reset-open').click();
  assert.equal(b.ids.get('rival-policy-2').disabled,false);
  b.handle(async(_url,options)=>response(snapshot('settled',JSON.parse(options.body).rival_count)));
  for(const count of [0,1,2]) {
    b.ids.get('rival-count').value=String(count);b.ids.get('rival-count').handlers.change();
    b.ids.get('rival-policy-1').value='dealer-aware-v1';b.ids.get('rival-policy-2').value='stand17-v1';
    assert.equal(b.ids.get('rival-policy-row-2').hidden,count<2);
    assert.equal(b.ids.get('rival-policy-1').disabled,count===0);
    const version=b.run('window.BlackjackRivalsView.protocolVersion(state)');
    b.ids.get('reset-confirm').click();await tick();
    const payload=JSON.parse(b.calls.at(-1).body);
    assert.equal(payload.version,version);assert.equal(payload.rival_count,count);
    assert.deepEqual(payload.rival_policies,['dealer-aware-v1','stand17-v1'].slice(0,count));
  }
});

test('settled summaries preserve authoritative ROI including zero and null sample semantics',async()=>{
  const b=await browser();
  assert.match(b.ids.get('session-statistics').textContent,/ROI \(Net chips \/ resolved wager\): 0.5/);
  assert.match(b.ids.get('session-statistics').textContent,/Settled samples: 1 rounds \/ 1 hands/);
  assert.match(b.ids.get('comparison-label').textContent,/unequal capabilities/);
  const label=b.context.window.BlackjackRivalsView.statisticsLabel;
  assert.match(label({...summary,roi:0}),/wager\): 0 ·/);
  assert.match(label({...summary,roi:null,sample_rounds:0,sample_hands:0}),/no resolved wager.*0 rounds \/ 0 hands/);
});

test('live and isolated replay results identify effective strategy and rules versions',async()=>{
  const s=snapshot('settled',2);s.rivals[1].policy_id='dealer-aware-v1';s.rivals[1].policy='Dealer-aware benchmark';
  const b=await browser(s);
  assert.match(texts(b.ids.get('rival-seats')),/stand17-v1 · version 1/);
  assert.match(texts(b.ids.get('rival-seats')),/dealer-aware-v1 · version 1/);
  assert.match(b.ids.get('rules-provenance').textContent,/s17-3to2-one-split-v1.*blackjack-rivals-v3/);
  b.context.completed=s;b.run('renderReplaySnapshot(completed)');
  const replay=texts(b.ids.get('replay-result'));
  for(const id of [s.rules_id,s.engine_id,'stand17-v1','dealer-aware-v1'])assert.ok(replay.includes(id),id);
  assert.match(replay,/version 1/);
});

test('public export and deliberate private download use settled GET routes and inert JSON blobs',async()=>{
  const b=await browser();
  b.handle(async url=>response({schema:url.includes('session-report')?'blackjack-session-report-v1':'blackjack-private-replay-v1',label:'<script>attack</script>'}));
  await b.ids.get('report-download').click();await tick();
  assert.equal(b.calls.at(-1).url,'/api/session-report');assert.equal(b.calls.at(-1).method,undefined);
  assert.equal(b.created.find(e=>e.download==='blackjack-session-report.json').clicked,true);
  const calls=b.calls.length;b.ids.get('replay-download-open').click();
  assert.equal(b.ids.get('private-replay-dialog').open,true);assert.equal(b.calls.length,calls);
  b.ids.get('replay-download-confirm').click();await tick();
  assert.equal(b.calls.at(-1).url,'/api/replay');assert.equal(b.calls.at(-1).method,undefined);
  assert.equal(b.created.find(e=>e.download==='blackjack-PRIVATE-replay.json').clicked,true);
  assert.match(await b.urls.at(-1).text(),/<script>attack<\/script>/);assert.equal(b.revoked.length,2);
  b.run("state.phase='playing';updateSessionTools()");assert.equal(b.ids.get('report-download').disabled,true);
  const before=b.calls.length;await b.run("downloadSession('report')");assert.equal(b.calls.length,before);
  assert.match(html,/PRIVATE replay includes every realized deck/);
});

test('playback during live play is isolated, inert and does not block a live action',async()=>{
  const b=await browser(snapshot('playing',1));const replay=snapshot('settled',2),attack='<img src=x onerror=alert(1)>';
  replay.rivals[0].name=attack;replay.comparison_label=attack;
  let completePlayback;
  b.handle(async(url,options)=>{
    if(url==='/api/replay')return new Promise(resolve=>{completePlayback=()=>resolve(response({schema:'blackjack-replay-result-v1',state:replay}));});
    assert.equal(url,'/api/action');const next=snapshot('playing',1);next.revision=8;return response(next);
  });
  const pending=upload(b,JSON.stringify({schema:'blackjack-private-replay-v1'}));await tick();
  assert.equal(b.ids.get('replay-file').disabled,true);assert.equal(b.run('busy'),false);
  await b.run("command('hit')");assert.equal(b.run('state.revision'),8);
  const before=b.run('JSON.stringify(state)'),history=b.ids.get('history').children,cardKey=b.run('lastCardKey');
  completePlayback();await pending;
  assert.equal(b.run('JSON.stringify(state)'),before);assert.equal(b.ids.get('history').children,history);
  assert.equal(b.run('lastRevision'),8);assert.equal(b.run('lastCardKey'),cardKey);
  assert.equal(b.run('busy'),false);assert.equal(b.ids.get('replay-file').disabled,false);
  assert.equal(b.ids.get('replay-result-dialog').open,true);assert.match(texts(b.ids.get('replay-result')),/onerror/);
  assert.equal(walk(b.ids.get('replay-result')).some(n=>['IMG','SCRIPT','INPUT','BUTTON'].includes(n.tagName)),false);
  const posted=JSON.parse(b.calls.find(c=>c.url==='/api/replay').body);assert.deepEqual(Object.keys(posted),['replay']);
});

test('upload limits, JSON errors, invalid replay snapshots and server errors leave live state untouched',async()=>{
  const b=await browser();const before=b.run('JSON.stringify(state)');let reads=0;
  b.context.uploadFile={size:1024*1024+1,text:async()=>{reads++;return '{}';}};
  await b.run('uploadReplay(uploadFile)');assert.equal(reads,0);assert.match(b.ids.get('session-tools-status').textContent,/1 MiB/);
  for(const text of ['not json','[]','null',JSON.stringify({padding:'x'.repeat(1024*1024-14)})])await upload(b,text);
  assert.equal(b.calls.length,1);assert.match(b.ids.get('session-tools-status').textContent,/wrapper/);
  b.handle(async()=>response({error:'Invalid state hash'},400));await upload(b,'{}');
  assert.equal(b.ids.get('session-tools-status').textContent,'Invalid state hash');
  b.handle(async()=>response({schema:'blackjack-replay-result-v1',state:snapshot('playing')}));await upload(b,'{}');
  assert.match(b.ids.get('session-tools-status').textContent,/completed snapshot/);
  b.handle(async()=>response({schema:'unknown',state:snapshot()}));await upload(b,'{}');
  assert.match(b.ids.get('session-tools-status').textContent,/response format/);
  assert.equal(b.ids.get('replay-result-dialog').open,undefined);
  assert.equal(b.run('JSON.stringify(state)'),before);assert.equal(b.run('offline'),false);assert.equal(b.run('busy'),false);
});

test('download failures and overlapping requests do not share live busy state',async()=>{
  const b=await browser();let complete;
  b.handle(async()=>new Promise(resolve=>{complete=()=>resolve(response({error:'Session expired; reload your table'},409));}));
  const pending=b.run("downloadSession('report')");await tick();
  const count=b.calls.length;await b.run("downloadSession('replay')");assert.equal(b.calls.length,count);
  assert.equal(b.run('busy'),false);assert.equal(b.ids.get('replay-file').disabled,false);
  complete();await pending;
  assert.match(b.ids.get('session-tools-status').textContent,/Session expired/);assert.equal(b.run('offline'),false);
  assert.equal(b.ids.get('report-download').disabled,false);
});
