// Verifiable inverse of this owner's cfg-only additions. It never writes source.
// Equality with a predeclared SHA is stronger than a guessed reconstruction.
const fs=require('fs'),crypto=require('crypto');
const sha=text=>crypto.createHash('sha256').update(text).digest('hex');
const evalPath='01_core/src/compiler/eval/mod.rs';
const stylePath='01_core/src/entities/style_chain.rs';
let text=fs.readFileSync(evalPath,'utf8');
const current=sha(text);
function removeBetween(start,end){const a=text.indexOf(start),b=text.indexOf(end,a+start.length);if(a<0||b<0)throw Error('missing exact addition boundary');text=text.slice(0,a)+text.slice(b);}
function replaceOnce(from,to){if(text.split(from).length!==2)throw Error('not exactly one inverse target');text=text.replace(from,to);}
removeBetween('    #[cfg(p1339_observation)]\n    #[test]\n    fn p1340_binding_actual_relation_prefix_and_lossless_payloads()', '    #[test]\n    fn p1339_observation_records_before_selection_and_preserves_error()');
removeBetween('#[cfg(p1339_observation)]\nuse serde_json::{Value as ObservationJson, json as p1340_json};', 'impl EvalContext {\n    #[cfg(p1339_observation)]\n    pub(crate) fn p1339_observe_callback');
replaceOnce('    phase: ContextObservationPhase,\n    relation: Option<ObservationRelation>,\n    actual_context: serde_json::Value,\n','');
replaceOnce('        #[cfg(p1339_observation)]\n        let actual_dispatch_context = p1340_observation_context(\n            ctx.current_location, ctx.in_context, ctx.target, ctx.features,\n            Some(transaction.current_file),\n        );\n','');
replaceOnce('                phase: ctx.context_reads.borrow().observation.phase,\n                relation: None,\n                actual_context: actual_dispatch_context,\n','');
const a=text.indexOf('            if {\n                #[cfg(not(p1339_observation))]');
const b=text.indexOf('            } {\n                valid = false;',a);
if(a<0||b<0)throw Error('missing cfg comparison expression');
text=text.slice(0,a)+'            if read.result.observation_relation(&result) != ObservationRelation::Same {\n'+text.slice(b+'            } {\n'.length);
const style=fs.readFileSync(stylePath,'utf8');
const restoredStyle=style.replace('    pub fn p1339_observation_identity(&self) -> Option<usize> {','    pub(crate) fn p1339_observation_identity(&self) -> Option<usize> {');
const receipt={schema:'p1340-exact-inverse-cfg-binding-check-v1',meaning:'Static restriction witness only; not runtime equivalence or final verdict',owners:[{path:evalPath,current_sha256:current,inverse_sha256:sha(text),sealed_before_sha256:'8cac57e3d1d1f1401a8e0d63ede40d4fc0772d1df60134478a73f37d37b47e6a'},{path:stylePath,current_sha256:sha(style),inverse_sha256:sha(restoredStyle),sealed_before_sha256:'3f77f12991d4038b3c695606aec5d18934869c3a17a23bbe45e3c60f594287ed'}],ordinary_comparison:'The only shared expression is wrapped by cfg alternatives. The ordinary alternative is exactly the original comparison; the instrumented alternative computes it once, logs that discriminant, and tests it. All other additions and visibility changes are guarded by cfg(p1339_observation).'};
receipt.matches_sealed_preimages=receipt.owners.every(x=>x.inverse_sha256===x.sealed_before_sha256);
console.log(JSON.stringify(receipt,null,2));
if(!receipt.matches_sealed_preimages)process.exitCode=1;
