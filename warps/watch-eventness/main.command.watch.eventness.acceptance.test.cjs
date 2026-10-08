'use strict';
// trigger: main.command.watch.eventness.final source-bound acceptance controls
const test=require('node:test'),crypto=require('node:crypto');
const {fs,path,assert,ROOT,CORE,fixture,invoke,success}=require('./main.command.watch.eventness.helpers.cjs');
const OBS=path.join(__dirname,'observations');
const BUNDLE=process.env.RECUR_WATCH_EVIDENCE_PREFIX||'watch-native-v7';
assert.match(BUNDLE,/^[A-Za-z0-9._-]+$/);
const bundleApi=require('../watch-evidence/main.command.watch.eventness.evidence.bundle.cjs');
const retained=JSON.parse(fs.readFileSync(path.join(OBS,BUNDLE+'.binaries.json')));
function fnv(bytes){let h=0xcbf29ce484222325n;for(const b of bytes)h=BigInt.asUintN(64,(h^BigInt(b))*0x100000001b3n);return 'fnv1a64:'+h.toString(16).padStart(16,'0');}
for(const mode of ['native','integration','rust','julia'])test(mode+': current evidence is checked with no missing or skipped tests',()=>{
 const rel='warps/watch-eventness/observations/'+BUNDLE+'.'+mode+'.evidence.json';
 const assessment=success(invoke(CORE,ROOT,['warp','evidence',rel]));
 assert.equal(assessment.status,'checked',JSON.stringify(assessment));
 const run=JSON.parse(fs.readFileSync(path.join(OBS,BUNDLE+'.'+mode+'.run.json')));
 assert.equal(run.exit_code,0);assert.ok(run.outcome.tests.passed>0);assert.equal(run.outcome.tests.failed,0);assert.equal(run.outcome.tests.skipped,0);
 bundleApi.verify(ROOT,retained);
 assert.equal(run.binary_bundle,'warps/watch-eventness/observations/'+BUNDLE+'.binaries.json');
 if(mode!=='rust'){assert.equal(run.binary_binding,'retained executables; verified before and after');assert.equal(run.selected_environment.RECUR_BIN,bundleApi.paths(ROOT,retained).core);assert.equal(run.selected_environment.RECUR_WATCH_BIN,bundleApi.paths(ROOT,retained).watch);assert.equal(run.selected_environment.WATCH_TEST_BIN,bundleApi.paths(ROOT,retained).fault);}
 for(const [binary,hash]of Object.entries(run.binaries_sha256))assert.equal(crypto.createHash('sha256').update(fs.readFileSync(binary)).digest('hex'),hash,'binary drift: '+binary);
});
test('negative control: source drift and failed results cannot masquerade as checked',t=>{
 const root=fixture(t);fs.writeFileSync(path.join(root,'input.txt'),'current');
 const result={schema:'warp-external-result-v1',kind:'test',outcome:'passed',exit_code:0,tests:{discovered:1,executed:1,passed:1,failed:0,skipped:0}};
 fs.writeFileSync(path.join(root,'result.json'),JSON.stringify(result));
 const evidence={schema:'warp-external-evidence-v1',kind:'test',producer:'negative control fixture',project:'fixture',configuration:'test',platform:process.platform,executed_at_unix:Math.floor(Date.now()/1000),result_artifact:'result.json',result_fingerprint:fnv(fs.readFileSync(path.join(root,'result.json'))),source:{revision:null,dirty:true,files:{'input.txt':fnv(Buffer.from('current'))}}};
 const write=()=>fs.writeFileSync(path.join(root,'evidence.json'),JSON.stringify(evidence));write();
 const assess=()=>success(invoke(CORE,root,['warp','evidence','evidence.json']));
 assert.equal(assess().status,'checked');fs.writeFileSync(path.join(root,'input.txt'),'changed');assert.equal(assess().status,'stale');
 fs.writeFileSync(path.join(root,'input.txt'),'current');result.outcome='failed';result.exit_code=1;result.tests={discovered:1,executed:1,passed:0,failed:1,skipped:0};fs.writeFileSync(path.join(root,'result.json'),JSON.stringify(result));evidence.result_fingerprint=fnv(fs.readFileSync(path.join(root,'result.json')));write();assert.equal(assess().status,'failed');
});
test('historical expected-red contract and native suite remain frozen',()=>{
 const baseline=JSON.parse(fs.readFileSync(path.join(OBS,'baseline.json')));
 for(const name of ['main.command.watch.eventness.contract.md','main.command.watch.eventness.native.red.test.cjs','main.command.watch.eventness.helpers.cjs']){
  const rel='warps/watch-eventness/'+name;
  assert.equal(crypto.createHash('sha256').update(fs.readFileSync(path.join(ROOT,rel))).digest('hex'),baseline.inputs_sha256[rel]);
 }
 assert.equal(baseline.native_semantics_exercised,false);
});
test('skills retain narrow names and optional native workflow guidance',()=>{
 for(const name of ['recur-watch','recur-expert']){
  const text=fs.readFileSync(path.join(ROOT,name,'SKILL.md'),'utf8');
  assert.match(text,new RegExp('^---\\r?\\nname: '+name+'\\r?\\ndescription: [^\\r\\n]+\\r?\\n---\\r?\\n'));
  assert.match(text,/recur watch topics/);
 }
 assert.match(fs.readFileSync(path.join(ROOT,'recur-watch/SKILL.md'),'utf8'),/Trace IDs and topics are not required/);
});
