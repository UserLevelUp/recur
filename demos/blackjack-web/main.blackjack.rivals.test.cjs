'use strict';
const test=require('node:test'), assert=require('node:assert/strict');
class Element {
  constructor(tag){this.tagName=tag;this.children=[];this.attrs={};this.textContent='';this.className='';this.hidden=false;}
  append(...children){this.children.push(...children);}
  replaceChildren(...children){this.children=[...children];}
  setAttribute(k,v){this.attrs[k]=String(v);}
}
global.document={createElement:tag=>new Element(tag)};
global.window={};
const view=require('./main.blackjack.rivals.js');
const texts=n=>[n.textContent,...n.children.flatMap(texts)].join(' ');
test('schema and transport versions',()=>{
  assert.equal(view.accepts('blackjack-web-state-v2'),true);
  assert.equal(view.accepts('blackjack-web-state-v3'),true);
  assert.equal(view.accepts('unknown'),false);
  assert.equal(view.protocolVersion({schema:'blackjack-web-state-v2'}),2);
  assert.equal(view.protocolVersion({schema:'blackjack-web-state-v3',protocol_version:3}),3);
});

const walk = n => [n, ...n.children.flatMap(walk)];
const fixture = () => ({id:2,name:'Computer rival',balance:490,net:-10,escrow:10,
  policy:'Stand on 17 benchmark',result:'loss',profit:-10,
  hands:[{id:1,cards:[1,26,27,52],score:{total:7,soft:true,bust:false,natural:false},
    stake:10,status:'settled',result:'loss',profit:-10}]});

test('exports work in a browser without CommonJS and in Node without a DOM',()=>{
  const vm = require('node:vm');
  const source = require('node:fs').readFileSync(require.resolve('./main.blackjack.rivals.js'),'utf8');
  const browser = {window:{}};
  vm.runInNewContext(source,browser);
  assert.equal(typeof browser.window.BlackjackRivalsView.render,'function');
  const node = {module:{exports:{}}};
  vm.runInNewContext(source,node);
  assert.equal(node.module.exports.accepts('blackjack-web-state-v3'),true);
  assert.equal(window.BlackjackRivalsView,view);
});

test('version selection is schema based and unsupported schemas fail explicitly',()=>{
  for(const schema of [null,undefined,'blackjack-web-state-v1',3,{},'blackjack-web-state-v30']){
    assert.equal(view.accepts(schema),false);
    assert.throws(()=>view.protocolVersion({schema}),TypeError);
  }
  assert.throws(()=>view.protocolVersion(null),TypeError);
  assert.equal(view.protocolVersion({schema:'blackjack-web-state-v3'}),3);
  assert.equal(view.protocolVersion({schema:'blackjack-web-state-v2',protocol_version:3}),2);
});

test('repeated renders replace seats and zero rivals clear and hide, then recover',()=>{
  const box=new Element('section');
  view.render(box,[fixture(),fixture()]);
  assert.equal(box.children.length,2);
  assert.equal(box.hidden,false);
  view.render(box,[fixture()]);
  assert.equal(box.children.length,1);
  for(const empty of [[],undefined,null]){
    view.render(box,empty);
    assert.equal(box.children.length,0);
    assert.equal(box.hidden,true);
  }
  view.render(box,[fixture()]);
  assert.equal(box.hidden,false);
  assert.equal(box.children.length,1);
});

test('renders accessible cards and authoritative scores and money without mutation',()=>{
  const rival=fixture();
  function freeze(x){if(x && typeof x==='object'){Object.values(x).forEach(freeze);Object.freeze(x);}return x;}
  freeze(rival);
  const box=new Element('section');
  view.render(box,Object.freeze([rival]));
  const content=texts(box);
  for(const expected of ['Bankroll: 490','Net: -10','In play: 10','Round profit: -10',
    'Score: 7 · Soft','A of spades','K of hearts','A of clubs','K of diamonds',
    'Bet: 10','Status: settled','Outcome: loss']) assert.ok(content.includes(expected),expected);
  assert.equal(box.children[0].attrs['aria-label'],'Computer rival: Computer rival');
  assert.ok(walk(box).some(n=>n.tagName==='h3' && n.className==='rival-heading'));
  assert.ok(walk(box).some(n=>n.tagName==='ul' && n.attrs['aria-label']==='Rival hand 1 cards'));
  assert.equal(walk(box).filter(n=>n.tagName==='li').length,4);
  assert.equal(walk(box).some(n=>['button','input'].includes(n.tagName)),false);
});

test('snapshot text stays inert and rendering uses the container document',()=>{
  const box=new Element('section');
  box.ownerDocument={createElement(tag){
    const element=new Element(tag);
    Object.defineProperty(element,'innerHTML',{set(){throw new Error('Unsafe HTML write');}});
    return element;
  }};
  const rival=fixture(), attack='<img src=x onerror=alert(1)>';
  rival.name=attack;rival.policy=attack;rival.result=attack;rival.hands[0].status=attack;
  view.render(box,[rival]);
  assert.ok(texts(box).includes(attack));
  assert.equal(box.children[0].attrs['aria-label'],'Computer rival: '+attack);
  assert.equal(walk(box).some(n=>['img','script'].includes(n.tagName)),false);
});

test('empty hands, hidden scores, zero amounts and supplied result flags are preserved',()=>{
  const box=new Element('section'), rival=fixture();
  rival.balance=0;rival.net=0;rival.profit=0;rival.escrow=0;rival.hands=[];rival.result='sitting_out';
  view.render(box,[rival]);
  for(const expected of ['Bankroll: 0','Net: 0','Round profit: 0','No hand in play.','sitting_out'])
    assert.ok(texts(box).includes(expected),expected);
  rival.hands=[{id:1,cards:[null,0,53],score:null}];
  view.render(box,[rival]);
  assert.ok(texts(box).includes('Score: —'));
  assert.ok(texts(box).includes('Face-down card'));
  assert.ok(texts(box).includes('Unknown card'));
  for(const score of [{total:22,bust:true},{total:21,natural:true}]){
    rival.hands[0].score=score;rival.hands[0].cards=[];
    view.render(box,[rival]);
    assert.ok(texts(box).includes(score.bust?'22 · Bust':'21 · Blackjack'));
    assert.ok(texts(box).includes('No cards dealt.'));
  }
});
test('zero, one and two rival panels are safe and descriptive',()=>{
  const box=new Element('section'); view.render(box,[]);
  for(const count of [1,2]){
    const rivals=Array.from({length:count},(_,i)=>({id:i+1,name:i?'Rival <two>':'Rival one',balance:510,starting:500,escrow:0,net:10,profit:10,result:'win',policy:'Stand on 17 benchmark',stats:{rounds:1,hands:1,wins:1,losses:0,pushes:0},hands:[{id:1,cards:[10,9],score:{total:19,bust:false,soft:false,natural:false},stake:10,status:'settled',result:'win',profit:10}]}));
    view.render(box,rivals);
    assert.ok(texts(box).includes('Rival one'));
    assert.ok(texts(box).includes('510'));
    assert.ok(texts(box).includes('Stand on 17 benchmark'));
    if(count===2)assert.ok(texts(box).includes('Rival <two>'));
    assert.ok(box.children.length>=count);
  }
});
