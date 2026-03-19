function __wbg_get_imports(){
const import0={
__proto__:null,
__wbg___wbindgen_is_undefined_52709e72fb9f179c:function(arg0){
const ret=getObject(arg0)===undefined;
return ret;
},
__wbg___wbindgen_throw_6ddd609b62940d55:function(arg0,arg1){
throw new Error(getStringFromWasm0(arg0,arg1));
},
__wbg_now_e7c6795a7f81e10f:function(arg0){
const ret=getObject(arg0).now();
return ret;
},
__wbg_performance_3fcf6e32a7e1ed0a:function(arg0){
const ret=getObject(arg0).performance;
return addHeapObject(ret);
},
__wbg_static_accessor_GLOBAL_8adb955bd33fac2f:function(){
const ret=typeof global==='undefined'?null:global;
return isLikeNone(ret)?0:addHeapObject(ret);
},
__wbg_static_accessor_GLOBAL_THIS_ad356e0db91c7913:function(){
const ret=typeof globalThis==='undefined'?null:globalThis;
return isLikeNone(ret)?0:addHeapObject(ret);
},
__wbg_static_accessor_SELF_f207c857566db248:function(){
const ret=typeof self==='undefined'?null:self;
return isLikeNone(ret)?0:addHeapObject(ret);
},
__wbg_static_accessor_WINDOW_bb9f1ba69d61b386:function(){
const ret=typeof window==='undefined'?null:window;
return isLikeNone(ret)?0:addHeapObject(ret);
},
__wbindgen_object_clone_ref:function(arg0){
const ret=getObject(arg0);
return addHeapObject(ret);
},
__wbindgen_object_drop_ref:function(arg0){
takeObject(arg0);
},
};
return{
__proto__:null,
"./bindgen_bg.js":import0,
};
}
function addHeapObject(obj){
if(heap_next===heap.length)heap.push(heap.length+1);
const idx=heap_next;
heap_next=heap[idx];
heap[idx]=obj;
return idx;
}
function dropObject(idx){
if(idx<1028)return;
heap[idx]=heap_next;
heap_next=idx;
}
function getStringFromWasm0(ptr,len){
ptr=ptr>>>0;
return decodeText(ptr,len);
}
let cachedUint8ArrayMemory0=null;
function getUint8ArrayMemory0(){
if(cachedUint8ArrayMemory0===null||cachedUint8ArrayMemory0.byteLength===0){
cachedUint8ArrayMemory0=new Uint8Array(wasm.memory.buffer);
}
return cachedUint8ArrayMemory0;
}
function getObject(idx){return heap[idx];}
let heap=new Array(1024).fill(undefined);
heap.push(undefined,null,true,false);
let heap_next=heap.length;
function isLikeNone(x){
return x===undefined||x===null;
}
function takeObject(idx){
const ret=getObject(idx);
dropObject(idx);
return ret;
}
let cachedTextDecoder=new TextDecoder('utf-8',{ignoreBOM:true,fatal:true});
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES=2146435072;
let numBytesDecoded=0;
function decodeText(ptr,len){
numBytesDecoded+=len;
if(numBytesDecoded>=MAX_SAFARI_DECODE_BYTES){
cachedTextDecoder=new TextDecoder('utf-8',{ignoreBOM:true,fatal:true});
cachedTextDecoder.decode();
numBytesDecoded=len;
}
return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr,ptr+len));
}
let wasmModule,wasm;
function __wbg_finalize_init(instance,module){
wasm=instance.exports;
wasmModule=module;
cachedUint8ArrayMemory0=null;
wasm.__wbindgen_start();
return instance;
}
async function __wbg_load(module,imports){
if(typeof Response==='function'&&module instanceof Response){
if(typeof WebAssembly.instantiateStreaming==='function'){
try{
return await WebAssembly.instantiateStreaming(module,imports);
}catch(e){
const validResponse=module.ok&&expectedResponseType(module.type);
if(validResponse&&module.headers.get('Content-Type')!=='application/wasm'){
console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e);
}else{throw e;}
}
}
const bytes=await module.arrayBuffer();
return await WebAssembly.instantiate(bytes,imports);
}else{
const instance=await WebAssembly.instantiate(module,imports);
if(instance instanceof WebAssembly.Instance){
return{instance,module};
}else{
return instance;
}
}
function expectedResponseType(type){
switch(type){
case'basic':case'cors':case'default':return true;
}
return false;
}
}
function initSync(module){
if(wasm!==undefined)return wasm;
if(module!==undefined){
if(Object.getPrototypeOf(module)===Object.prototype){
({module}=module)
}else{
console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
}
}
const imports=__wbg_get_imports(); imports.env=env;imports.env=env;
if(!(module instanceof WebAssembly.Module)){
module=new WebAssembly.Module(module);
}
const instance=new WebAssembly.Instance(module,imports);
return __wbg_finalize_init(instance,module);
}
async function __wbg_init(module_or_path,env){let memory;
if(wasm!==undefined)return wasm;
if(module_or_path!==undefined){
if(Object.getPrototypeOf(module_or_path)===Object.prototype){
({module_or_path}=module_or_path)
}else{
console.warn('using deprecated parameters for the initialization function; pass a single object instead')
}
}
if(module_or_path===undefined){
module_or_path=new URL('bindgen_bg.wasm',import.meta.url);
}
const imports=__wbg_get_imports(); imports.env=env;imports.env=env;
if(typeof module_or_path==='string'||(typeof Request==='function'&&module_or_path instanceof Request)||(typeof URL==='function'&&module_or_path instanceof URL)){
module_or_path=fetch(module_or_path);
}
const{instance,module}=await __wbg_load(await module_or_path,imports);
return __wbg_finalize_init(instance,module);
}
export{initSync,__wbg_init as default};