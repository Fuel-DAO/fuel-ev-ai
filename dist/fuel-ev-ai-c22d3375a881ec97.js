let V=null,a5=`Object`,_=0,ad=`bigint`,aj=319,ak=259,ae=8,X=`undefined`,T=128,W=1,a0=`function`,a2=`boolean`,a1=`number`,a3=`string`,a9=16,ac=`same-origin`,Y=`utf-8`,a8=4,S=Array,a4=Array.isArray,af=BigInt,ag=Date,Z=Error,a7=FinalizationRegistry,ai=JSON,a6=JSON.stringify,ab=Object,al=Object.getPrototypeOf,aa=Promise,ah=Reflect,$=Uint8Array,U=undefined;var j=(a=>{if(a<132)return;b[a]=d;d=a});var C=((a,b)=>{if(a===_){return c(b)}else{return i(a,b)}});var M=(async(a,b)=>{if(typeof Response===a0&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===a0){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var u=((b,c,d,e)=>{const f={a:b,b:c,cnt:W,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=_;try{return e(c,f.b,...b)}finally{if(--f.cnt===_){a.__wbindgen_export_2.get(f.dtor)(c,f.b);t.unregister(f)}else{f.a=c}}};g.original=f;t.register(g,f,f);return g});var P=((b,c)=>{a=b.exports;R.__wbindgen_wasm_module=c;m=V;g=V;a.__wbindgen_start();return a});var w=((b,c)=>{a._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h53d43deeacf5dcbf(b,c)});var v=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h25bc921a41c9562c(b,c,e(d))});var k=(a=>{const b=c(a);j(a);return b});var N=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return e(b)});b.wbg.__wbg_length_f2469772b8ec9ea3=(a=>{const b=c(a).length;return b});b.wbg.__wbg_querySelector_e21c39150aa72078=function(){return D(((a,b,d)=>{var f=C(b,d);const g=c(a).querySelector(f);return l(g)?_:e(g)}),arguments)};b.wbg.__wbg_head_6c6317d70f23ff16=(a=>{const b=c(a).head;return l(b)?_:e(b)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=i(a,b);return e(c)});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new Z();return e(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{var d=C(b,c);if(b!==_){a.__wbindgen_free(b,c,W)};console.error(d)});b.wbg.__wbindgen_object_drop_ref=(a=>{k(a)});b.wbg.__wbg_new_b85e72ed1bfd57f9=((a,b)=>{try{var c={a:a,b:b};var d=(a,b)=>{const d=c.a;c.a=_;try{return F(d,c.b,a,b)}finally{c.a=d}};const f=new aa(d);return e(f)}finally{c.a=c.b=_}});b.wbg.__wbg_location_af118da6c50d4c3f=(a=>{const b=c(a).location;return e(b)});b.wbg.__wbg_readyState_23d3f3866f5b8419=(a=>{const b=c(a).readyState;return b});b.wbg.__wbg_abort_142046036bb690a6=(a=>{c(a).abort()});b.wbg.__wbg_open_43b3c6577af2a808=function(){return D(((a,b,d,f,g)=>{var h=C(b,d);var i=C(f,g);const j=c(a).open(h,i);return l(j)?_:e(j)}),arguments)};b.wbg.__wbg_sethash_35265f1963425076=((a,b,d)=>{var e=C(b,d);c(a).hash=e});b.wbg.__wbg_open_b7cad2c5dc0b336e=function(){return D(((a,b,d,f,g,h,i)=>{var j=C(b,d);var k=C(f,g);var m=C(h,i);const n=c(a).open(j,k,m);return l(n)?_:e(n)}),arguments)};b.wbg.__wbg_stopPropagation_dd0d50059627b362=(a=>{c(a).stopPropagation()});b.wbg.__wbg_navigator_6210380287bf8581=(a=>{const b=c(a).navigator;return e(b)});b.wbg.__wbg_new_525245e2b9901204=(()=>{const a=new ab();return e(a)});b.wbg.__wbg_seturl_059b5aa8effbe502=((a,b,d)=>{var e=C(b,d);c(a).url=e});b.wbg.__wbg_share_e12c6369dbb0e77f=((a,b)=>{const d=c(a).share(c(b));return e(d)});b.wbg.__wbg_createDocumentFragment_5d919bd9d2e05b55=(a=>{const b=c(a).createDocumentFragment();return e(b)});b.wbg.__wbg_host_a46347409a9511bd=function(){return D(((b,d)=>{const e=c(d).host;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)}),arguments)};b.wbg.__wbg_instanceof_HtmlInputElement_88bf515ab1d9511d=(a=>{let b;try{b=c(a) instanceof HTMLInputElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_clipboard_0d7b5c390c14b0e6=(a=>{const b=c(a).clipboard;return e(b)});b.wbg.__wbg_writeText_20fb3f7393d34052=((a,b,d)=>{var f=C(b,d);const g=c(a).writeText(f);return e(g)});b.wbg.__wbg_preventDefault_c55d86c27b2dfa6e=(a=>{c(a).preventDefault()});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==V;return d});b.wbg.__wbg_subarray_7c2e3576afe181d1=((a,b,d)=>{const f=c(a).subarray(b>>>_,d>>>_);return e(f)});b.wbg.__wbg_getRandomValues_3aa56aa6edec874c=function(){return D(((a,b)=>{c(a).getRandomValues(c(b))}),arguments)};b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return e(b)});b.wbg.__wbg_buffer_b7b08af79b0b0974=(a=>{const b=c(a).buffer;return e(b)});b.wbg.__wbg_newwithbyteoffsetandlength_8a2cb9ca96b27ec9=((a,b,d)=>{const f=new $(c(a),b>>>_,d>>>_);return e(f)});b.wbg.__wbg_randomFillSync_5c9c955aa56b6049=function(){return D(((a,b)=>{c(a).randomFillSync(k(b))}),arguments)};b.wbg.__wbg_crypto_1d1f22824a6a080c=(a=>{const b=c(a).crypto;return e(b)});b.wbg.__wbg_process_4a72847cc503995b=(a=>{const b=c(a).process;return e(b)});b.wbg.__wbg_versions_f686565e586dd935=(a=>{const b=c(a).versions;return e(b)});b.wbg.__wbg_node_104a2ff8d6ea03a2=(a=>{const b=c(a).node;return e(b)});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===a3;return b});b.wbg.__wbg_require_cca90b1a94a0255b=function(){return D((()=>{const a=module.require;return e(a)}),arguments)};b.wbg.__wbg_msCrypto_eb05e62b530a1508=(a=>{const b=c(a).msCrypto;return e(b)});b.wbg.__wbg_newwithlength_ec548f448387c968=(a=>{const b=new $(a>>>_);return e(b)});b.wbg.__wbg_setcapture_4818ebe9ef88b2f6=((a,b)=>{c(a).capture=b!==_});b.wbg.__wbg_setonce_06b35a72a3fafc15=((a,b)=>{c(a).once=b!==_});b.wbg.__wbg_setpassive_70ce6704aec553f6=((a,b)=>{c(a).passive=b!==_});b.wbg.__wbg_new_8515b7401632bd44=function(){return D((()=>{const a=new FileReader();return e(a)}),arguments)};b.wbg.__wbg_readAsArrayBuffer_6475a86a924a8856=function(){return D(((a,b)=>{c(a).readAsArrayBuffer(c(b))}),arguments)};b.wbg.__wbg_error_d05a87e2d8211d2e=(a=>{const b=c(a).error;return l(b)?_:e(b)});b.wbg.__wbg_name_fe926223443dc728=((b,d)=>{const e=c(d).name;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)});b.wbg.__wbg_new_ea1883e1e5e86686=(a=>{const b=new $(c(a));return e(b)});b.wbg.__wbg_result_3869032b57f861ac=function(){return D((a=>{const b=c(a).result;return e(b)}),arguments)};b.wbg.__wbg_clearTimeout_541ac0980ffcef74=(a=>{const b=clearTimeout(k(a));return e(b)});b.wbg.__wbg_setTimeout_7d81d052875b0f4f=function(){return D(((a,b)=>{const d=setTimeout(c(a),b);return e(d)}),arguments)};b.wbg.__wbg_instanceof_Response_e91b7eb7c611a9ae=(a=>{let b;try{b=c(a) instanceof Response}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_status_ae8de515694c5c7c=(a=>{const b=c(a).status;return b});b.wbg.__wbg_url_1bf85c8abeb8c92d=((b,d)=>{const e=c(d).url;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)});b.wbg.__wbg_headers_5e283e8345689121=(a=>{const b=c(a).headers;return e(b)});b.wbg.__wbg_stringify_bbf45426c92a6bf5=function(){return D((a=>{const b=a6(c(a));return e(b)}),arguments)};b.wbg.__wbg_setmethod_dc68a742c2db5c6a=((a,b,d)=>{var e=C(b,d);c(a).method=e});b.wbg.__wbg_new_e27c93803e1acc42=function(){return D((()=>{const a=new Headers();return e(a)}),arguments)};b.wbg.__wbg_setheaders_be10a5ab566fd06f=((a,b)=>{c(a).headers=c(b)});b.wbg.__wbg_setmode_a781aae2bd3df202=((a,b)=>{c(a).mode=[ac,`no-cors`,`cors`,`navigate`][b]});b.wbg.__wbg_setcredentials_2b67800db3f7b621=((a,b)=>{c(a).credentials=[`omit`,ac,`include`][b]});b.wbg.__wbg_append_f3a4426bb50622c5=function(){return D(((a,b,d,e,f)=>{var g=C(b,d);var h=C(e,f);c(a).append(g,h)}),arguments)};b.wbg.__wbg_setbody_734cb3d7ee8e6e96=((a,b)=>{c(a).body=c(b)});b.wbg.__wbg_new_ebf2727385ee825c=function(){return D((()=>{const a=new AbortController();return e(a)}),arguments)};b.wbg.__wbg_signal_41e46ccad44bb5e2=(a=>{const b=c(a).signal;return e(b)});b.wbg.__wbg_setsignal_91c4e8ebd04eb935=((a,b)=>{c(a).signal=c(b)});b.wbg.__wbg_newwithstrandinit_a31c69e4cc337183=function(){return D(((a,b,d)=>{var f=C(a,b);const g=new Request(f,c(d));return e(g)}),arguments)};b.wbg.__wbg_fetch_f8d735ba6fe1b719=(a=>{const b=fetch(c(a));return e(b)});b.wbg.__wbg_fetch_ba7fe179e527d942=((a,b)=>{const d=c(a).fetch(c(b));return e(d)});b.wbg.__wbg_read_e48a676fb81ea800=(a=>{const b=c(a).read();return e(b)});b.wbg.__wbg_done_510de141aaf69a99=(a=>{const b=c(a).done;return b});b.wbg.__wbg_value_3ef4965e9c7085be=(a=>{const b=c(a).value;return e(b)});b.wbg.__wbg_length_8339fcf5d8ecd12e=(a=>{const b=c(a).length;return b});b.wbg.__wbg_closed_f6c4b96b9528b431=function(){return D((a=>{const b=c(a).closed;return b}),arguments)};b.wbg.__wbg_instanceof_MessageEvent_9951ccea5e1f35a2=(a=>{let b;try{b=c(a) instanceof MessageEvent}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_data_5c47a6985fefc490=(a=>{const b=c(a).data;return e(b)});b.wbg.__wbg_getwithrefkey_edc2c8960f0f1191=((a,b)=>{const d=c(a)[c(b)];return e(d)});b.wbg.__wbg_toString_e17a6671146f47c1=(a=>{const b=c(a).toString();return e(b)});b.wbg.__wbg_postMessage_74a40207a78694e7=function(){return D(((a,b,d,e)=>{var f=C(d,e);c(a).postMessage(c(b),f)}),arguments)};b.wbg.__wbg_warn_71afa7f8150659a1=((b,c)=>{var d=E(b,c).slice();a.__wbindgen_free(b,c*a8,a8);console.warn(...d)});b.wbg.__wbg_error_a526fb08a0205972=((b,c)=>{var d=E(b,c).slice();a.__wbindgen_free(b,c*a8,a8);console.error(...d)});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===U;return b});b.wbg.__wbindgen_in=((a,b)=>{const d=c(a) in c(b);return d});b.wbg.__wbindgen_is_bigint=(a=>{const b=typeof c(a)===ad;return b});b.wbg.__wbindgen_bigint_get_as_i64=((a,b)=>{const d=c(b);const e=typeof d===ad?d:U;n().setBigInt64(a+ ae*W,l(e)?af(_):e,!0);n().setInt32(a+ a8*_,!l(e),!0)});b.wbg.__wbindgen_bigint_from_u64=(a=>{const b=af.asUintN(64,a);return e(b)});b.wbg.__wbindgen_jsval_eq=((a,b)=>{const d=c(a)===c(b);return d});b.wbg.__wbindgen_error_new=((a,b)=>{const c=new Z(i(a,b));return e(c)});b.wbg.__wbg_localStorage_90db5cb66e840248=function(){return D((a=>{const b=c(a).localStorage;return l(b)?_:e(b)}),arguments)};b.wbg.__wbg_String_b9412f8799faab3e=((b,d)=>{const e=String(c(d));const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)});b.wbg.__wbg_now_b7a162010a9e75b4=(()=>{const a=ag.now();return a});b.wbg.__wbg_get_3baa728f9d58d3f6=((a,b)=>{const d=c(a)[b>>>_];return e(d)});b.wbg.__wbg_length_ae22078168b726f5=(a=>{const b=c(a).length;return b});b.wbg.__wbg_next_f9cb570345655b9a=function(){return D((a=>{const b=c(a).next();return e(b)}),arguments)};b.wbg.__wbg_done_bfda7aa8f252b39f=(a=>{const b=c(a).done;return b});b.wbg.__wbg_value_6d39332ab4788d86=(a=>{const b=c(a).value;return e(b)});b.wbg.__wbg_iterator_888179a48810a9fe=(()=>{const a=Symbol.iterator;return e(a)});b.wbg.__wbg_next_de3e9db4440638b2=(a=>{const b=c(a).next;return e(b)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===a0;return b});b.wbg.__wbg_call_1084a111329e68ce=function(){return D(((a,b)=>{const d=c(a).call(c(b));return e(d)}),arguments)};b.wbg.__wbg_self_3093d5d1f7bcb682=function(){return D((()=>{const a=self.self;return e(a)}),arguments)};b.wbg.__wbg_window_3bcfc4d31bc012f8=function(){return D((()=>{const a=window.window;return e(a)}),arguments)};b.wbg.__wbg_globalThis_86b222e13bdf32ed=function(){return D((()=>{const a=globalThis.globalThis;return e(a)}),arguments)};b.wbg.__wbg_global_e5a3fe56f8be9485=function(){return D((()=>{const a=global.global;return e(a)}),arguments)};b.wbg.__wbg_newnoargs_76313bd6ff35d0f2=((a,b)=>{var c=C(a,b);const d=new Function(c);return e(d)});b.wbg.__wbg_decodeURI_27d956972029c70b=function(){return D(((a,b)=>{var c=C(a,b);const d=decodeURI(c);return e(d)}),arguments)};b.wbg.__wbg_isArray_8364a5371e9737d8=(a=>{const b=a4(c(a));return b});b.wbg.__wbg_instanceof_ArrayBuffer_61dfc3198373c902=(a=>{let b;try{b=c(a) instanceof ArrayBuffer}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_call_89af060b4e1523f2=function(){return D(((a,b,d)=>{const f=c(a).call(c(b),c(d));return e(f)}),arguments)};b.wbg.__wbg_is_009b1ef508712fda=((a,b)=>{const d=ab.is(c(a),c(b));return d});b.wbg.__wbg_exec_a29a4ce5544bd3be=((a,b,d)=>{var f=C(b,d);const g=c(a).exec(f);return l(g)?_:e(g)});b.wbg.__wbg_new_13847c66f41dda63=((a,b,c,d)=>{var f=C(a,b);var g=C(c,d);const h=new RegExp(f,g);return e(h)});b.wbg.__wbg_set_d1e79e2388520f18=((a,b,d)=>{c(a).set(c(b),d>>>_)});b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===a3?e:U;var g=l(f)?_:r(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=o;n().setInt32(b+ a8*W,h,!0);n().setInt32(b+ a8*_,g,!0)});b.wbg.__wbg_get_224d16597dbbfd96=function(){return D(((a,b)=>{const d=ah.get(c(a),c(b));return e(d)}),arguments)};b.wbg.__wbg_has_4bfbc01db38743f7=function(){return D(((a,b)=>{const d=ah.has(c(a),c(b));return d}),arguments)};b.wbg.__wbg_set_eacc7d73fefaafdf=function(){return D(((a,b,d)=>{const e=ah.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbg_parse_52202f117ec9ecfa=function(){return D(((a,b)=>{var c=C(a,b);const d=ai.parse(c);return e(d)}),arguments)};b.wbg.__wbg_setmode_395d7387c9026960=((a,b)=>{c(a).mode=[`open`,`closed`][b]});b.wbg.__wbg_attachShadow_e1c4b053d4754872=function(){return D(((a,b)=>{const d=c(a).attachShadow(c(b));return e(d)}),arguments)};b.wbg.__wbg_createComment_7a1d9856e50567bb=((a,b,d)=>{var f=C(b,d);const g=c(a).createComment(f);return e(g)});b.wbg.__wbg_new_95093d1a71aba61d=function(){return D((()=>{const a=new Range();return e(a)}),arguments)};b.wbg.__wbg_composedPath_d1052062308beae5=(a=>{const b=c(a).composedPath();return e(b)});b.wbg.__wbindgen_is_falsy=(a=>{const b=!c(a);return b});b.wbg.__wbg_cancelBubble_0374b329f66f59b5=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_instanceof_ShadowRoot_72d8e783f8e0093c=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_fdfe1258b06fe937=(a=>{const b=c(a).host;return e(b)});b.wbg.__wbindgen_is_null=(a=>{const b=c(a)===V;return b});b.wbg.__wbg_setdata_27c6828c5a5a5ce4=((a,b,d)=>{var e=C(b,d);c(a).data=e});b.wbg.__wbg_remove_5b68b70c39041e2a=(a=>{c(a).remove()});b.wbg.__wbg_log_b103404cc5920657=(a=>{console.log(c(a))});b.wbg.__wbg_warn_2b3adb99ce26c314=(a=>{console.warn(c(a))});b.wbg.__wbg_error_09480e4aadca50ad=(a=>{console.error(c(a))});b.wbg.__wbg_classList_d725bcb3b32c27b5=(a=>{const b=c(a).classList;return e(b)});b.wbg.__wbg_remove_0dd2beafdaa4d9ba=function(){return D(((a,b,d)=>{var e=C(b,d);c(a).remove(e)}),arguments)};b.wbg.__wbg_add_e210e3b838bff57f=function(){return D(((a,b,d)=>{var e=C(b,d);c(a).add(e)}),arguments)};b.wbg.__wbg_requestAnimationFrame_b4b782250b9c2c88=function(){return D(((a,b)=>{const d=c(a).requestAnimationFrame(c(b));return d}),arguments)};b.wbg.__wbg_removeEventListener_b6cef5ad085bea8f=function(){return D(((a,b,d,e)=>{var f=C(b,d);c(a).removeEventListener(f,c(e))}),arguments)};b.wbg.__wbg_childNodes_031aa96d5e3d21b0=(a=>{const b=c(a).childNodes;return e(b)});b.wbg.__wbg_length_4919f4a62b9b1e94=(a=>{const b=c(a).length;return b});b.wbg.__wbg_createTextNode_8bce33cf33bf8f6e=((a,b,d)=>{var f=C(b,d);const g=c(a).createTextNode(f);return e(g)});b.wbg.__wbg_document_8554450897a855b9=(a=>{const b=c(a).document;return l(b)?_:e(b)});b.wbg.__wbindgen_cb_drop=(a=>{const b=k(a).original;if(b.cnt--==W){b.a=_;return !0};const c=!1;return c});b.wbg.__wbindgen_number_get=((a,b)=>{const d=c(b);const e=typeof d===a1?d:U;n().setFloat64(a+ ae*W,l(e)?_:e,!0);n().setInt32(a+ a8*_,!l(e),!0)});b.wbg.__wbg_pathname_adec1eb7f76356a8=((b,d)=>{const e=c(d).pathname;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)});b.wbg.__wbg_search_f384756d8e27fd66=((b,d)=>{const e=c(d).search;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)});b.wbg.__wbg_searchParams_8b40e0942f870b44=(a=>{const b=c(a).searchParams;return e(b)});b.wbg.__wbg_hash_50828fbc16613897=((b,d)=>{const e=c(d).hash;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)});b.wbg.__wbg_decodeURIComponent_e4bdc451fcc55015=function(){return D(((a,b)=>{var c=C(a,b);const d=decodeURIComponent(c);return e(d)}),arguments)};b.wbg.__wbg_state_b863826253700666=function(){return D((a=>{const b=c(a).state;return e(b)}),arguments)};b.wbg.__wbg_pathname_6e6871539b48a0e5=function(){return D(((b,d)=>{const e=c(d).pathname;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)}),arguments)};b.wbg.__wbg_search_20c15d493b8602c5=function(){return D(((b,d)=>{const e=c(d).search;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)}),arguments)};b.wbg.__wbg_replaceState_c3213575ed65bac2=function(){return D(((a,b,d,e,f,g)=>{var h=C(d,e);var i=C(f,g);c(a).replaceState(c(b),h,i)}),arguments)};b.wbg.__wbg_getElementById_f56c8e6a15a6926d=((a,b,d)=>{var f=C(b,d);const g=c(a).getElementById(f);return l(g)?_:e(g)});b.wbg.__wbg_scrollIntoView_4b805e2532108e71=(a=>{c(a).scrollIntoView()});b.wbg.__wbg_scrollTo_19dc1dbbc8c19fa8=((a,b,d)=>{c(a).scrollTo(b,d)});b.wbg.__wbg_defaultPrevented_9e2309e82258aee7=(a=>{const b=c(a).defaultPrevented;return b});b.wbg.__wbg_button_460cdec9f2512a91=(a=>{const b=c(a).button;return b});b.wbg.__wbg_metaKey_be0158b14b1cef4a=(a=>{const b=c(a).metaKey;return b});b.wbg.__wbg_altKey_d3fbce7596aac8cf=(a=>{const b=c(a).altKey;return b});b.wbg.__wbg_ctrlKey_957c6c31b62b4550=(a=>{const b=c(a).ctrlKey;return b});b.wbg.__wbg_shiftKey_8c0f9a5ca3ff8f93=(a=>{const b=c(a).shiftKey;return b});b.wbg.__wbg_href_31456ceb26f92368=((b,d)=>{const e=c(d).href;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)});b.wbg.__wbg_target_fed794e9a6ed73fe=((b,d)=>{const e=c(d).target;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)});b.wbg.__wbg_getAttribute_e867e037f066c410=((b,d,e,f)=>{var g=C(e,f);const h=c(d).getAttribute(g);var i=l(h)?_:r(h,a.__wbindgen_malloc,a.__wbindgen_realloc);var j=o;n().setInt32(b+ a8*W,j,!0);n().setInt32(b+ a8*_,i,!0)});b.wbg.__wbg_instanceof_HtmlAnchorElement_7a88f0b97085fa30=(a=>{let b;try{b=c(a) instanceof HTMLAnchorElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbindgen_boolean_get=(a=>{const b=c(a);const d=typeof b===a2?(b?W:_):2;return d});b.wbg.__wbg_abort_8659d889a7877ae3=(a=>{c(a).abort()});b.wbg.__wbg_body_40b0ed27714d00ce=(a=>{const b=c(a).body;return l(b)?_:e(b)});b.wbg.__wbg_getReader_584431a478f1339c=function(){return D((a=>{const b=c(a).getReader();return e(b)}),arguments)};b.wbg.__wbindgen_jsval_loose_eq=((a,b)=>{const d=c(a)==c(b);return d});b.wbg.__wbg_instanceof_Uint8Array_247a91427532499e=(a=>{let b;try{b=c(a) instanceof $}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_isSafeInteger_7f1ed56200d90674=(a=>{const b=Number.isSafeInteger(c(a));return b});b.wbg.__wbindgen_as_number=(a=>{const b=+c(a);return b});b.wbg.__wbg_new0_65387337a95cf44d=(()=>{const a=new ag();return e(a)});b.wbg.__wbg_getTime_91058879093a1589=(a=>{const b=c(a).getTime();return b});b.wbg.__wbindgen_debug_string=((b,d)=>{const e=s(c(d));const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)});b.wbg.__wbindgen_throw=((a,b)=>{throw new Z(i(a,b))});b.wbg.__wbindgen_rethrow=(a=>{throw k(a)});b.wbg.__wbg_then_876bb3c633745cc6=((a,b,d)=>{const f=c(a).then(c(b),c(d));return e(f)});b.wbg.__wbg_queueMicrotask_48421b3cc9052b68=(a=>{const b=c(a).queueMicrotask;return e(b)});b.wbg.__wbg_resolve_570458cb99d56a43=(a=>{const b=aa.resolve(c(a));return e(b)});b.wbg.__wbg_then_95e6edc0f89b73b1=((a,b)=>{const d=c(a).then(c(b));return e(d)});b.wbg.__wbg_queueMicrotask_12a30234db4045d3=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_byobRequest_b32c77640da946ac=(a=>{const b=c(a).byobRequest;return l(b)?_:e(b)});b.wbg.__wbg_view_2a901bda0727aeb3=(a=>{const b=c(a).view;return l(b)?_:e(b)});b.wbg.__wbg_byteLength_850664ef28f3e42f=(a=>{const b=c(a).byteLength;return b});b.wbg.__wbg_close_aca7442e6619206b=function(){return D((a=>{c(a).close()}),arguments)};b.wbg.__wbg_new_796382978dfd4fb0=((a,b)=>{var c=C(a,b);const d=new Z(c);return e(d)});b.wbg.__wbg_buffer_0710d1b9dbe2eea6=(a=>{const b=c(a).buffer;return e(b)});b.wbg.__wbg_byteOffset_ea14c35fa6de38cc=(a=>{const b=c(a).byteOffset;return b});b.wbg.__wbg_close_cef2400b120c9c73=function(){return D((a=>{c(a).close()}),arguments)};b.wbg.__wbg_enqueue_6f3d433b5e457aea=function(){return D(((a,b)=>{c(a).enqueue(c(b))}),arguments)};b.wbg.__wbg_cancel_97a2795574a4f522=(a=>{const b=c(a).cancel();return e(b)});b.wbg.__wbg_catch_a279b1da46d132d8=((a,b)=>{const d=c(a).catch(c(b));return e(d)});b.wbg.__wbg_performance_fa12dc8712926291=(a=>{const b=c(a).performance;return l(b)?_:e(b)});b.wbg.__wbg_history_489e13d0b625263c=function(){return D((a=>{const b=c(a).history;return e(b)}),arguments)};b.wbg.__wbg_close_c599eb1ebdaf9f6a=function(){return D((a=>{c(a).close()}),arguments)};b.wbg.__wbg_setTimeout_73b734ca971c19f4=function(){return D(((a,b,d)=>{const e=c(a).setTimeout(c(b),d);return e}),arguments)};b.wbg.__wbg_body_b3bb488e8e54bf4b=(a=>{const b=c(a).body;return l(b)?_:e(b)});b.wbg.__wbg_createElement_5921e9eb06b9ec89=function(){return D(((a,b,d)=>{var f=C(b,d);const g=c(a).createElement(f);return e(g)}),arguments)};b.wbg.__wbg_createElementNS_78308ee7091c53f7=function(){return D(((a,b,d,f,g)=>{var h=C(b,d);var i=C(f,g);const j=c(a).createElementNS(h,i);return e(j)}),arguments)};b.wbg.__wbg_addEventListener_e167f012cbedfa4e=function(){return D(((a,b,d,e)=>{var f=C(b,d);c(a).addEventListener(f,c(e))}),arguments)};b.wbg.__wbg_addEventListener_14b036ff7cb8747c=function(){return D(((a,b,d,e,f)=>{var g=C(b,d);c(a).addEventListener(g,c(e),c(f))}),arguments)};b.wbg.__wbg_removeEventListener_7878b86efe1ab901=function(){return D(((a,b,d,e,f)=>{var g=C(b,d);c(a).removeEventListener(g,c(e),f!==_)}),arguments)};b.wbg.__wbg_deleteContents_45ba9b733b3b97ea=function(){return D((a=>{c(a).deleteContents()}),arguments)};b.wbg.__wbg_setEndBefore_7d55a9db0e0f4c41=function(){return D(((a,b)=>{c(a).setEndBefore(c(b))}),arguments)};b.wbg.__wbg_setStartBefore_a28dcb3c6ece9e07=function(){return D(((a,b)=>{c(a).setStartBefore(c(b))}),arguments)};b.wbg.__wbg_releaseLock_1d2d93e9dc8d76e2=(a=>{c(a).releaseLock()});b.wbg.__wbg_now_a69647afb1f66247=(a=>{const b=c(a).now();return b});b.wbg.__wbg_message_b477ea215924b777=((b,d)=>{const e=c(d).message;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)});b.wbg.__wbg_href_f1d20018a97415a0=((b,d)=>{const e=c(d).href;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)});b.wbg.__wbg_origin_b1cdab9cfa04b734=((b,d)=>{const e=c(d).origin;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)});b.wbg.__wbg_new_33db4be5d9963ec1=function(){return D(((a,b)=>{var c=C(a,b);const d=new URL(c);return e(d)}),arguments)};b.wbg.__wbg_newwithbase_ba5e3790a41efd02=function(){return D(((a,b,c,d)=>{var f=C(a,b);var g=C(c,d);const h=new URL(f,g);return e(h)}),arguments)};b.wbg.__wbg_back_2b44401f98571e5e=function(){return D((a=>{c(a).back()}),arguments)};b.wbg.__wbg_pushState_fc8b2d0c45854901=function(){return D(((a,b,d,e,f,g)=>{var h=C(d,e);var i=C(f,g);c(a).pushState(c(b),h,i)}),arguments)};b.wbg.__wbg_parentNode_3e06cf96d7693d57=(a=>{const b=c(a).parentNode;return l(b)?_:e(b)});b.wbg.__wbg_previousSibling_076df2178284ef97=(a=>{const b=c(a).previousSibling;return l(b)?_:e(b)});b.wbg.__wbg_nextSibling_f6396d6fd0b97830=(a=>{const b=c(a).nextSibling;return l(b)?_:e(b)});b.wbg.__wbg_settextContent_cd38ea7d4e0f7260=((a,b,d)=>{var e=C(b,d);c(a).textContent=e});b.wbg.__wbg_appendChild_ac45d1abddf1b89b=function(){return D(((a,b)=>{const d=c(a).appendChild(c(b));return e(d)}),arguments)};b.wbg.__wbg_cloneNode_629a1b180e91c467=function(){return D((a=>{const b=c(a).cloneNode();return e(b)}),arguments)};b.wbg.__wbg_removeChild_139b30d19f579e41=function(){return D(((a,b)=>{const d=c(a).removeChild(c(b));return e(d)}),arguments)};b.wbg.__wbg_files_b94d8f21e2b53924=(a=>{const b=c(a).files;return l(b)?_:e(b)});b.wbg.__wbg_value_d4a95e7a0d390578=((b,d)=>{const e=c(d).value;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)});b.wbg.__wbg_setvalue_688819688274bec0=((a,b,d)=>{var e=C(b,d);c(a).value=e});b.wbg.__wbg_getItem_cab39762abab3e70=function(){return D(((b,d,e,f)=>{var g=C(e,f);const h=c(d).getItem(g);var i=l(h)?_:r(h,a.__wbindgen_malloc,a.__wbindgen_realloc);var j=o;n().setInt32(b+ a8*W,j,!0);n().setInt32(b+ a8*_,i,!0)}),arguments)};b.wbg.__wbg_removeItem_f10a84254de33054=function(){return D(((a,b,d)=>{var e=C(b,d);c(a).removeItem(e)}),arguments)};b.wbg.__wbg_setItem_9482185c870abba6=function(){return D(((a,b,d,e,f)=>{var g=C(b,d);var h=C(e,f);c(a).setItem(g,h)}),arguments)};b.wbg.__wbg_append_d510a297e3ba948e=function(){return D(((a,b)=>{c(a).append(c(b))}),arguments)};b.wbg.__wbg_origin_13af45ba03476638=((b,d)=>{const e=c(d).origin;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)});b.wbg.__wbg_target_b7cb1739bee70928=(a=>{const b=c(a).target;return l(b)?_:e(b)});b.wbg.__wbg_respond_a799bab31a44f2d7=function(){return D(((a,b)=>{c(a).respond(b>>>_)}),arguments)};b.wbg.__wbg_setinnerHTML_ea7e3c6a3c4790c6=((a,b,d)=>{var e=C(b,d);c(a).innerHTML=e});b.wbg.__wbg_hasAttribute_a17d49194d050f19=((a,b,d)=>{var e=C(b,d);const f=c(a).hasAttribute(e);return f});b.wbg.__wbg_removeAttribute_c80e298b60689065=function(){return D(((a,b,d)=>{var e=C(b,d);c(a).removeAttribute(e)}),arguments)};b.wbg.__wbg_setAttribute_d5540a19be09f8dc=function(){return D(((a,b,d,e,f)=>{var g=C(b,d);var h=C(e,f);c(a).setAttribute(g,h)}),arguments)};b.wbg.__wbg_before_ac3792b457802cbf=function(){return D(((a,b)=>{c(a).before(c(b))}),arguments)};b.wbg.__wbg_append_e2fc366a9f0be0e9=function(){return D(((a,b)=>{c(a).append(c(b))}),arguments)};b.wbg.__wbg_instanceof_Window_5012736c80a01584=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_href_9c2fe204628af7a3=function(){return D(((b,d)=>{const e=c(d).href;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)}),arguments)};b.wbg.__wbg_sethref_9d76f6c9356e9638=function(){return D(((a,b,d)=>{var e=C(b,d);c(a).href=e}),arguments)};b.wbg.__wbg_origin_648082c4831a5be8=function(){return D(((b,d)=>{const e=c(d).origin;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)}),arguments)};b.wbg.__wbg_hash_313d7fdf42f6e7d3=function(){return D(((b,d)=>{const e=c(d).hash;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)}),arguments)};b.wbg.__wbg_reload_72886b41d52c6162=function(){return D((a=>{c(a).reload()}),arguments)};b.wbg.__wbg_name_ed3cda975cce080d=((b,d)=>{const e=c(d).name;const f=r(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=o;n().setInt32(b+ a8*W,g,!0);n().setInt32(b+ a8*_,f,!0)});b.wbg.__wbg_get_6d8ff52d2078d871=((a,b)=>{const d=c(a)[b>>>_];return l(d)?_:e(d)});b.wbg.__wbindgen_closure_wrapper261=((a,b,c)=>{const d=u(a,b,aj,v);return e(d)});b.wbg.__wbindgen_closure_wrapper640=((a,b,c)=>{const d=u(a,b,ak,w);return e(d)});b.wbg.__wbindgen_closure_wrapper2654=((a,b,c)=>{const d=u(a,b,ak,z);return e(d)});b.wbg.__wbindgen_closure_wrapper3053=((a,b,c)=>{const d=A(a,b,aj,B);return e(d)});b.wbg.__wbindgen_closure_wrapper3338=((a,b,c)=>{const d=u(a,b,262,v);return e(d)});b.wbg.__wbindgen_closure_wrapper3441=((a,b,c)=>{const d=u(a,b,331,v);return e(d)});b.wbg.__wbindgen_closure_wrapper3491=((a,b,c)=>{const d=u(a,b,ak,z);return e(d)});b.wbg.__wbindgen_closure_wrapper3522=((a,b,c)=>{const d=u(a,b,ak,w);return e(d)});b.wbg.__wbindgen_closure_wrapper5597=((a,b,c)=>{const d=u(a,b,327,w);return e(d)});b.wbg.__wbindgen_closure_wrapper6360=((a,b,c)=>{const d=u(a,b,ak,v);return e(d)});b.wbg.__wbindgen_closure_wrapper6499=((a,b,c)=>{const d=u(a,b,ak,w);return e(d)});return b});var F=((b,c,d,f)=>{a.wasm_bindgen__convert__closures__invoke2_mut__h4eeb87f6523e5651(b,c,e(d),e(f))});var h=(()=>{if(g===V||g.byteLength===_){g=new $(a.memory.buffer)};return g});var r=((a,b,c)=>{if(c===U){const c=p.encode(a);const d=b(c.length,W)>>>_;h().subarray(d,d+ c.length).set(c);o=c.length;return d};let d=a.length;let e=b(d,W)>>>_;const f=h();let g=_;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==_){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,W)>>>_;const b=h().subarray(e+ g,e+ d);const f=q(a,b);g+=f.written;e=c(e,d,g,W)>>>_};o=g;return e});var n=(()=>{if(m===V||m.buffer.detached===!0||m.buffer.detached===U&&m.buffer!==a.memory.buffer){m=new DataView(a.memory.buffer)};return m});var B=((b,c)=>{a._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hac380f3d04435cfa(b,c)});var Q=(b=>{if(a!==U)return a;if(typeof b!==X&&al(b)===ab.prototype)({module:b}=b);else console.warn(`using deprecated parameters for \`initSync()\`; pass a single object instead`);const c=N();O(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return P(d,b)});var s=(a=>{const b=typeof a;if(b==a1||b==a2||a==V){return `${a}`};if(b==a3){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==V){return `Symbol`}else{return `Symbol(${b})`}};if(b==a0){const b=a.name;if(typeof b==a3&&b.length>_){return `Function(${b})`}else{return `Function`}};if(a4(a)){const b=a.length;let c=`[`;if(b>_){c+=s(a[_])};for(let d=W;d<b;d++){c+=`, `+ s(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>W){d=c[W]}else{return toString.call(a)};if(d==a5){try{return `Object(`+ a6(a)+ `)`}catch(a){return a5}};if(a instanceof Z){return `${a.name}: ${a.message}\n${a.stack}`};return d});var z=((c,d,e)=>{try{a._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h08909dceae9ad871(c,d,y(e))}finally{b[x++]=U}});var y=(a=>{if(x==W)throw new Z(`out of js stack`);b[--x]=a;return x});var O=((a,b)=>{});var E=((a,b)=>{a=a>>>_;const c=n();const d=[];for(let e=a;e<a+ a8*b;e+=a8){d.push(k(c.getUint32(e,!0)))};return d});var i=((a,b)=>{a=a>>>_;return f.decode(h().subarray(a,a+ b))});var e=(a=>{if(d===b.length)b.push(b.length+ W);const c=d;d=b[c];b[c]=a;return c});var c=(a=>b[a]);var R=(async(b)=>{if(a!==U)return a;if(typeof b!==X&&al(b)===ab.prototype)({module_or_path:b}=b);else console.warn(`using deprecated parameters for the initialization function; pass a single object instead`);if(typeof b===X){b=new URL(`fuel-ev-ai_bg.wasm`,import.meta.url)};const c=N();if(typeof b===a3||typeof Request===a0&&b instanceof Request||typeof URL===a0&&b instanceof URL){b=fetch(b)};O(c);const {instance:d,module:e}=await M(await b,c);return P(d,e)});function D(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(e(b))}}var A=((b,c,d,e)=>{const f={a:b,b:c,cnt:W,dtor:d};const g=(...b)=>{f.cnt++;try{return e(f.a,f.b,...b)}finally{if(--f.cnt===_){a.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=_;t.unregister(f)}}};g.original=f;t.register(g,f,f);return g});var l=(a=>a===U||a===V);let a;const b=new S(T).fill(U);b.push(U,V,!0,!1);let d=b.length;const f=typeof TextDecoder!==X?new TextDecoder(Y,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Z(`TextDecoder not available`)}};if(typeof TextDecoder!==X){f.decode()};let g=V;let m=V;let o=_;const p=typeof TextEncoder!==X?new TextEncoder(Y):{encode:()=>{throw Z(`TextEncoder not available`)}};const q=typeof p.encodeInto===a0?((a,b)=>p.encodeInto(a,b)):((a,b)=>{const c=p.encode(a);b.set(c);return {read:a.length,written:c.length}});const t=typeof a7===X?{register:()=>{},unregister:()=>{}}:new a7(b=>{a.__wbindgen_export_2.get(b.dtor)(b.a,b.b)});let x=T;const G=typeof a7===X?{register:()=>{},unregister:()=>{}}:new a7(b=>a.__wbg_intounderlyingbytesource_free(b>>>_,W));class H{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=_;G.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingbytesource_free(b,_)}type(){try{const e=a.__wbindgen_add_to_stack_pointer(-a9);a.intounderlyingbytesource_type(e,this.__wbg_ptr);var b=n().getInt32(e+ a8*_,!0);var c=n().getInt32(e+ a8*W,!0);var d=C(b,c);if(b!==_){a.__wbindgen_free(b,c,W)};return d}finally{a.__wbindgen_add_to_stack_pointer(a9)}}autoAllocateChunkSize(){const b=a.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);return b>>>_}start(b){a.intounderlyingbytesource_start(this.__wbg_ptr,e(b))}pull(b){const c=a.intounderlyingbytesource_pull(this.__wbg_ptr,e(b));return k(c)}cancel(){const b=this.__destroy_into_raw();a.intounderlyingbytesource_cancel(b)}}const I=typeof a7===X?{register:()=>{},unregister:()=>{}}:new a7(b=>a.__wbg_intounderlyingsink_free(b>>>_,W));class J{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=_;I.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsink_free(b,_)}write(b){const c=a.intounderlyingsink_write(this.__wbg_ptr,e(b));return k(c)}close(){const b=this.__destroy_into_raw();const c=a.intounderlyingsink_close(b);return k(c)}abort(b){const c=this.__destroy_into_raw();const d=a.intounderlyingsink_abort(c,e(b));return k(d)}}const K=typeof a7===X?{register:()=>{},unregister:()=>{}}:new a7(b=>a.__wbg_intounderlyingsource_free(b>>>_,W));class L{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=_;K.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsource_free(b,_)}pull(b){const c=a.intounderlyingsource_pull(this.__wbg_ptr,e(b));return k(c)}cancel(){const b=this.__destroy_into_raw();a.intounderlyingsource_cancel(b)}}export default R;export{H as IntoUnderlyingByteSource,J as IntoUnderlyingSink,L as IntoUnderlyingSource,Q as initSync}