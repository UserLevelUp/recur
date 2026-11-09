'use strict';
// trigger: main.improvement.34.implementation.trace.context.verify
// Explicit local verification; outputs are create-new and accepted separately.
const fs=require('node:fs'),path=require('node:path'),os=require('node:os'),cp=require('node:child_process'),crypto=require('node:crypto'),assert=require('node:assert/strict');
const root=path.resolve(__dirname,'../../..'), rel='warps/improvement34/trace-context';
const id='main.improvement.34.implementation.trace.context';
const julia=process.argv[2], prefix=process.argv[3];
assert(julia&&/^context-v[0-9]+$/.test(prefix),'usage: node verify.cjs JULIA context-vN');
const bundle=process.argv[5] || `.recur/evidence-binaries/improvement34-${prefix}`;
const reuse=!!process.argv[5];
const abs=p=>path.join(root,p),read=p=>fs.readFileSync(abs(p));
const put=(p,data)=>fs.writeFileSync(abs(p),typeof data==='string'?data:JSON.stringify(data,null,2)+'\n',{flag:'wx'});
const sha=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
const fp=bytes=>{let h=14695981039346656037n;for(const b of bytes)h=BigInt.asUintN(64,(h^BigInt(b))*1099511628211n);return 'fnv1a64:'+h.toString(16).padStart(16,'0')};
const run=(exe,args,options={})=>{const r=cp.spawnSync(exe,args,{cwd:root,encoding:'utf8',maxBuffer:16*1024*1024,...options});if(r.error)throw r.error;return r};
const checked=(exe,args)=>{const r=run(exe,args);assert.equal(r.status,0,r.stderr);return r.stdout.trim()};
const out=`${rel}/observations/${prefix}`;
if(!reuse)fs.mkdirSync(abs(bundle));
function retain(source,dest){
 if(reuse){assert.equal(sha(read(source)),sha(read(dest)),'reuse requires identical retained executable bytes')}
 else fs.copyFileSync(abs(source),abs(dest),fs.constants.COPYFILE_EXCL);
}
const executables={};
for(const name of ['recur','recur-warp','recur-git']){
 const p=`${bundle}/${name}.exe`;
 retain(`target/release-safe/${name}.exe`,p);
 executables[name]={path:p,sha256:sha(read(p))};
}
const old=JSON.parse(read('warps/improvement34/trace/observations/trace-v2.evidence.json'));
const previous=process.argv[4]?JSON.parse(read(process.argv[4])):old;
const sources=[...new Set([...Object.keys(old.source.files),
 ...Object.keys(previous.source.files),
 ...fs.readdirSync(abs('src'),{recursive:true}).filter(p=>p.endsWith('.rs')).map(p=>'src/'+p.replaceAll('\\','/')),
 `${rel}/${id}.contract.md`,`${rel}/${id}.warp-map.json`,`${rel}/${id}.verify.cjs`,
 'julia-tests/main.improvement.34.trace-context.test.jl','julia-tests/runtests.jl',
 'warps/improvement34/trace/observations/review-v1.separator.test.jl',
 ...Object.values(executables).map(x=>x.path)
])].sort();
const fingerprints=()=>Object.fromEntries(sources.map(p=>[p,fp(read(p))]));
const before=fingerprints();
const revision=checked('git',['rev-parse','HEAD']);
const dirty=!!checked('git',['status','--porcelain']);
const observations={};
function juliaSuite(name,file){
 const args=['--project='+abs('demos/web-evidence-lab'),abs(file)];
 const r=run(julia,args,{cwd:fs.mkdtempSync(path.join(os.tmpdir(),'recur-context-tests-')),env:{...process.env,RECUR_BIN:abs(executables.recur.path)}});
 const log=`${out}.${name}.log.txt`;put(log,r.stdout+r.stderr);
 const rows=r.stdout.split(/\r?\n/),i=rows.findLastIndex(x=>x.startsWith('Test Summary:'));
 assert(i>=0,`missing ${name} summary; see ${log}`);
 const headers=rows[i].split('|')[1].trim().split(/\s+/),values=rows[i+1].split('|')[1].trim().split(/\s+/);
 const counts=Object.fromEntries(headers.filter(h=>h!=='Time').map(h=>[h,Number(values[headers.indexOf(h)])]));
 const record={command:[julia,...args],exit_code:r.status,counts,log};
 observations[name]=record;console.log(JSON.stringify({suite:name,...record}));
 assert.equal(r.status,0,`failed ${name}; see ${log}`);
 return counts;
}
juliaSuite('context','julia-tests/main.improvement.34.trace-context.test.jl');
juliaSuite('trace','julia-tests/main.improvement.34.trace.test.jl');
juliaSuite('separator','warps/improvement34/trace/observations/review-v1.separator.test.jl');
juliaSuite('baseline','julia-tests/main.improvement.34.baseline.test.jl');
juliaSuite('legacy','julia-tests/main.command.trace-id.test.jl');
const cargoArgs=['test','--locked','--lib','--bin','recur'];
let rust, rustCommands;
if(reuse){
 // Re-execute the exact test harnesses only when their Rust build inputs have
 // not changed. Non-code evidence/map updates need no Windows relink.
 for(const p of sources.filter(p=>p.startsWith('src/')||['Cargo.toml','Cargo.lock','build.rs'].includes(p))){
  assert.equal(fp(read(p)),previous.source.files[p],`Rust build input changed; cannot reuse test harness: ${p}`);
 }
 rustCommands=[0,1].map(i=>[abs(`${bundle}/rust-tests-${i}.exe`)]);
 const runs=rustCommands.map(([exe])=>run(exe,[]));
 rust={status:runs.every(r=>r.status===0)?0:1,stdout:runs.map(r=>r.stdout).join('\n'),stderr:runs.map(r=>r.stderr).join('\n')};
 for(let i=0;i<2;i++){
  const p=`${bundle}/rust-tests-${i}.exe`;
  assert.equal(fp(read(p)),previous.source.files[p],'retained test harness changed');
  executables[`rust-tests-${i}`]={path:p,sha256:sha(read(p))};
 }
}else{
 rust=run('cargo',cargoArgs);rustCommands=[['cargo',...cargoArgs]];
}
put(`${out}.rust.log.txt`,rust.stdout+rust.stderr);
const counts=[...rust.stdout.matchAll(/test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;/g)].map(m=>({passed:+m[1],failed:+m[2],ignored:+m[3]}));
assert.equal(rust.status,0,`Rust tests failed; see ${out}.rust.log.txt`);assert.equal(counts.length,2);
observations.rust={commands:rustCommands,unchanged_build_inputs:reuse,exit_code:rust.status,counts,log:`${out}.rust.log.txt`};
for(const [i,m] of (reuse?[]:[...rust.stderr.matchAll(/Running unittests [^\r\n]*?\((target[^)]+\.exe)\)/g)]).entries()){
 const source=m[1].replaceAll('\\','/'),p=`${bundle}/rust-tests-${i}.exe`;
 retain(source,p);
 executables[`rust-tests-${i}`]={path:p,source,sha256:sha(read(p))};
 assert.equal(sha(read(source)),executables[`rust-tests-${i}`].sha256);
}
assert.equal(Object.keys(executables).length,5,'retain both actual Rust test executables');
assert.deepEqual(fingerprints(),before,'source changed during verification');
for(const x of Object.values(executables))assert.equal(sha(read(x.path)),x.sha256);
const metadata=`${out}.runs.json`;
put(metadata,{revision,dirty,observations,executables,runtimes:{julia:checked(julia,['--version']),cargo:checked('cargo',['--version']),rustc:checked('rustc',['-Vv'])}});
const primary=['context','trace','separator'].map(name=>observations[name].counts);
for(const c of primary)assert.equal(c.Pass,c.Total,'primary Julia gate permits no skips');
for(const c of counts){assert.equal(c.failed,0);assert.equal(c.ignored,0)}
const total=primary.reduce((n,c)=>n+c.Pass,0)+counts.reduce((n,c)=>n+c.passed,0);
const result=`${out}.result.json`;
put(result,{schema:'warp-external-result-v1',kind:'test',outcome:'passed',exit_code:0,tests:{discovered:total,executed:total,passed:total,failed:0,skipped:0}});
const additional=[metadata,...Object.values(observations).map(x=>x.log),...Object.values(executables).map(x=>x.path)];
const sourceFiles={...before,...Object.fromEntries(additional.map(p=>[p,fp(read(p))]))};
const manifest=`${out}.evidence.json`;
put(manifest,{schema:'warp-external-evidence-v1',kind:'test',producer:'Julia trace context CLI and Rust core tests',project:'recur',configuration:'release-safe retained CLI; explicit core tests; legacy skipped placeholders excluded from gate',platform:process.platform,executed_at_unix:Math.floor(Date.now()/1000),result_artifact:result,result_fingerprint:fp(read(result)),source:{revision,dirty,files:sourceFiles}});
const live=JSON.parse(checked(abs(executables.recur.path),['warp','evidence',manifest,'-d',root,'--json']));
assert.equal(live.status,'checked');
put(`${out}.checked.json`,live);
console.log(JSON.stringify({manifest,source_files:Object.keys(sourceFiles).length,result_hash:'sha256:'+sha(read(result)),total,live}));
