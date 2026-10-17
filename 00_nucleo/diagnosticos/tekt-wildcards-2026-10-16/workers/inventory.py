from pathlib import Path
import json,hashlib,subprocess,datetime,tempfile,gzip,collections,re
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-wildcards-2026-10-16';stage=root/'00_nucleo/diagnosticos/tekt-enums-2026-10-16';assert json.loads((stage/'validation-result.json').read_text())['exit']==0
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();validation=json.loads((stage/'validation-details.json').read_text());assert all(sha(p)==h for p,h in validation['source_hashes'].items())
cache=Path('/tmp/tekt-linter-ci-target/debug/deps');externs={n:list(cache.glob('lib'+n+'-*.rlib')) for n in ['tree_sitter','tree_sitter_rust','serde_json']};assert all(len(v)==1 for v in externs.values());source=out/'workers/contexts.rs';record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=validation['head'],validation_source_hashes=validation['source_hashes'],input_sarif=str(stage/'validation-architecture.log'),input_sarif_sha256=sha(stage/'validation-architecture.log'),helper_source_sha256=sha(source),externs={n:dict(path=str(v[0]),sha256=sha(v[0])) for n,v in externs.items()},role='read-only AST contexts for the next V16 decisions; not L0 or a code prescription')
with tempfile.TemporaryDirectory(prefix='tekt-v16-ast-') as tmp:
 binary=Path(tmp)/'contexts';cmd=['rustc','--edition=2021','--crate-name','v16_contexts','-L','dependency='+str(cache)]
 for n,v in externs.items():cmd+=['--extern',n+'='+str(v[0])]
 cmd += [str(source),'-o',str(binary)];record['compile_command']=cmd
 with (out/'compile.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 assert r.returncode==0,(out/'compile.log').read_text()
 command=[str(binary),str(root),str(stage/'validation-architecture.log'),str(root/'00_nucleo/diagnosticos/tekt-guards-2026-10-16/latent-v16.json')];record['parse_command']=command
 with (out/'contexts.json').open('w') as f:r=subprocess.run(command,stdout=f,stderr=subprocess.PIPE,text=True)
 assert r.returncode==0,r.stderr
 rows=json.loads((out/'contexts.json').read_text());assert len(rows)==314 and sum(r['status']=='emitted-warning' for r in rows)==312
 owners=sorted(set(r['owner'] for r in rows));files={p:Path(p).read_text() for p in owners};record.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),rows=len(rows),emitted=312,tracked_without_emission=2,owners=len(owners),contexts_sha256=sha(out/'contexts.json'),sources_unchanged=all(sha(p)==h for p,h in validation['source_hashes'].items()));assert record['sources_unchanged']
 with gzip.open(out/'owners-context.json.gz','wt') as f:json.dump(dict(record=record,files=files),f)
 (out/'inventory-details.json').write_text(json.dumps(record,indent=2)+'\n')
 groups=collections.Counter()
 for row in rows:
  prefixes=collections.Counter(re.findall(r'\b([A-Z][A-Za-z0-9_]*)::',' '.join(a['pattern'] for a in row['arms'])))
  for key in ['Some','None','Ok','Err','Self']:prefixes.pop(key,None)
  groups[prefixes.most_common(1)[0][0] if prefixes else 'unresolved-type']+=1
 (out/'family-index.json').write_text(json.dumps(dict(groups),indent=2)+'\n');print(json.dumps(dict(rows=len(rows),owners=len(owners),groups=groups,sources_unchanged=True)))
