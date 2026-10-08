'use strict';
// defines: main.command.watch.eventness.evidence.bundle exact retained artifact integrity
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto'),assert=require('node:assert/strict');
const ROLES={core:'recur',watch:'recur-watch',warp:'recur-warp',fault:'recur-watch-fault'};
const suffix=process.platform==='win32'?'.exe':'';
const sha=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
// Same FNV-1a64 identity used by Rust, with exact two-word multiplication.
function fingerprint(bytes){let hi=0xcbf29ce4,lo=0x84222325;for(const b of bytes){lo=(lo^b)>>>0;hi=(Math.imul(hi,435)+(lo<<8)+Math.floor(lo*435/4294967296))>>>0;lo=Math.imul(lo,435)>>>0;}return 'fnv1a64:'+hi.toString(16).padStart(8,'0')+lo.toString(16).padStart(8,'0');}
function validId(id){assert.ok(typeof id==='string'&&/^[A-Za-z0-9][A-Za-z0-9._-]{0,79}$/.test(id)&&!id.endsWith('.')&&!/^(?:CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])(?:\.|$)/i.test(id),'invalid bundle prefix');}
function contained(root,relative){
 assert.ok(typeof relative==='string'&&relative.length>0&&!relative.includes('\\')&&!path.isAbsolute(relative)&&relative.split('/').every(p=>p&&p!=='.'&&p!=='..'),'unsafe contained artifact path');
 const base=fs.realpathSync(root);let p=base;
 for(const component of relative.split('/')){p=path.join(p,component);if(fs.existsSync(p)){const stat=fs.lstatSync(p);assert.ok(!stat.isSymbolicLink(),'unsafe linked artifact path');const resolved=fs.realpathSync(p),rel=path.relative(base,resolved);assert.ok(rel!== '..'&&!rel.startsWith('..'+path.sep)&&!path.isAbsolute(rel),'artifact is not contained');}}
 return p;
}
function bytes(file){const stat=fs.lstatSync(file);assert.ok(stat.isFile()&&!stat.isSymbolicLink()&&stat.size>0&&stat.size<=32*1024*1024,'missing, unsafe or oversized binary');const b=fs.readFileSync(file);assert.equal(b.length,stat.size,'binary changed while reading');return b;}
function writeNew(file,value,mode=0o666){const fd=fs.openSync(file,'wx',mode);try{fs.writeFileSync(fd,value);fs.fsyncSync(fd);}finally{fs.closeSync(fd);}}
function create(root,id,inputs,provenance){
 validId(id);assert.deepEqual(Object.keys(inputs).sort(),Object.keys(ROLES).sort(),'required artifact roles: core, watch, warp, fault');
 const artifacts={},payloads={};
 for(const [role,name]of Object.entries(ROLES)){
  const rel=path.relative(fs.realpathSync(root),path.resolve(inputs[role])).replaceAll('\\','/');const file=contained(root,rel);payloads[role]=bytes(file);
  artifacts[role]={path:'.recur/evidence-binaries/'+id+'/'+name+suffix,sha256:sha(payloads[role]),source:rel};
 }
 const base=contained(root,'.recur/evidence-binaries');fs.mkdirSync(base,{recursive:true});contained(root,'.recur/evidence-binaries');
 const directory=contained(root,'.recur/evidence-binaries/'+id);fs.mkdirSync(directory); // create-new reservation; a partial bundle remains a blocker
 for(const [role,artifact]of Object.entries(artifacts)){const dest=contained(root,artifact.path);writeNew(dest,payloads[role],fs.statSync(inputs[role]).mode&0o777);}
 const manifest={schema:'recur-tested-binaries-v1',bundle:id,created_at:new Date().toISOString(),provenance,artifacts};
 writeNew(path.join(directory,'manifest.json'),JSON.stringify(manifest,null,2)+'\n');verify(root,manifest);return manifest;
}
function paths(root,manifest){
 assert.equal(manifest.schema,'recur-tested-binaries-v1');validId(manifest.bundle);assert.deepEqual(Object.keys(manifest.artifacts).sort(),Object.keys(ROLES).sort(),'required artifact roles');
 return Object.fromEntries(Object.entries(ROLES).map(([role,name])=>{const a=manifest.artifacts[role];assert.equal(a.path,'.recur/evidence-binaries/'+manifest.bundle+'/'+name+suffix,'unexpected binary binding');assert.match(a.sha256,/^[a-f0-9]{64}$/);return [role,contained(root,a.path)];}));
}
function verify(root,manifest){for(const [role,file]of Object.entries(paths(root,manifest)))assert.equal(sha(bytes(file)),manifest.artifacts[role].sha256,'binary drift: '+file);}
function guard(root,manifest,work){verify(root,manifest);const result=work(paths(root,manifest));verify(root,manifest);return result;}
function recorded(root,manifest,work){let result=null;try{guard(root,manifest,p=>{result=work(p);return result;});return {result,error:null};}catch(error){return {result,error:String(error)};}}
module.exports={create,verify,paths,guard,recorded,writeNew,contained,validId,sha,fingerprint};
