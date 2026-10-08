'use strict';
// trigger: main.command.watch.eventness.verification source-bound native checks
const fs=require('node:fs'),path=require('node:path'),cp=require('node:child_process');
const api=require('../watch-evidence/main.command.watch.eventness.evidence.bundle.cjs');
const ROOT=path.resolve(__dirname,'../..'),OBS=path.join(__dirname,'observations'),NODE=process.execPath;
const suffix=process.platform==='win32'?'.exe':'',BIN=path.join(ROOT,'target','release-safe');
const JULIA=process.env.JULIA_BIN||'C:/Users/marcn/.julia/juliaup/julia-1.12.7+0.x64.w64.mingw32/bin/julia.exe';
const mode=process.argv[2],bundle=process.env.RECUR_WATCH_EVIDENCE_PREFIX||'watch-native-v7';api.validId(bundle);
const prefix=bundle+'.'+mode,manifestPath=path.join(OBS,bundle+'.binaries.json');
const fingerprint=api.fingerprint;
function run(program,args,env=process.env){const r=cp.spawnSync(program,args,{cwd:ROOT,env,encoding:'utf8',timeout:600000,windowsHide:true,maxBuffer:32*1024*1024});if(r.error)throw r.error;return r;}
function command(program,args){const r=run(program,args);if(r.status!==0)throw Error(r.stderr);return r.stdout.trim();}
function sources(){
 const listed=command('git',['ls-files','src','tests','julia-tests']).split(/\r?\n/);
 const explicit=['.gitattributes','Cargo.toml','Cargo.lock','build.rs','recur-watch/SKILL.md','recur-expert/SKILL.md','julia-expert/references/recur-playbook.md','docs/main.command.watch.readme.md','docs/main.command.watch.eventness.readme.md','docs/main.command.watch.eventness.evidence.readme.md','docs/main.command.warp.readme.md','docs/main.command.warp.evidence-refresh.readme.md','docs/recur-watch.recur.md','warps/main.command.watch.eventness.warp-map.json','warps/main.command.watch.eventness.recur.md','demos/main.lang/main.lang.skippy-watch-coordination.recur','demos/main.lang/main.lang.algorithm-lab.recur','julia-tests/main.command.watch.eventness.test.jl','src/watch_eventness.rs','tests/warp_evidence_scope.rs'];
 for(const dir of ['watch-eventness','watch-evidence'])for(const name of fs.readdirSync(path.join(ROOT,'warps',dir)))if(/\.(?:cjs|jl|md|json)$/.test(name))explicit.push('warps/'+dir+'/'+name);
 if(fs.existsSync(path.join(ROOT,'.cargo')))for(const name of ['config','config.toml'])if(fs.existsSync(path.join(ROOT,'.cargo',name)))explicit.push('.cargo/'+name);
 const files={};for(const name of [...new Set([...listed,...explicit])].sort())if(name)files[name]=fingerprint(fs.readFileSync(path.join(ROOT,name)));return files;
}
const suites={
 native:[NODE,['--test','--test-reporter=tap','warps/watch-eventness/main.command.watch.eventness.native.red.test.cjs','warps/watch-eventness/main.command.watch.eventness.scaffold.test.cjs']],
 integration:[NODE,['--test','--test-reporter=tap','warps/watch-eventness/main.command.watch.eventness.integration.test.cjs']],
 rust:['cargo',['test','--profile','release-safe','--lib','--bins','--tests']],
 julia:[JULIA,['--project=demos/web-evidence-lab','warps/watch-eventness/main.command.watch.eventness.julia.verify.jl']],
 bundle:[NODE,['--test','--test-reporter=tap','warps/watch-evidence/main.command.watch.eventness.evidence.bundle.test.cjs']],
 final:[NODE,['--test','--test-reporter=tap','warps/watch-eventness/main.command.watch.eventness.acceptance.test.cjs']],
};
if(!Object.hasOwn(suites,mode))throw Error('Expected rust|native|integration|julia|bundle|final');
for(const ext of ['log.txt','result.json','evidence.json','run.json','reservation.json'])if(fs.existsSync(path.join(OBS,prefix+'.'+ext)))throw Error('Evidence output already exists: '+prefix+'.'+ext);
if(mode==='rust'&&fs.existsSync(manifestPath))throw Error('Binary bundle already exists; use a new evidence prefix');
let manifest=mode==='rust'?null:JSON.parse(fs.readFileSync(manifestPath));
if(manifest){if(manifest.bundle!==bundle)throw Error('Unexpected binary bundle identity');api.verify(ROOT,manifest);}
api.writeNew(path.join(OBS,prefix+'.reservation.json'),JSON.stringify({bundle,mode,started:new Date().toISOString()})+'\n');
const inputs=sources(),[program,args]=suites[mode],started=new Date().toISOString();
function bind(m){const p=api.paths(ROOT,m);return {...process.env,RECUR_BIN:p.core,RECUR_WATCH_BIN:p.watch,RECUR_WARP_BIN:p.warp,WATCH_TEST_BIN:p.fault,RECUR_WATCH_EVIDENCE_PREFIX:bundle};}
function bindInputs(m){inputs[path.relative(ROOT,manifestPath).replaceAll('\\','/')]=fingerprint(fs.readFileSync(manifestPath));for(const a of Object.values(m.artifacts))inputs[a.path]=fingerprint(fs.readFileSync(path.join(ROOT,a.path)));}
let selectedEnv=manifest?bind(manifest):null;if(manifest)bindInputs(manifest);
let buildLog='';
if(mode==='rust'){
 const hook=run('cargo',['build','--profile','release-safe','--features','watch-test-hooks','--bin','recur-watch']);
 buildLog='Fault-hook build:\n'+hook.stdout+'\n'+hook.stderr+'\n';
 if(hook.status!==0){api.writeNew(path.join(OBS,prefix+'.log.txt'),buildLog);throw Error('Fault-hook build failed; no passing evidence published');}
 const dest=path.join(ROOT,'target/watch-fault-tests/release-safe/recur-watch'+suffix);fs.mkdirSync(path.dirname(dest),{recursive:true});fs.copyFileSync(path.join(BIN,'recur-watch'+suffix),dest);
}
const checked=manifest?api.recorded(ROOT,manifest,()=>run(program,args,selectedEnv)):{result:run(program,args),error:null};
const r=checked.result||{status:-1,stdout:'',stderr:'Selected artifact verification failed before child execution'};
const verificationErrors=checked.error?[checked.error]:[];
if(mode==='rust'){
 manifest=api.create(ROOT,bundle,{core:path.join(BIN,'recur'+suffix),watch:path.join(BIN,'recur-watch'+suffix),warp:path.join(BIN,'recur-warp'+suffix),fault:path.join(ROOT,'target/watch-fault-tests/release-safe/recur-watch'+suffix)},
 {rustc:command('rustc',['--version','--verbose']),cargo:command('cargo',['--version']),profile:'release-safe',normal_features:['default'],fault_features:['default','watch-test-hooks'],commands:{normal:[program,...args],fault:['cargo','build','--profile','release-safe','--features','watch-test-hooks','--bin','recur-watch']},binary_binding:'Rust build products retained after Cargo; native and Julia suites execute retained paths',build_environment:Object.fromEntries(['RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','CARGO_BUILD_TARGET','CARGO_TARGET_DIR'].filter(k=>process.env[k]!==undefined).map(k=>[k,process.env[k]]))});
 api.writeNew(manifestPath,JSON.stringify(manifest,null,2)+'\n');
 selectedEnv=bind(manifest);bindInputs(manifest);
}
let log=buildLog+r.stdout+'\n'+r.stderr;
let passed=0,failed=0,skipped=0;
if(mode==='rust'){for(const m of log.matchAll(/test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;/g)){passed+=+m[1];failed+=+m[2];skipped+=+m[3];}}
else if(mode==='julia'){const lines=log.split(/\r?\n/),i=lines.findIndex(l=>l.startsWith('Watch affected core compatibility')&&l.includes('|'));if(i>0){const names=lines[i-1].split('|')[1].trim().split(/\s+/),values=lines[i].split('|')[1].trim().split(/\s+/),counts=Object.fromEntries(names.map((n,j)=>[n,+values[j]||0]));passed=counts.Pass||0;failed=(counts.Fail||0)+(counts.Error||0);skipped=counts.Broken||0;}}
else{passed=+(log.match(/# pass (\d+)/)||[])[1]||0;failed=+(log.match(/# fail (\d+)/)||[])[1]||0;skipped=+(log.match(/# skipped (\d+)/)||[])[1]||0;}
for(const [name,h]of Object.entries(inputs))try{if(fingerprint(fs.readFileSync(path.join(ROOT,name)))!==h)verificationErrors.push('Verification input drift: '+name);}catch(e){verificationErrors.push('Verification input unavailable: '+name+': '+e.message);}
try{api.verify(ROOT,manifest);}catch(e){if(!verificationErrors.includes(String(e)))verificationErrors.push(String(e));}
if(verificationErrors.length){failed+=1;log+='\nVerification failed:\n'+verificationErrors.join('\n')+'\n';}
const outcome={schema:'warp-external-result-v1',kind:'test',outcome:r.status===0&&passed>0&&failed===0&&skipped===0?'passed':'failed',exit_code:verificationErrors.length?1:r.status??-1,tests:{discovered:passed+failed+skipped,executed:passed+failed,passed,failed,skipped}};
api.writeNew(path.join(OBS,prefix+'.log.txt'),log);inputs['warps/watch-eventness/observations/'+prefix+'.log.txt']=fingerprint(Buffer.from(log));
const resultPath='warps/watch-eventness/observations/'+prefix+'.result.json';api.writeNew(path.join(ROOT,resultPath),JSON.stringify(outcome,null,2)+'\n');
const binaries=Object.fromEntries(Object.entries(manifest.artifacts).map(([role,a])=>[path.join(ROOT,a.path),a.sha256]));
const env=selectedEnv,record={started,finished:new Date().toISOString(),program,args,exit_code:outcome.exit_code,child_exit_code:r.status,verification_errors:verificationErrors,outcome,log:prefix+'.log.txt',log_sha256:api.sha(Buffer.from(log)),binary_bundle:path.relative(ROOT,manifestPath).replaceAll('\\','/'),binaries_sha256:binaries,binary_binding:mode==='rust'?'post-build products':'retained executables; verified before and after',selected_environment:Object.fromEntries(['RECUR_BIN','RECUR_WATCH_BIN','RECUR_WARP_BIN','WATCH_TEST_BIN','RECUR_WATCH_EVIDENCE_PREFIX'].map(k=>[k,env[k]]))};
const runPath='warps/watch-eventness/observations/'+prefix+'.run.json';api.writeNew(path.join(ROOT,runPath),JSON.stringify(record,null,2)+'\n');inputs[runPath]=fingerprint(fs.readFileSync(path.join(ROOT,runPath)));
const evidence={schema:'warp-external-evidence-v1',kind:'test',producer:program+' '+args.join(' '),project:'recur',configuration:'release-safe; exact retained binary bundle; native core only; demo suites not selected',platform:process.platform,executed_at_unix:Math.floor(Date.now()/1000),result_artifact:resultPath,result_fingerprint:fingerprint(fs.readFileSync(path.join(ROOT,resultPath))),source:{revision:command('git',['rev-parse','HEAD']),dirty:true,files:inputs}};
api.writeNew(path.join(OBS,prefix+'.evidence.json'),JSON.stringify(evidence,null,2)+'\n');
console.log(JSON.stringify({mode,...outcome,bundle,log:prefix+'.log.txt'}));if(outcome.outcome!=='passed')process.exit(1);
