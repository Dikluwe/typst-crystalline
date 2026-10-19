from pathlib import Path
import json,subprocess,hashlib,tempfile,datetime,sys
root=Path.cwd();out=root/'00_nucleo/diagnosticos/tekt-frame-items-2026-10-16';phase=sys.argv[1];assert phase in ['baseline','green'];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();source=out/'workers/audit.rs';meta=out/'audit-tool.json'
if meta.is_file():
 tool=json.loads(meta.read_text());assert tool['source_sha256']==sha(source) and sha(tool['binary'])==tool['binary_sha256'];binary=Path(tool['binary'])
else:
 cache=Path('/tmp/tekt-linter-ci-target/debug/deps');externs={n:list(cache.glob('lib'+n+'-*.rlib')) for n in ['tree_sitter','tree_sitter_rust','serde_json']};assert all(len(v)==1 for v in externs.values());directory=Path(tempfile.mkdtemp(prefix='tekt-frame-family-'));binary=directory/'audit';cmd=['rustc','--edition=2021','--crate-name','frame_family','-L','dependency='+str(cache)]
 for n,v in externs.items():cmd+=['--extern',n+'='+str(v[0])]
 cmd += [str(source),'-o',str(binary)]
 with (out/'audit-tool-compile.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
 assert r.returncode==0,(out/'audit-tool-compile.log').read_text()
 tool=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source=str(source),source_sha256=sha(source),binary=str(binary),binary_sha256=sha(binary),command=cmd,externs={n:dict(path=str(v[0]),sha256=sha(v[0])) for n,v in externs.items()});meta.write_text(json.dumps(tool,indent=2)+'\n')
cmd=['/tmp/tekt-linter-ci-target/debug/crystalline-lint','.','--checks','v16','--format','sarif','--fail-on','warning'];sarif=out/f'family-{phase}-sarif.json'
with sarif.open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
results=json.loads(sarif.read_text())['runs'][0]['results'];assert not any(x['level']=='error' for x in results)
contexts=out/f'family-{phase}-contexts.json';command=[str(binary),str(root),str(sarif),str(root/'00_nucleo/diagnosticos/tekt-enums-2026-10-16/latent-v16.json')]
with contexts.open('w') as f:parse=subprocess.run(command,stdout=f,stderr=subprocess.PIPE,text=True)
assert parse.returncode==0,parse.stderr
import re
rows=json.loads(contexts.read_text());selected=[x for x in rows if x['status']=='emitted-warning' and re.search(r'\b(?:FrameItem|FI)::',' '.join(a['pattern'] for a in x['arms']))];global_count=sum(x['level']=='warning' for x in results);expected=70 if phase=='baseline' else 0;expected_global=312 if phase=='baseline' else 242
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),phase=phase,linter_command=cmd,global_exit=r.returncode,global_v16_warnings=global_count,parse_command=command,selected_frameitem_warnings=len(selected),selected=[dict(source=x['source'],line=x['line'],function=x['function']) for x in selected],outside_warnings=global_count-len(selected),exit=int(bool(selected)),sarif_sha256=sha(sarif),contexts_sha256=sha(contexts));(out/f'family-{phase}-result.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n');print(json.dumps({k:record[k] for k in ['phase','global_exit','global_v16_warnings','selected_frameitem_warnings','outside_warnings','exit']}));assert len(selected)==expected and global_count==expected_global and r.returncode!=0;sys.exit(record['exit'])
