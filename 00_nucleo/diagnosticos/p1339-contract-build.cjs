// Contract author only: emit normative contract; do not run products or create oracles.
const fs = require('node:fs');
const crypto = require('node:crypto');
const D = '00_nucleo/diagnosticos/';
const read = n => JSON.parse(fs.readFileSync(D + n, 'utf8'));
const sha = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
// Retain every byte outside the unique validated canonical metadata line.
const l0Body = p => {
  const bytes=fs.readFileSync(p), text=bytes.toString('utf8');
  if(!Buffer.from(text).equals(bytes)) throw Error('Non-UTF8 L0 '+p);
  const lines=text.match(/[^\n]*\n|[^\n]+$/g)||[];
  let preamble=true,count=0,metadata;
  const kept=lines.filter((line,index)=>{
    const body=line.replace(/\n$/,'').replace(/\r$/,'').replace(index===0?/^\uFEFF/:/$^/,'');
    const authorized=preamble&&!/^\s*(```|~~~)/.test(body);
    let drop=false;
    if(authorized&&body.startsWith('Hash do Código:')) {
      if(!/^Hash do Código: [0-9a-f]{8}$/.test(body)) throw Error('Malformed L0 metadata '+p);
      count++; metadata=body.slice(-8); drop=true;
    }
    if(body===''||/^\s*(```|~~~)/.test(body)) preamble=false;
    return !drop;
  });
  if(count!==1) throw Error('Expected one canonical L0 metadata '+p);
  return {sha256:crypto.createHash('sha256').update(kept.join('')).digest('hex'),metadata};
};
const freeze = read('p1339-l0-freeze.json');
const authority = read('p1339-authority-manifest-r2.json');
for (const [p,h] of Object.entries(freeze.l0_sha256)) {
  if (sha(p) !== h) throw Error('Changed frozen L0: ' + p);
}
const inputNames = [
  'p1339-authority-manifest-r2.json','p1339-l0-freeze.json',
  'p1339-full-catalog.json','p1339-full-final-manifest.json',
  'p1339-full-final-vanilla-runs.json','p1339-full-final-crystalline-before-runs.json',
  'p1339-full-boundaries-manifest.json','p1339-full-boundaries-vanilla-runs.json',
  'p1339-full-boundaries-crystalline-before-runs.json','p1339-full-receipt.json',
  'p1339-nan-resolution.md','p1339-nan-review-r2.md',
  'p1339-show-witness-receipt.md','p1339-show-witness-bridge.json',
  'p1339-show-witness-final-manifest.json','p1339-show-witness-final-observation.json',
  'p1339-show-witness-final-vanilla-runs.json','p1339-show-witness-final-baseline-runs.json',
  'p1339-where-l0-vanilla.json','p1339-where-integration-probe-runs.json',
  'p1339-where-integration-probe-supplement-runs.json','p1339-where-occurrence-probe-runs.json',
  'p1339-where-counter-runtime-probe-runs.json','p1339-context-dependency-probe-runs.json',
  'p1339-stabilization-boundaries-runs.json','p1339-observation-design-boundaries.md',
  'p1339-observation-design-resolution.md','p1339-remaining-l0-design.md',
  'p1339-contract-design.md','p1335-manifest.json','p1335-probe-catalog.json',
  'p1335-matrix-normal.json',
];
const inputPins = Object.fromEntries(inputNames.map(n => [D+n,sha(D+n)]));
for(const p of ['00_nucleo/materialization/typst-passo-1339.md',
 '/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md',
 '/home/dikluwe/.codex/skills/tekt-materializacao-segregada/references/papeis-e-capacidades.md',
 '/home/dikluwe/.codex/skills/tekt-materializacao-segregada/references/artefatos-e-gates.md',
 '/repos/Antigravity/tekt-linter/03_infra/prompt_io.rs',
 '/repos/Antigravity/tekt-linter/03_infra/prompt_reader.rs',
 '/repos/Antigravity/tekt-linter/03_infra/hash_writer.rs',
 '/repos/Antigravity/tekt-linter/03_infra/nucleus.rs',
 '/home/dikluwe/.cargo/bin/crystalline-lint']) inputPins[p]=sha(p);
const catalog = read('p1339-full-catalog.json');
const legacyIntSignum = new Set(['signum-bound-1','signum-bound--1','signum-bound-true','signum-bound-"1"','signum-bound-1deg','signum-bound-none']);
const cases = catalog.cases.map(({stage,case:c}) => {
  let target = 'vanilla', obligation = c.route || 'function.where';
  let reason = 'Required public route or its expressly amended diagnostic/consumer';
  if(stage === 'show-final') { target='successor_compile_witness'; reason='Use immutable show bridge; historical transport Unknown is retained'; }
  else if(c.id.startsWith('angle-construction-')) { target='baseline'; reason='Producer control outside conversion change; does not establish actual Angle NaN coverage'; }
  else if(/^angle\.(deg|rad)-(static|bound)-\(float\("nan"\)/.test(c.id)) {
    target='conditional_unactivated'; reason='Actual NaN Angle not constructible bilaterally per pinned resolution; no success credit';
  } else if(/^version-(control-|display-|field-)/.test(c.id) || legacyIntSignum.has(c.id)) {
    target='baseline'; reason='Preexisting member/receiver behavior explicitly preserved by frozen L0';
  }
  return {stage,id:c.id,obligation,target,mandatory:target!=='conditional_unactivated',reason};
});
const obligations = [
 {id:'angle.deg',owner:'compiler/eval/call_dispatch.md',cause:'entities/layout_types.md::Angle::to_deg',requirements:['Function, public name/repr deg; Angle -> Float degrees','Static and called bound forms share one parser and existing semantic method','Finite degree/radian inputs, both signed zeros, positive/negative infinity','Reject unitless Int/Float and other values; self positional; exact missing/cast/extra/named/hints/value spans/call spans','Alias/With/spread preserve argument origins; field extraction without call follows measured error'],witnesses:['angle.deg-static-90deg','angle.deg-static-1rad','angle.deg-zero-observable','angle.deg-invalid-1']},
 {id:'angle.rad',owner:'compiler/eval/call_dispatch.md',cause:'entities/layout_types.md::Angle::to_rad',requirements:['Function, public name/repr rad; Angle -> Float radians','Same parser and existing cause for static/bound; no reinterpretation as constructor','All corresponding angle.deg input/cast/origin/control dimensions in requested radian unit'],witnesses:['angle.rad-static-90deg','angle.rad-static-1rad','angle.rad-zero-observable']},
 {id:'float.inf',owner:'compiler/stdlib/foundations/float.md',requirements:['Float positive infinity, repr float.inf, not function or finite sentinel','Type, arithmetic, sign and comparison observations'],witnesses:['discovery-float.inf','float.inf-arithmetic']},
 {id:'float.nan',owner:'compiler/stdlib/foundations/float.md',requirements:['Float NaN, repr float.nan, not equal to itself','NaN remains distinct from zero and infinity; no global equality repair'],witnesses:['discovery-float.nan','float.nan-arithmetic']},
 {id:'float.signum',owner:'compiler/stdlib/foundations/float.md',requirements:['Function signum; Float or coerced Int -> Float','Positive and +0 -> 1.0; negative and -0 -> -1.0; infinities retain sign; NaN -> NaN','Static and Float-bound call use same native parser/cause','Reject Bool/Str/Ratio/Angle; preserve existing Int-bound signum and non-Float methods','self positional; exact evaluation order, first extra/named occurrence, hint and diagnostic origin'],witnesses:['signum-static--0.0','signum-static-float("nan")','signum-bound--0.0']},
 {id:'float.from-bytes',owner:'compiler/stdlib/foundations/float.md',requirements:['Function from-bytes; strictly Bytes positional; returns Float','Only length4 binary32 promoted to64 or length8 binary64','Default endian little; big/little exact independent byte vectors','Preserve signed zero, finite, infinity, NaN and subnormal decoding','Reject lengths0,3,5,7,9; exact casts, enum, arity, named, hints and spans','Float-bound lookup reaches bytes cast error on receiver; Bytes does not acquire method'],witnesses:['from-bytes-default','from-bytes-length-3','from-bytes-independent-bits-(0,0,0,128)']},
 {id:'float.to-bytes',owner:'compiler/stdlib/foundations/float.md',requirements:['Function to-bytes; Float/Int receiver; returns Bytes, never integer Array','size u32 default8, only4/8; endian defaultlittle, big/little','Exact IEEE result bytes, including negativezero, independent bit inputs, binary32 rounding/subnormal/underflow/overflow','Preserve conversion errors and parser order self,endian,size,then leftovers; sizecast precedes domain4/8','Static/Float-bound one native implementation; complete diagnostics and trace through alias/With'],witnesses:['to-bytes-default','to-bytes-4-big-static-1.5','to-bytes-size-3','bytes-rounding-0.1-little']},
 {id:'function.with',owner:'compiler/eval/call_dispatch.md',cause:'Func::with plus causal Args merge',requirements:['Function with; static requires Func self, bound receiver resolves before args','Construct lazily without invoking receiver or validating its target signature','Closure/native/element/With receivers; positional and named preargs before newer bindings and callargs','All original occurrences retained; last named value with native causal validation; literal duplicate remains syntax error','Spread order, alias, panic order, single receiver evaluation and original spans/traces','Do not promote Type to Func or expose bound method extraction'],witnesses:['with-static-multi','with-bound-multi','with-static-spread-duplicate','with-static-invalid-receiver-panic','with-bound-invalid-receiver-panic','with-static-lazy-receiver']},
 {id:'function.where',owner:'compiler/eval/bindings/value_methods.md',requirements:['Function where; only real native element functions by typed identity, alias preserved','Reject closure/non-element native/nonfunction/With(element); no executing receiver to identify it','Static and bound share one semantic constructor after distinct measured evaluation order','Ordered unique field group, last spread value at first key position; empty group retained','Validate field names, never cast filter values using constructor parameter types','Selector repr/type/equality/Neq/membership nesting preserve order, int/float language equality, NaN nonreflexivity','Mandatory integrated consumers described in W01-W10 below; discovery alone insufficient'],witnesses:['where-static-element-multifield','where-static-closure','where-field-priority-static-level:"bad"','where-static-spread-duplicate','where-selector-morphology']},
 {id:'version.at',owner:'compiler/stdlib/primitives-constructors/version.md',cause:'entities/version.md::Version::at',requirements:['Function at; static/bound share parser and existing Version::at formula','Strict Version self and Int i64 index; positive outoflength gives0','Negative index uses explicit component list incl trailingzeros, never infinite padding','Empty version, -1, lower boundary, i64 MIN/MAX; original outofbounds index/message no overflow','Exact missing/self/index named-position errors, casts, first extras, hints, call/value/occurrence spans','Preserve constructor/major/minor/patch/repr/display/ordering/PARITY_VERSION and no bound value extraction'],witnesses:['version-static-version(1,2,3)--4','version-bound-version(1,2,3)--4','version-static-version()-0','version-static-version()--1']},
];
const integrated = [
 ['W01','Selector representation and matching','Element function + ordered complete group incl empty; native identity distinct from names; clone/selector/And/Or/Within transport intact. Repr emits one .where group. Equality uses language recursively and preserves field order; no global Rust Eq/Hash change. Show must execute callback on matching element nodes, not mere repr or descendant text.'],
 ['W02','Occurrences and queried content','Strong/Emph own nodes, including empty/nested/reused bodies, each have one Location and correct parent/document order. Styled and render bold/italic create no extra occurrence. Walk/layout align. Snapshot Some complete and authoritative incl delta300/body/label; None uses legacy fallback. Missing differs from explicit None. Text where valid for show but query/counter rejects nonlocatable.'],
 ['W03','Filtered counter actions','Preserve entire selector key, empty distinct from bare, field order distinct, int/float equal, NaN nonreflexive. Automatic effective actions and manual Step/Set/Func in causal order; Step/Func/Step result12; do not infer actions from count/query.len/snapshotdifferences.'],
 ['W04','Demand and callback phase','Only actual get/at/final/display demand of a key containing Element replays its complete sequence before prefix selection. Later failing callback may fail earlier get. Undemanded callback never runs. Callback receives state components without introspective context, retaining lexical capture and Engine. Legacy keys/state/numbering phases preserved.'],
 ['W05','Context selection and ordinary seed boundary','Record reads from start of each block, including legacy reads before actual Element demand. Demand marks before location/label/fold error. No AST/name/errorstring selection. Blocks/independent siblings not reaching demand preserve ordinary baseline contribution/error. A selected generation discards ordinary realization/sink and starts I0 observationally empty; no legacy producer updates in I0.'],
 ['W06','All observed dependencies and projections','Selected block validates counter resolutions/full fold/prefix/total, state key/init/fallback/label and value before displaycallback, query order/Location/fullcarrier/fields, locate first/none, here, Location page/position/rawpage-numbering, including errors. Same locations/pages alone do not certify full query; x/y change affects position but not page; page-numbering callback is never invoked merely for comparison.'],
 ['W07','Closed observation relation and causal retention','Private Same/Different/Unproven, exhaustive variant/field recursion; IEEEbits include signedzero and reflexive identical NaN only for observations. Arrays/dicts/Args/Selectors/Content/LocatedContent recurse including styles and presence. Same immutable Func/Module producer preserves identity; newly recreated Func is not Same by body/Debug. Retain output when producerinputs valid; invalidate descendants when producer/capture/chain/resources change. Do not modify public language equality.'],
 ['W08','Attempts and final document','Ak reads I(k-1), builds Ck/Dk and complete Ik including page/position stores. Reevaluate only invalid selected blocks and replaced descendants. No accumulated updates or obsolete success after error. New real nested context discovery does not reset budget. Five total attempts shared with paginated invalidation; without final error return D5 computed from I4, never A6/D4/replay-output from I5.'],
 ['W09','Diagnostics and sinks','Same persistent read error can validate stable but body remains Err. Final pending error prevents export. Invalidated attempt/validation sinks discarded, retained contributions published once. Nonconvergence warnings/valuehistory from final actual requests projected over I0..I5; I0=run1,I4=run5,I5=final. No invented warnings for Unproven or generic error at valid nonconvergence cap. Preserve exact warning order, hints, spans, history and observed final value.'],
 ['W10','Architecture and preserved resources','One semantic cause for each static/bound family; field_access/hubs contain routing only. L1 pure; no registry/dyn/reflection/namefallback. EntityIntrospector trait/struct unchanged; concrete impl transferred to compiler owner without legacy behavioral changes. Only approved SelectorElement, ShowNativeElement, PayloadNativeElement and EvalContext interface changes. World/capture/Location/styles/target/features immutable per retained attempt, with per-request style capture.'],
].map(([id,title,predicate])=>({id,title,predicate,mandatory:true,requires:'Independent public fixtures plus targeted structural/trait tests where property is not externally discriminable; all must be mapped before seal'}));
const mutations = [
 ['M01','Swap deg and rad','angle.deg/rad','Distinct unit value and name; 90deg and1rad conversion witnesses'],
 ['M02','Interpret radians as degrees or converse','angle.deg/rad','Nonzero unit cross-conversion in static and bound calls; zero alone insufficient'],
 ['M03','Finite float.inf','float.inf','Positive infinity type/repr/arithmetic/finite predicates'],
 ['M04','Reflexive float.nan','float.nan','NaN self equality false with ordinary float equality positive control'],
 ['M05','Drop negativezero sign in signum','float.signum','-0.0 -> -1.0 and +0.0 ->1.0'],
 ['M06','signumNaN returns zero','float.signum','Result NaN/nonreflexive, not zero; finite controls'],
 ['M07','Swap endian','float.from-bytes/float.to-bytes','Independent 4/8 byte vectors in both directions; roundtrip alone insufficient'],
 ['M08','Accept illegal byte length','float.from-bytes','Length0/3/5/7/9 exact failure; valid4/8 controls'],
 ['M09','Ignore size','float.to-bytes','Same value with size4 and8 has exact distinct length/bits; default8'],
 ['M10','Return Array instead of Bytes','float.to-bytes','Public type bytes plus exact array projection; equal integers do not satisfy type'],
 ['M11','Discovery-only stub','all ten routes','At least real successful invocation/value for each callable family, not only type/repr'],
 ['M12','Duplicate static and bound semantic formula','W10','Compile-valid structural negative has two effective semantic definitions instead of shared owner; blackbox equality expected and not sufficient'],
 ['M13','Reverse with preargs/newargs','function.with','Asymmetric multi-binding positional/named call result and panic/origin witnesses'],
 ['M14','Allow closure where','function.where','Closure and non-element native rejected before any receiver body execution'],
 ['M15','Discard where named fields','W01/W02','Match/miss applied show and query/counter witnesses, ordered nonempty repr/equality'],
 ['M16','Different static version.at rule','version.at','Paired static/bound boundary indices incl positive padding and negative bounds'],
 ['M17','Negative version padding','version.at','Empty.at(-1) and below explicit length fail with original index/len'],
 ['M18','Accept rejected named arguments','angle/float/version','Named required-position and unknownnamed fail with exact hint/span and nonambiguous valid control'],
 ['M19','Alter one of18 scopeout routes','scopeout18','Probe present/discovery/behavior differs from original matrix baseline; negative compiled against vanilla must differ from vanilla control on same protected route'],
 ['M20','Mandatory error becomes Unknown','protocol','Real execution becomes unobservable; raw Unknown retained, mandatory predicate rejects with mandatory_unknown; absent diagnostic span alone is known Violation'],
].map(([id,mutation,obligation,witness])=>({id,mutation,obligation,witness,required:true,expected_gate:'Violated',eligibility:'Compile-valid, real executed behavioral change or M12 structural change, one independently identified cause; no changed oracle/result JSON'}));
const supplements = [
 ['where-l0','p1339-where-l0-vanilla.json','rows','W01'],
 ['where-integration','p1339-where-integration-probe-runs.json','cases','W01,W02,W03'],
 ['where-integration-supplement','p1339-where-integration-probe-supplement-runs.json','cases','W02,W03'],
 ['where-occurrence','p1339-where-occurrence-probe-runs.json','cases','W02'],
 ['where-counter-runtime','p1339-where-counter-runtime-probe-runs.json','cases','W03,W04,W05'],
 ['context-dependency','p1339-context-dependency-probe-runs.json','cases','W05,W08,W09'],
 ['stabilization-boundaries','p1339-stabilization-boundaries-runs.json','cases','W05,W07,W08,W09'],
].map(([id,artifact,collection,obligation])=>({id,artifact:D+artifact,sha256:inputPins[D+artifact],collection,obligation,policy:'All source cases require an oracle mapping before seal; named preservation exceptions below use baseline. A unilaterally measured case needs bilateral baseline evidence before RED. Historical query transport may be replaced only by pinned causal successor bridge, never silently treated as a passing probe.'}));
const result = {
 schema:'p1339-contract-v1',revision:1,status:'candidate_for_independent_oracles_and_discrimination_NOT_SEALED',
 author:'/root/p1316_review',regime:'executado sem atestação de isolamento',
 authority_manifest:{path:D+'p1339-authority-manifest-r2.json',sha256:inputPins[D+'p1339-authority-manifest-r2.json']},
 l0_freeze:{path:D+'p1339-l0-freeze.json',sha256:inputPins[D+'p1339-l0-freeze.json']},
 provenance:{created_utc:new Date().toISOString(),head:freeze.head,baseline_state_receipt:D+'p1339-l0-freeze.json',baseline_diff_stat:freeze.diff_stat,baseline_binaries:freeze.binary_sha256,inherited_context:'Completed P1316 independent review; no P1339 candidate source read; contract role only',sources_read:'Frozen L0 sections for all31 owners and allowed diagnostic measurements; no P1339 productive candidate',writes:['p1339-contract.json','p1339-contract-design.md','p1339-contract-build.cjs'],tooling_sha256:sha(__filename)},
 input_sha256:inputPins,l0_raw_sha256:freeze.l0_sha256,
 l0_normative_sha256:Object.fromEntries(Object.keys(freeze.l0_sha256).map(p=>[p,l0Body(p).sha256])),
 l0_derived_metadata_at_freeze:Object.fromEntries(Object.keys(freeze.l0_sha256).map(p=>[p,l0Body(p).metadata])),
 l0_hash_policy:{raw:'Raw hashes pin the exact pre-candidate snapshot and remain immutable historical records. Final raw may differ solely in the eight lowercase hex digits of the one canonical Hash do Código line.',normative:'Remove only that validated line in preamble before first empty line/fence, preserving all other bytes. Full SHA256 must equal frozen normative SHA256. Do not strip similarly named body examples, pins, headings, whitespace, comments or other metadata.',derivation:'Before/after raw hashes, exact eight-digit old/new field, productive owner source hash excluding canonical @prompt-hash and independent strict V5/V15/V26 plus fix-hashes dryrun Nothing to fix are mandatory. No nucleus refresh, owner change or other L0 change qualifies. Any other changed byte invalidates the seal.',source_evidence:['/repos/Antigravity/tekt-linter/03_infra/prompt_io.rs:164','/repos/Antigravity/tekt-linter/03_infra/hash_writer.rs:15','/repos/Antigravity/tekt-linter/03_infra/hash_writer.rs:63','/repos/Antigravity/tekt-linter/03_infra/nucleus.rs:200'],interpretation:'Derived lineage metadata is not a normative amendment. This exception is fixed before seal, not retroactive waiver. The pinned skill/step require frozen obligation/provenance; neither mandates an impossible immutable derived code hash after implementation.'},
 routes:authority.obligations,obligations,integrated_obligations:integrated,
 historical_case_index:cases,supplement_sets:supplements,
 oracle_interface:{schema:'p1339-oracle-v1',required_fields:['id','contract_sha256','authority_manifest_sha256','obligation_ids','mandatory','role','source_ref_or_literal_with_sha256','adapter','profiles','orders','reference_policy','expected_observation_or_reference_with_sha256','raw_input_provenance'],roles:['positive','expected_error','preservation','opaque'],reference_policies:['vanilla','baseline','normative_l0','causal_successor'],coverage:'Every mandatory historical case, supplement source case and W01-W10 requirement maps to at least one frozen independent oracle; listed cases are lower bounds, not semantic domain limits. Duplicates explicitly mapped, never silently dropped.'},
 observation_interface:{schema:'p1339-observation-v1',required_fields:['oracle_id','case_id','profile','order','adapter','source_sha256','binary_sha256','argv','env','cwd','start_utc','end_utc','exit','stdout','stderr','execution','artifacts'],execution:['Observed','Unknown'],raw_policy:'Keep byte-complete stdout/stderr and exit/signal. Preserve source and tool/binary identity; no trimming, line dropping, rewriting paths/messages, NaN coercion, sorting diagnostics or guessed ranges. A declared adapter may parse an observable but never overwrites raw channels.'},
 adapters:{
  eval_exact:'For identical frozen expression, argv and diagnostic environment compare full exit/stdout/stderr to designated reference. Public repr strings and exact IEEE bytes remain observable; no float tolerance or repr rounding.',
  compile_witness:'Same pinned source path for both products, explicit PDF target and profile flags. CallbackExecuted requires exact primary sentinel error and nonzero exit; marker printed inside an unrelated error is not execution. NoCallback requires successful compile and produced artifact. Compare entire fresh reference diagnostics separately. Output PDF bytes are not parity criterion.',
  query_values:'Only where CLI transport is established on both products/profiles. Parse specified output field without losing type/order/presence and retain full raw diagnostics. Compare actual query values and exact language warnings; command deprecation is a separate transport observation, not a fabricated product warning.',
  causal_successor:'For unsupported historical query --features or compile stdin transport, oracle author freezes a named-source compile/assert or supported transport fixture before candidate. Retain original source/raw Unknown, source transformation and obligation mapping; measure new vanilla and baseline. Same selector/subject/read graph must be demonstrated; assertions/sentinels must not replace the value under test or erase convergence/history. A metadata wrapper may observe its already-evaluated argument and return original metadata; it may not introduce a new context/query demand. Assertion failures during invalid attempts must demonstrably be discarded, not terminate before the intended A5 observation; serializability alone does not prove an assertion bridge. Compare fresh reference warnings with exact source map and final value; retain original diagnostics separately. Profiles cannot be omitted; env-based features require a positive enabled-feature control and negative control proving actual propagation.',
  architecture:'Verifier resolves source definitions/call edges with exact path:line and hashes, compiler configuration and successful build, plus productive ownership. Static/bound paths must share semantic owner and parser; lexical name/regex alone insufficient, unresolved macros/call identity -> Unknown. Baseline vanilla negative tests shared-cause property, not crystalline L0 ownership.',
  closed_state:'Targeted tests of real approved APIs/carriers may exercise Same/Different/Unproven, projection and causal-retention requirements not separable through CLI. Inputs/outputs must be constructed and executed in compiled harness, not asserted JSON. Test-only access must not change public product API; missing ability to observe an obligatory property blocks seal.'
 },
 classification:{Observed_preserved:'All required dimensions equal designated reference and provenance valid => Preserved',Observed_difference:'Any required value/type/morphology/error/origin/order/effect/architectural difference => Violated with first concrete witness and reason',Unobservable:'Missing input/provenance, timeout/crash, unsupported transport, opaque identity or comparator Unproven => raw Unknown; mandatory obligation is rejected as mandatory_unknown without recoding raw outcome',Opaque:'A deliberately declared opaque control must produce raw Unknown and no false Preserved; it has no positive coverage credit',Aggregation:'Seal/closure require all mandatory positives preserved, expected-errors exact, all valid negative families rejected, declared opaque controls Unknown, deterministic replays, no missing cells or unresolved mandatory Unknown. Unknown is never success by default.'},
 preserved_exceptions:{
  conditional_angle_nan:{ids:cases.filter(c=>c.target==='conditional_unactivated').map(c=>c.id),status:'Unknown',mandatory:false,credit:0,basis:['p1339-nan-resolution.md','p1339-nan-review-r2.md'],refutation:'Safe public source/binary-pinned producer yields actual NaN Angle in vanilla, or explicit normative change; infinity remains required'},
  optional_plugin:{id:'plugin-function-receiver',status:'Unknown',mandatory:false,credit:0,basis:'Historical catalog optional dimension; not an excuse to omit required closed Func/Module cases'},
  occurrence_producers:{set_strong_delta:'baseline preservation; constructor/set delta is not admitted',explicit_delta:'baseline preservation; constructor/set delta is not admitted',constructor_projection:'baseline preservation of raw constructor projection; queried fields remain separate W02 obligation',text_style_controls:'baseline preservation of visual-style-only producer; no new locations from render style'},
  bare_consumers:'Baseline observations for counter_heading_bare/query_heading_bare and unrelated legacy counter/state helpers are protected; new Element-bearing query/counter requirements are not collapsed to those baselines',
  context_unselected:['plain_set_control','context_set_forward','context_set_final_before','stable_error_control','unrelated_error'],
  context_unselected_policy:'Keep baseline whole outcome for these no-Element cases and independently constructed seed/sibling-before-demand controls. Only an actually selected generation gains new retries. When the same selected block reads legacy data, those observations remain required.',
  direct_show_controls:'strong/emph/text direct-no-where controls retain baseline; known bare-text debt is not assigned to new where. The18 selectors themselves require reference matching.',
  independent_supplement_producers:'Do not repair Angle construction, strong named/set delta, Version named fields or legacy bare state/counter to make a larger fixture pass. Oracle must isolate its authorized observable and retain old producer control; unresolved inseparable required witness blocks rather than enlarging scope.'
 },
 mutation_protocol:{families:mutations,required_family_count:20,minimum_score:1,score:'valid required families with specific rejection witness /20; no score until all20 have compile-valid executed representatives. Additional mutants do not compensate missing families. Invalid/surviving/Unknown-eligibility/unexecuted mandatory family blocks.',pre_candidate_host:'Isolated copy of vanilla ratified source, never modify reference binary/lab or live crystalline consumers. P1338 absent route is RED baseline, not mutation positivebase.',multiplex:'Semantic modes may share an instrumental executable only if mode0 is exact control, one mode one isolated cause, explicit pinned env, invalid mode failclosed, no fixture/expected-data lookup. M12 must use distinct effective compilation artifacts/configs; runtime inactive duplicate still violates physical unique-cause property.',m20:'Only deliberately injected, provenance-complete M20 negative may demonstrate correct mandatory_unknown gate rejection while retaining raw Unknown. This is evidence the negative cannot be admitted, never product-preservation credit. Unknown eligibility, missing execution/provenance, or Unknown on unmutated positive/base remains a blocker. Missing span on ordinary observable error is known violation, not Unknown. Use real abnormal/inobservable execution to exercise Unknown boundary, not result fabrication.',calibration:'Focal cases plus positive/opaque controls first, then full gate including repetition/reorder. Freeze source/binary/patch/mode and witness. Verifier determines eligibility and rejection, adversary does not selfcertify.'},
 scopeout18:['color.spot','color.spot.tint','enum.item','figure.caption','footnote.entry','list.item','outline.entry','outline.entry.body','outline.entry.indented','outline.entry.inner','outline.entry.page','outline.entry.prefix','par.line','polygon.regular','raw.line','selector.after','selector.before','terms.item'],
 global_preservation:{manifest:D+'p1335-manifest.json',matrix:D+'p1335-matrix-normal.json',catalog:D+'p1335-probe-catalog.json',required:'Reexecute same4718-probe universe, four profiles; ten discovery routes may converge, every other changed cell must be explained against exact authorized L0. Never edit historical matrix or denominator. Preserve18scopeouts and all outside-lot semantics; no aggregate count substitutes case-by-case comparison.',expected_delta_only:{equal_probes:[4533,4543],divergent_probes:[185,175],vanilla_only_routes:[103,93],vanilla_only_cells:[324,284],equal_cells:[18220,18260],divergent_cells:[652,612]}},
 profiles:{default:[],html:['html'],a11y:['a11y-extras'],'html+a11y':['html','a11y-extras']},orders:['normal','repeat','reverse'],
 gates:{preseal:['input integrity including31normativeL0','complete oraclecoverage','mode0positivebase valid','all20negativefamilies compilevalid and rejected','positive/opaque discrimination and determinism','independent verifier seals hashes'],preimplementation:['seal valid','independent RED on actual P1338 baseline ten discovery routes plus functional missing obligations; positive controls green'],final:['focal suite','all frozen A/B orders/profiles','real mutant suite','scopeout/globalrebaseline','cargo fmt --all -- --check','git diff --check','cargo build --workspace --release --locked','cargo test --workspace --release --locked --no-fail-fast','crystalline-lint .','strict V5/V15/V26','sealed input integrity','independent PASS_SCOPED only after all gates']},
 budget:authority.budget,invalidation:'Normative L0, contract, oracle, fixture, baseline or protected mode changes invalidate successors from earliest affected phase. Canonical Hash do Código line reseal alone is allowed if31normative hashes stay same and lineage validates. Preserve previous revisions and raw histories.',
 no_claims:['No oracle authored here','No mutation executed here','No discrimination verdict','No seal','No implementation or GREEN','No isolation attestation','No general Typst equivalence'],
};
console.log(JSON.stringify(result,null,2));
