'use strict';
const test=require('node:test');
const {fs,path,assert,ROOT,CORE,fixture,invoke,success,snapshot}=require('./main.command.watch.eventness.helpers.cjs');
const dir=__dirname;
function validatePlan(p){assert.equal(p.schema,'watch-eventness-test-plan-v1');assert.equal(p.warp,'main.command.watch.eventness');assert.deepEqual(p.notification_keys,['trace_ids']);assert.equal(p.cases.length,11);assert.equal(new Set(p.cases.map(c=>c.id)).size,11);assert.ok(p.cases.every(c=>typeof c.purpose==='string'&&c.purpose.length>10));assert.ok(p.integration_pending.includes('crash-publication-ack-boundary'));}
test('test plan retains eleven distinct obligations and explicit remaining integration work',()=>{validatePlan(JSON.parse(fs.readFileSync(path.join(dir,'main.command.watch.eventness.cases.json'),'utf8')));});
test('negative controls reject duplicate obligations and an intelligence-carrying notification shape',()=>{const p=JSON.parse(fs.readFileSync(path.join(dir,'main.command.watch.eventness.cases.json'),'utf8'));const duplicate=structuredClone(p);duplicate.cases[1].id=duplicate.cases[0].id;assert.throws(()=>validatePlan(duplicate));const payload=structuredClone(p);payload.notification_keys.push('report');assert.throws(()=>validatePlan(payload));});
test('map starts at initial and implementation/final require checked, non-skipped test evidence',()=>{
 const map=JSON.parse(fs.readFileSync(path.join(ROOT,'warps/main.command.watch.eventness.warp-map.json'),'utf8'));
 assert.equal(map.warp_id,'main.command.watch.eventness');assert.equal(map.current_slice,'initial');assert.match(map.bubble_uuid,/^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/);
 assert.deepEqual(map.required_slices.map(s=>s.slice_id),['initial','tests','implementation','integration','final']);
 for(const slice of map.required_slices.slice(2)){assert.equal(slice.evidence_mode,'checked');assert.ok(slice.evidence_gates.length>0);assert.ok(Object.values(slice.gate_rules).some(g=>g.kind==='test'&&g.allow_skipped===false));}
});
test('legacy core Watch list is a working no-write positive control',t=>{const root=fixture(t),before=snapshot(root);const value=success(invoke(CORE,root,['watch','list']));assert.deepEqual(value,[]);assert.deepEqual(snapshot(root),before);});
