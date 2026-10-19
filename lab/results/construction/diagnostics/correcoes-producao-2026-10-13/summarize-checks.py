import pathlib,json,collections,re,gzip,hashlib,datetime,subprocess
root=pathlib.Path.cwd();dest=root/'00_nucleo/diagnosticos/correcoes-producao-2026-10-13'
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
for name in ['workspace-tests.log','workspace-tests-final.log','p1293-pinned-oracle.log','rustdoc-final.log','clippy-final-stderr.log']:
 p=dest/name;s=p.read_text() if p.exists() else ''
 counts=re.findall(r'test result: .*?\. (\d+) passed; (\d+) failed; (\d+) ignored;',s)
 result[name]={'sha256':hashlib.sha256(p.read_bytes()).hexdigest() if p.exists() else None,'test_targets':len(counts),'counts':{k:sum(int(x[i]) for x in counts) for i,k in enumerate(['passed','failed','ignored'])},'tail':s.splitlines()[-5:]}
unique={}
for l in (dest/'clippy-final.jsonl').read_text().splitlines():
 try:r=json.loads(l)
 except:continue
 if r.get('reason')!='compiler-message':continue
 m=r['message'];c=(m.get('code') or {}).get('code')
 if not c:continue
 spans=[s for s in m['spans'] if s['is_primary']]
 key=(c,m['message'],tuple((s['file_name'],s['byte_start'],s['byte_end']) for s in spans));unique[key]={'code':c,'level':m['level'],'message':m['message'],'locations':[{'file':s['file_name'],'line':s['line_start']} for s in spans]}
result['clippy']={'unique':len(unique),'by_code':dict(collections.Counter(x['code'] for x in unique.values()))}
(dest/'clippy-inventory.json').write_text(json.dumps(list(unique.values()),indent=2))
result['architecture']={}
for run in json.loads((dest/'architecture-final.sarif').read_text())['runs']:
 for r in run.get('results',[]):
  key=r.get('level','warning')+':'+r['ruleId'];result['architecture'][key]=result['architecture'].get(key,0)+1
with gzip.open(dest/'pre-final-checks.json.gz','rt') as f:before=json.load(f)
result['changes_during_final_checks']=[name for name,row in before['files'].items() if hashlib.sha256((root/name).read_bytes()).hexdigest()!=row['sha256']]
(dest/'validation-summary.json').write_text(json.dumps(result,indent=2))
print(json.dumps(result,indent=2))
