import fs from 'node:fs/promises';
import vm from 'node:vm';
import path from 'node:path';
import assert from 'node:assert/strict';
import {fileURLToPath} from 'node:url';

const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const code=await fs.readFile(path.join(root,'ui','bridge.js'),'utf8');
let checks=0;
function fixture(mode='wry'){
  const packets=[],handlers={};const window={addEventListener:(name,fn)=>handlers[name]=fn};
  if(mode==='wry')window.ipc={postMessage:raw=>packets.push(JSON.parse(raw))};
  if(mode==='chrome'||mode==='wry')window.chrome={webview:{postMessage:packet=>packets.push(packet),addEventListener:(name,fn)=>handlers['chrome-'+name]=fn}};
  const context=vm.createContext({window,chrome:window.chrome,console,Map,Set,Error,TypeError,Number,Array,Object,JSON,Promise,setTimeout});vm.runInContext(code,context);
  return {window,packets,handlers,api:window.electron};
}
function plain(value){return JSON.parse(JSON.stringify(value));}
for(const mode of ['wry','chrome']){
  const f=fixture(mode),p=f.api.invoke('store','get','uiVolume');assert.deepEqual(plain(f.packets[0]),{kind:'invoke',id:1,channel:'store',args:['get','uiVolume']});checks++;
  f.window.__panopticonReceive({kind:'result',id:1,ok:true,value:0.3});assert.equal(await p,0.3);checks++;
  const absent=f.api.invoke('getGmodPid');f.window.__panopticonReceive(JSON.stringify({kind:'result',id:2,ok:true,isUndefined:true}));assert.equal(await absent,undefined);checks++;
  const nullable=f.api.invoke('getProtocol');f.window.__panopticonReceive({kind:'result',id:3,ok:true,value:null});assert.equal(await nullable,null);checks++;
  const rejected=f.api.invoke('startGmod');f.window.__panopticonReceive({kind:'result',id:4,ok:false,error:{message:'Реальный отказ',code:'LAUNCH_FAILED'}});await assert.rejects(rejected,e=>e.message==='Реальный отказ'&&e.code==='LAUNCH_FAILED');checks++;
  let event;function onNotify(...args){event=args;}assert.equal(f.api.on('notify',onNotify),undefined);f.window.__panopticonReceive({kind:'event',channel:'notify',args:['error',{content:'Ошибка'}]});assert.deepEqual(event,['error',{content:'Ошибка'}]);checks++;
  f.api.removeListener('notify',onNotify);event=null;f.window.__panopticonReceive({kind:'event',channel:'notify',args:['error',{}]});assert.equal(event,null);checks++;
  f.api.on('gmodUpdatePid',value=>event=value);f.window.__panopticonReceive({kind:'event',channel:'gmodUpdatePid',args:[null],undefinedArgs:[0]});assert.equal(event,undefined);checks++;
  f.api.send('window-control','minimize');assert.deepEqual(plain(f.packets.at(-1)),{kind:'send',channel:'window-control',args:['minimize']});checks++;
  assert.throws(()=>f.api.send('window-control','execute'),'Запрещена несуществующая команда');await assert.rejects(f.api.invoke('eval','untrusted'));checks++;
  assert.equal(Object.isFrozen(f.api),true);assert.deepEqual(Object.keys(f.api),['invoke','on','removeListener','send']);checks++;
  const before=f.packets.length;vm.runInContext(code,vm.createContext({window:f.window,chrome:f.window.chrome,console}));assert.equal(f.packets.length,before);checks++;
  const abandoned=f.api.invoke('openDialog');f.handlers.beforeunload();await assert.rejects(abandoned,/закрыто/);checks++;
  if(mode==='chrome'){let value;f.api.on('hubWindowVisible',v=>value=v);f.handlers['chrome-message']({data:{kind:'event',channel:'hubWindowVisible',args:[true]}});assert.equal(value,true);checks++;}
}
await assert.rejects(fixture('none').api.invoke('findSteam'),e=>e.code==='NATIVE_HOST_UNAVAILABLE');checks++;
const setup=fixture();
const phases=[];
let finishStyle;
setup.window.panopticonInitialStyleReady=()=>new Promise(resolve=>{phases.push('style');finishStyle=resolve;});
setup.window.panopticonWorkshopReady=async()=>{phases.push('workshop');};
const passed=setup.api.invoke('passedChecks');
setup.window.__panopticonReceive({kind:'result',id:1,ok:true,isUndefined:true});
await passed;
await new Promise(resolve=>setTimeout(resolve,0));
assert.deepEqual(phases,['style']);checks++;
finishStyle();
await new Promise(resolve=>setTimeout(resolve,0));
assert.deepEqual(phases,['style','workshop']);checks++;
await fs.mkdir(path.join(root,'tests','ui'),{recursive:true});await fs.writeFile(path.join(root,'tests','ui','bridge-result.json'),JSON.stringify({pass:true,checks,transports:['Wry JSON string','WebView2 structured packet'],undefined_preserved:true},null,2));
console.log(JSON.stringify({pass:true,checks}));
