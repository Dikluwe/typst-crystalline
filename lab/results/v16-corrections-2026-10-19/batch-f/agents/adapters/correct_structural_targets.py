from pathlib import Path
import json,gzip,hashlib,datetime,subprocess
root=Path.cwd(); base=Path('lab/results/v16-corrections-2026-10-19/batch-f'); out=base/'agents/adapters'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
tp=base/'structural-targets.json'; before=tp.read_bytes(); targets=json.loads(before)
assert not (out/'structural-targets-before-correction.json').exists()
(out/'structural-targets-before-correction.json').write_bytes(before)
for t in targets:
 if t['id']=='V16-063':
  t['baseline']['after']='// Campos de payload cuja presença é estrutural.'
  t['green']['after']='// Campos de payload cuja presença é estrutural.'
 if t['id']=='V16-065':t['green']['after']='fn radius_diameter('
targets.append({'id':'V16-054-selector','file':'01_core/src/compiler/stdlib/counter/display.rs','domain':'Selector','green':{'after':'CounterKey::Selector(selector) =>','anchor':'match selector {'}})
targets.append({'id':'V16-059-extract-tracks-rejection','file':'01_core/src/compiler/stdlib/layout/grid.rs','domain':'Value','green':{'after':'fn extract_tracks(','anchor':'match value {'}})
tp.write_text(json.dumps(targets,indent=2)+'\n')
b=base/'structural-baseline'; original=json.loads(gzip.decompress((b/'original-sources.json.gz').read_bytes())); errors=[]
for line in (b/'all-domains.log').read_text().splitlines():
 try:x=json.loads(line)
 except ValueError:continue
 if x.get('reason')=='compiler-message' and x['message']['level']=='error':errors.append(x['message'])
t=next(t for t in targets if t['id']=='V16-063'); s=original[t['file']]; spec=t['baseline']; p=s.index(spec['anchor'],s.index(spec['after'])); line=s[:p].count('\n')+1; bs=len(s[:p].encode());be=bs+len(spec['anchor'].encode())
selected=[e for e in errors if (e.get('code') or {}).get('code')=='E0004' and any(z['is_primary'] and z['file_name'].endswith(t['file']) and z['line_start']==line and bs<=z['byte_start']<be for z in e['spans'])]
assert not selected
r={'status':'corrected baseline evidence; original report retained untouched','utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'working_tree':'uncommitted working tree','changed_file_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'measurement':'Independent reparse of original raw compiler JSON; no compilation or product execution','historical_baseline':json.loads((b/'results.json').read_text())[0]['utc_start'],'bound_artifacts':{str(p):sha(p) for p in [b/'results.json',b/'all-domains.log',b/'original-sources.json.gz',b/'all-domains-mutations.json.gz',b/'inputs.json.gz',tp,out/'structural-targets-before-correction.json']},'original_wrong_anchor':{'line':28,'role':'source/occurrence normalization match; does not prove payload match'},'corrected_target':dict(t,line=line,byte_start=bs,byte_end=be,archived_source_sha256=hashlib.sha256(s.encode()).hexdigest(),diagnostics=selected),'compiler_error_count':len(errors),'primary_e0004_count_at_exact_payload_match':len(selected),'green_only_targets':['V16-054-selector','V16-059-extract-tracks-rejection'],'pending':'Green compilation must witness every target; geometry must make new extract_tracks rejection nominal.'}
(out/'structural-baseline-063-correction.json').write_text(json.dumps(r,indent=2)+'\n')
print(json.dumps({'line':line,'selected':len(selected),'targets':len(targets),'sha256':sha(tp)}))
