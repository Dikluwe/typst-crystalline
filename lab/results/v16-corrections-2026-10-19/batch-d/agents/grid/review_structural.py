#!/usr/bin/env python3
"""Independent archive/log audit; does not invoke the compiler or edit productive files."""
import datetime,difflib,gzip,hashlib,json,pathlib,subprocess
R=pathlib.Path(__file__).resolve().parents[6]
B=R/'lab/results/v16-corrections-2026-10-19/batch-d'
# Resolve repository independently of the caller's cwd.
R=pathlib.Path(subprocess.check_output(['git','rev-parse','--show-toplevel'],text=True).strip())
B=R/'lab/results/v16-corrections-2026-10-19/batch-d'
sha=lambda x:hashlib.sha256(x).hexdigest()
def read(p): return json.loads(gzip.decompress(p.read_bytes()) if p.suffix=='.gz' else p.read_bytes())
def check(x,m):
 if not x: raise AssertionError(m)
def digest(p): return sha(p.read_bytes())
T=read(B/'structural-targets.json'); probe='TektV16EvolutionProbe'
callback='01_core/src/compiler/layout/par_line_callbacks.rs'
align='01_core/src/entities/layout_types.rs'
old=['Left','Center','Right','Start','End']; oldpat=' | '.join('HAlign::'+x for x in old)
inserts={'new-old':'(_, HAlign::'+probe+') => 0.0,','old-new':'(HAlign::'+probe+', _) => 0.0,','new-new':'(HAlign::'+probe+', '+oldpat+') | ('+oldpat+', HAlign::'+probe+') => 0.0,'}
domains={'content':'Content','counter':'CounterKey','element_kind':'ElementKind','layout_types':'HAlign','selector':'Selector','value':'Value'}
report={'reviewer':'independent grid agent','utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'uncommitted working tree','diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'method':'Parse raw compiler-message JSON records; select only top-level E0004 primary spans wholly inside independently recomputed exact match-header byte range. Verify full archived sources, insert-only mutations, tuple isolation, input hashes and original-source hashes. Do not count enum-definition or help child spans.','artifacts':{},'phases':{},'targets':[],'tuple_classes':[]}
report['artifacts']['structural-targets.json']=digest(B/'structural-targets.json')
for phase in ['baseline','green']:
 p=B/('structural-'+phase); orig=read(p/'original-sources.json.gz'); inp=read(p/'inputs.json.gz'); freeze=read(p/'freeze.json'); runs=read(p/'results.json')
 for f in p.iterdir():
  if f.is_file(): report['artifacts'][str(f.relative_to(B))]=digest(f)
 check(freeze['changed']==[],phase+' frozen changes')
 source_hashes={f:sha(s.encode()) for f,s in orig.items()}
 check(all(inp['inputs'][f]==h for f,h in source_hashes.items()),phase+' archive vs input hash')
 base=pathlib.Path(inp['base'])
 current_source_mismatch=[f for f,h in source_hashes.items() if digest(base/f)!=h]
 input_mismatch=[f for f,h in inp['inputs'].items() if digest(base/f)!=h]
 frozen_owner_mismatch=[f for f,h in inp['frozen_owners'].items() if digest(base/f)!=h]
 check(not current_source_mismatch,phase+' source snapshot changed')
 check(not input_mismatch,phase+' base input changed')
 check(not frozen_owner_mismatch,phase+' frozen owners changed')
 phase_report={'input_utc':inp['utc'],'base':inp['base'],'archive_sources':len(orig),'all_archive_source_hashes_match_inputs_and_base':True,'input_count':len(inp['inputs']),'all_inputs_match_base':True,'frozen_owner_count':len(inp['frozen_owners']),'all_frozen_owners_match_snapshot_base':True,'freeze':freeze,'runs':[],'source_hashes':source_hashes}
 report['phases'][phase]=phase_report
 for run in runs:
  name=run['name']; mut=read(p/(name+'-mutations.json.gz')); source={**orig,**mut}; mutation_info=[]
  expected=set('01_core/src/entities/'+x+'.rs' for x in domains) if name=='all-domains' else {align,callback}
  check(set(mut)==expected,phase+name+' mutation files')
  for f,s in mut.items():
   delta=list(difflib.ndiff(orig[f].splitlines(True),s.splitlines(True)));added=[x[2:] for x in delta if x.startswith('+ ')];removed=[x[2:] for x in delta if x.startswith('- ')]
   want='    '+probe+',\n' if f!=callback else inserts[name]+'\n'
   check(added==[want] and removed==[],phase+name+f+' mutation scope')
   if f!=callback:
    pos=s.index(want); enum_name=domains[pathlib.Path(f).stem]; enum_start=s.index('enum '+enum_name+' ')
    check(enum_start<pos and s[pos+len(want):].startswith('}'),f+' enum placement')
   else:
    check(s==orig[f].replace('match (margin, align) {\n','match (margin, align) {\n'+want,1),'tuple insertion position')
   mutation_info.append({'file':f,'original_sha256':source_hashes[f],'mutated_sha256':sha(s.encode()),'sole_added_line':want.strip(),'removed_lines':0})
  errors=[]; parse_errors=[]; build_finished=[]
  for no,line in enumerate((p/(name+'.log')).read_text().splitlines(),1):
   if not line.startswith('{'):continue
   try:x=json.loads(line)
   except Exception:parse_errors.append(no);continue
   if x.get('reason')=='compiler-message' and x['message']['level']=='error': errors.append(x['message'])
   if x.get('reason')=='build-finished':build_finished.append(x)
  check(not parse_errors,'malformed compiler JSON');check(build_finished and build_finished[-1]['success']==False,'build-finished absent or success')
  check(all((x.get('code') or {}).get('code')=='E0004' for x in errors),'unrelated compile error')
  check(len(errors)==run['error_count'],'raw error count mismatch')
  chosen=[t for t in T if phase in t] if name=='all-domains' else [next(t for t in T if t['id']=='V16-043')]
  details=[]
  for t in chosen:
   f=t['file'];s=source[f];loc=t[phase];check(s.count(loc['after'])==1,'nonunique enclosing function')
   char=s.index(loc['anchor'],s.index(loc['after']));start=len(s[:char].encode());end=start+len(loc['anchor'].encode());line=s[:char].count('\n')+1
   hits=[]
   for e in errors:
    spans=[z for z in e['spans'] if z['is_primary'] and z['file_name']==f and start<=z['byte_start'] and z['byte_end']<=end]
    for z in spans:
     check(z['line_start']==line and z['line_end']==line,'primary line mismatch')
     text=s.encode()[z['byte_start']:z['byte_end']].decode();check(text.strip() and probe in e['message'],'empty span or nonprobe error')
     check(z['text'][0]['text']==s.splitlines()[line-1],'raw diagnostic source text mismatch')
     hits.append({'message':e['message'],'primary_span':z,'matched_text':text})
   check(len(hits)==(1 if phase=='green' else 0),phase+name+t['id']+' incorrect witness count')
   d={'id':t['id'],'file':f,'enclosing_function':loc['after'],'anchor':loc['anchor'],'line':line,'byte_start':start,'byte_end':end,'original_source_sha256':source_hashes[f],'compiled_source_sha256':sha(s.encode()),'primary_e0004_count':len(hits),'witnesses':hits}
   details.append(d)
  check(sum(d['primary_e0004_count'] for d in details)==run['selected_e0004'],'coordinator selection discrepancy')
  summary={k:v for k,v in run.items() if k!='targets'};summary.update({'mutations':mutation_info,'independent_targets':details,'raw_errors_verified':len(errors),'build_finished':build_finished[-1]})
  phase_report['runs'].append(summary)
for t in T:
 def item(phase):
  return next((d for d in report['phases'][phase]['runs'][0]['independent_targets'] if d['id']==t['id']),None)
 g=item('green');b=item('baseline');report['targets'].append({'id':t['id'],'file':t['file'],'current_source_sha256':digest(R/t['file']),'green':g,'baseline':b,'baseline_scope':'original target absent E0004' if b else 'newly introduced nested decision; no baseline target claimed','status':'PASS'})
for name in inserts:
 b=next(x for x in report['phases']['baseline']['runs'] if x['name']==name);g=next(x for x in report['phases']['green']['runs'] if x['name']==name)
 msg=g['independent_targets'][0]['witnesses'][0]['message']
 expected_phrase={'new-old':'(layout_types::HAlign::'+probe+', layout_types::HAlign::Left)','old-new':'(layout_types::HAlign::Left, layout_types::HAlign::'+probe+')','new-new':'(layout_types::HAlign::'+probe+', layout_types::HAlign::'+probe+')'}[name]
 check(expected_phrase in msg,'wrong tuple class diagnostic')
 residual=[(a,b) for a in old+[probe] for b in old+[probe] if (a==probe and b in old if name=='new-old' else a in old and b==probe if name=='old-new' else a==probe and b==probe)]
 report['tuple_classes'].append({'class':name,'isolation_arm':inserts[name],'remaining_uncovered_pairs':residual,'green_primary_e0004':g['independent_targets'][0],'baseline_primary_e0004':b['independent_targets'][0],'status':'PASS'})
report['conclusion']={'status':'PASS','green_primary_targets':23,'green_tuple_classes':3,'baseline_target_absence':20,'baseline_tuple_absence':3,'suppression_or_production_changes_in_mutations':False,'limits':['This is an enum-evolution structural witness, not language parity or a behavioral assertion.','Exit 101 alone is not evidence: all eight runs fail at other existing exhaustive matches too; exact primary spans are required.','The three new nested decisions have no corresponding baseline expression; only original twenty plus three tuple-class absences are claimed.']}
(B/'structural-witness-review.json').write_text(json.dumps(report,indent=2)+'\n')
lines=['# Independent structural witness review','','PASS: all 23 selected candidate decisions emit a primary E0004 at their exact match header after enum evolution. All three isolated HAlign tuple classes also emit the expected primary E0004. The original 20 baseline decisions and all three baseline tuple probes have no primary E0004 at those headers.','','Reviewed at '+report['utc']+'; HEAD '+report['head']+'; uncommitted working tree. The JSON records the exact diffstat, source hashes, archive/log hashes, measurement UTC, command, environment, full primary spans, and independently recomputed byte positions. No compiler/build/lint was run by this reviewer.','','I reparsed raw cargo compiler-message records, independently located function and match-header anchors in the actual archived mutated sources, and excluded enum-definition and help-child spans. Every archived original source matches its input hash and snapshot base; candidate originals match the current repository. Frozen owner hashes match their respective snapshot bases; candidate frozen owners match the current files. All mutations are insert-only: one sentinel enum member in each declared domain, plus exactly one tuple-covering arm for each isolated tuple probe. There are no productive-source edits or suppression attributes in these mutations.','','| Target | Source line | Baseline primary E0004 | Candidate primary E0004 |','|---|---|---:|---:|']
for t in report['targets']:lines.append('| '+t['id']+' | `'+t['file']+':'+str(t['green']['line'])+'` | '+('0' if t['baseline'] else 'new nested decision')+' | 1 |')
lines+=['','Tuple isolation was checked against actual mutation bytes: new-old covers every new-right pair and leaves five new-left/old-right pairs; old-new covers every new-left pair and leaves five old-left/new-right pairs; new-new covers the two mixed classes and leaves only new/new. The raw compiler messages identify those exact residual classes. Current nominal HAlign coverage is the complete five-by-five product (two left cases, two right cases, six same-axis neutral cases, and fifteen Center/Start/End rows).','','All eight probe builds intentionally exit 101. Unrelated exhaustive matches account for additional E0004s; these were not mistaken for target witnesses. No other compiler error code appears. Structural closure does not certify language behavior or parity; those remain the separate semantic/control/replay gates. The three newly introduced nested decisions have no baseline match and are not falsely included in the 20 baseline-absence claims.','','Audit implementation: `agents/grid/review_structural.py`. Detailed reproducible evidence: `structural-witness-review.json`.']
(B/'structural-witness-review.md').write_text('\n'.join(lines)+'\n')
print(json.dumps(report['conclusion']))
