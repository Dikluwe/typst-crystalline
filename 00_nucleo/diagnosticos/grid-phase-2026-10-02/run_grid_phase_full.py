import json, pathlib, subprocess, hashlib, statistics, time, os
root=pathlib.Path('/repos/Antigravity/typst-crystalline')
fx=root/'00_nucleo/diagnosticos/grid-time-2026-10-02'
bin=root/'target/release/typst'
rows=[]
for n in (1,20,80):
    for i in range(10):
        pdf=pathlib.Path(f'/tmp/grid-phase-{n}-{i}.pdf')
        cmd=['gdb','-q','-batch','-x','/tmp/grid_phase_full.gdb','--args',str(bin),'compile',str(fx/f'grid{n}.typ'),str(pdf),'--root',str(fx),'--pdf-tagged=false','--pretty=false']
        p=subprocess.run(cmd,cwd=root,capture_output=True,text=True,env=dict(os.environ,SOURCE_DATE_EPOCH='0',RAYON_NUM_THREADS='1'))
        if p.returncode: raise RuntimeError(p.stdout+p.stderr)
        chunks=[line.removeprefix('GRID_PHASES ') for line in p.stdout.splitlines() if line.startswith('GRID_PHASES ')]
        if len(chunks)!=1: raise RuntimeError(p.stdout+p.stderr)
        spans=json.loads(chunks[0])
        if len(spans)!=6: raise RuntimeError(spans)
        rows.append({'fixture_rows':n,'sample':i,'spans':spans,'columns_ms':sum(x['ms'] for x in spans if x['phase']=='columns'),'rows_ms':sum(x['ms'] for x in spans if x['phase']=='rows'),'emit_ms':sum(x['ms'] for x in spans if x['phase']=='emit')})
summary={}
for n in (1,20,80):
    xs=[x for x in rows if x['fixture_rows']==n]
    summary[n]={name:{'median':statistics.median(x[name] for x in xs),'range':[min(x[name] for x in xs),max(x[name] for x in xs)]} for name in ('columns_ms','rows_ms','emit_ms')}
result={'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip(),'status':subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],cwd=root,text=True),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=root,text=True),'utc':time.strftime('%Y-%m-%d %H:%M:%S UTC',time.gmtime()),'binary_sha256':hashlib.sha256(bin.read_bytes()).hexdigest(),'gdb_script_sha256':hashlib.sha256(pathlib.Path('/tmp/grid_phase_full.gdb').read_bytes()).hexdigest(),'summary':summary,'samples':rows}
pathlib.Path('/tmp/grid_phase_full_results.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(summary,indent=2))
