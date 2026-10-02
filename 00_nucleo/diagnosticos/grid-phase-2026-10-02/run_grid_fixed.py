import subprocess, json, statistics, pathlib, os, hashlib, time
rows=[]
for i in range(10):
 cmd=['gdb','-q','-batch','-x','/tmp/grid_phase_full.gdb','--args','/repos/Antigravity/typst-crystalline/target/release/typst','compile','/tmp/grid80-fixed.typ',f'/tmp/grid80-fixed-{i}.pdf','--root','/tmp','--pdf-tagged=false','--pretty=false']
 p=subprocess.run(cmd,capture_output=True,text=True,env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1'))
 if p.returncode: raise RuntimeError(p.stdout+p.stderr)
 out=[x.removeprefix('GRID_PHASES ') for x in p.stdout.splitlines() if x.startswith('GRID_PHASES ')]
 if len(out)!=1: raise RuntimeError(p.stdout+p.stderr)
 spans=json.loads(out[0]); rows.append(spans)
summary={phase:statistics.median([x['ms'] for x in spans if x['phase']==phase][1] for spans in rows) for phase in ('columns','rows','emit')}
result={'utc':time.strftime('%Y-%m-%d %H:%M:%S UTC',time.gmtime()),'fixture_sha256':hashlib.sha256(pathlib.Path('/tmp/grid80-fixed.typ').read_bytes()).hexdigest(),'summary_second_pass_ms':summary,'samples':rows}
pathlib.Path('/tmp/grid_fixed_results.json').write_text(json.dumps(result,indent=2)+'\n')
print(summary)
