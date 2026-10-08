'use strict';
// consumer: demo.blackjack.rivals.coordination deterministic CLI verification
const {spawnSync}=require('node:child_process');
const path=require('node:path');
const cwd=path.resolve(__dirname,'..');
const env={...process.env,RECUR_BIN:'C:/src/recur/target/debug/recur.exe'};
for(const [program,args] of [
  [process.execPath,['--test','main.blackjack.rivals.test.cjs']],
  ['C:/Users/marcn/.julia/juliaup/julia-1.12.7+0.x64.w64.mingw32/bin/julia.exe',
   ['--startup-file=no','-O0','-C','generic','--project=../web-evidence-lab','main.blackjack.web.test.jl']]
]){
  const result=spawnSync(program,args,{cwd,env,stdio:'inherit',windowsHide:true});
  if(result.error){console.error(result.error.message);process.exit(2);}
  if(result.status!==0)process.exit(result.status===null?2:result.status);
}
