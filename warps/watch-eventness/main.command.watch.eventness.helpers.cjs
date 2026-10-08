'use strict';
const fs=require('node:fs'),path=require('node:path'),os=require('node:os'),cp=require('node:child_process'),assert=require('node:assert/strict');
const ROOT=path.resolve(__dirname,'../..');
const suffix=process.platform==='win32'?'.exe':'';
const CORE=process.env.RECUR_BIN||path.join(ROOT,'target',process.env.RECUR_PROFILE||'release-safe','recur'+suffix);
const WATCH=process.env.RECUR_WATCH_BIN||path.join(ROOT,'target',process.env.RECUR_PROFILE||'release-safe','recur-watch'+suffix);
const WARP='demo.watch.eventness',UUID='01a11724-8143-7837-85b1-5a94bc2a2c0d',TOPIC=WARP+'.results',SUB='coordinator';
function fixture(t){
 const tempParent=fs.realpathSync(os.tmpdir()),root=fs.mkdtempSync(path.join(tempParent,'recur-watch-eventness-'));
 t.after(()=>{assert.equal(path.dirname(root),tempParent);assert.ok(path.basename(root).startsWith('recur-watch-eventness-'));fs.rmSync(root,{recursive:true,force:true});});
 fs.mkdirSync(path.join(root,'warps'));fs.mkdirSync(path.join(root,'eventness'));
 fs.writeFileSync(path.join(root,'warps',WARP+'.warp-map.json'),JSON.stringify({schema:'warp-bubble-map-v1',warp_id:WARP,bubble_uuid:UUID,goal:'Watch fixture',required_slices:[{slice_id:'work',contract_hash:'work:v1',evidence_gates:['review'],evidence_mode:'declared'}]}));
 return root;
}
function invoke(binary,root,args){const r=cp.spawnSync(binary,[...args,'-d',root,'--json'],{encoding:'utf8',timeout:10000,windowsHide:true});if(r.error)throw r.error;return {status:r.status,stdout:r.stdout,stderr:r.stderr};}
function success(r){assert.equal(r.status,0,`Native command unavailable or failed: ${r.stderr}`);return JSON.parse(r.stdout);}
function create(root){return success(invoke(WATCH,root,['topic','create',TOPIC,'--warp',WARP,'--filter',WARP+'.**.ready','--eventness-dir','eventness','--confirm']));}
function subscribe(root){return success(invoke(WATCH,root,['topic','subscribe',TOPIC,'--id',SUB,'--confirm']));}
function drain(root){return invoke(WATCH,root,['topic','drain',TOPIC,'--id',SUB,'--max-events','10','--confirm']);}
function notifications(r){const value=success(r);assert.ok(Array.isArray(value));for(const n of value){assert.deepEqual(Object.keys(n),['trace_ids']);assert.ok(n.trace_ids.length>0);assert.equal(new Set(n.trace_ids).size,n.trace_ids.length);for(const id of n.trace_ids)assert.match(id,/^[A-Za-z0-9_]+(?:\.[A-Za-z0-9_]+)+$/);}return value.flatMap(n=>n.trace_ids);}
function publish(root,name='alpha',options={}){
 const id=WARP+'.'+name+'.ready',file=path.join(root,'eventness',`main.watch.${name}.${options.revision||1}.complete.md`);
 const refs=options.refs||[];
 fs.writeFileSync(file,`artifact.type = lane\npublish: ${id} ready artifact\nwarp.id = ${options.warp||WARP}\nwarp.uuid = ${options.uuid||UUID}\nattempt.id = ${options.revision||1}\nresult.revision = ${options.revision||1}\nartifact.refs = ${JSON.stringify(refs)}\n\n${options.body||'Useful persisted intelligence; never copied into notifications.'}\n`);
 return {id,file};
}
function snapshot(root){const records={};function walk(dir){for(const e of fs.readdirSync(dir,{withFileTypes:true})){const f=path.join(dir,e.name);if(e.isDirectory())walk(f);else records[path.relative(root,f)]=fs.readFileSync(f).toString('base64');}}walk(root);return records;}
module.exports={fs,path,assert,ROOT,CORE,WATCH,WARP,UUID,TOPIC,SUB,fixture,invoke,success,create,subscribe,drain,notifications,publish,snapshot};
